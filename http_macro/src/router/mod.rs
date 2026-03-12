use proc_macro2::{TokenStream, Span};
use syn::{parse, ItemFn};
use quote::quote;
use inputs::RouterAttributes;


mod inputs;

pub fn build(attributes:proc_macro::TokenStream, data:proc_macro::TokenStream) -> TokenStream {
    let RouterAttributes{path, methods} = parse(attributes).unwrap();
    let handler = parse::<ItemFn>(data).unwrap();

    let name = &handler.sig.ident;
    
    let methods_name = syn::Ident::new(
        &format!("{}Methods", &name),
        Span::call_site()
    );

    let (handler_name, path, match_capture) = path.build_pattern(&name); 

    let public = &handler.vis;
    let hand_attr:Vec<_> = handler.sig.inputs.iter().collect();
    let hand_block = &handler.block;
    let hand_return =& handler.sig.output;
    let hand_genics = &handler.sig.generics;
    let async_call = &handler.sig.asyncness;

    quote! {
        const #methods_name:&'static str = #methods;
        #path

        #async_call fn #handler_name #hand_genics( #(#hand_attr),* ) #hand_return #hand_block
        
        #[allow(non_snake_case)]
        #public #async_call fn #name(req:&mut http::server::RequestBuilder<http::async_net::TcpStream>) -> http::Result<Option<http::Response>> {
            #match_capture

            Ok(None)
        }
    }
}