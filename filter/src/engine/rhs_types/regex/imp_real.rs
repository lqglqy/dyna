use std::str;
use std::str::FromStr;
pub use pcre2::Error;

extern crate base64;

#[derive(Clone)]
pub struct Regex(pcre2::bytes::Regex);

impl FromStr for Regex {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let ss = base64::decode(s).unwrap();
        
        ::pcre2::bytes::RegexBuilder::new()
        .caseless(true)
        .jit(true)
        .build(str::from_utf8(&ss).unwrap())
        .map(Regex)

        /*
        ::fancy_regex::RegexBuilder::new(str::from_utf8(&ss).unwrap())
            //.unicode(false)
            .build()
            .map(Regex)
            */
    }
}

impl Regex {
    pub fn is_match(&self, text: &[u8]) -> bool {
        //self.0.is_match(str::from_utf8(text).unwrap()).unwrap()
        self.0.is_match(text).unwrap()
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
