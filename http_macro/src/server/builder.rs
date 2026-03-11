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
        async fn handler(builder: &mut http::builder::RequestBuilder<http::async_net::TcpStream>) -> http::Result {
            #handle_router

            Err(http :: HttpErrorKind :: NotFound.into())
        }
    }
}

pub fn build_server(att:ServerAttributes, hostname:&str, port:u16) -> TokenStream {
    let name = att.name;
    let handler = build_handler(&att.routers, );

    let error_handler = match &att.err_handler {
        Some(func) => quote!(#func( builder.build(e) ).await ),
        None => quote!(http::types::Response::from_error(e) )
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
            port: u16
        }

        impl #name {
            fn new(hostname:Option<String>, port:Option<u16>) -> Self {
                Self {
                    hostname: hostname.unwrap_or(#hostname.to_string()),
                    port: port.unwrap_or(#port)
                }
            }

            #handler
        }
        
        impl http::builder::Server for #name {
            fn port(&self) -> u16 {
                self.port
            }

            fn hostname(&self) -> &str {
                &self.hostname
            }

            async fn handle_request(&self, mut builder: &mut http::builder::RequestBuilder<http::async_net::TcpStream>) -> http::types::Response {
                match Self::handler(&mut builder).await {
                    Ok(resp) => resp,
                    Err(e) => #error_handler
                }
            }
        }
    }
}