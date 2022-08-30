mod packets;
mod replace;

use proc_macro::TokenStream;
use protocol::tostaticgenerics;
use quote::ToTokens;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Protocol, attributes(varint, case, count, from, fixed, stringuuid))]
pub fn protocol(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match input.data {
        syn::Data::Struct(strukt) => {
            protocol::struct_protocol(input.ident, input.attrs, input.generics, strukt)
        }
        syn::Data::Enum(enom) => {
            protocol::enum_protocol(input.attrs, input.ident, input.generics, enom)
        }
        syn::Data::Union(_) => panic!("Union structs not supported"),
    }
}
#[proc_macro_derive(ToStatic)]
pub fn to_static(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let mut tostaticgenerics = tostaticgenerics(input.generics.clone());
    let generics = input.generics;
    let ident = input.ident;
    let where_clause = tostaticgenerics.where_clause.take();

    match input.data {
        syn::Data::Struct(strukt) => {
            let (destructuring, to_static, into_static) = struct_to_static(strukt.fields);
            quote::quote! {
                impl #generics ToStatic for #ident #generics
                where
                    #where_clause
                {
                    type Static = #ident #tostaticgenerics;
                    fn to_static(&self) -> Self::Static {
                        let Self #destructuring = self;
                        #to_static
                        Self::Static #destructuring
                    }
                    fn into_static(self) -> Self::Static {
                        let Self #destructuring = self;
                        #into_static
                        Self::Static #destructuring
                    }
                }
            }
            .into()
        }
        syn::Data::Enum(enom) => {
            let mut to_static_match_contents = proc_macro2::TokenStream::new();
            let mut into_static_match_contents = proc_macro2::TokenStream::new();
            for variant in enom.variants {
                let (destructuring, to_static, into_static) = struct_to_static(variant.fields);
                let ident = variant.ident;
                quote::quote! {
                    Self::#ident #destructuring => {
                        #to_static
                        Self::Static::#ident #destructuring
                    }
                }
                .to_tokens(&mut to_static_match_contents);
                quote::quote! {
                    Self::#ident #destructuring => {
                        #into_static
                        Self::Static::#ident #destructuring
                    }
                }
                .to_tokens(&mut into_static_match_contents);
            }
            quote::quote! {
                impl #generics ToStatic for #ident #generics
                where
                    #where_clause
                {
                    type Static = #ident #tostaticgenerics;
                    fn to_static(&self) -> Self::Static {
                        match self {
                            #to_static_match_contents
                        }
                    }
                    fn into_static(self) -> Self::Static {
                        match self {
                            #into_static_match_contents
                        }
                    }
                }
            }
            .into()
        }
        syn::Data::Union(_) => todo!("calling ToStatic on Unions is not supported"),
    }
}

fn struct_to_static(
    fields: syn::Fields,
) -> (
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
) {
    let field_iter = fields
        .iter()
        .enumerate()
        .map(|(num, syn::Field { ident, .. })| {
            ident.clone().unwrap_or_else(|| {
                proc_macro2::Ident::new(&format!("_{num}"), proc_macro2::Span::mixed_site())
            })
        });
    let destructuring: proc_macro2::TokenStream = field_iter
        .clone()
        .map(|ident| quote::quote!(#ident,))
        .collect();
    let to_static: proc_macro2::TokenStream = field_iter
        .clone()
        .map(|ident| quote::quote!(let #ident = #ident.to_static();))
        .collect();
    let into_static: proc_macro2::TokenStream = field_iter
        .map(|ident| quote::quote!(let #ident = #ident.into_static();))
        .collect();
    let destructuring = match fields {
        syn::Fields::Named(_) => quote::quote! {{#destructuring}},
        syn::Fields::Unnamed(_) => quote::quote! {(#destructuring)},
        syn::Fields::Unit => quote::quote!(),
    };
    (destructuring, to_static, into_static)
}

#[proc_macro]
pub fn packets(input: TokenStream) -> TokenStream {
    let x = parse_macro_input!(input as packets::PacketsInput);

    packets::packets(x)
}

#[proc_macro]
pub fn replace(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as replace::ReplaceInput);

    let mut output = proc_macro2::TokenStream::new();

    replace::match_group(input.rest.into_iter(), &mut output, &input.types);

    output
        .into_iter()
        .collect::<proc_macro2::TokenStream>()
        .into()
}

mod protocol;
