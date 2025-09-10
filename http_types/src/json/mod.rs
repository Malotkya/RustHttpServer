use core::fmt;
pub use reference::JsonRef;

mod number;
mod array;
mod integer;
mod string;
mod object;
mod boolean;
mod reference;



pub(crate) fn gen_indent(value:Option<usize>) -> (String, String) {
    match value {
        Some(size) => (
            format!("\n{:>size$}", " "),
            format!(",\n{:>size$}", " "),
        ),
        None => (
            String::from(" "),
            String::from(", ")
        )
    }
}

#[derive(Clone, PartialEq)]
pub enum JsonValue {
    Boolean(bool),
    Integer(integer::Type),
    Number(number::Type),
    String(string::Type),
    Array(array::Type),
    Object(Option<object::Type>),
    Undefined
}

pub enum JsonError {
    NotABoolean(&'static str),
    NotAnInteger(&'static str),
    NotANumber(&'static str),
    NotAString(&'static str),
    NotAnArray(&'static str),
    NotAnObject(&'static str)
}

impl fmt::Debug for JsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (expected, actual) = match self {
            Self::NotABoolean(v) => ("boolean", v),
            Self::NotAnInteger(v) => ("integer", v),
            Self::NotANumber(v) => ("number", v),
            Self::NotAString(v) => ("string", v),
            Self::NotAnArray(v) => ("array", v),
            Self::NotAnObject(v) => ("object", v)
        };

        write!(f, "JsonError{{ expected:{expected}, actual:{actual}}}")
    }
}

impl JsonValue {
    pub fn from<T>(value:&T) -> Self where T: Into<Self> + Clone {
        value.clone().into()
    }

    pub fn to_ref<'a>(&'a self) -> JsonRef<'a> {
        self.into()
    }

    pub fn null() -> Self {
        Self::Object(None)
    }
    
    fn stringify(&self, indent:Option<usize>) -> String {
        match self {
            Self::Array(a) => array::stringify(a, indent),
            Self::Object(Some(o)) => object::stringify(o, indent),
            _ => self.to_string()
        }
    }

    pub fn type_of(&self) -> &'static str {
        match self {
            Self::Undefined => string::UNDEFINED,
            Self::Object(None) => string::NULL,
            Self::Boolean(_) => "boolean",
            Self::Integer(_) => "integer",
            Self::Number(_) => "number",
            Self::String(_) => "string",
            Self::Array(_) => "array",
            Self::Object(_) => "object"
        }
    }
    
}

pub struct Json(object::Type);

impl Json {
    pub fn set<K, I>(&mut self, key:&str, value:I)
        where I: Into<JsonValue> {
        self.0.insert(key.to_string(), value.into());
    }

    pub fn get<K>(&self, key:&str) -> Option<&JsonValue> {
        
        self.0.get(key)
    } 

    pub fn delete<K>(&mut self, key:&str) -> Option<JsonValue>{
        self.0.remove(key)
    }

    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.0.keys()
    }

    pub fn values<'a>(&'a self) -> impl Iterator<Item = JsonRef<'a>> {
        self.0.values().map(|v|v.into())
    }

    pub fn stringify(&self, indent:Option<usize>) -> String {
        object::stringify(&self.0, indent)
    }

    pub fn opjectify(_string:&str) -> Self {
        todo!("Objectify not yet implemented!")
    }
}
