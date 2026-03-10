use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};

pub(crate) struct ElementData {
    tag_name: String,
    attributes: Vec<Attribute>,
    children: Vec<TokenStream>
}

enum Attribute {
    Value(syn::Path),
    KeyValue(String, syn::Lit)
}

impl Parse for ElementData {
    fn parse(input:ParseStream) -> syn::Result<Self> {
        
        let tag_name = input.parse::<syn::LitStr>()?.value();

        if input.is_empty() {
            return Ok(ElementData {
                tag_name,
                attributes: Vec::new(),
                children: Vec::new()
            })
        }
        input.parse::<syn::Token![,]>()?;

        let mut attributes:Vec<Attribute> = Vec::new();
        if input.peek(syn::token::Bracket) {
            let attributes_list;
            syn::bracketed!(attributes_list in input);

            while !attributes_list.is_empty() {
                if let Ok(value) = attributes_list.fork().parse::<syn::Path>() {
                    attributes.push(Attribute::Value(value));

                } else {
                    let key = attributes_list.parse::<syn::LitStr>()?.value();
                    attributes_list.parse::<syn::Token![,]>()?;
                    let value = attributes_list.parse::<syn::Lit>()?;
                    attributes.push(Attribute::KeyValue(key, value))
                }
            }
            
        }

        let children:Vec<TokenStream> = syn::punctuated::Punctuated::<TokenStream, syn::Token![,]>::parse_terminated(input)?
            .into_iter().collect();
        
        Ok(Self { tag_name, attributes, children })
    }
}

const VOID_ELEMENTS:[&'static str; 13] = [
    "area",
    "base",
    "br",
    "col",
    "embed",
    "hr",
    "img",
    "input",
    "link",
    "meta",
    "source",
    "track",
    "wbr"
];

fn choose_constructor(tagname:&str) -> TokenStream {
    let mut output = quote::quote!(html_type::element::Element);

    if VOID_ELEMENTS.contains(&tagname.to_lowercase().trim()) {
        output.extend(quote::quote!(::new_void));
    } else {
        output.extend(quote::quote!(::new));
    }

    return output;
}

pub(crate) fn build_element(input:ElementData)->TokenStream {
    let ElementData {tag_name, attributes, children} = input;
    let constructor = choose_constructor(&tag_name);
    let attributes:Vec<TokenStream> = attributes.iter().map(|att|match att{
        Attribute::Value(path) => quote::quote!(#path.into()),
        Attribute::KeyValue(key, value) => quote::quote!(html_type::Attribute::new(#key, #value))
    }).collect();

    quote::quote!{
        #constructor(
            #tag_name,
            vec![ #( #attributes )* ],
            vec![ #( #children )* ]
        )
    }
}