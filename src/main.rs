pub mod filter;
use self::filter::prefilter::*;
use self::filter::rule::*;
fn main() {
    let mut rs = Vec::new();
    let r1 = Rule {
        keyword: "abc".to_string()
    };
    rs.push(r1);
    let r2 = Rule {
        keyword: "def".to_string()
    };
    rs.push(r2);
    let mut pf = Prefilter::new(&rs);

    pf.find_all("123abcdef456".to_string());


}
