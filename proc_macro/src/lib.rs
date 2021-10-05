extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};

#[proc_macro_derive(New)]
pub fn derive_new(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    let field_name = fields.iter().map(|field| &field.ident);

    (quote! {
        impl tools::New for #name {
            fn new() -> Self {
                Self {
                    #(
                    #field_name: tools::new(),
                    )*
                }
            }
        }
    })
    .into()
}

#[proc_macro_derive(Boxed)]
pub fn derive_boxed(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    let field_name = fields.iter().map(|field| &field.ident);

    (quote! {
        impl tools::Boxed for #name {
            fn boxed() -> Box<Self> {
                Box::new(Self {
                    #(
                    #field_name: tools::new(),
                    )*
                })
            }
        }
    })
    .into()
}
