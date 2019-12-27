extern crate proc_macro;

use std::borrow::Cow;

use lazy_static::lazy_static;
use proc_macro::TokenStream;
use quote::quote;
use regex::Regex;
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, Data, DataEnum, DeriveInput, Fields,
    Ident, Variant,
};

lazy_static! {
    static ref WORDS_SPLITTER: Regex = Regex::new("([a-z0-9])([A-Z])").unwrap();
}

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
    let splitted_variants_names = splitted_variants_names(variants_names);

    quote! {
        match self {
            #(
                #enum_name :: #variants #variants_fields => #splitted_variants_names,
            )*
        }
    }
}

fn generate_struct(struct_name: &Ident) -> proc_macro2::TokenStream {
    let name = struct_name.to_string();
    let splitted_name = split_words(name.as_str());

    quote! {
        #splitted_name
    }
}

fn generate_union(union_name: &Ident) -> proc_macro2::TokenStream {
    let name = union_name.to_string();
    let splitted_name = split_words(name.as_str());

    quote! {
        #splitted_name
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

fn splitted_variants_names(names: impl Iterator<Item = String>) -> impl Iterator<Item = String> {
    names.map(|v| split_words(v.as_str()).to_string())
}

fn split_words(s: &str) -> Cow<'_, str> {
    WORDS_SPLITTER.replace_all(s, "$1 $2")
}
