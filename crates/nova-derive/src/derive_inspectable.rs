use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::{quote, quote_spanned};
use syn::{
    parse::ParseStream, parse_macro_input, spanned::Spanned, Attribute, Data, DeriveInput, Fields,
    Index,
};

fn crate_path() -> TokenStream {
    if let Ok(FoundCrate::Name(name)) = crate_name("nova-inspect") {
        let ident = Ident::new(&name, Span::call_site());

        quote! {
            #ident
        }
    } else if let Ok(FoundCrate::Name(name)) = crate_name("nova-game") {
        let ident = Ident::new(&name, Span::call_site());

        quote! {
            #ident::nova_inspect
        }
    } else {
        panic!("could not find nova-inspect or nova-game");
    }
}

#[derive(Default)]
struct Attrs {
    ignore: bool,
}

impl Attrs {
    const NAME: &'static str = "inspectable";

    fn parse(attrs: &[Attribute]) -> Self {
        attrs
            .iter()
            .find(|a| *a.path.get_ident().as_ref().unwrap() == Self::NAME)
            .map_or_else(Self::default, |a| {
                syn::custom_keyword!(ignore);
                let mut attrs = Self::default();

                a.parse_args_with(|input: ParseStream| {
                    if input.parse::<Option<ignore>>()?.is_some() {
                        attrs.ignore = true;
                    }

                    Ok(())
                })
                .expect("Invalid 'inspectable' attr format.");

                attrs
            })
    }
}

pub fn derive_inspectable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let inspect_fields = inspect_fields(input.data);

    let crate_path = crate_path();

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl<#impl_generics> #crate_path::Inspectable for #name #ty_generics #where_clause {
            #[inline]
            fn name(&self) -> &'static str {
                stringify!(#name)
            }

            #[inline]
            fn inspect(&mut self, ui: &mut #crate_path::egui::Ui) -> Option<#crate_path::egui::Response> {
                #inspect_fields
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn inspect_fields(data: Data) -> TokenStream {
    match data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(named) => {
                let mut fields = named.named.iter().filter_map(|f| {
                    let attrs = Attrs::parse(&f.attrs);

                    let ident = f.ident.clone().unwrap();

                    if attrs.ignore {
                        None
                    } else {
                        Some(quote_spanned! {f.span()=>
                            self.#ident.inspect(ui)?
                        })
                    }
                });

                let field = fields.next().unwrap();

                quote! {
                    Some(#field #(| #fields)*)
                }
            }
            Fields::Unnamed(unnamed) => {
                let mut index = 0u32;

                let mut fields = unnamed.unnamed.iter().map(|f| {
                    let idx = Index {
                        index,
                        span: f.span(),
                    };

                    index += 1;

                    quote_spanned! {f.span()=>
                        self.#idx.inspect(ui)?
                    }
                });

                let field = fields.next().unwrap();

                quote! {
                    Some(#field #(| #fields)*)
                }
            }
            Fields::Unit => {
                quote! { None }
            }
        },
        Data::Enum(data) => {
            let variants = data.variants.iter().map(|var| {
                let ident = &var.ident;
                let mut named = None;

                let mut index = 0u32;
                let pat = var.fields.iter().map(|f| {
                    named = Some(f.ident.is_some());

                    if let Some(ident) = &f.ident {
                        quote_spanned! {f.span()=>
                            #ident,
                        }
                    } else {
                        let ident = Ident::new(&format!("i{}", index), f.span());

                        index += 1;

                        quote_spanned! {f.span()=>
                            #ident,
                        }
                    }
                });
                let pat = pat.collect::<Vec<_>>();

                let mut index = 0u32;
                let mut fields = var.fields.iter().map(|f| {
                    if let Some(ident) = &f.ident {
                        quote_spanned! {f.span()=>
                            #ident.inspect(ui)
                        }
                    } else {
                        let ident = Ident::new(&format!("i{}", index), f.span());

                        index += 1;

                        quote_spanned! {f.span()=>
                            #ident.inspect(ui)
                        }
                    }
                });

                match named.clone() {
                    Some(true) => {
                        let field = fields.next().unwrap();

                        quote! {
                            Self::#ident { #(#pat)* } => {
                                Some(#field? #(| #fields?)*)
                            }
                        }
                    }
                    Some(false) => {
                        let field = fields.next().unwrap();

                        quote! {
                            Self::#ident(#(#pat)*) => {
                                Some(#field? #(| #fields?)*)
                            }
                        }
                    }
                    None => quote! {
                        Self::#ident => {
                            None
                        }
                    },
                }
            });

            quote! {
                match self {
                    #(#variants)*
                }
            }
        }
        _ => unimplemented!(),
    }
}
