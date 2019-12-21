extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, Data, DataEnum, DeriveInput, Fields,
    Ident, Variant,
};

#[proc_macro_derive(AsShortName)]
pub fn as_short_name(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let body = generate_body(&input);
    let name = &input.ident;

    let gen = quote! {
        impl ShortName for #name {
            fn as_short_name(&self) -> &str {
                #body
            }
        }
    };

    gen.into()
}

fn generate_body(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;
    match input.data {
        Data::Enum(DataEnum { ref variants, .. }) => generate_enum(variants, name),
        Data::Struct(_) => generate_struct(name),
        Data::Union(_) => generate_union(name),
    }
}

fn generate_enum(
    punctuated_variants: &Punctuated<Variant, Comma>,
    enum_name: &Ident,
) -> proc_macro2::TokenStream {
    let variants = variants_from_punctuated(punctuated_variants);
    let variants_fields = variants_fields_from_punctuated(punctuated_variants);
    let variants_names = variants_names_from_punctuated(punctuated_variants);

    quote! {
        match self {
            #(
                #enum_name :: #variants #variants_fields => #variants_names,
            )*
        }
    }
}

fn generate_struct(struct_name: &Ident) -> proc_macro2::TokenStream {
    let name = struct_name.to_string();

    quote! {
        #name
    }
}

fn generate_union(union_name: &Ident) -> proc_macro2::TokenStream {
    let name = union_name.to_string();

    quote! {
        #name
    }
}

fn variants_from_punctuated(
    punctuated: &Punctuated<Variant, Comma>,
) -> impl Iterator<Item = &Ident> {
    punctuated.iter().map(|v| &v.ident)
}

fn variants_fields_from_punctuated<'a>(
    punctuated: &'a Punctuated<Variant, Comma>,
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
    punctuated.iter().map(|v| &v.fields).map(|f| match f {
        Fields::Named(_) => quote!({ .. }),
        Fields::Unnamed(_) => quote!((..)),
        Fields::Unit => quote!(),
    })
}

fn variants_names_from_punctuated<'a>(
    punctuated: &'a Punctuated<Variant, Comma>,
) -> impl Iterator<Item = String> + 'a {
    punctuated.iter().map(|v| v.ident.to_string())
}
