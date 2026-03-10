use std::fmt;
pub use html_macros::{js_function, ts_function};

pub struct JsFunction {
    pub is_async: bool,
    pub name: Option<&'static str>,
    pub args: Vec<&'static str>,
    pub body: &'static str,
    pub source_map: Option<&'static str>
}

impl JsFunction {
    pub fn len(&self) -> usize {
        self.args.len()
    }
}

impl fmt::Display for JsFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}({}){{{}}}",
            if self.is_async {
                "async "
            } else {
                ""
            },
            self.name.unwrap_or("anonymous"),
            self.args.join(", "),
            self.body
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macro_test() {
        let f = super::ts_function!{
            const sleep = (n:number) => new Promise((res)=>{
                setTimeout(n, res);
            })
        };
    }
}