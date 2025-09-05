use quote::ToTokens;

fn unwrap_return(value: &syn::ReturnType) -> proc_macro2::TokenStream {
    match value{
        syn::ReturnType::Default => quote::quote!(),
        syn::ReturnType::Type(_, t) => match (*t).as_ref() {
            syn::Type::Path(path) => if path.qself.is_none() {
                let t = unwrap_return_segments(&path.path.segments);
                quote::quote!(#t)
            } else {
                let t = path.qself.clone().unwrap().ty;
                quote::quote!(#t)
            },
            bad => panic!("Unable to unwrap type {:?}", bad)
        }
    }
}

fn unwrap_return_segments(segs: &syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep>) -> syn::Type {
    let mut it = segs.iter();

    while let Some(seg) = it.next() {
        if let syn::PathArguments::AngleBracketed(value) = &seg.arguments {
            if let syn::GenericArgument::Type(t) = value.args.first().unwrap() {
                return t.clone();
            }
        }
    }

    panic!("Unable to unwrap segment {:?}", segs)
}

fn get_argument_ident(arg:&syn::FnArg) -> syn::Ident {
    match arg {
        syn::FnArg::Receiver(_) => syn::Ident::new(
            "self",
            proc_macro2::Span::call_site()
        ),
        syn::FnArg::Typed(t) => match t.pat.as_ref() {
            syn::Pat::Ident(i) => i.ident.clone(),
            _ => panic!("Found non-ident pattern when reading types!")
        }
        
    }
}

fn build_function_arguments(args: &syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let mut it = args.iter();
    it.next(); //self
    it.next(); //context

    let mut args_list = quote::quote!();
    let mut args_name = quote::quote!();

    if let Some(first) = it.next() {
        let name = get_argument_ident(first);
        args_list.extend(quote::quote!( #first, ));
        args_name.extend(quote::quote!( #name ));
    }

    while let Some(next) = it.next() {
        let name = get_argument_ident(next);
        args_list.extend(quote::quote!( ,#next ));
        args_name.extend(quote::quote!( ,#name ));
    }

    (args_list, args_name)
}

fn build_function_name(ident:&syn::Ident, index:usize) -> syn::Ident {
    let name = ident.to_string();

    syn::Ident::new(
        &(String::from(&name[..index]) + &name[index+5..]),
        ident.span()
    )
}

fn build_async_function(func: &syn::TraitItemFn, index:usize) -> proc_macro2::TokenStream {
    let func_ident = &func.sig.ident;
    let name = build_function_name(func_ident, index);
    let (args_types, args_names) = build_function_arguments(&func.sig.inputs);
    let output = unwrap_return(&func.sig.output);

    quote::quote!{
        fn #name (&mut self, #args_types) -> impl Future<Output = #output> {
            std::future::poll_fn(move |cx|{
                self.#func_ident(cx, #args_names)
            })
        }
    }
}

fn build_trait_functions(list:&mut Vec<syn::TraitItem>) -> proc_macro2::TokenStream {
    let mut output = quote::quote!();
    let mut keep = Vec::new();

    for item in list {
        if let syn::TraitItem::Fn(func) = item {
            if let Some(index) = func.sig.ident.to_string().find("poll") {
                output.extend(build_async_function(func, index));
                keep.push(item.clone())
            } else {
                output.extend(item.into_token_stream())
            }
        } else {
            keep.push(item.clone())
        }
    }

    output
}

fn build_trait_name(ident:&syn::Ident) -> syn::Ident {
    let mut name = ident.to_string();

    if let Some(index) = name.find("Poll") {
        name = vec![
            &name[..index],
            "Async",
            &name[index+4..]
        ].join("");
    } else {
        name = String::from("Async") + &name;
    }

    syn::Ident::new(
        &name,
        ident.span()
    )
}

fn build_async_path(value: &syn::TraitBound) -> Result<syn::TraitBound, bool> {
    let mut async_trait = value.clone();

    if let Some(ident) = async_trait.path.segments.last_mut() {
        let name = ident.ident.to_string();

        if let Some(index) = name.find("Poll") {
            ident.ident = syn::Ident::new(
                &format!("{}Async{}", &name[..index], &name[index+4..]),
                ident.ident.span()
            );
            
            Ok(async_trait)
        } else {
            Err(name == "Sized")
        }
    } else {
        Err(false)
    }
}

fn build_async_supers(list: &syn::punctuated::Punctuated<syn::TypeParamBound, syn::token::Plus>, super_name:&syn::Ident) -> proc_macro2::TokenStream {
    let mut is_sized = false;
    let mut output = quote::quote!(#super_name);

    for item in list.iter() {
        if let syn::TypeParamBound::Trait(value) = item {
            match build_async_path(value) {
                Ok(path) => {
                    output.extend(quote::quote!( + #path));
                    continue;
                },
                Err(sized) => {
                    if !is_sized && sized {
                        is_sized = sized;
                    }
                    
                }
            }
        } 
        
        output.extend(quote::quote!( + #item ));
    }

    if !is_sized {
        output.extend(quote::quote!(+ Sized));
    }

    output
}

fn build_async_trait(poll_trait:&mut syn::ItemTrait) -> proc_macro2::TokenStream {
    let visibility = poll_trait.vis.clone();
    let super_name = &poll_trait.ident;
    let name = build_trait_name(super_name);
    let funcs = build_trait_functions(&mut poll_trait.items);
    let supers = build_async_supers(&poll_trait.supertraits, super_name);

    quote::quote!{
        #visibility trait #name: #supers {
            #funcs
        }
    }
}

pub fn async_trait(input:proc_macro::TokenStream) -> proc_macro2::TokenStream {
    let mut poll_trait:syn::ItemTrait = syn::parse(input).unwrap();
    let async_trait = build_async_trait(&mut poll_trait);

    //panic!("{}", async_trait);

    quote::quote!{
        #poll_trait
        #async_trait
    }
}