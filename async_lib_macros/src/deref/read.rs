
pub fn implement_deref_read(struct_name:&syn::Ident, trait_name: &syn::Ident) -> proc_macro2::TokenStream {
    quote::quote!{
        impl crate::future::io::AsyncRead for #struct_name {
            fn poll_read(mut self:Pin<&mut Self>, cx: &mut std::task::Context<'_>, buf: &mut [u8]) -> std::task::Poll<std::io::Result<usize>> {
                use std::io::Read;

                match self.#trait_name.read(buf) {
                    Ok(amt) => std::task::Poll::Ready(Ok(amt)),
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

pub fn implement_deref_read_buf(struct_name:&syn::Ident, trait_name: &syn::Ident) -> proc_macro2::TokenStream {
    quote::quote!{
        impl crate::future::io::AsyncBufRead for #struct_name {
            fn poll_fill_buf(mut self:Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<std::io::Result<&[u8]>> {
                use std::io::BufRead;

                match self.#trait_name.fill_buf() {
                    Ok(b) => std::task::Poll::Ready(Ok(b)),
                    Err(e) => if e.kind() == std::io::ErrorKind::WouldBlock {
                        cx.waker().wake_by_ref();
                        std::task::Poll::Pending
                    } else {
                        std::task::Poll::Ready(Err(e))
                    }
                }
            }

            fn consume(&mut self, amt:usize) {
                use std::io::BufRead;
                self.#trait_name.consume(amt);
            }
        } 
    }
}