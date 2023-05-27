use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let original_struct_ident = &derive_input.ident;
    let builder_struct_name = std::format!("{}{}", derive_input.ident, "Builder");
    let builder_struct_ident = syn::Ident::new(&builder_struct_name, Span::call_site());

    let expanded = quote! {
        impl #original_struct_ident {
            fn builder() {}
        }
        struct #builder_struct_ident {

        }
    };

    TokenStream::from(expanded)
}
