use proc_macro::TokenStream;
use syn::{Data, DeriveInput};

pub fn generate_domain_event(ast: DeriveInput) -> TokenStream {
    let identity = ast.ident;
    let generics = ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // 1. Extract the concrete type of 'aggregate_id' for the Associated Type
    let agg_id_type = match &ast.data {
        Data::Struct(s) => s
            .fields
            .iter()
            .find(|f| f.ident.as_ref().unwrap() == "aggregate_id")
            .map(|f| &f.ty)
            .expect("Struct must have 'aggregate_id' field"),
        Data::Enum(e) => {
            let first_var = e.variants.first().expect("Enum must have variants");
            first_var
                .fields
                .iter()
                .find(|f| f.ident.as_ref().unwrap() == "aggregate_id")
                .map(|f| &f.ty)
                .expect("Enum variants must have 'aggregate_id' field")
        }
        _ => panic!("Only structs and enums are supported"),
    };

    // 2. Generate the logic for each method
    let (id_body, agg_id_body, agg_ver_body, occurred_body) = match &ast.data {
        Data::Struct(_) => (
            quote::quote!(&self.id),
            quote::quote!(&self.aggregate_id),
            quote::quote!(self.aggregate_version),
            quote::quote!(&self.occurred_at),
        ),
        Data::Enum(data_enum) => {
            let variants: Vec<&syn::Ident> = data_enum.variants.iter().map(|v| &v.ident).collect();
            (
                quote::quote!(match self { #( #identity::#variants { id, .. } => id, )* }),
                quote::quote!(match self { #( #identity::#variants { aggregate_id, .. } => aggregate_id, )* }),
                quote::quote!(match self { #( #identity::#variants { aggregate_version, .. } => *aggregate_version, )* }),
                quote::quote!(match self { #( #identity::#variants { occurred_at, .. } => occurred_at, )* }),
            )
        }
        _ => unreachable!(),
    };

    quote::quote!(
        impl #impl_generics kern::building_blocks::domain_event::DomainEvent for #identity #ty_generics #where_clause where Self: Send + Sync + 'static {
            type Id = #agg_id_type;

            fn id(&self) -> &kern::building_blocks::ids::EventId {
                #id_body
            }

            fn aggregate_id(&self) -> &Self::Id {
                #agg_id_body
            }

            fn aggregate_version(&self) -> u32 {
                #agg_ver_body
            }

            fn occurred_at(&self) -> &chrono::DateTime<chrono::Utc> {
                #occurred_body
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    ).into()
}
