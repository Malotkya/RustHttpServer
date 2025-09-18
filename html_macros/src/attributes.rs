use syn::parse::{Parse, ParseStream};

pub(crate) struct AttributeProps {
    function_name: syn::Ident,
    literal: syn::LitStr,
    return_type: syn::Type
}

pub(crate) struct AttributePropsList(Vec<AttributeProps>);

impl Parse for AttributePropsList {
    fn parse(input:ParseStream) -> syn::Result<Self> {

        let mut vec = Vec::new();

        while !input.is_empty() {
            
            let function_name = input.parse::<syn::Ident>()?;
            input.parse::<syn::Token![:]>()?;

            let next;
            syn::parenthesized!(next in input);

            let literal = next.parse::<syn::LitStr>()?;
            next.parse::<syn::Token![,]>()?;

            let return_type = next.parse::<syn::Type>()?;

            vec.push(AttributeProps { 
                function_name, literal, return_type
            });

            if !input.is_empty() {
                input.parse::<syn::Token![,]>()?;
            }
        }

        if input.is_empty() {
            Ok(Self(vec))
        } else {
            Err(input.error(format!("{:?}", input)))
        }
    }
}

fn is_boolean(value: &syn::Type) -> bool {
    let ident = match value {
        syn::Type::Path(p) => p.path.get_ident(),
        syn::Type::Paren(a) => return is_boolean(&a.elem),
        syn::Type::Ptr(ptr) => return is_boolean(&ptr.elem),
        syn::Type::Reference(r) => return is_boolean(&r.elem),
        _ => return false
    };

    if let Some(value) = ident {
        value.to_string() == "bool"
    } else {
        false
    }
}

fn build_boolean_getter_setter(value: AttributeProps) -> proc_macro2::TokenStream {
    let AttributeProps{function_name, literal, ..} = value;
    
    let getter_name = syn::Ident::new(
        &format!("get_{}", function_name.to_string()),
        function_name.span()
    );

    let setter_name = syn::Ident::new(
        &format!("toggle_{}", function_name.to_string()),
        function_name.span()
    );
    
    quote::quote! {
        pub fn #setter_name(&self, value:Option<bool>) {
            let value = !value.unwrap_or(
                self.#getter_name()
                    .unwrap_or(false)
            );

            let mut interanl = self.0.borrow_mut();
            for att in &mut interanl.attributes {
                if att.key() ==  #literal {
                    att.toggle_value(value);
                    return;
                }
            }
        }

        pub fn #getter_name(&self) -> Option<bool> {
            let interanl = self.0.borrow();
            for att in & interanl.attributes {
                if att.key() == #literal {
                    return Some(
                        att.value().parse()
                    )
                }
            }

            None
        }
    }
}

fn build_getter_setter(value: AttributeProps) -> proc_macro2::TokenStream {
    let AttributeProps{function_name, literal, return_type} = value;
    
    let getter_name = syn::Ident::new(
        &format!("get_{}", function_name.to_string()),
        function_name.span()
    );

    let setter_name = syn::Ident::new(
        &format!("toggle_{}", function_name.to_string()),
        function_name.span()
    );

    quote::quote! {
        pub fn #setter_name(&self, value:#return_type) -> Option<#return_type>{
            let mut interanl = self.0.borrow_mut();
            for att in &mut interanl.attributes {
                if att.key() ==  #literal {
                    return Some(
                        att.set_value(value).parse()
                    )
                }
            }

            None
        }

        pub fn #getter_name(&self) -> Option<bool> {
            let interanl = self.0.borrow_mut();
            for att in & interanl.attributes {
                if att.key() == #literal {
                    return Some(
                        att.value().parse()
                    )
                }
            }

            None
        }
    }
}


pub(crate) fn build_attributes(input:proc_macro::TokenStream) -> proc_macro2::TokenStream {
    let list: AttributePropsList = match syn::parse(input) {
        Ok(v) => v,
        Err(e) => panic!("{}", e)
    };
    let mut output = quote::quote!();

    for arg in list.0 {
        if is_boolean(&arg.return_type) {
            output.extend(
                build_boolean_getter_setter(arg)
            );
        } else {
            output.extend(
                build_getter_setter(arg)
            );
        }
    }

    output
}