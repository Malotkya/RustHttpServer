use std::collections::HashMap;
use proc_macro2::Span;
use syn::parse::ParseStream;
use syn::Error;
use paste::paste;

macro_rules! get_value {
    ($type:ty, $name:ident, $error:literal) => {
        paste!{
            get_value!($type, $name, $error, |x:&syn::[<Lit $name>]|->Result<$type, syn::Error> {Ok(x.value())});
        }
    };
    ($type:ty, $name:ident, $error:literal, $helper:expr) => {
        paste!{
            pub fn [<get_ $type:lower>](&self, key:&str) -> Result<$type, syn::Error>{
                match self.0.get(key) {
                Some(literal) => match literal {
                    syn::Lit::$name(v) => Ok($helper(v)?),
                    _ => Err(
                        Error::new(
                            literal.span(),
                            format!("Expected {} for {}!", $error, key)
                        )
                    )
                },
                None => Err(
                    Error::new(
                        Span::call_site(),
                        format!("Missing {} for {}!", $error, key))
                    )
                }
            }
        }
    };
}

pub struct InputParser(HashMap<String, syn::Lit>);

impl InputParser {
    pub fn new(input:ParseStream) -> Result<Self, syn::Error>{
        let mut map:HashMap<String, syn::Lit> = HashMap::new();

        loop {
            let key: syn::Ident = input.parse()?;
            let _: syn::Token![=] = input.parse()?;
            let value: syn::Lit = input.parse()?;

            map.insert(key.to_string(), value);

            if input.is_empty() {
                break;
            } else {
                input.parse::<syn::Token![,]>()?;
            }
        }

        Ok(Self(map))
    }

    get_value!(String, Str, "string literal");
    get_value!(bool, Bool, "boolean");
    get_value!(u16, Int, "u16", |x: &syn::LitInt|x.base10_parse());

}