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

pub struct RuleResult {
    pub rule_id: String,
}

pub struct MatchResult{
    keywords: Vec<MatchKeyword>,
    key_string: HashSet<String>,
    pub must_match_rules: Vec<String>,
}

impl<'s> MatchResult{
    pub fn new() -> Self {
        MatchResult{
            keywords: Vec::new(),
            key_string: HashSet::new(),
            must_match_rules: Vec::new(),
        }
    }

    pub(crate) fn add_keyword(&mut self, content: String, id: i64, rid: String, feild: String) {
        self.keywords.push(MatchKeyword{content: content, rid: rid, id: id, feild: feild.clone()});
        self.key_string.insert(format!("{}:{}|", id.to_string(), feild));
    }

    pub fn get_hit_keyword(&mut self, ret_str: &mut String)  {
        for k in &self.key_string{
            //ret_str.push_str(&format!("{}:{}|", &k.id.to_string(), &k.feild));
            ret_str.push_str(k);
        }
    } 

    pub fn get_hit_rules(&self, rule: &mut HashSet<String>) {
        for k in &self.keywords {
            rule.insert(k.rid.clone());
        }
        for k in &self.must_match_rules {
            rule.insert(k.clone());
        }
    }
}