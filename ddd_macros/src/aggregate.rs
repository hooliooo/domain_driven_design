use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{Data, DeriveInput, Field, Ident, Type};
use crate::entity;

pub fn generate_aggregate(ast: DeriveInput) -> TokenStream {
    let entity_ast = ast.clone();
    let identity = ast.ident;

    let fields: Vec<Field> = match ast.data {
        Data::Struct(data) => data.fields.into_iter().collect(),
        _ => panic!("Not a struct"),
    };

    let id_field = fields
        .into_iter()
        .find(|field| field.ident.as_ref().unwrap() == "id")
        .expect("No id field found.");

    let generate_id_attribute = id_field
        .attrs
        .into_iter()
        .find(|attribute| attribute.path().is_ident("generate_id"));

    let generated_id_quote = match generate_id_attribute {
        Some(attribute) => {
            let id_type = attribute.parse_args::<Type>().unwrap();
            let id_identity_name = identity.to_string() + "Id";
            let id_identity = Ident::new(id_identity_name.as_str(), Span::call_site());
            quote::quote!(
                // Generate the ID struct of the struct
                #[derive(PartialEq, Eq, Hash, Clone, Debug)]
                pub struct #id_identity {
                    id: #id_type
                }

                impl #id_identity {
                    pub fn new(id: #id_type) -> Self {
                        Self { id }
                    }

                    pub fn value_as_ref(&self) -> &#id_type {
                        &self.id
                    }

                    pub fn value(self) -> #id_type {
                        self.id
                    }
                }

                impl From<#id_type> for #id_identity {
                    fn from(value: #id_type) -> Self {
                        Self { id: value }
                    }
                }

                impl ddd::traits::aggregate_id::AggregateId for #id_identity {}
            )
        }
        None => quote::quote!(),
    };
    let entity_quote: proc_macro2::TokenStream = entity::generate_entity(entity_ast).into();
    let identity_name = super::to_snake_case(identity.clone().to_string());

    quote::quote!(
        #generated_id_quote

        impl ddd::traits::aggregate::Aggregate for #identity {

            fn type_name() -> &'static str {
                #identity_name
            }

        }

        #entity_quote
    )
    .into()
}
