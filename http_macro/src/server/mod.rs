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
        port
    );

    quote::quote! {
        #server

        fn main() {
            use http::builder::{Server, ServerOpts, get_user_options};

            let ServerOpts{hostname, port, threads} = get_user_options(#config).unwrap();
            let thread_count = threads.unwrap_or(#threads);

            //TODO parse inputs here? or in start possibly?
            let server = #name::new(hostname, port);
            server.start(thread_count).unwrap();
        }

    }
}


