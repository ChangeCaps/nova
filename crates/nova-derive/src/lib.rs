mod derive_inspectable;
mod derive_vertex;

#[proc_macro_derive(Vertex)]
pub fn derive_vertex(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_vertex::derive_vertex(input)
}

#[proc_macro_derive(Inspectable, attributes(inspectable))]
pub fn derive_inspectable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_inspectable::derive_inspectable(input)
}
