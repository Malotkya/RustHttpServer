use proc_macro::TokenStream;
use syn::parse;

mod path;
mod router;
mod header;
mod util;
mod server;

#[proc_macro]
pub fn path(input:TokenStream) -> TokenStream {
    path::build(input).into()
}

#[proc_macro]
pub fn build_header_value(data:TokenStream) -> TokenStream {
    parse::<header::HeaderValue>(data)
        .unwrap()
        .build()
        .into()
}

#[proc_macro_attribute]
pub fn router(attributes:TokenStream, data:TokenStream) -> TokenStream {
    router::build(attributes, data).into()
}

#[proc_macro]
pub fn build_headers(data:TokenStream) -> TokenStream {
    parse::<header::HeaderEnum>(data)
        .unwrap()
        .build()
        .into()
}

#[proc_macro_attribute]
pub fn server(attributes: TokenStream, data:TokenStream) -> TokenStream {
    server::build(attributes, data).into()
}


