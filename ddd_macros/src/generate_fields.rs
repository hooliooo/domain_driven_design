use proc_macro2::{Ident, TokenStream};
use syn::Field;
use crate::FIELD_ATTR;

pub fn generate_fields(identity: &Ident, fields: Vec<Field>) -> TokenStream {

    let getters = fields
        .into_iter()
        .filter(|field| {
            field
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident(FIELD_ATTR))
        })
        .map(|field| {
            let field_ident = field.ident.as_ref().unwrap();
            let ty = &field.ty;
            quote::quote!(
                pub fn #field_ident(&self) -> &#ty {
                    &self.#field_ident
                }
            )
        });

    quote::quote!(
        impl #identity {
            #(#getters)*
        }
    )

}