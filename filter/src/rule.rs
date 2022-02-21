use serde::{Serialize, Deserialize};
use crate::engine::{Scheme, Filter};
pub struct RtRule<'s> {
    pub filter: Filter<'s>,
    pub rid: String,
    pub kw: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rule {
    pub id: String,
    //pub keyword: Vec<Keyword>,
    pub rule: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Keyword {
    pub id: i64,
    pub content: String,
    pub target: Vec<String>,
    #[serde(rename = "after_check")]
    pub after_check: Option<String>,
}
/*
impl Rule {
    pub fn get_keyword(self: &Rule) -> String {
        return serde_json::to_string(&self.keyword).unwrap();
    }
}*/

impl<'s> RtRule<'s> {
    pub(crate) fn new(r: &Rule, scheme: &'s Scheme) -> Self {
        println!("parse id: {} rule: {}", r.id.clone(), r.rule.clone());
        let ast = scheme.parse(&r.rule).unwrap();
        RtRule {
            kw: ast.function_to_string(),
            filter: scheme.parse(&r.rule).unwrap().compile(),
            rid: r.id.clone(),
        }
    }
    /*
    pub fn build(&'s mut self, scheme: &'s Scheme) {
        println!("parse rule {}", self.rule.content);
        self.filter = Some(scheme.parse(&self.rule.content).unwrap().compile());
    }*/
}
