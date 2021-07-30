use proc_macro::TokenStream;

use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Field, Fields, parse_macro_input};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { attrs: _, vis, ident, generics, data } = parse_macro_input!(input as DeriveInput);
    let data = if let Data::Struct(data) = data { data } else { return TokenStream::new(); };
    let fields = if let Fields::Named(fields) = data.fields { fields } else { return TokenStream::new(); };
    let field_names = fields.named.iter().map(|Field { ident, .. }| quote! {#ident});
    let field_setters = fields.named.iter().map(|Field { ident, ty, .. }| quote! {
        fn #ident(&mut self, item: #ty)->&mut Self {
            self.#ident = Some(item);
            self
        }
    });
    let option_fields = fields.named.iter().map(|Field { attrs, vis, ident, ty, .. }| quote! {#(#attrs)* #vis #ident : Option<#ty>});
    let builder_struct_name = format_ident!("{}Builder",&ident);

    let expanded = quote! {
        impl #generics #ident {
            #vis fn builder()->#builder_struct_name #generics{
                #builder_struct_name {
                    #(#field_names: None),*
                }
            }
        }

        #vis struct #builder_struct_name #generics {
            #(#option_fields),*
        }

        impl #generics #builder_struct_name {
            #(#vis #field_setters)*
        }
    };
    TokenStream::from(expanded)
}
