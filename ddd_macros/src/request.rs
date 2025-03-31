use proc_macro::TokenStream;
use syn::{Data, DeriveInput, Field};

pub fn generate_request(ast: DeriveInput) -> TokenStream {
    let identity = ast.ident;
    let fields: Vec<Field> = match ast.data {
        Data::Struct(data) => data.fields.into_iter().collect(),
        _ => panic!("Not a struct"),
    };

    let _ = fields
        .iter()
        .find(|field| field.ident.as_ref().unwrap() == "request_id")
        .expect("No command_id field found.");

    let _ = fields
        .iter()
        .find(|field| field.ident.as_ref().unwrap() == "environment")
        .expect("No environment field found.");

    let issuer_id_field = fields
        .iter()
        .find(|field| field.ident.as_ref().unwrap() == "issuer_id")
        .expect("No issuer id field found.");

    let issuer_id_type = &issuer_id_field.ty;

    let _ = fields
        .iter()
        .find(|field| field.ident.as_ref().unwrap() == "issued_at")
        .expect("No issued at field found.");

    quote::quote!(

        impl <'a> ddd::traits::request::Request<'a> for #identity {
            type RequestId = ddd::structs::ids::RequestId;
            type IssuerId = #issuer_id_type;

            fn request_id(&'a self) -> &'a Self::RequestId {
                &self.request_id
            }

            fn environment(&'a self) -> &'a ddd::enums::environment::Environment {
                &self.environment
            }

            fn issuer_id(&'a self) -> &'a Self::IssuerId {
                &self.issuer_id
            }

            fn issued_at(&'a self) -> &'a chrono::DateTime<chrono::Utc> {
                &self.issued_at
            }

        }
    ).into()

}
