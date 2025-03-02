use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use syn::{Data, DataEnum, DataStruct, DeriveInput, Field, Fields};
use crate::FIELD_ATTR;
use crate::generate_fields::generate_fields;


pub fn generate_value_object(ast: DeriveInput) -> TokenStream {
    let identity = ast.ident;

    match ast.data {
        Data::Struct(data) => generate_value_object_for_struct(&identity, data),
        Data::Enum(data) => generate_value_object_for_enum(&identity, data),
        _ => panic!("Not a struct"),
    }
}

pub fn generate_value_object_for_struct(identity: &Ident, data: DataStruct) -> TokenStream {
    let fields: Vec<Field> = data.fields.into_iter().collect();
    let MetaData(filtered_fields, field) = fields.into_iter()
        .fold(MetaData(Vec::default(), Vec::default()), |mut data, field| {
            if field.attrs.iter().any(|attr| attr.path().is_ident(FIELD_ATTR)) {
                data.0.push(field.clone())
            }

            if let Some(ident) = field.ident.map(|x| x.clone()) {
                data.1.push(ident)
            }
            data
        });

    let getters = generate_fields(&identity, filtered_fields);

    quote::quote!(
        impl ddd::traits::value_object::ValueObject for #identity {}

        impl Clone for #identity {
            fn clone(&self) -> Self {
                Self {
                    #(#field: self.#field.clone()),*
                }
            }
        }

        impl PartialEq for #identity {
            fn eq(&self, other: &Self) -> bool {
                true #( && self.#field == other.#field)*
            }
        }

        impl Eq for #identity {}

        impl std::hash::Hash for #identity {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                #(self.#field.hash(state);)*
            }
        }

        #getters

    )
        .into()
}

struct MetaData(Vec<Field>, Vec<Ident>);
struct VariantQuote { clone_quote: proc_macro2::TokenStream, partial_eq_quote: proc_macro2:: TokenStream, hash_quote: proc_macro2::TokenStream }

fn generate_value_object_for_enum(identity: &Ident, data: DataEnum) -> TokenStream {
    let variant_quotes: Vec<_> = data.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        match &variant.fields {
            Fields::Named(named) => {
                // Struct-like variant
                let fields: Vec<_> = named.named.iter()
                    .map(|f| f.ident.as_ref().unwrap().clone())
                    .collect();

                let clone_quote = quote::quote!(
                    #identity::#variant_name { #(#fields),* } => #identity::#variant_name { #(#fields: #fields.clone()),* }
                );

                let partial_eq_fields_lhs: Vec<Ident> = fields.iter().map( |f| {
                    Ident::new(&format!("lhs_{}", f), Span::call_site())
                }).collect();

                let partial_eq_fields_rhs: Vec<Ident> = fields.iter().map( |f| {
                    Ident::new(&format!("rhs_{}", f), Span::call_site())
                }).collect();

                let partial_eq_quote = quote::quote!(
                    (
                        #identity::#variant_name { #(#fields: #partial_eq_fields_lhs),* },
                        #identity::#variant_name { #(#fields: #partial_eq_fields_rhs),* }
                    ) => true #(&& #partial_eq_fields_lhs == #partial_eq_fields_rhs)*
                );

                let hash_quote = quote::quote! (
                    #identity::#variant_name { #(#fields),* } => {
                        #(#fields.hash(state);)*
                    }
                );

                VariantQuote { clone_quote, partial_eq_quote, hash_quote }
            }

            Fields::Unnamed(unnamed) => {
                // Tuple variant
                let fields: Vec<_> = (0..unnamed.unnamed.len())
                    .map(|i| Ident::new(&format!("field{}", i), Span::call_site()))
                    .collect();

                let clone_quote = quote::quote!(
                    #identity::#variant_name(#(#fields),*) => #identity::#variant_name(#(#fields.clone()),*)
                );

                let partial_eq_fields_lhs: Vec<Ident> = fields.iter().map( |f| {
                    Ident::new(&format!("lhs_{}", f), Span::call_site())
                }).collect();

                let partial_eq_fields_rhs: Vec<Ident> = fields.iter().map( |f| {
                    Ident::new(&format!("rhs_{}", f), Span::call_site())
                }).collect();

                let partial_eq_quote = quote::quote!(
                    (
                        #identity::#variant_name(#(#partial_eq_fields_lhs),*),
                        #identity::#variant_name(#(#partial_eq_fields_rhs),*)
                    ) => true #(&& #partial_eq_fields_lhs == #partial_eq_fields_rhs)*
                );

                let hash_quote = quote::quote!(
                    #identity::#variant_name(#(#fields),*) => {
                        #(#fields.hash(state);)*
                    }
                );

                VariantQuote { clone_quote, partial_eq_quote, hash_quote }
            }

            Fields::Unit => {
                // Unit variant (no fields)
                let clone_quote = quote::quote!(
                    #identity::#variant_name => #identity::#variant_name
                );

                let partial_eq_quote = quote::quote!(
                    (#identity::#variant_name, #identity::#variant_name) => true
                );

                let hash_quote = quote::quote!(
                    #identity::#variant_name => {
                        std::mem::discriminant(self).hash(state);
                    }
                );

                VariantQuote { clone_quote, partial_eq_quote, hash_quote }
            }
        }
    }).collect();

    let (clone_quotes, partial_eq_quotes, hash_quotes) = variant_quotes.into_iter().fold(
        (Vec::default(), Vec::default(), Vec::default()), |mut curr, element| {
            curr.0.push(element.clone_quote);
            curr.1.push(element.partial_eq_quote);
            curr.2.push(element.hash_quote);
            curr
        });

    quote::quote!(
        impl ddd::traits::value_object::ValueObject for #identity {}

        impl Clone for #identity {
            fn clone(&self) -> Self {
                match self {
                     #(#clone_quotes),*
                }
            }
        }

        impl PartialEq for #identity {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    #(#partial_eq_quotes),*,
                    _ => false,
                }
            }
        }

        impl Eq for #identity {}

        impl std::hash::Hash for #identity {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                match self {
                    #(#hash_quotes),*
                }
            }
        }
    ).into()
}

