use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Fields};

pub fn derive_vertex(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let vertex = vertex(&input.data);

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics nova_render::Vertex for #name #ty_generics #where_clause {
            fn layout() -> nova_wgpu::VertexBufferLayout<'static> {
                #vertex
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn vertex(data: &Data) -> TokenStream {
    match data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(named) => {
                let mut last_field = None;

                let fields = named.named.iter().map(|f| {
                    let name = f.ident.as_ref().unwrap().to_string();
                    let ty = &f.ty;

                    let format_ident = Ident::new(&format!("FORMAT_{}", &name), f.span());
                    let offset_ident = Ident::new(&format!("OFFSET_{}", &name), f.span());
                    let size_ident = Ident::new(&format!("SIZE_{}", &name), f.span());

                    let size = if let Some(last_field) = &last_field {
                        let field = Ident::new(&format!("SIZE_{}", last_field), f.span());

                        quote_spanned! {f.span()=>
                            <#ty>::SIZE + #field
                        }
                    } else {
                        quote_spanned! {f.span()=>
                            <#ty>::SIZE
                        }
                    };

                    let offset = if let Some(last_field) = &last_field {
                        let size = Ident::new(&format!("SIZE_{}", last_field), f.span());
                        quote_spanned! {f.span()=>
                            #size
                        }
                    } else {
                        quote!(0u64)
                    };

                    last_field = Some(name);

                    quote_spanned! {f.span()=>
                        #[allow(non_upper_case_globals)]
                        const #format_ident: nova_wgpu::VertexFormat = <#ty>::FORMAT;
                        #[allow(non_upper_case_globals)]
                        const #offset_ident: u64 = #offset;
                        #[allow(non_upper_case_globals)]
                        const #size_ident: u64 = #size;

                        {struct _VertexFormatVerify where #ty: nova_render::vertex::AsVertexFormat;}
                    }
                });

                let fields = quote!(#(#fields)*);

                let size = last_field.as_ref().map_or(quote!(0), |field| {
                    let offset = Ident::new(&format!("SIZE_{}", field), Span::call_site());
                    quote! {#offset}
                });

                let mut shader_location: u32 = 0;

                let attributes = named.named.iter().map(|f| {
                    let name = f.ident.as_ref().unwrap().to_string();

                    let format = Ident::new(&format!("FORMAT_{}", &name), f.span());
                    let offset = Ident::new(&format!("OFFSET_{}", &name), f.span());

                    let quote = quote_spanned! {f.span()=>
                        nova_wgpu::VertexAttribute {
                            format: #format,
                            offset: #offset,
                            shader_location: #shader_location,
                        }
                    };

                    shader_location += 1;

                    quote
                });

                quote! {
                    use nova_render::vertex::AsVertexFormat;

                    #fields

                    nova_wgpu::VertexBufferLayout {
                        array_stride: #size,
                        step_mode: nova_wgpu::InputStepMode::Vertex,
                        attributes: &[
                            #(#attributes,)*
                        ],
                    }
                }
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}
