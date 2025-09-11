use proc_macro::TokenStream;

mod path;
mod router;
mod headers;
mod util;
mod server;

#[proc_macro]
pub fn path(data:TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(data as syn::LitStr).value();
    let (regex_str, keys) = path::compile(&input, false, false);

    quote::quote! {
        http::types::Path{
            regex: regex::RegexBuilder::new(#regex_str)
                    .case_insensitive(true).build().unwrap(),
            keys: [#(#keys),*]
        }
    }.into()
}

#[proc_macro_attribute]
pub fn router(attrs: TokenStream, input: TokenStream) -> TokenStream {
    
    let args: router::RouterAttributes = syn::parse_macro_input!(attrs);
    let handler: syn::ItemFn = syn::parse_macro_input!(input);

    router::build_router(
        args, handler
    ).unwrap().into()
}

#[proc_macro]
pub fn build_headers(data:TokenStream) -> TokenStream {
    let input: headers::HeaderInput = syn::parse_macro_input!(data);
    headers::generate_header_name_enums(input).into()
}

#[proc_macro_attribute]
pub fn server(attrs:TokenStream, input: TokenStream) -> TokenStream {
    let args: server::ServerArguments = syn::parse_macro_input!(attrs);
    let att: server::ServerAttributes = syn::parse_macro_input!(input);

    server::build_server(args, att).into()
}


#[cfg(test)]
mod path_tests {
    use super::path;

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