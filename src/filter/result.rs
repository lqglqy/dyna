use std::collections::HashSet;
#[derive(PartialEq, Eq, Clone)]
pub struct MatchKeyword{
    id: i64,
    rid: String,
    content: String,
    feild: String
}

impl MatchKeyword {
    pub fn get_id(self) -> i64{
        self.id
    }
    pub fn get_feild(self) -> String {
        self.feild
    }
}

pub struct MatchResult{
    keywords: Vec<MatchKeyword>,
}

impl<'s> MatchResult{
    pub(crate) fn new() -> Self {
        MatchResult{
            keywords: Vec::new(),
        }
    }

    pub(crate) fn add_keyword(&mut self, content: String, id: i64, rid: String, feild: String) {
        self.keywords.push(MatchKeyword{content: content, rid: rid, id: id, feild: feild});
    }

    pub fn get_hit_keyword(&mut self, ret_str: &mut String)  {
        for k in &self.keywords {
            ret_str.push_str(&format!("{}:{}|", &k.id.to_string(), &k.feild));
        }
    } 

    pub fn get_hit_rules(self, rule: &mut HashSet<String>) {
        for k in &self.keywords {
            rule.insert(k.rid.clone());
        }
    }
}