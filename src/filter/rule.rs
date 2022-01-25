use serde::{Serialize, Deserialize};
use wirefilter::{Scheme, Filter};
pub struct RtRule<'s> {
    pub filter: Filter<'s>,
    pub rid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rule {
    pub id: String,
    pub keyword: Vec<Keyword>,
    pub content: String,
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

impl Rule {
    pub fn get_keyword(self: &Rule) -> String {
        return serde_json::to_string(&self.keyword).unwrap();
    }
}

impl<'s> RtRule<'s> {
    pub(crate) fn new(r: &Rule, scheme: &'s Scheme) -> Self {
        println!("parse rule: {}", r.content.clone());
        RtRule {
            filter: scheme.parse(&r.content).unwrap().compile(),
            rid: r.id.clone(),
        }
    }
    /*
    pub fn build(&'s mut self, scheme: &'s Scheme) {
        println!("parse rule {}", self.rule.content);
        self.filter = Some(scheme.parse(&self.rule.content).unwrap().compile());
    }*/
}
