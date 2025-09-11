pub fn unwrap_return(value: &syn::ReturnType) -> proc_macro2::TokenStream {
    match value{
        syn::ReturnType::Default => quote::quote!(),
        syn::ReturnType::Type(_, t) => unwrap_type(t)
            .expect(&format!("Unable to unwrap type {:?}", t))
    }
}

pub fn unwrap_type(value: &syn::Type) -> Option<proc_macro2::TokenStream> {
    match value{
        syn::Type::Path(path) => if path.qself.is_none() {
            let t = unwrap_segments(&path.path.segments);
            Some(quote::quote!(#t))
        } else {
            let t = path.qself.clone().unwrap().ty;
            Some(quote::quote!(#t))
        },
        _ => None
    }
}

fn unwrap_segments(segs: &syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep>) -> syn::Type {
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
        syn::FnArg::Receiver(r) => {
            (
            syn::Ident::new(
                "self",
                proc_macro2::Span::call_site()
            ),
            unwrap_type(&r.ty).map(|t|quote::quote!(self: #t)).unwrap_or(quote::quote!(#r)),
            true
        )
        },
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
        let (_, pattern, is_self)= get_argument_ident(first);
        if is_self {
            args_list.extend(quote::quote!( #pattern, ));
            //args_name.extend(quote::quote!( #name ));
            it.next(); //Context
        } //Else assume Context
    }

    while let Some(next) = it.next() {
        let (name, pattern, _) = get_argument_ident(next);
        args_list.extend(quote::quote!( #pattern, ));
        args_name.extend(quote::quote!( #name, ));
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

pub fn build_async_function(sig:&syn::Signature, index: usize) -> proc_macro2::TokenStream {

    let ident = &sig.ident;
    let generics = &sig.generics;

    let name = build_function_name(ident, index);
    let (args_types, args_names) = build_function_arguments(&sig.inputs);
    let output = unwrap_return(&sig.output);
    

    quote::quote!{
        fn #name #generics (#args_types) -> impl Future<Output = #output> {
            let mut pin = unsafe{ std::pin::Pin::new_unchecked(self) };
            std::future::poll_fn(move |cx|{
                pin.as_mut().#ident(cx, #args_names)
            })
        }
    }
}

pub fn async_function(input:proc_macro::TokenStream) -> proc_macro2::TokenStream {
    let mut output: proc_macro2::TokenStream;
    let public: syn::Visibility;
    let signature = match syn::parse::<syn::ItemFn>(input.clone()) {
        Ok(v) => {
            output = quote::quote!( #v );
            public = v.vis;
            v.sig
        },
        Err(e) => match syn::parse::<syn::TraitItemFn>(input) {
            Ok(v) => {
                output = quote::quote!( #v );
                public = syn::Visibility::Inherited;
                v.sig
            },
            Err(_) => panic!("{}", e)
        }
    };
    
    let index = signature.ident.to_string().find("poll")
        .expect("Cannot turn non-poll function into an async function!");

    let async_func = build_async_function(&signature, index);

    output.extend(quote::quote!{
        #public #async_func
    });

    output
}