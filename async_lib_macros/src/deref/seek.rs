pub fn implement_deref_seek(struct_name:&syn::Ident, trait_name: &syn::Ident) -> proc_macro2::TokenStream {
    quote::quote!{
        use crate::future::io::{PollSeek, AsyncSeek};
        impl PollSeek for #struct_name {
            fn poll_seek(&mut self, cx: &mut std::task::Context<'_>, pos:std::io::SeekFrom) -> std::task::Poll<std::io::Result<u64>> {
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

        impl AsyncSeek for #struct_name {}
    }
}