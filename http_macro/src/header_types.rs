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

        if input.peek(syn::Token![;]) {
            input.parse::<syn::Token![;]>()?;
        }

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

            if !input.peek(syn::parse::End) {
                input.parse::<syn::Token![;]>()?;
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

fn append_lifetime(original:&syn::Generics) -> (syn::Lifetime, syn::Generics) {
    let mut clone = original.clone();
    if let Some(value) = clone.lifetimes().next() {
        (
            value.lifetime.clone(),
            clone
        )
    } else {
        let new = syn::Lifetime::new("'a", proc_macro2::Span::call_site());
        clone.params.push(syn::GenericParam::Lifetime(
            syn::LifetimeParam::new(new.clone())
        ));
        (
            new,
            clone
        )
    }
}

fn build_impl_generics(original: &syn::Generics) -> syn::Result<Option<syn::Generics>> {
    if original.params.iter().next().is_none() {
        return Ok(None)
    }

    let mut params: syn::punctuated::Punctuated<syn::GenericParam, syn::Token![,]>
        = syn::punctuated::Punctuated::new();

    for param in original.params.iter() {
        match param {
            syn::GenericParam::Type(t) => params.push(syn::GenericParam::Type(
                syn::TypeParam::from(t.ident.clone())
            )),
            syn::GenericParam::Const(c) => return Err(
                syn::Error::new(
                    c.span(),
                    "Unable to parse const generics!"
                )
            ),
            syn::GenericParam::Lifetime(l) => {
                params.push(syn::GenericParam::Lifetime(
                syn::LifetimeParam::new(
                    l.lifetime.clone()
                )
            ))}
        }
    }

    Ok(Some(syn::Generics{
        lt_token: Some(syn::Token![<](original.lt_token.span())),
        gt_token: Some(syn::Token![>](original.gt_token.span())),
        where_clause: None,
        params
    }))
}

pub fn build_header_type(input:proc_macro::TokenStream) -> proc_macro2::TokenStream {
    let HeaderArguments{
        mut item, new, name, from, to_string, parse
    } = syn::parse(input).unwrap();

    item.vis = syn::Visibility::Public(syn::token::Pub(item.span()));

    item.fields.iter_mut().for_each(|field|{
        field.vis = syn::Visibility::Public(syn::token::Pub(field.span()));
    });

    let struct_name = &item.ident;
    let impl_generics =  if item.generics.params.iter().count() == 0 {
        None
    } else {
        Some(item.generics.clone())
    };
    let (from_lifetime, from_gen_impl) = append_lifetime(&item.generics);
    let struct_generics = build_impl_generics(&item.generics).unwrap();

    let parse = parse.map(|func| quote::quote!{
        impl<'a> super::ParseType<'a> for #struct_name #impl_generics {
            #func
        }
        
    }).unwrap_or(quote::quote!());

    quote::quote!{
        #item

        #parse

        impl #impl_generics #struct_name #struct_generics {
            pub #new
        }

        impl #impl_generics super::HttpHeader for #struct_name #struct_generics {
            fn name() -> super::HeaderName {
                #name
            }
        }

        impl #from_gen_impl From<& #from_lifetime super::HeaderType<#from_lifetime>>
                for #struct_name #struct_generics {
            #from
        }

        impl #impl_generics ToString for #struct_name #struct_generics {
            #to_string
        }
    }
}