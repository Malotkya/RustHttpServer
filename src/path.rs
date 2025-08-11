/** /path
 * 
 * Based on path-to-regex:
 * https://github.com/pillarjs/path-to-regexp
 * 
 * @author Alex Malotky
 */
use std::collections::HashMap;

#[allow(dead_code)]
pub struct Path<'k, const N:usize> {
    pub regex: regex::Regex,
    pub keys: [&'k str; N], 
}

pub fn match_path<'a, 'k, const N:usize>(pathname:&'a str, path:&'a Path<'k, N>) -> Option<HashMap<&'k str, &'a str>>{
    match path.regex.captures(pathname) {
        Some(caps) => {
            let mut map = HashMap::new();
            let (_, matches) = caps.extract() as (&str, [&str; N]);
            
            for i in 0..N {
                map.insert(path.keys[i], matches[N]);
            }

            Some(map)
        },
        None => None
    }
}