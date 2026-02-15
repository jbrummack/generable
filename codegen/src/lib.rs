use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Field, Fields, parse_macro_input};
#[allow(unused)]
fn extract_description(input: &Field) -> Option<String> {
    input
        .attrs
        .iter()
        .find(|a| a.path().is_ident("description"))
        .and_then(|attr| attr.parse_args::<syn::LitStr>().ok())
        .map(|lit| lit.value())
}
#[allow(unused)]
fn extract_parent_description(input: &DeriveInput) -> Option<String> {
    input
        .attrs
        .iter()
        .find(|a| a.path().is_ident("description"))
        .and_then(|attr| attr.parse_args::<syn::LitStr>().ok())
        .map(|lit| lit.value())
}

#[proc_macro_derive(DynamicGenerable, attributes(description))]
pub fn derive_dynamic_generable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let schema_impl = match &input.data {
        Data::Struct(data_struct) => {
            let fields_gen = generate_fields_schema(&data_struct.fields);
            quote! {
                DynamicSchema::Struct(
                    stringify!(#name),
                    DynamicStruct(#fields_gen.into())
                )
            }
        }
        Data::Enum(data_enum) => {
            let is_pure_enum = data_enum
                .variants
                .iter()
                .all(|v| matches!(v.fields, Fields::Unit));
            if is_pure_enum {
                let variant_names = data_enum.variants.iter().map(|v| {
                    let v_name = &v.ident;
                    quote! { stringify!(#v_name) }
                });
                quote! {
                    DynamicSchema::Enum(vec![ #(#variant_names),* ])
                }
            } else {
                let variants = data_enum.variants.iter().map(|variant| {
                    let v_name = &variant.ident;
                    match &variant.fields {
                        Fields::Unit => {
                            quote! { UnionVariant::Enum(stringify!(#v_name)) }
                        }
                        Fields::Named(_fields) => {
                            let fields_gen = generate_fields_schema(&variant.fields);
                            quote! {
                                UnionVariant::Struct(
                                    stringify!(#v_name),
                                    DynamicStruct(#fields_gen.into())
                                )
                            }
                        }
                        Fields::Unnamed(_) => {
                            panic!(
                                "Unnamed fields (tuple variants) are not supported by Generable!"
                            )
                        }
                    }
                });

                quote! {
                    DynamicSchema::Union(vec![
                        #(#variants),*
                    ])
                }
            }
        }
        Data::Union(_) => panic!("Rust Unions are not supported"),
    };

    let expanded = quote! {
        impl DynamicGenerable for #name {
            fn dynamic_schema() -> DynamicSchema<&'static str> {
                #schema_impl
            }
        }
    };

    TokenStream::from(expanded)
}

fn generate_fields_schema(fields: &Fields) -> proc_macro2::TokenStream {
    match fields {
        Fields::Named(named) => {
            let field_quotes = named.named.iter().map(|f| {
                let f_name = &f.ident;
                let f_ty = &f.ty;
                quote! { (stringify!(#f_name), <#f_ty>::dynamic_schema()) }
            });
            quote! { [ #(#field_quotes),* ] }
        }
        _ => quote! { [] },
    }
}
