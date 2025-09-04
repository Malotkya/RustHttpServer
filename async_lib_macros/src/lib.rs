use proc_macro::TokenStream;

mod async_trait;

#[proc_macro_attribute]
pub fn async_trait(_:TokenStream, input:TokenStream) -> TokenStream {
    async_trait::async_trait(input).into()
}