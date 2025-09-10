use proc_macro::TokenStream;

mod async_func;
mod async_trait;
mod deref;

#[proc_macro_attribute]
pub fn async_fn(_:TokenStream, input:TokenStream) -> TokenStream {
    async_func::async_function(input).into()
}

#[proc_macro_attribute]
pub fn async_trait(_:TokenStream, input:TokenStream) -> TokenStream {
    async_trait::async_trait(input).into()
}

#[proc_macro_attribute]
pub fn deref_inner_async(args:TokenStream, item:TokenStream) -> TokenStream {
    let input: deref::DerefArgs = syn::parse_macro_input!(args);
    let set = input.validate().unwrap();

    deref::implement_deref_traits(
        set, 
        syn::parse(item).unwrap()
    ).into()
}