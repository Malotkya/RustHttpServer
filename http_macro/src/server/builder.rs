use proc_macro2::TokenStream;
use quote::quote;
use super::inputs::ServerAttributes;

fn build_handler(routers:&Vec<syn::Ident>) -> TokenStream {
    let mut handle_router = quote!();

    for r in routers {
        handle_router.extend(quote!{
            if let Some(resp) = #r(builder).await? {
                return Ok(resp)
            }
        });
    }

    quote!{
        async fn handler(builder: &mut http::server::RequestBuilder<http::async_net::TcpStream>) -> http::Result<http::Response> {
            #handle_router
            use http::types::ValidHttpError;

            http::HttpErrorKind::NotFound.send()
        }
    }
}

pub fn build_server(att:ServerAttributes, hostname:&str, port:u16, threads:usize) -> TokenStream {
    let name = att.name;
    let handler = build_handler(&att.routers, );

    let error_handler = match &att.err_handler {
        Some(func) => quote!(#func( builder.build(e) ).await ),
        None => quote!(http::Response::from_error(e) )
    }; 

    let struct_start = if att.public {
        quote!{pub struct}
    } else {
        quote!{struct}
    };

    quote!{
        #[derive(Clone)]
        #struct_start #name{
            hostname:String,
            port: u16,
            threads: usize
        }

        impl #name {
            #handler
        }
        
        impl http::server::Server for #name {

            fn new(opts:http::server::ServerOpts) -> Self {
                Self {
                    hostname: opts.hostname.unwrap_or(#hostname.to_string()),
                    port: opts.port.unwrap_or(#port),
                    threads: opts.threads.unwrap_or(#threads)
                }
            }

            fn port(&self) -> u16 {
                self.port
            }

            fn hostname(&self) -> &str {
                &self.hostname
            }

            fn threads(&self) -> usize {
                self.threads
            }

            async fn handle_request(&self, mut builder: &mut http::server::RequestBuilder<http::async_net::TcpStream>) -> http::Response {
                match Self::handler(&mut builder).await {
                    Ok(resp) => resp,
                    Err(e) => #error_handler
                }
            }
        }
    }
}