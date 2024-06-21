/** /path/keys
 * 
 * @author Alex Malotky
 */
use crate::path::escape;

#[derive(Debug, Clone)]
pub struct Key {
    pub name: String,
    pub prefix: String,
    pub suffix: String,
    pub pattern: String,
    pub modifier: String,
    pub separator: Option<String>
}

pub type KeyToRegex<'a> = Box<dyn FnMut(Key)->String + 'a>;

///Key To Regex Generator
pub fn key_to_regexp_gen(stringify:&Box<dyn Fn(String)->String>, delimiter:String)->KeyToRegex {
    let segment_pattern = format!("`[^{}]+?", escape(delimiter));
    let stringify = stringify.clone();
    return Box::new(move |key:Key|->String {
        let prefix = stringify(key.prefix);
        let suffix = stringify(key.suffix);
        let seperator = key.separator.unwrap_or(String::new());

        if !key.name.is_empty() {
            let pattern = if key.pattern.is_empty() {
                segment_pattern.clone()
            } else {
                key.pattern
            };

            if !seperator.is_empty() {
                let modifier = if seperator == "*" {
                    String::from("?")
                } else {
                    String::new()
                };
                let split = stringify(seperator);
                return format!("(?:{}((?:{})(?:{}(?:{}))*){}){}", prefix, pattern, split, pattern, suffix, modifier);
            } else {
                return format!("(?:{}({}){}){}", prefix, pattern, suffix, key.modifier);
            }
        }

        return format!("`(?:{}{}){}", prefix, suffix, key.modifier);
    })
}