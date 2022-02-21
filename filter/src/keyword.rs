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
fn _boundary_char_check(ch: char) -> bool {
    if (ch >= '0' && ch <= '9')
                        || (ch >= 'a' && ch <= 'z')
                        || (ch >= 'A' && ch <= 'Z')
                        || (ch == '_') {
                return false;
        }
        return true;

}
enum BoundaryCheck {
    Right,
    Left,
}
fn _boundary_check(payload: &String, start: usize, end: usize, bc: BoundaryCheck) -> bool {
    match bc {
        BoundaryCheck::Right => {
            if end == payload.len() - 1 {
                return true
            }
            match payload.chars().nth(end) {
                Some(c) => {
                    return _boundary_char_check(c)
                },
                _ => {
                    return false
                }
            }
        },
        BoundaryCheck::Left => {
            if start == 0 {
                return true
            }
            match payload.chars().nth(start - 1) {
                Some(c) => {
                    return _boundary_char_check(c)
                },
                _ => {
                    return false
                }
            }
        }
    }
}
fn keyword_func_exec(func: &String, payload: &String, start: usize, end: usize) -> bool {
    //TODO: func exec 
    //println!("exec func: {} content: {} start: {} end: {} ", func, payload, start, end);
    match func.as_str() {
        "right" => {
           return _boundary_check(payload, start, end, BoundaryCheck::Right); 
        },
        "left" => {
           return _boundary_check(payload, start, end, BoundaryCheck::Left); 
        },
        "both" => {
            if _boundary_check(payload, start, end, BoundaryCheck::Right) {
                if _boundary_check(payload, start, end, BoundaryCheck::Left) {
                    return true
                }
            }
            false
        },
        "none" => {
            true
        },
        _ => {
            true
        }
    }
}
impl KeywordFilter {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build(&mut self){
        for (k, _) in &self.keyword_map{
            //println!("push keyword: {}", k.clone());
            self.keyword_vec.push(k.clone());
        }
        self.ac_filter = Some(AhoCorasickBuilder::new()
                                .ascii_case_insensitive(true)
                                .dfa(true).build(&self.keyword_vec));
        return;
    }
    pub fn find_all(&self, mctx: & mut MatchResult, content: &String, feild: &String) {
        println!("find {} in {}", content.clone(), feild.clone());
        match &self.ac_filter{
            Some(ac) => {
                println!("find ac....");
                for mat in ac.find_iter(content) {
                    println!("ac match: {}", self.keyword_vec[mat.pattern()]);
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