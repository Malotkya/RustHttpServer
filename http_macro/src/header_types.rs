use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;

pub(crate) struct HeaderArguments {
    item: syn::ItemStruct,
    new: syn::ItemFn,
    name: syn::Path,
    from: syn::ItemFn,
    to_string: syn::ItemFn,
    parse: Option<syn::ItemFn>
}

fn duplicate_error(name: &str) -> String {
    format!("Duplicate function \"{}\"!", name)
}

fn missing_error(name: &str) -> String {
    format!("Missing item \"{}\"!", name)
}

impl Parse for HeaderArguments {
    fn parse(input:ParseStream) -> syn::Result<Self> {
        let item = input.parse::<syn::ItemStruct>()?;
        input.parse::<syn::Token![,]>()?;

        let mut new:  Option<syn::ItemFn> = None;
        let mut name: Option<syn::Path> = None;
        let mut from: Option<syn::ItemFn> = None;
        let mut to_string: Option<syn::ItemFn> = None;
        let mut parse: Option<syn::ItemFn> = None;

        while !input.is_empty() {
            if input.peek(syn::token::Fn) {
                let func = input.parse::<syn::ItemFn>()?;
                match func.sig.ident.to_string().as_str() {
                    "new" => {
                        if new.is_none() {
                            new = Some(func)
                        } else {
                            return Err(syn::Error::new(
                                func.sig.ident.span(),
                                duplicate_error("new")
                            ))
                        }
                    },
                    "from" => {
                        if from.is_none() {
                            from = Some(func)
                        } else {
                            return Err(syn::Error::new(
                                func.sig.ident.span(),
                                duplicate_error("from")
                            ))
                        }
                    },
                    "to_string" => {
                        if to_string.is_none() {
                            to_string = Some(func)
                        } else {
                            return Err(syn::Error::new(
                                func.sig.ident.span(),
                                duplicate_error("to_string")
                            ))
                        }
                    },
                    "parse" => {
                        if parse.is_none() {
                            parse = Some(func)
                        } else {
                            return Err(syn::Error::new(
                                func.sig.ident.span(),
                                duplicate_error("parse")
                            ))
                        }
                    }
                    ident => {
                        return Err(syn::Error::new(
                            func.sig.ident.span(),
                            format!("Unknown funciton name: {}!", ident)
                        ))
                    }
                } //End Match
            } else {
                let path = input.parse::<syn::Path>()?;

                if name.is_none() {
                    name = Some(path);
                } else {
                    return Err(syn::Error::new(
                        path.span(),
                        "Duplicate Header Name!"
                    ))
                }
            }

            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            }
        }

        Ok(Self{
            item,
            new : new.ok_or_else(||syn::Error::new(
                proc_macro2::Span::call_site(),
                missing_error("new")
            ))?,
            name : name.ok_or_else(||syn::Error::new(
                proc_macro2::Span::call_site(),
                missing_error("name")
            ))?,
            from : from.ok_or_else(||syn::Error::new(
                proc_macro2::Span::call_site(),
                missing_error("from")
            ))?,
            to_string : to_string.ok_or_else(||syn::Error::new(
                proc_macro2::Span::call_site(),
                missing_error("to_string")
            ))?,
            parse
        })
    }
}

pub fn build_header_type(input:proc_macro::TokenStream) -> proc_macro2::TokenStream {
    let HeaderArguments{
        item, new, name, from, to_string, parse
    } = syn::parse(input).unwrap();

    let struct_name = &item.ident;
    let generics =  if item.generics.lifetimes().count() > 0 {
        quote::quote!(<'a>)
    } else {
        quote::quote! ()
    };

    let parse = parse.map(|func| quote::quote!{
        impl<'a> super::ParseType<'a> for #struct_name #generics {
            #func
        }
        
    }).unwrap_or(quote::quote!());

    quote::quote!{
        #item

        #parse

        impl #generics #struct_name #generics {
            pub #new
        }

        impl #generics super::HttpHeader for #struct_name #generics {
            fn name() -> super::HeaderName {
                #name
            }
        }

        impl <'a> From<&'a super::HeaderType<'a>> for #struct_name #generics {
            #from
        }

        impl #generics ToString for #struct_name #generics {
            #to_string
        }
    }
}