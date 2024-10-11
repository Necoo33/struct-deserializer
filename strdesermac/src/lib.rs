extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(DeserializeStruct)]
pub fn reconstruct_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields = if let Data::Struct(data) = input.data {
        if let Fields::Named(fields) = data.fields {
            fields.named
        } else {
            panic!("DeserializeStruct can only be used with named fields");
        }
    } else {
        panic!("DeserializeStruct can only be used with structs");
    };

    let field_names = fields.iter().map(|f| &f.ident);
    let field_types = fields.iter().map(|f| &f.ty);

    let expanded = quote! {
        impl Deserializable for #name {
            fn deserialize_struct(fields: Vec<(String, String, String)>) -> Self {
                let mut map = std::collections::HashMap::new();
                for (field, value, ty) in fields {
                    map.insert(field, (value, ty));
                }

                #name {
                    #(
                        #field_names: {
                            let (value, ty) = map.get(stringify!(#field_names)).unwrap();
                            match ty.as_str() {
                                stringify!(#field_types) => value.parse::<#field_types>().unwrap(),
                                _ => panic!("Field type mismatch"),
                            }
                        },
                    )*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
