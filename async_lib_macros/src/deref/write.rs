pub fn implement_deref_write(struct_name:&syn::Ident, trait_name: &syn::Ident) -> proc_macro2::TokenStream {
    quote::quote!{
        use crate::future::io::{PollWrite, AsyncWrite};
        impl PollWrite for #struct_name {
            fn poll_write(&mut self, cx: &mut std::task::Context<'_>, buf: &[u8]) -> std::task::Poll<std::io::Result<usize>> {
                use std::io::Write;
                match self.#trait_name.write(buf) {
                    Ok(amt) => std::task::Poll::Ready(Ok(amt)),
                    Err(e) => if e.kind() == std::io::ErrorKind::WouldBlock {
                        cx.waker().wake_by_ref();
                        std::task::Poll::Pending
                    } else {
                        std::task::Poll::Ready(Err(e))
                    }
                }
            }

            fn poll_flush(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<std::io::Result<()>> {
                use std::io::Write;
                match self.#trait_name.flush() {
                    Ok(_) => std::task::Poll::Ready(Ok(())),
                    Err(e) => if e.kind() == std::io::ErrorKind::WouldBlock {
                        cx.waker().wake_by_ref();
                        std::task::Poll::Pending
                    } else {
                        std::task::Poll::Ready(Err(e))
                    }
                }
            }

            fn poll_close(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<std::io::Result<()>> {
                self.poll_flush(cx)
            }
        } 

        impl AsyncWrite for #struct_name {}
    }
}