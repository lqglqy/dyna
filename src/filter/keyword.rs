use aho_corasick::AhoCorasickBuilder;
use aho_corasick::AhoCorasick;
use std::collections::HashMap;
use super::result::MatchResult;

#[derive(PartialEq, Eq, Clone)]
pub struct Keyword {
    id: i64,
    rid: String,
    func: String,
}
#[derive(Default, Clone)]
pub struct KeywordFilter {
    ac_filter: Option<AhoCorasick>,
    keyword_vec: Vec<String>,
    keyword_map: HashMap<String, Vec<Keyword>>,
}
fn keyword_func_exec(func: &String, content: &String, start: usize, end: usize) -> bool {
    //TODO: func exec 
    println!("exec func: {} content: {} start: {} end: {} ", func, content, start, end);
    return true
}
impl KeywordFilter {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build(&mut self){
        for (k, _) in &self.keyword_map{
            self.keyword_vec.push(k.clone());
        }
        self.ac_filter = Some(AhoCorasickBuilder::new().dfa(true).build(&self.keyword_vec));
        return;
    }
    pub fn find_all(&self, mctx: & mut MatchResult, content: &String, feild: &String) {
        match &self.ac_filter{
            Some(ac) => {
                for mat in ac.find_iter(content) {
                    println!("match: {}", self.keyword_vec[mat.pattern()]);
                    let k = self.keyword_map.get(&self.keyword_vec[mat.pattern()]);
                    match k {
                        Some(kk) => {
                            for kw in kk {
                                if keyword_func_exec(&kw.func, content, mat.start(), mat.end()) {
                                    mctx.add_keyword(self.keyword_vec[mat.pattern()].clone(), kw.id, kw.rid.clone(), feild.clone());
                                }
                            }
                        },
                        None => {
                            println!("cannot find {} from keymap", self.keyword_vec[mat.pattern()]);
                        }
                    }
                }
            },
            None => {
                println!("not init ac");
            }
        }
    }
    pub fn add(&mut self, kid: i64, rid: String, kword: String, func: String) {
        let k = self.keyword_map.entry(kword).or_insert(vec![]);
        k.push(Keyword{
            id: kid,
            rid: rid,
            func: func
        })
    }
}