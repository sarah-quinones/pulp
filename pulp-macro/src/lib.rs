use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::{Brace, Colon};
use syn::{FieldsNamed, FnArg, ItemFn};

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

    let mut struct_generics = Punctuated::new();
    let mut struct_generics_lifetimes = Vec::new();
    let mut struct_generics_names = Vec::new();
    let mut struct_field_names = Vec::new();
    let mut struct_fields = FieldsNamed {
        brace_token: Brace {
            span: sig.paren_token.span,
        },
        named: Punctuated::new(),
    };

    let mut first_non_lifetime = usize::MAX;
    for (idx, param) in sig.generics.params.clone().into_pairs().enumerate() {
        let (param, comma) = param.into_tuple();
        match &param {
            syn::GenericParam::Lifetime(_) => {}
            _ => {
                if first_non_lifetime == usize::MAX {
                    first_non_lifetime = idx;
                    continue;
                }
            }
        }
        match &param {
            syn::GenericParam::Type(ty) => struct_generics_names.push(ty.ident.clone()),
            syn::GenericParam::Lifetime(lt) => struct_generics_lifetimes.push(lt.lifetime.clone()),
            syn::GenericParam::Const(const_) => struct_generics_names.push(const_.ident.clone()),
        };
        struct_generics.push_value(param);
        if let Some(comma) = comma {
            struct_generics.push_punct(comma);
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
    new_fn_sig.inputs = new_fn_sig.inputs.into_iter().skip(1).collect();
    new_fn_sig.ident = name.clone();

    for param in sig.inputs.clone().into_pairs().skip(1) {
        let (param, comma) = param.into_tuple();
        let FnArg::Typed(param) = param.clone() else {
            return quote! {
                ::core::compile_error!(::core::concat!(
                    "pulp::with_simd only accepts free functions"
                ));
                #item
            }
            .into();
        };

        let name = *param.pat;
        let syn::Pat::Ident(name) = name else {
            return quote! {
                ::core::compile_error!(::core::concat!(
                        "pulp::with_simd requires function parameters to be idents"
                        ));
                #item
            }
            .into();
        };

        struct_field_names.push(name.ident.clone());
        let field = syn::Field {
            attrs: param.attrs,
            vis: syn::Visibility::Public(syn::token::Pub {
                span: proc_macro2::Span::call_site(),
            }),
            mutability: syn::FieldMutability::None,
            ident: Some(name.ident),
            colon_token: Some(Colon {
                spans: [proc_macro2::Span::call_site()],
            }),
            ty: *param.ty,
        };
        struct_fields.named.push_value(field);
        if let Some(comma) = comma {
            struct_fields.named.push_punct(comma);
        }
    }

    let output_ty = match sig.output.clone() {
        syn::ReturnType::Default => quote! { () },
        syn::ReturnType::Type(_, ty) => quote! { #ty },
    };

    let fn_name = sig.ident.clone();

    let arch = attr.value;

    quote! {
        #(#attrs)*
        #vis #new_fn_sig {
            #[allow(non_camel_case_types)]
            struct #name<#struct_generics> #struct_fields

            impl<#struct_generics> ::pulp::WithSimd for #name<#(#struct_generics_lifetimes,)*
    #(#struct_generics_names,)*> {             type Output = #output_ty;

                #[inline(always)]
                fn with_simd<__S: ::pulp::Simd>(self, __simd: __S) -> <Self as
    ::pulp::WithSimd>::Output {                 let Self { #(#struct_field_names,)* } = self;
                    #[allow(unused_unsafe)]
                    unsafe {
                        #fn_name::<__S,
                        #(#struct_generics_names,)*
                        >(__simd, #(#struct_field_names,)*)
                    }
                }
            }

            (#arch).dispatch( #name::<#(#struct_generics_names,)*> { #(#struct_field_names,)* } )
        }

        #(#attrs)*
        #vis #sig #block
    }
    .into()
}
