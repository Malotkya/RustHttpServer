use proc_macro2::{TokenStream, Span};
use syn::parse::{Parse, ParseStream};
use deluxe::Result;
use quote::quote;

pub(crate) struct RouterAttributes {
    path: String,
    //default = false
    path_end:bool,
    //defailt = false
    path_trailing:bool,
    //default = true
    path_insensitive:bool,
    //default == "All"
    methods:String
    
}

impl Parse for RouterAttributes {

    fn parse(input:ParseStream) -> Result<Self> {
        let map = super::util::InputParser::new(input);

        let path = map.get_string("path").unwrap();
        let path_end = map.get_bool("path_end").unwrap_or(false);
        let path_trailing = map.get_bool("path_trailing").unwrap_or(false);
        let path_insensitive = map.get_bool("path_insensitive").unwrap_or(true);
        let methods = map.get_string("methods").unwrap_or(String::from("ALL"));
        
        return Ok(Self{
            path,
            path_end,
            path_trailing,
            path_insensitive,
            methods
        })
    }
}

pub fn build_router(args:RouterAttributes, handler:syn::ItemFn) -> Result<TokenStream> {
    let i = args.path_insensitive;
    let name = handler.sig.ident;
    let methods = args.methods; //attr.methods;

    let (regex, keys) = crate::path::compile(&args.path, args.path_trailing, args.path_end);
    let path_struct_name = syn::Ident::new(
        &format!("{}PathParam", name),
        Span::call_site()
    );
    let (path_struct, path_size) = super::path::build_path_types(
        &path_struct_name,
        &keys
    );

    let hand_attr:Vec<_> = handler.sig.inputs.iter().collect();
    let hand_block = handler.block;

    //panic!("{:?}", hand_attr);

    Ok(
        quote! {
            #path_struct

            struct #name {
                path:regex::Regex,
                methods:&'static str//Vec<&'static str>
            }

            impl #name {
                pub fn new() -> Self {
                    Self {
                        path: regex::RegexBuilder::new(#regex)
                            .case_insensitive(#i).build().unwrap(),
                        methods: #methods//vec![#(#methods),*]
                    }
                }

                fn match_path<'a>(&self, pathname:&'a str) -> Option<#path_struct_name<'a>> {
                    match self.path.captures(pathname) {
                        Some(caps) => {
                            let (_, list) = caps.extract() as (&str, [&str; #path_size]);
                            Some(#path_struct_name::new(list))
                        },
                        None => None
                    }
                }

                fn handler(&self, #(#hand_attr),* ) {
                    #hand_block
                }
            }

            impl Router for #name {
                fn handle<'a>(&self, url:&'a str) -> Option<()> {
                    match self.match_path(url) {
                        Some(param) => {
                            self.handler(param);
                            Some(())
                        },
                        None => None
                    }
                }
            }
        }
    )
}