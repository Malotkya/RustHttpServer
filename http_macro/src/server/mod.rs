use inputs::{parse_server_arguments, parse_server_attributes, ServerArguments};
use builder::build_server;

mod inputs;
mod builder;

pub fn build(args: proc_macro::TokenStream, attr: proc_macro::TokenStream) -> proc_macro2::TokenStream {
    let ServerArguments{hostname, config:_, port, threads}
        = parse_server_arguments(args);
    let attributes = parse_server_attributes(attr);
    let name = attributes.name.clone();

    let server = build_server(
        attributes,
        &hostname,
        port
    );

    quote::quote! {
        #server

        fn main() {
            use http::builder::Server;

            //TODO parse inputs here? or in start possibly?
            let server = #name::new(None, None);
            server.start(#threads).unwrap();
        }

    }
}


