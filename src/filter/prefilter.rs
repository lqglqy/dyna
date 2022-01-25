use super::rule::Rule;
use super::rule::RtRule;
use super::keyword::KeywordFilter;
use std::collections::HashMap;
use std::collections::HashSet;
use super::result::MatchResult;
use wirefilter::{ExecutionContext, Scheme};

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

impl<'s> Prefilter {
    pub(crate) fn new(rs: &Vec<Rule>) -> Self {
        let mut kmf: HashMap<String, KeywordFilter> = HashMap::new();

        for r in rs {
            for k in &r.keyword {
                for feild in &k.target {
                    let kf = kmf.entry(feild.clone()).or_insert(KeywordFilter::new());
                    kf.add(k.id, r.id.clone(), k.content.clone(), match &k.after_check {
                        Some(f) => f.clone(),
                        _ => "none".to_string()});
                }
            }
            for (_, v) in &mut kmf {
                v.build();
            }
        }
        Prefilter {
            maps: kmf,
        }
    }
    
    pub fn destroy(self) {

    }

    pub fn exec(self, feilds: &HashMap<String,String>, mctx: & mut MatchResult) {
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

pub struct RuleFilter<'s> {
    pub rules: HashMap<String, RtRule<'s>>
}

impl<'s> RuleFilter<'s> {
    pub(crate) fn new(rules: &Vec<Rule>, scheme: &'s Scheme) -> Self {
        let mut hmap = HashMap::new();
        for v in rules {
            hmap.insert(v.id.clone(), RtRule::new(&v, scheme));
        }
        RuleFilter {
            rules: hmap
        }
    }
    pub fn exec(self, scheme: &'s Scheme, feilds: &'s HashMap<String,String>, mctx: MatchResult) {
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
                    println!("hit rule: {}", rule.rid.clone());
                    println!("Filter matches: {:?}", rule.filter.execute(&ctx)); 
                },
                _ => {
                    println!("not found rule {}", &k);
                }
            }
        }
    }

}