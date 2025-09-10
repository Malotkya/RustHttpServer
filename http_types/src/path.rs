use std::collections::HashMap;

#[allow(dead_code)]
pub struct Path<'k, const N:usize> {
    pub regex: regex::Regex,
    pub keys: [&'k str; N], 
}

impl<'k, const N:usize> Path<'k, N> {
    pub fn match_path<'a>(&self, pathname: &'a str) -> Option<HashMap<&'k str, &'a str>> {
        match self.regex.captures(pathname) {
            Some(caps) => {
                let mut map = HashMap::new();
                let (_, matches) = caps.extract() as (&str, [&str; N]);
                
                for i in 0..N {
                    map.insert(self.keys[i], matches[N]);
                }

                Some(map)
            },
            None => None
        }
    }
}