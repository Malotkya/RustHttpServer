use proc_macro2::{TokenStream, Span};
use syn::{parse, LitStr};
use quote::quote;
pub use path_to_regex::compile;

mod path_to_regex;

pub fn build(input:proc_macro::TokenStream) -> TokenStream {
    let input = parse::<LitStr>(input).unwrap().value();
    let (regex_str, keys) = compile(&input, false, false);

    quote::quote! {
        http::server::router::Path{
            regex: regex::RegexBuilder::new(#regex_str)
                    .case_insensitive(true).build().unwrap(),
            keys: [#(#keys),*]
        }
    }
}

pub fn build_path_types(name:&syn::Ident, keys:&Vec<String>) -> (TokenStream, usize) {
    let mut fields = quote!();
    let mut constructor = quote!();

    let length = keys.len();
    for i in 0..length {
        let field = syn::Ident::new(&keys[i], Span::call_site());

        fields.extend(quote!{
            pub #field: String,
        });

        constructor.extend(quote!{
            #field: list[#i].to_owned(),
        });
    }

    (quote! {
        pub struct #name {
            #fields
        }

        impl #name {
            fn new(list: [&str; #length]) -> Self {
                Self { #constructor }
            }
        }
    }, length )
}


#[cfg(test)]
mod tests {
    use super::*;

    const PATH:&'static str = "/Hello/:World";
    const QUOTES_PATH:&'static str = "/:\"Good Bye\"";

    #[test]
    fn lexer_test() {
        let mut it = path::lexer(PATH);
        assert_eq!(it.tokens.len(), 9);

        it = path::lexer(QUOTES_PATH);
        assert_eq!(it.tokens.len(), 3);
    }

    #[test]
    fn iter_test() {
        let mut it = path::lexer(PATH);
        let mut seg = it.parse(path::TokenType::End);

        assert_eq!(seg.len(), 2);
        while let Some(value) = seg.pop_front() {
            match value {
                path::Segment::Parameter(value) => {
                    assert_eq!(value, "World")
                },
                path::Segment::Text(value) => {
                    assert_eq!(value, "Hello")
                },
                _ => { panic!("Unexpected Segment Type!")}
            }
        }
    }

    #[test]
    fn compile_test() {
        let (regex_str, keys) = path::compile(PATH, false, true);
        println!("{}", regex_str);
        assert_eq!(keys.len(), 1);
    }
}