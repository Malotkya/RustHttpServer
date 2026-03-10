use proc_macro::TokenStream;

mod functions;
mod element;

#[proc_macro]
pub fn ts_function(input: TokenStream) -> TokenStream {
    let (code, sm) = functions::compile_ts(input.to_string());
    functions::parse_javascript(code, Some(sm)).into()
}

#[proc_macro]
pub fn js_function(input:TokenStream) -> TokenStream {
    functions::parse_javascript(input.to_string(), None).into()
}

#[proc_macro]
pub fn create_element(input:TokenStream) -> TokenStream {
    let elm:element::ElementData = syn::parse_macro_input!(input);
    element::build_element(elm).into()
}