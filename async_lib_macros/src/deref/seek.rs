pub fn implement_deref_seek(struct_name:&syn::Ident, struct_generics:&syn::Generics, trait_name:&proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    quote::quote!{
        impl #struct_generics crate::future::io::AsyncSeek for #struct_name #struct_generics{
            fn poll_seek(mut self:std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>, pos:std::io::SeekFrom) -> std::task::Poll<std::io::Result<u64>> {
                use std::io::Seek;

                match self.#trait_name.seek(pos) {
                    Ok(off) => std::task::Poll::Ready(Ok(off)),
                    Err(e) => if e.kind() == std::io::ErrorKind::WouldBlock {
                        cx.waker().wake_by_ref();
                        std::task::Poll::Pending
                    } else {
                        std::task::Poll::Ready(Err(e))
                    }
                }
            }
        } 
    }
}