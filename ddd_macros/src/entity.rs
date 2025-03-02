use proc_macro::TokenStream;
use syn::{Data, DeriveInput, Field};
use crate::{ENTITY_ID_ATTR, FIELD_ATTR};
use crate::generate_fields::generate_fields;

pub fn generate_entity(ast: DeriveInput) -> TokenStream {
    let identity = ast.ident;
    let fields: Vec<Field> = match ast.data {
        Data::Struct(data) => data.fields.into_iter().collect(),
        _ => panic!("Not a struct"),
    };

    struct Fields(Option<Field>, Vec<Field>);
    let Fields(id_field, filtered_fields) = fields
        .into_iter()
        .fold(Fields(None, Vec::new()), |mut fields, field| {
            if field.attrs.iter().any(|attr| attr.path().is_ident(ENTITY_ID_ATTR)) {
                fields.0 = Some(field)
            } else if field.attrs.iter().any(|attr| attr.path().is_ident(FIELD_ATTR)) {
                fields.1.push(field)
            }
            fields
        });
    let id_field = id_field.expect("Missing `id` field");
    let id_field_type = id_field.ty;

    let getters = generate_fields(&identity, filtered_fields);

    quote::quote!(

        impl ddd::traits::entity::Entity for #identity {
            type Id = #id_field_type;

            fn id(&self) -> &Self::Id {
                &self.id
            }
        }

        impl PartialEq for #identity {
            fn eq(&self, other: &Self) -> bool {
                self.id() == other.id()
            }
        }

        impl Eq for #identity {}

        impl std::hash::Hash for #identity {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.id.hash(state);
            }
        }

        #getters

    )
    .into()
}
