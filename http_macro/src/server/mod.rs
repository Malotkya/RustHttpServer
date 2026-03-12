use inputs::{parse_server_arguments, parse_server_attributes, ServerArguments};
use builder::build_server;

mod inputs;
mod builder;

pub fn build(args: proc_macro::TokenStream, attr: proc_macro::TokenStream) -> proc_macro2::TokenStream {
    let ServerArguments{hostname, config, port, threads}
        = parse_server_arguments(args);
    let attributes = parse_server_attributes(attr);
    let name = attributes.name.clone();

    let config = match config {
        Some(str) => quote::quote!(Some(#str)),
        None => quote::quote!(None)
    };

    let server = build_server(
        attributes,
        &hostname,
        port,
        threads
    );

    quote::quote! {
        #server

        fn main() {
            use http::server::{Server, ServerOpts, get_server_opts};

            let opts = get_server_opts(#config).unwrap();
            let server = #name::new(opts);

            server.start().unwrap();
        }

    }
}


