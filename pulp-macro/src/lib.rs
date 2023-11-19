use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::{Colon, PathSep};
use syn::{
    ConstParam, FnArg, GenericParam, ItemFn, LifetimeParam, Pat, PatIdent, PatType, Path,
    PathSegment, Type, TypeParam, TypePath,
};

#[proc_macro_attribute]
pub fn with_simd(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr: TokenStream = attr.into();
    let item: TokenStream = item.into();
    let Ok(syn::Meta::NameValue(attr)) = syn::parse2::<syn::Meta>(attr.clone()) else {
        return quote! {
            ::core::compile_error!("pulp::with_simd expected function name and arch expression");
            #item
        }
        .into();
    };
    let Some(name) = attr.path.get_ident() else {
        return quote! {
            ::core::compile_error!("pulp::with_simd expected function name and arch expression");
            #item
        }
        .into();
    };
    let Ok(item) = syn::parse2::<syn::ItemFn>(item.clone()) else {
        return quote! {
            ::core::compile_error!("pulp::with_simd expected function");
            #item
        }
        .into();
    };

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = item.clone();

    let mut struct_generics = Vec::new();
    let mut struct_field_names = Vec::new();
    let mut struct_field_types = Vec::new();

    let mut first_non_lifetime = usize::MAX;
    for (idx, param) in sig.generics.params.clone().into_pairs().enumerate() {
        let (param, _) = param.into_tuple();
        match &param {
            syn::GenericParam::Lifetime(_) => {}
            _ => {
                if first_non_lifetime == usize::MAX {
                    first_non_lifetime = idx;
                    continue;
                }
            }
        }
    }
    let mut new_fn_sig = sig.clone();
    new_fn_sig.generics.params = new_fn_sig
        .generics
        .params
        .into_iter()
        .enumerate()
        .filter(|(idx, _)| *idx != first_non_lifetime)
        .map(|(_, arg)| arg)
        .collect();
    new_fn_sig.inputs = new_fn_sig
        .inputs
        .into_iter()
        .skip(1)
        .enumerate()
        .map(|(idx, arg)| {
            FnArg::Typed(PatType {
                attrs: Vec::new(),
                pat: Box::new(Pat::Ident(PatIdent {
                    attrs: Vec::new(),
                    by_ref: None,
                    mutability: None,
                    ident: Ident::new(&format!("__{idx}"), Span::call_site()),
                    subpat: None,
                })),
                colon_token: Colon {
                    spans: [Span::call_site()],
                },
                ty: match arg {
                    FnArg::Typed(ty) => ty.ty,
                    FnArg::Receiver(_) => panic!(),
                },
            })
        })
        .collect();
    new_fn_sig.ident = name.clone();
    let mut param_ty = Vec::new();

    for (idx, param) in new_fn_sig.inputs.clone().into_pairs().enumerate() {
        let (param, _) = param.into_tuple();
        let FnArg::Typed(param) = param.clone() else {
            panic!();
        };
        let name = *param.pat;
        let syn::Pat::Ident(name) = name else {
            panic!();
        };

        let anon_ty = Ident::new(&format!("__T{idx}"), Span::call_site());

        struct_field_names.push(name.ident.clone());
        let mut ty = Punctuated::<_, PathSep>::new();
        ty.push_value(PathSegment {
            ident: anon_ty.clone(),
            arguments: syn::PathArguments::None,
        });
        struct_field_types.push(Type::Path(TypePath {
            qself: None,
            path: Path {
                leading_colon: None,
                segments: ty,
            },
        }));
        struct_generics.push(anon_ty);
        param_ty.push(*param.ty);
    }

    let output_ty = match sig.output.clone() {
        syn::ReturnType::Default => quote! { () },
        syn::ReturnType::Type(_, ty) => quote! { #ty },
    };

    let fn_name = sig.ident.clone();

    let arch = attr.value;
    let new_fn_generics = new_fn_sig.generics.clone();
    let params = new_fn_generics.params.clone();
    let generics = params.into_iter().collect::<Vec<_>>();
    let non_lt_generics_names = generics
        .iter()
        .map(|p| match p {
            GenericParam::Type(TypeParam { ident, .. })
            | GenericParam::Const(ConstParam { ident, .. }) => {
                quote! { #ident, }
            }
            _ => quote! {},
        })
        .collect::<Vec<_>>();
    let generics_decl = generics
        .iter()
        .map(|p| match p {
            GenericParam::Lifetime(LifetimeParam {
                lifetime,
                colon_token,
                bounds,
                ..
            }) => {
                quote! { #lifetime #colon_token #bounds }
            }
            GenericParam::Type(TypeParam {
                ident,
                colon_token,
                bounds,
                ..
            }) => {
                quote! { #ident #colon_token #bounds }
            }
            GenericParam::Const(ConstParam {
                const_token,
                ident,
                colon_token,
                ty,
                ..
            }) => {
                quote! { #const_token #ident #colon_token #ty }
            }
        })
        .collect::<Vec<_>>();
    let generics_where_clause = new_fn_generics.where_clause;

    let code = quote! {
        #(#attrs)*
        #vis #new_fn_sig {
            #[allow(non_camel_case_types)]
            struct #name<#(#struct_generics,)*> (#(#struct_field_types,)*);

            impl<#(#generics_decl,)*> ::pulp::WithSimd for #name<
                #(#param_ty,)*
            > #generics_where_clause {
                type Output = #output_ty;

                #[inline(always)]
                fn with_simd<__S: ::pulp::Simd>(self, __simd: __S) -> <Self as ::pulp::WithSimd>::Output {
                    let Self ( #(#struct_field_names,)* ) = self;
                    #[allow(unused_unsafe)]
                    unsafe {
                        #fn_name::<__S,
                        #(#non_lt_generics_names)*
                        >(__simd, #(#struct_field_names,)*)
                    }
                }
            }

            (#arch).dispatch( #name ( #(#struct_field_names,)* ) )
        }

        #(#attrs)*
        #vis #sig #block
    };
    code.into()
}
