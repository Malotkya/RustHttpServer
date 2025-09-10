pub fn unwrap_return(value: &syn::ReturnType) -> proc_macro2::TokenStream {
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

fn get_argument_ident(arg:&syn::FnArg) -> (syn::Ident, proc_macro2::TokenStream, bool) {
    match arg {
        syn::FnArg::Receiver(r) => (
            syn::Ident::new(
                "self",
                proc_macro2::Span::call_site()
            ),
            quote::quote!(#r),
            true
        ),
        syn::FnArg::Typed(t) => match t.pat.as_ref() {
            syn::Pat::Ident(i) => {
                let ident = i.ident.clone();
                if ident.to_string() == "self" {
                    let mut pattern = quote::quote!();

                    if let Some(value) = i.by_ref {
                        pattern.extend(quote::quote!( #value ))
                    } 

                    if let Some(value) = i.mutability {
                        pattern.extend(quote::quote!( #value ));
                    }

                    pattern.extend(quote::quote!( self ));

                    (
                        ident,
                        pattern,
                        true
                    )
                } else {
                    (
                        ident,
                        quote::quote!( #t ),
                        false
                    )
                }
            },
            _ => panic!("Found non-ident pattern when reading types!")
        }
        
    }
}

pub fn build_function_arguments(args: &syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let mut it = args.iter();

    let mut args_list = quote::quote!();
    let mut args_name = quote::quote!();

    if let Some(first) = it.next() {
        let (name, pattern, is_self)= get_argument_ident(first);
        if is_self {
            args_list.extend(quote::quote!( #pattern ));
            args_name.extend(quote::quote!( #name ));
            it.next(); //Context

        } else if let Some(new_first) = it.next(){
            let (name, pattern, _) = get_argument_ident(new_first);
            args_list.extend(quote::quote!( #pattern ));
            args_name.extend(quote::quote!( #name ));
        }
    }

    while let Some(next) = it.next() {
        let (name, pattern, _) = get_argument_ident(next);
        args_list.extend(quote::quote!( ,#pattern ));
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

pub fn build_async_function(
    ident:&syn::Ident, index: usize,
    inputs: &syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>,
    output: &syn::ReturnType
) -> proc_macro2::TokenStream {

    let name = build_function_name(ident, index);
    let (args_types, args_names) = build_function_arguments(inputs);
    let output = unwrap_return(output);

    quote::quote!{
        fn #name (#args_types) -> impl Future<Output = #output> {
            let pin = std::pin::Pin::new(self);
            std::future::poll_fn(move |cx|{
                pin.#ident(cx, #args_names)
            })
        }
    }
}

pub fn async_function(input:proc_macro::TokenStream) -> proc_macro2::TokenStream {
    let item = syn::parse::<syn::ItemFn>(input).unwrap();
    
    let index = item.sig.ident.to_string().find("poll")
        .expect("Cannot turn non-poll function into an async function!");

    let async_func = build_async_function(
        &item.sig.ident, index,
        &item.sig.inputs,
        &item.sig.output
    );
    
    quote::quote! {
        #item
        #async_func
    }
}