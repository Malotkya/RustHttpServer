use syn::parse::ParseStream;

pub struct HeaderInput(Vec<(syn::Ident, syn::LitStr)>);

impl syn::parse::Parse for HeaderInput {
    fn parse(input:ParseStream) -> syn::Result<Self> {
        let mut vec: Vec<_> = Vec::new();

        loop {
            let content;
            syn::parenthesized!(content in input);
            let _:syn::Token![;] = input.parse()?;

            let name: syn::Ident = content.parse()?;
            let _: syn::Token![,] = content.parse()?;
            let value: syn::LitStr = content.parse()?;

            vec.push((name, value));

            if input.is_empty() {
                break;
            }
        }

        Ok(Self(vec))
    }
}

pub fn generate_header_name_enums(input:HeaderInput) -> proc_macro2::TokenStream {
    let mut enum_values = quote::quote!();
    let mut name_values = quote::quote!();
    let mut string_values = quote::quote!();
    let mut number_values = quote::quote!();

    let mut i: u8 = 1;
    for (name, literal) in input.0 {
        let lower_case_literal = literal.value().to_lowercase();
        enum_values.extend(quote::quote!( #name, ));
        name_values.extend(quote::quote!( Self::#name => #literal, ));
        string_values.extend(quote::quote!( #lower_case_literal => Self::#name, ));
        number_values.extend(quote::quote!( HeaderName::#name => #i, ));
        i += 1
    }
    
    quote::quote! {
        pub enum HeaderName{
            CustomHeaderName(HeaderValue),
            #enum_values
        }

        impl HeaderName {
            pub fn name<'a>(&'a self) -> &'a str {
                match self {
                    Self::CustomHeaderName(name) => name.into(),
                    #name_values
                }
            }

            pub fn from(string:&str) -> Self {
                match string.to_lowercase().as_str() {
                    #string_values
                    _ => Self::CustomHeaderName(
                        HeaderValue::from(string)
                    )
                }
            }
        }

        impl Into<u8> for &HeaderName {
            fn into(self) -> u8 {
                match self {
                    HeaderName::CustomHeaderName(_) => 0,
                    #number_values
                }
            }
        }
    }
}