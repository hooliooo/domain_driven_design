use proc_macro::TokenStream;
use syn::DeriveInput;

mod aggregate;
mod entity;
mod value_object;
mod generate_fields;

/// Generates the required methods for the Aggregate struct
///
/// Generates the required equality and hashing logic that follows the `Entity` semantics
/// in the domain-driven design context
///
/// Add the `field` attributes to the properties you want to generate getters for
#[proc_macro_derive(Aggregate, attributes(generate_id, entity_id, field))]
pub fn aggregate_macro(item: TokenStream) -> TokenStream {
    // parse
    let ast: DeriveInput = syn::parse_macro_input!(item as DeriveInput);
    // generate
    aggregate::generate_aggregate(ast)
}

/// Generates the required equality and hashing logic that follows the `Entity` semantics
/// in the domain-driven design context
///
/// Add the `field` attributes to the properties you want to generate getters for
///
/// Make sure to import ddd::Entity and ddd::traits::entity::Entity
#[proc_macro_derive(Entity, attributes(entity_id, field))]
pub fn entity_macro(item: TokenStream) -> TokenStream {
    // parse
    let ast: DeriveInput = syn::parse_macro_input!(item as DeriveInput);
    // generate
    entity::generate_entity(ast)
}

/// Generates the required equality and hashing logic that follows the `Value Object` semantics
/// in the domain-driven design context
///
/// Add the `field` attributes to the properties you want to generate getters for
///
/// Make sure to import ddd::ValueObject and ddd::traits::value_object::ValueObject
#[proc_macro_derive(ValueObject, attributes(field))]
pub fn value_object_macro(item: TokenStream) -> TokenStream {
    // parse
    let ast: DeriveInput = syn::parse_macro_input!(item as DeriveInput);
    // generate
    value_object::generate_value_object(ast)
}

/// Turns a string into snake case
fn to_snake_case(name: String) -> String {
    let mut snake_case = String::new();
    let mut chars = name.chars().peekable();

    while let Some(c) = chars.next() {
        if c.is_uppercase() {
            if let Some(next_char) = chars.peek() {
                if !snake_case.is_empty() && next_char.is_lowercase() {
                    snake_case.push('_')
                }
            }
            snake_case.push(c.to_ascii_lowercase())
        } else {
            snake_case.push(c)
        }
    }
    snake_case
}

const FIELD_ATTR: &str  = "field";
const ENTITY_ID_ATTR: &str = "entity_id";
