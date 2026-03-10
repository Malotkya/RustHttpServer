use std::{fmt, string::String};
use crate::node::Node;

#[derive(Clone, PartialEq)]
pub enum AttributeValue {
    String(String),
    Number(f64),
    Integer(i128),
    Boolean(bool)
}

impl Eq for AttributeValue {}

impl AttributeValue {
    pub(crate) fn is_truthy(&self) -> bool {
        match self {
            Self::String(str) => match str.trim().to_ascii_lowercase().as_str() {
                "false"  => false,
                "" => false,
                "0" => false,
                _ => true
            },
            Self::Number(num) => *num != 0.0,
            Self::Integer(int) => *int != 0,
            Self::Boolean(b) => *b
        }
    }

    pub (crate) fn clear(&mut self) {
        match self {
            Self::String(str) => str.clear(),
            Self::Number(num) => {
                let mut str = num.to_string();
                str.clear();
                *self = Self::String(str)
            },
            Self::Integer(int) => {
                let mut str = int.to_string();
                str.clear();
                *self = Self::String(str)
            },
            Self::Boolean(_) => *self = Self::String(String::with_capacity(5))
        }
    }
}

impl fmt::Display for AttributeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(v) => write!(f, "{}", v),
            Self::Number(n) => write!(f, "{}", n),
            Self::Integer(i) => write!(f, "{}", i),
            Self::Boolean(b) => if *b {
                write!(f, "true")
            } else {
                write!(f, "false")
            }
        }
    }
}

impl From<bool> for AttributeValue {
    fn from(value:bool) -> Self {
        Self::Boolean(value)
    }
}

/*impl<T:ToString> From<T> for AttributeValue {
    fn from(value: T) -> Self {
        Self::String(value.to_string())
    }
}*/

impl From<&str> for AttributeValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<char> for AttributeValue {
    fn from(value: char) -> Self {
        Self::String(String::from(value))
    }
}

impl From<String> for AttributeValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<u8> for AttributeValue {
    fn from(value: u8) -> Self {
        Self::Integer(value.into())
    }
}

impl From<u16> for AttributeValue {
    fn from(value: u16) -> Self {
        Self::Integer(value.into())
    }
}

impl From<u32> for AttributeValue {
    fn from(value: u32) -> Self {
        Self::Integer(value.into())
    }
}

impl From<u64> for AttributeValue {
    fn from(value: u64) -> Self {
        Self::Integer(value.into())
    }
}

impl From<i32> for AttributeValue {
    fn from(value: i32) -> Self {
        Self::Integer(value.into())
    }
}

impl From<i64> for AttributeValue {
    fn from(value: i64) -> Self {
        Self::Integer(value.into())
    }
}

impl From<f32> for AttributeValue {
    fn from(value: f32) -> Self {
        Self::Number(value.into())
    }
}

impl From<f64> for AttributeValue {
    fn from(value: f64) -> Self {
        Self::Number(value.into())
    }
}

impl From<usize> for AttributeValue {
    fn from(value: usize) -> Self {
        Self::Integer(value as i128)
    }
}

pub struct Attribute {
    pub name: String,
    pub value: AttributeValue
}

impl Attribute {
    pub fn new<Key:ToString, Value:Into<AttributeValue>>(name:Key, value:Value) -> Self {
        Self {
            name: name.to_string(),
            value: value.into()
        }
    }
}

impl Into<Node> for Attribute {
    fn into(self) -> Node {
        Node::Attribute(self.name, self.value)
    }
}

impl Into<Node> for &Attribute {
    fn into(self) -> Node {
        Node::Attribute(
            self.name.clone(),
            self.value.clone()
        )
    }
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            AttributeValue::String(str) => write!(f, "{}=\"{}\"", self.name, str),
            AttributeValue::Number(num,) => write!(f, "{}=\"{}\"", self.name, num),
            AttributeValue::Integer(int) => write!(f, "{}=\"{}\"", self.name, int),
            AttributeValue::Boolean(b) => if *b {
                write!(f, "{}", self.name)
            } else {
                Ok(())
            }
        }
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::string::ToString;

    #[test]
    fn string_attribute() {
        let att = Attribute::new("string", "Hello World!");

        assert_eq!(
            att.to_string().as_str(),
            "string=\"Hello World!\""
        );
    }

    #[test]
    fn integer_attribute() {
        let att = Attribute::new("integer", 537);

        assert_eq!(
            att.to_string().as_str(),
            "integer=\"537\""
        );
    }

    #[test]
    fn number_attribute() {
        let att = Attribute::new("number", 3.14);

        assert_eq!(
            att.to_string().as_str(),
            "number=\"3.14\""
        );
    }

    #[test]
    fn boolean_attribute() {
        let att_true = Attribute::new("boolean", true);

        assert_eq!(
            att_true.to_string().as_str(),
            "boolean"
        );

        let att_false = Attribute::new("boolean", false);

        assert_eq!(
            att_false.to_string().as_str(),
            ""
        );
    }

    #[test]
    fn attribute_type_enum() {
        let att:Attribute = types::AutoComplete::On.into();

        assert_eq!(
            att.to_string().as_str(),
            "autocomplete=\"on\""
        );
    }

    #[test]
    fn attribute_type_value() {
        let att:Attribute = types::AccessKey('s').into();

        assert_eq!(
            att.to_string().as_str(),
            "accesskey=\"s\""
        );
    }

     #[test]
    fn attribute_type_boolean() {
        let att_true:Attribute = types::Async::True.into();

        assert_eq!(
            att_true.to_string().as_str(),
            "async"
        );

        let att_false:Attribute = types::Async::False.into();

        assert_eq!(
            att_false.to_string().as_str(),
            ""
        );
    }
}

