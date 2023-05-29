use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, Data, DeriveInput, Fields};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let original_struct_ident = &derive_input.ident;
    let builder_struct_name = std::format!("{}{}", derive_input.ident, "Builder");
    let builder_struct_ident = syn::Ident::new(&builder_struct_name, Span::call_site());

    let named_fields = match derive_input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(named_fields) => named_fields.named,
            _ => Punctuated::new(),
        },
        _ => Punctuated::new(),
    };

    let builder_fields: Vec<proc_macro2::TokenStream> = named_fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            let ty = &f.ty;
            quote! { #ident: Option<#ty> }
        })
        .collect();

    let constructor_fields: Vec<proc_macro2::TokenStream> = named_fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            quote! { #ident: None }
        })
        .collect();

    let expanded = quote! {
        impl #original_struct_ident {
            fn builder() {
                #builder_struct_ident {
                    #( #constructor_fields ),*
                }
            }
        }
        pub struct #builder_struct_ident {
            #( #builder_fields ),*
        }
    };

    TokenStream::from(expanded)
}
