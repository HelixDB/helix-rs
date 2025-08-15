use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::{Parse, ParseStream}, parse_macro_input, Data, DataStruct, DeriveInput, Fields, LitStr};

/// Derive macro for HelixDB query input structures.
/// 
/// # Example
/// ```no_run
/// use helix_rs::HelixQuery;
/// 
/// #[derive(HelixQuery)]
/// #[helix(endpoint = "add_user")]
/// struct AddUserQuery {
///     name: String,
///     age: i32,
/// }
/// ```
#[proc_macro_derive(TraversalValue)]
pub fn derive_traversal_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fields = match &input.data {
        Data::Struct(DataStruct { fields, .. }) => fields.iter(),
        _ => panic!("TraversalValue can only be derived for structs"),
    };
    
    let expanded = quote! {
        pub struct #name {
            id: String,
            #(#fields),*
        }
    };

    TokenStream::from(expanded)
}