#[macro_export]
macro_rules! create_attributes {
    ( $($att_name:literal: $att_value:expr),+) => {
        vec![ $(
            $crate::attributes::Attribute{
                name: $att_name.to_string(),
                value: $att_value.into()
            }
        ),+ ]
    };
    () => {
        Vec::new()
    }
}


macro_rules! build_attribute_type {
    (
        $name:ident => $lit:literal;
        $( $key:ident => $value:literal ),+
    ) => {
        pub enum $name {
            $($key, )+
        }

        impl Into<AttributeValue> for $name {
            fn into(self) -> AttributeValue {
                match self {
                    $(Self::$key => AttributeValue::String(
                        $value.to_string()
                    ),)+
                }
            }
        }

        impl Into<Attribute> for $name {
            fn into(self) -> Attribute {
                Attribute {
                    name: String::from($lit),
                    value: self.into()
                }
            }
        }
    };
    (
        $name:ident => $lit:literal($type:ty)
    ) => {
        pub struct $name (pub $type);

        impl Into<AttributeValue> for $name {
            fn into(self) -> AttributeValue {
                AttributeValue::String(self.0.to_string())
            }
        }

        impl Into<Attribute> for $name {
            fn into(self) -> Attribute {
                Attribute {
                    name: String::from($lit),
                    value: self.into()
                }
            }
        }
    };
    (
        $name:ident => $lit:literal
    ) => {
        pub enum $name {
            True,
            False
        }

        impl Into<AttributeValue> for $name {
            fn into(self) -> AttributeValue {
                match self {
                    Self::True => AttributeValue::Boolean(true),
                    Self::False => AttributeValue::Boolean(false)
                }
            }
        }

        impl Into<Attribute> for $name {
            fn into(self) -> Attribute {
                Attribute {
                    name: String::from($lit),
                    value: self.into()
                }
            }
        }
    }
}

impl<I:Into<AttributeValue>> Into<AttributeValue> for Vec<I> {
    fn into(self) -> AttributeValue {
        AttributeValue::String(
            self.into_iter()
                .map(|value|value.into().to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

pub mod types {
    use super::*;

    build_attribute_type!(Accept=>"accept"(&'static str));
    build_attribute_type!(AccessKey=>"accesskey"(char));
    build_attribute_type!(Action=>"action"(&'static str));
    build_attribute_type!(Alpha=>"alpha");
    build_attribute_type!(AlternateText=>"alt"(&'static str));
    build_attribute_type!(
        As=>"as";
        Audio => "audio",
        Document => "document",
        Embed => "embed",
        Fetch => "fetch",
        Font => "font",
        Image => "image",
        Json => "json",
        Object => "object",
        Script => "script",
        Style => "style",
        Track => "track",
        Video => "video",
        Worker => "worker"
    );
    build_attribute_type!(Async=>"async");
    build_attribute_type!(
        AutoCapitalize => "autocapitalize";
        None => "none",
        Off => "off",
        Sentences => "sentences",
        On => "on",
        Words => "words",
        Characters => "characters"
    );
    build_attribute_type!(
        AutoComplete => "autocomplete";
        On => "on",
        Off => "off",
        Shipping => "shipping",
        Billing => "billing",
        Home => "home",
        Work => "work",
        Mobile => "mobile",
        Fax => "fax",
        Pager => "pager",
        Telephone => "tel",
        TelephoneContryCode => "tel-country-code",
        TelephoneNational => "tel-national",
        TelephoneAreaCode => "tel-area-code",
        TelephoneLocal => "tel-local",
        TelephoneExtension => "tel-extension",
        Email => "email",
        Impp => "impp",
        Name => "name",
        HonorificPrefix => "honorific-prefix",
        GivenName => "given-name",
        AdditionalName => "additional-name",
        FamilyName => "family-name",
        HonorifixSuffix => "honorific-suffix",
        Nickname => "nickname",
        Username => "username",
        NewPassword => "new-password",
        CurrentPassword => "current-password",
        OneTimeCode => "one-time-code",
        OrganizationTitle => "organization-title",
        Organization => "organization",
        StreetAddress => "street-address",
        StreetAddressLine1 => "address-line1",
        StreetAddressLine2 => "address-line2",
        StreetAddressLine3 => "address-line3",
        AddressLevel1 => "address-level1",
        AddressLevel2 => "address-level2",
        AddressLevel3 => "address-level3",
        AddressLevel4 => "address-level4",
        Country => "coutry",
        CountryName => "country-name",
        PostalCode => "postal-code",
        CreditCardName => "cc-name",
        CreditCardGivenName => "cc-given-name",
        CreditCardAdditionalName => "cc-additional-name",
        CreditCardFamilyName => "cc-family-name",
        CreditCardNumber => "cc-number",
        CreditCardExperation => "cc-exp",
        CreditCardSecurityCode => "cc-csc",
        CreditCardType => "cc-type",
        TransactionCurrency => "transaction-currency",
        TransactionAmount => "transaction-amount",
        Language => " language",
        Birthday => "bday",
        BirthdayDay => "bday-day",
        BirthdayMonth => "bday-month",
        BirthdayYear => "bday-year",
        Sex => "sex",
        Url => "url",
        Photo => "photo",
        WebAuthentication => "webauthn"
    );
    build_attribute_type!(AutoPlay=>"autoplay");
    //Capture
}

