use proc_macro::TokenStream;
use syn::{Data, DeriveInput, Field};

pub fn generate_domain_event(ast: DeriveInput) -> TokenStream {
    let identity = ast.ident;
    let generics = ast.generics;
    let fields: Vec<Field> = match ast.data {
        Data::Struct(data) => data.fields.into_iter().collect(),
        _ => panic!("Not a struct"),
    };

    let _ = fields
        .iter()
        .find(|field| field.ident.as_ref().unwrap() == "command_id")
        .expect("No command_id field found.");

    let _ = fields
        .iter()
        .find(|field| field.ident.as_ref().unwrap() == "environment")
        .expect("No environment field found.");

    let _ = fields
        .iter()
        .find(|field| field.ident.as_ref().unwrap() == "event_id")
        .expect("No event id field found.");

    let _ = fields
        .iter()
        .find(|field| field.ident.as_ref().unwrap() == "issuer_id")
        .expect("No issuer id field found.");

    let _ = fields
        .iter()
        .find(|field| field.ident.as_ref().unwrap() == "issued_at")
        .expect("No issued at field found.");

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    quote::quote!(

        impl #impl_generics ddd::traits::domain_event::DomainEvent #ty_generics for #identity #where_clause {

            fn command_id(&self) -> &ddd::structs::ids::CommandId {
                &self.command_id
            }

            fn environment(&self) -> &ddd::enums::environment::Environment {
                &self.environment
            }

            fn event_id(&self) -> &ddd::structs::ids::EventId {
                &self.event_id
            }

            fn issuer_id(&self) -> &ddd::structs::ids::IssuerId {
                &self.issuer_id
            }

            fn issued_at(&self) -> &chrono::DateTime<chrono::Utc> {
                &self.issued_at
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    )
    .into()
}
