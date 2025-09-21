use proc_macro::TokenStream;
use syn::{Data, DeriveInput, Field};

pub fn generate_request(ast: DeriveInput) -> TokenStream {
    let identity = ast.ident;
    let generics = ast.generics;
    let fields: Vec<Field> = match ast.data {
        Data::Struct(data) => data.fields.into_iter().collect(),
        _ => panic!("Not a struct"),
    };

    let _ = fields
        .iter()
        .find(|field| field.ident.as_ref().unwrap() == "request_id")
        .expect("No 'request_id' field found.");

    let _ = fields
        .iter()
        .find(|field| field.ident.as_ref().unwrap() == "environment")
        .expect("No 'environment' field found.");

    let issuer_id_field = fields
        .iter()
        .find(|field| field.ident.as_ref().unwrap() == "issuer_id")
        .expect("No 'issuer_id' field found.");

    let issuer_id_type = &issuer_id_field.ty;

    let _ = fields
        .iter()
        .find(|field| field.ident.as_ref().unwrap() == "issued_at")
        .expect("No 'issued_at' field found.");

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    quote::quote!(

        impl #impl_generics ddd::traits::request::Request #ty_generics for #identity #where_clause {
            type RequestId = ddd::structs::ids::RequestId;
            type IssuerId = #issuer_id_type;

            fn request_id(&self) -> &Self::RequestId {
                &self.request_id
            }

            fn environment(&self) -> &ddd::enums::environment::Environment {
                &self.environment
            }

            fn issuer_id(&self) -> &Self::IssuerId {
                &self.issuer_id
            }

            fn issued_at(&self) -> &chrono::DateTime<chrono::Utc> {
                &self.issued_at
            }
        }
    )
    .into()
}

pub fn generate_authenticated_request(ast: DeriveInput) -> TokenStream {
    let ast_clone = ast.clone();
    let identity = ast.ident;
    let generics = ast.generics;
    let fields: Vec<Field> = match ast.data {
        Data::Struct(data) => data.fields.into_iter().collect(),
        _ => panic!("Not a struct"),
    };

    let _ = fields
        .iter()
        .find(|field| field.ident.as_ref().unwrap() == "user_id")
        .expect("No 'user_id' field found.");

    let _ = fields
        .iter()
        .find(|field| field.ident.as_ref().unwrap() == "roles")
        .expect("No 'roles' field found.");

    let request_token_stream: proc_macro2::TokenStream = generate_request(ast_clone).into();

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    quote::quote!(

        #request_token_stream

        impl #impl_generics ddd::traits::request::AuthenticatedRequest #ty_generics for #identity #where_clause {

            fn user_id(&self) -> &ddd::structs::ids::UserId {
                &self.user_id
            }

            fn roles(&self) -> &std::collections::HashSet<ddd::structs::role::Role> {
                &self.roles
            }
        }
    )
    .into()
}
