use quote::ToTokens;
use super::async_func::*;

fn build_async_trait_functions(list:&mut Vec<syn::TraitItem>) {
    let mut new_list = Vec::with_capacity(list.len() * 2);

    while let Some(item) = list.pop() {
        if let syn::TraitItem::Fn(func) = &item {
            if let Some(index) = func.sig.ident.to_string().find("poll") {
                let async_func = build_async_function(&func.sig, index);
                
                new_list.push(syn::TraitItem::Fn(
                    syn::parse(async_func.into()).unwrap()
                ));
            }
        }

        new_list.push(item);
    }

    list.append(&mut new_list);
}

pub fn async_trait(input:proc_macro::TokenStream) -> proc_macro2::TokenStream {
    let mut item:syn::ItemTrait = syn::parse(input).unwrap();
    build_async_trait_functions(&mut item.items);

    item.into_token_stream()
}