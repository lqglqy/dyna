use super::rule::Rule;
use super::rule::RtRule;
use super::keyword::KeywordFilter;
use std::collections::HashMap;
use std::collections::HashSet;
use super::result::MatchResult;
use crate::engine::{ExecutionContext, Scheme};



pub struct RuleFilter<'s> {
    pub rules: HashMap<String, RtRule<'s>>
}

impl<'s> RuleFilter<'s> {
    pub fn new(rules: &Vec<Rule>, scheme: &'s Scheme) -> Self {
        let mut hmap = HashMap::new();
        for v in rules {
            hmap.insert(v.id.clone(), RtRule::new(&v, scheme));
        }
        RuleFilter {
            rules: hmap
        }
    }
    pub fn exec(&self, scheme: &'s Scheme, feilds: &'s HashMap<String,String>, mctx: &MatchResult) {
        let mut hit_rules:HashSet<String> = HashSet::new();
        mctx.get_hit_rules(&mut hit_rules);
        let mut ctx = ExecutionContext::new(&scheme);
        let mut fs = vec![];
        let mut val = vec![];
        for (k,v) in feilds.iter() {
            fs.push(&k[..]);
            val.push(&v[..]);
            match ctx.set_field_value(fs[fs.len()-1], val[val.len()-1]) {
                Ok(_) => {
                    println!("@@@set feild OK: {} value: {}", fs[fs.len()-1], val[val.len()-1]);
                },
                Err(err) => {
                    println!("###set filed value error: {}", err);
                }
            }
        }

        for k in hit_rules {
            match self.rules.get(&k) {
                Some(rule) => {
                    //println!("hit rule: {}", rule.rid.clone());
                    //rule.filter.execute(&ctx);
                    println!("Filter rule: {} matches: {:?}", rule.rid.clone(), rule.filter.execute(&ctx)); 
                },
                _ => {
                    println!("not found rule {}", &k);
                }
            }
        }
    }

}

pub struct Prefilter {
    maps: HashMap<String, KeywordFilter>, // key: req.filename value: 
}

pub fn add_rule(rs: &mut HashMap<String, RtRule>, r: &Rule) {

    match rs.get(&r.id.clone()) {
        Some(_) => {
            println!("rule already exist!");
        }
        _ => {
            //rs.insert(r.id.clone(), RtRule::new(r));
        }
    }
}

use serde::Deserialize;
use serde::Serialize;

pub type RuleFunctions = Vec<RuleFunction>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleFunction {
    pub name: String,
    pub args: Vec<Arg>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Arg {
    pub kind: String,
    pub value: String,
}


impl<'s> Prefilter {
    pub fn new(rs: &RuleFilter) -> Self {
        let mut kmf: HashMap<String, KeywordFilter> = HashMap::new();
        for (id, rule) in rs.rules.iter() {
            let kw: RuleFunctions = serde_json::from_str(&rule.kw).unwrap();
            for f in kw {
                if f.name == "prefilter" {
                    if f.args.len() != 5 {
                        println!("args count fail!!!");
                        continue;
                    }
                    let target = f.args[1].value.split('|');
                    let kid = f.args[2].value.clone();
                    let keyword = f.args[3].value.clone();
                    let after_check = f.args[4].value.clone(); 
                    for feild in target {
                        println!("feild:{} id:{} kid:{} keyword:{} after_check:{}", feild.clone(), id.clone(), kid.clone(), keyword.clone(), after_check.clone());
                        let kf = kmf.entry(feild.to_string()).or_insert(KeywordFilter::new());
                        kf.add(kid.parse::<i64>().unwrap(), id.clone(), keyword.to_string(), after_check.to_string());
                    }
                }
            }

        }
        for (_, v) in &mut kmf {
            v.build();
        }
        Prefilter {
            maps: kmf,
        }
    }
    
    pub fn destroy(self) {

    }

    pub fn exec(&self, feilds: &HashMap<String,String>, mctx: & mut MatchResult) {
        for (feild, content) in feilds.iter() {
            match self.maps.get(feild) {
                Some(v) => {
                    v.find_all(mctx, content, feild);
                }
                _ => {
                    println!("not found!!!")
                }
            }
        }

    }
}