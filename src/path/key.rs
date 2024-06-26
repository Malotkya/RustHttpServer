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

pub type KeyToRegex = Box<dyn FnMut(Key)->String>;

///Key To Regex Generator
pub fn key_to_regexp_gen(delimiter:String)->KeyToRegex {
    let segment_pattern = format!("[^{}]+?", escape(delimiter));
    return Box::new(move |key:Key|->String {
        let prefix = escape(key.prefix);
        let suffix = escape(key.suffix);
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
                let split = escape(seperator);
                return format!("(?:{}((?:{})(?:{}(?:{}))*){}){}", prefix, pattern, split, pattern, suffix, modifier);
            } else {
                return format!("(?:{}({}){}){}", prefix, pattern, suffix, key.modifier);
            }
        }

        return format!("(?:{}{}){}", prefix, suffix, key.modifier);
    })
}