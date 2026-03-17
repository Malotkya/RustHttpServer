pub fn implement_deref_write(struct_name:&syn::Ident, struct_generics:&syn::Generics, trait_name:&proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    quote::quote!{
        impl #struct_generics crate::future::io::AsyncWrite for #struct_name #struct_generics{
            fn poll_write(mut self:std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>, buf: &[u8]) -> std::task::Poll<std::io::Result<usize>> {
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

            fn poll_flush(mut self:std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<std::io::Result<()>> {
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

            fn poll_close(self:std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<std::io::Result<()>> {
                self.poll_flush(cx)
            }
        } 
    }
}