use std::str;
use std::str::FromStr;
pub use fancy_regex::Error;

extern crate base64;

#[derive(Clone)]
pub struct Regex(fancy_regex::Regex);

impl FromStr for Regex {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let ss = base64::decode(s).unwrap();
        ::fancy_regex::RegexBuilder::new(str::from_utf8(&ss).unwrap())
            //.unicode(false)
            .build()
            .map(Regex)
    }
}

impl Regex {
    pub fn is_match(&self, text: &[u8]) -> bool {
        self.0.is_match(str::from_utf8(text).unwrap()).unwrap()
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
