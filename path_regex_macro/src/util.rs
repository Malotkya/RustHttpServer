use std::collections::HashMap;
use proc_macro2::Span;
use syn::parse::ParseStream;
use syn::Error;

pub struct InputParser(HashMap<String, syn::Lit>);

impl InputParser {
    pub fn new(input:ParseStream) -> Self{
        let mut map:HashMap<String, syn::Lit> = HashMap::new();

        loop {
            let key: syn::Ident = input.parse().unwrap();
            let _: syn::Token![=] = input.parse().unwrap();
            let value: syn::Lit = input.parse().unwrap();

            map.insert(key.to_string(), value);

            if input.is_empty() {
                break;
            }
        }

        Self {
            0: map
        }
    }

    pub fn get_string<'a>(&self, key:&'a str) -> Result<String, syn::Error> {
        match self.0.get(key) {
            Some(literal) => {
                match literal {
                    syn::Lit::Str(str) => Ok(str.value()),
                    _ => Err(Error::new(literal.span(), format!("Expected a stirng literal for {}!", key)))
                }
            },
            None => Err(Error::new(Span::call_site(), format!("Missing string literal for {}!", key)))
        }
    }

    pub fn get_bool<'a>(&self, key:&'a str) -> Result<bool, syn::Error> {
        match self.0.get(key) {
            Some(literal) => {
                match literal {
                    syn::Lit::Bool(bool) => Ok(bool.value()),
                    _ => Err(Error::new(literal.span(), format!("Expected a boolean for {}!", key)))
                }
            },
            None => Err(Error::new(Span::call_site(), format!("Missing boolean for {}!", key)))
        }
    }

}