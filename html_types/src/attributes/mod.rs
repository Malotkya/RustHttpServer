mod aria;
mod types;

trait Attribute {
    fn generate(&self, name:&str) -> String;
}

impl Attribute for String {
    fn generate(&self, name:&str) -> String {
        let mut output = String::with_capacity(self.len() + name.len() + 3);
        
        output.push_str(name);
        output.push_str("=\"");
        output.push_str(&self);
        output.push('"');

        output
    }
}

macro_rules! AttributeEnum {
    (
        $enum_name:ident,
        Boolean
    ) => {
        AttributeEnum!(
            $enum_name,
            (True, "true"),
            (False, "false")
        );

        impl From<bool> for $enum_name {
            fn from(value: bool) -> Self {
                if value {
                    Self::True
                } else {
                    Self::False
                }
            }
        }
    };
    (
        $enum_name:ident,
        $( $default:ident, )?
        $( ($name:ident, $value:literal), )*
        Boolean
    ) => {
        AttributeEnum!(
            $enum_name, $($default,)?
            $( ($name, $value), )*
            (True, "true"),
            (False, "false")
        );

        impl From<bool> for $enum_name {
            fn from(value: bool) -> Self {
                if value {
                    Self::True
                } else {
                    Self::False
                }
            }
        }
    };
    (
        $enum_name:ident,
        $( $default:ident, )?
        $( ($name:ident, $value:literal) ),*
    ) => {
        pub enum $enum_name {
            $( $name ),+
        }

        impl $enum_name {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$name => $value),*
                }
            }
        }

        impl crate::attributes::Attribute for $enum_name {
            fn generate(&self, name:&str) -> String {
                let value = self.as_str();
                let mut output = String::with_capacity(value.len() + name.len() + 3);

                output.push_str(name);
                output.push_str("=\"");
                output.push_str(value);
                output.push('"');

                output
            }
        }

        impl ToString for $enum_name {
            fn to_string(&self) -> String {
                self.as_str().to_string()
            }
        }

        impl TryFrom<&str> for $enum_name {
            type Error = String;

            fn try_from(value:&str) -> Result<Self, String> {
                match value.to_ascii_lowercase().as_str() {
                    $( $value => Ok(Self::$name), )*
                    _ => Err(format!("{} is not {}!", value, stringify!($enum_name)))
                }
            }
        }

        impl TryFrom<String> for $enum_name {
            type Error = String;

            fn try_from(value: String) -> Result<Self, String> {
                TryInto::<Self>::try_into(value.as_str())
            }
        }

        $(
            impl Default for $enum_name {
                fn default() -> Self {
                    Self::$default
                }
            }
        )?
    };
}

pub(crate) use AttributeEnum;