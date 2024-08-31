extern crate proc_macro2;

use proc_macro::TokenStream;
use quote::quote;
use syn::Data;

#[proc_macro_derive(MessageParsers)]
pub fn message_parsers_fn(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);

    let struct_identifier = &input.ident;

    match &input.data {
        Data::Struct(syn::DataStruct { fields, .. }) => {
            let mut implementation = quote! {};
            let mut values = quote! {};
            for field in fields {
                let identifier = field.ident.as_ref().unwrap();
                let ty = &field.ty;
                implementation.extend(quote! {
                    let #identifier = #ty;
                });
                values.extend(quote! {
                    #identifier,
                });
            }

            quote! {
                #[automatically_derived]
                impl #struct_identifier {
                    fn from_bytes(src: &BytesMut) -> Result<Self, DecodeError> {
                        #implementation
                        Ok(Self {
                            #values
                        })
                    }
                }
            }
        }
        _ => unimplemented!(),
    }
    .into()
}
