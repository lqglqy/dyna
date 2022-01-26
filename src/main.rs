pub mod filter;
use self::filter::prefilter::*;
use self::filter::rule::*;
use self::filter::result::*;
use std::collections::HashMap;
use wirefilter::{Scheme};
fn main() {
    let mut rs = Vec::new();
    let r1: Rule = serde_json::from_str(&r#"{"id":"010000092","keyword":[{"id":4000,"content":"=","check_after":"none","target":["req.filename","req.args"]},{"id":4003,"content":"ref","check_after":"both","target":["req.filename","req.args"]}],"content":"((keyword contains \"|4000:req.filename|\" && keyword contains \"|4003:req.filename|\") && req.filename matches \"href=\") || ((keyword contains \"|4000:req.args|\" && keyword contains \"|4003:req.args|\") && req.args matches \"href=\")"}"#.to_string()).unwrap();
    println!("r1.content: {}", r1.content.clone());
    rs.push(r1);
    let r2: Rule = serde_json::from_str(&r#"{"id":"090000072","keyword":[{"id":4006,"content":"def","check_after":"none","target":["req.filename","req.args"]},{"id":4007,"content":"abc","check_after":"both","target":["req.filename","req.args"]}],"content":"((keyword contains \"|4006:req.filename|\" && keyword contains \"|4007:req.filename|\") && req.filename matches \"abc\") || ((keyword contains \"|4006:req.args|\" && keyword contains \"|4007:req.args|\") && req.args matches \"def\")"}"#.to_string()).unwrap();
    rs.push(r2);
    let scheme = Scheme! {
        req.filename: Bytes,
        req.args: Bytes,
        keyword: Bytes
    };
    println!("111Rule Filter build done!!!");
    let rf = RuleFilter::new(&rs, &scheme);
    println!("Rule Filter build done!!!");
    let pf = Prefilter::new(&rs);
    let mut feilds = HashMap::new();
    feilds.insert("req.filename".to_string(), "a=1&&xxx=abcdef".to_string());
    feilds.insert("req.args".to_string(), "bbb=def".to_string());
    let mut mctx = MatchResult::new();
    pf.exec(&feilds, &mut mctx);

    let mut kw_str = String::from("|");
    mctx.get_hit_keyword(&mut kw_str);
    println!("!!!match keywords: {}", kw_str.clone());
    feilds.insert("keyword".to_string(), kw_str.clone());
    
    rf.exec(&scheme, &feilds, mctx);
}
