use proc_macro::TokenStream;


mod attributes;

#[proc_macro]
pub fn attribute_functions(args: TokenStream) -> TokenStream {
    attributes::build_attributes(args).into()
}