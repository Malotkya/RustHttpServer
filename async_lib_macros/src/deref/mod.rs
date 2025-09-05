use syn::parse::{Parse, ParseStream};
use std::collections::HashSet;

pub struct DerefArgs(Vec<syn::Ident>);

mod read;
mod write;
mod seek;

impl Parse for DerefArgs {
    fn parse(input:ParseStream) -> syn::Result<Self> {
        let mut list = Vec::new();

        while !input.is_empty() {
            list.push(input.parse()?);

            if !input.is_empty() {
                input.parse::<syn::Token![,]>()?;
            }
        }

        Ok(Self(list))
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum AsyncTraits {
    Read,
    Write,
    Seek,
    BufRead
}

impl DerefArgs {
    pub fn validate(&self) -> syn::Result<HashSet<AsyncTraits>> {
        let mut set = HashSet::with_capacity(self.0.len());

        for ident in self.0.iter() {
            let name = ident.to_string();
            match name.to_ascii_lowercase().as_str() {
                "read"    => set.insert(AsyncTraits::Read),
                "write"   => set.insert(AsyncTraits::Write),
                "seek"    => set.insert(AsyncTraits::Seek),
                "bufread" => set.insert(AsyncTraits::BufRead),
                _ => return Err(
                    syn::Error::new(
                        ident.span(),
                        format!("Unknown Async Trait: {}", name)
                    )
                )
            };
        }

        Ok(set)
    }
}

fn find_inner_trait(list:&syn::Fields) -> Option<syn::Ident> {
    let mut it = list.iter();
    let first = it.next().map(|a|a.ident.as_ref()).flatten();

    if let Some(ident) = first {
        match ident.to_string().as_str() {
            "io" | "inner" => return Some(ident.clone()),
            _ => {}
        }
    }

    while let Some(next) = it.next().map(|a|a.ident.as_ref()).flatten() {
        match next.to_string().as_str() {
            "io" | "inner" => return Some(next.clone()),
            _ => {}
        }
    }

    first.map(|i|i.clone())
}

pub fn implement_deref_traits(list: HashSet<AsyncTraits>, item: syn::ItemStruct) -> proc_macro2::TokenStream {
    let struct_name = &item.ident;
    let trait_name = match find_inner_trait(&item.fields) {
        Some(ident) => ident,
        None => panic!("Unable to find inner trait!")
    };

    //panic!("{}", trait_name);

    let mut output = quote::quote!(#item);

    for item in list {
        match item {
            AsyncTraits::Read => output.extend(read::implement_deref_read(struct_name, &trait_name)),
            AsyncTraits::BufRead => output.extend(read::implement_deref_read_buf(struct_name, &trait_name)),
            AsyncTraits::Write => output.extend(write::implement_deref_write(struct_name, &trait_name)),
            AsyncTraits::Seek => output.extend(seek::implement_deref_seek(struct_name, &trait_name))
        }
    }

    //panic!("{}", output);

    output
}