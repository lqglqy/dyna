use dynafilter::prefilter::*;
use dynafilter::rule::*;
use dynafilter::result::*;
use std::collections::HashMap;
use std::borrow::Cow;
use dynafilter::engine::{Type, FunctionArgs, Function, FunctionParam, FunctionArgKind, FunctionImpl, LhsValue};
fn arg_to_string(arg: LhsValue) -> String {
    let mut s = String::new();
    match arg {
        LhsValue::Bytes(bytes) => {
            match bytes {
                Cow::Borrowed(bytes) => {
                    s.push_str(std::str::from_utf8(bytes).unwrap())
                },
                _ => {},
            };
        }
        _ => panic!("Invalid type: expected Bytes, got {:?}", arg),
    }
    s
}
fn prefilter_function<'a>(args: FunctionArgs<'_, 'a>) -> LhsValue<'a> {
    let kw = arg_to_string(args.next().unwrap());

    let feild = arg_to_string(args.next().unwrap());

    let key_id = arg_to_string(args.next().unwrap());
    let key = format!("|{}:{}|", key_id, feild);
    println!("keyword is: {}", kw.clone());
    LhsValue::Bool(kw.contains(&key))
}
#[test]
fn main() {
    let input_rs: Vec<Rule> = serde_json::from_str(&r#"[
        {"id": "080120001", "rule": "prefilter(keyword, \"http.response.body\", \"358\", \"String.fromCharCode\", \"both\") && (http.response.body matches \"(String\\.fromCharCode\\(.*){4,}\")"},
        {"id": "080120002", "rule": "prefilter(keyword, \"http.response.header\", \"359\", \"eval(\", \"left\") && (http.response.header matches \"(?i)(eval\\(.{0,15}unescape\\()\")"}
        ]"#.to_string()).unwrap();

    let mut scheme = dynafilter::Scheme! {
        keyword: Bytes,
        http.response.status: Int,
        http.response.header: Bytes,
        http.response.body: Bytes,
    };
    match scheme
    .add_function(
        "prefilter".into(),
        Function {
            params: vec![FunctionParam {
                arg_kind: FunctionArgKind::Field,
                val_type: Type::Bytes,
            },FunctionParam {
                arg_kind: FunctionArgKind::Literal,
                val_type: Type::Bytes,
            },FunctionParam {
                arg_kind: FunctionArgKind::Literal,
                val_type: Type::Bytes,
            },FunctionParam {
                arg_kind: FunctionArgKind::Literal,
                val_type: Type::Bytes,
            },FunctionParam {
                arg_kind: FunctionArgKind::Literal,
                val_type: Type::Bytes,
                //default_value: LhsValue::Bytes(Cow::Borrowed(".".as_bytes())),
            }],
            opt_params: vec![],
            return_type: Type::Bool,
            implementation: FunctionImpl::new(prefilter_function),
        },
    ) {
        Ok(_) => {},
        _ => {
            println!("add function failed!")
        }

    }
    let rf = RuleFilter::new(&input_rs, &scheme);
    println!("Rule Filter build done!!!");
    let pf = Prefilter::new(&rf);
    println!("PreFilter build done!!!");
    let mut feilds = HashMap::new();
    feilds.insert("http.response.body".to_string(), r#"<script type="text/javascript">
    alert(<#String.fromCharCode(72, 69, 76, 76, 79)String.fromCharCode(72, 69, 76, 76, 79)String.fromCharCode(72, 69, 76, 76, 79)String.fromCharCode(72, 69, 76, 76, 79)#>)
    </script>"#.to_string());
    feilds.insert("http.response.header".to_string(), "a=#eval(u".to_string());
    let mut mctx = MatchResult::new();
    pf.exec(&feilds, &mut mctx);

    let mut kw_str = String::from("|");
    mctx.get_hit_keyword(&mut kw_str);
    println!("!!!match keywords: {}", kw_str.clone());
    feilds.insert("keyword".to_string(), kw_str.clone());
    
    rf.exec(&scheme, &feilds, &mctx);
}
