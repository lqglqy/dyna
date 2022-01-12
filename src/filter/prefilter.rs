use aho_corasick::AhoCorasickBuilder;
use aho_corasick::AhoCorasick;
use super::rule::Rule;

pub struct Prefilter<'s> {
    ac: AhoCorasick,
    rules: Vec<&'s Rule>,
    keywords: Vec<String>
}

impl<'s> Prefilter<'s> {
    pub(crate) fn new(rules: &'s Vec<Rule>) -> Self {
        let mut ks = Vec::new();
        let mut rs = Vec::new();
        for r in rules {
            ks.push(r.get_keyword());
            rs.push(r);
        }
        Prefilter {
            ac: AhoCorasickBuilder::new()
            .dfa(true)
            .build(ks.clone()),
            rules: rs,
            keywords: ks
        }
    }

    pub fn find(&mut self, s: String) {
        let mat = self.ac.find(s).expect("should have a match");
        println!("match {}", mat.pattern());
    }

    pub fn find_all(&mut self, s: String) {
        for mat in self.ac.find_iter(&s) {
            println!("pattern {} start {} end {} ",mat.pattern(), mat.start(), mat.end());
            println!("keyword {}", self.rules[mat.pattern()].get_keyword());
            println!("keywords {}", self.keywords[mat.pattern()]);
        }
    }

}

