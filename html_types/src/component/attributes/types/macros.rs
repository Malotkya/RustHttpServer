macro_rules! AttributeEnum {
    (
        $enum_name:ident,
        Boolean
    ) => {
        $crate::component::attributes::AttributeEnum!(
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
        $enum_name:ident, $($default:ident, )?
        $( ($name:ident, $value:literal), )+
        Boolean
    ) => {
        $crate::component::attributes::AttributeEnum!(
            $enum_name, $($default, )?
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
        $enum_name:ident, $($default:ident, )?
        $( ($name:ident, $value:literal) ),+
    ) => {
        pub enum $enum_name {
            $( $name ),+
        }

        impl $enum_name {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$name => $value),+
                }
            }
        }

        $(
            impl Default for $enum_name {
                fn default() -> Self {
                    Self::$default
                }
            }
        )?

        impl ToString for $enum_name {
            fn to_string(&self) -> String {
                self.as_str().to_string()
            }
        }

        impl std::str::FromStr for $enum_name {
            type Err = String;

            fn from_str(value:&str) -> Result<Self, Self::Err> {
                match value.to_ascii_lowercase().as_str() {
                    $( $value => Ok(Self::$name), )*
                    _ => Err(format!("{} is not {}!", value, stringify!($enum_name)))
                }
            }
        }
    };
}

pub(crate) use AttributeEnum;