macro_rules! AttributeEnum {
    /* Empty Boolean */
    (   
        $enum_name:ident,
        Boolean
    ) => {

        $crate::component::attributes::AttributeEnum!(
            $enum_name, False,
            Boolean
        );
    };

    /* Boolean Without Default */
    (   
        $enum_name:ident, 
        $( ($name:ident, $value:literal), )+
        Boolean
    ) => {
        $crate::component::attributes::AttributeEnum!(
            $enum_name, False,
            $( ($name, $value), )+
            Boolean
        );
    };

    /* Full Boolean With Default */
    (
        $enum_name:ident, $default:ident,
        $( ($name:ident, $value:literal), )*
        Boolean
    ) => {
        $crate::component::attributes::AttributeEnum!(
            $enum_name, $default,
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

    /* Normal Enum Without Default */
    (   
        $enum_name:ident, 
        $( ($name:ident, $value:literal) ),+
    ) => {
        $crate::component::attributes::AttributeEnum!(
            $enum_name, _Blank,
            $( ($name, $value), )+
            (_Blank, "")
        );
    };

    /* Normal Enum With Default*/
    (
        $enum_name:ident, $default:ident,
        $( ($name:ident, $value:literal) ),+
    ) => {
        #[derive(Clone, PartialEq)]
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

        impl $crate::component::attributes::ToAttributeValue for $enum_name {
            fn into_value(&self) -> $crate::component::attributes::AttributeValue {
                $crate::component::attributes::AttributeValue::String(self.as_str().to_owned())
            }
        }

        impl $crate::component::attributes::FromAttribteValue for $enum_name {
            fn parse_from(value: &$crate::component::attributes::AttributeValue) -> Self {
                match value.as_str().to_ascii_lowercase().as_str() {
                    $( $value => Self::$name, )*
                    _ => Self::$default
                }
            }
        }

        impl Default for $enum_name {
            fn default() -> Self {
                Self::$default
            }
        }
    };
}

pub(crate) use AttributeEnum;