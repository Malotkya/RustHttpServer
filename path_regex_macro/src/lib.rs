use proc_macro::TokenStream;

mod regex;

#[proc_macro]
pub fn path(input:TokenStream) -> TokenStream {
    let path_string = syn::parse_macro_input!(input as syn::LitStr).value();
    let (regex_str, keys) = regex::compile(&path_string, false, false);

    quote::quote! {
        Path{
            regex: regex::RegexBuilder::new(#regex_str)
                    .case_insensitive(true).build().unwrap(),
            keys: [#(#keys),*]
        }
    }.into()
}

#[cfg(test)]
mod tests {
    use super::regex;

    const PATH:&'static str = "/Hello/:World";
    const QUOTES_PATH:&'static str = "/:\"Good Bye\"";

    #[test]
    fn lexer_test() {
        let mut it = regex::lexer(PATH);
        assert_eq!(it.tokens.len(), 9);

        it = regex::lexer(QUOTES_PATH);
        assert_eq!(it.tokens.len(), 3);
    }

    #[test]
    fn iter_test() {
        let mut it = regex::lexer(PATH);
        let mut seg = it.parse(regex::TokenType::End);

        assert_eq!(seg.len(), 2);
        while let Some(value) = seg.pop_front() {
            match value {
                regex::Segment::Parameter(value) => {
                    assert_eq!(value, "World")
                },
                regex::Segment::Text(value) => {
                    assert_eq!(value, "Hello")
                },
                _ => { panic!("Unexpected Segment Type!")}
            }
        }
    }

    #[test]
    fn compile_test() {
        let (regex_str, keys) = regex::compile(PATH, false, true);
        println!("{}", regex_str);
        assert_eq!(keys.len(), 1);
    }
}