use std::fs::File;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use std::ffi::OsStr;
use std::env;
use dynafilter::prefilter::*;
use dynafilter::rule::*;
use dynafilter::result::*;
use std::collections::HashMap;
use std::borrow::Cow;
use dynafilter::engine::{Type, FunctionArgs, Function, FunctionParam, FunctionArgKind, FunctionImpl, LhsValue};
use sigtest::http_response_parse::*;
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
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn main() {
    let args: Vec<String> = env::args().collect();

    let rule_file = &args[1];
    let sig_dir = &args[2];
    let mut input_rs: Vec<Rule> = Vec::new();

    if let Ok(lines) = read_lines(rule_file) {
        for line in lines {
            if let Ok(rule_str) = line {
                println!("{}", rule_str);
                input_rs.push(serde_json::from_str(&rule_str).unwrap());
            }
        }
    }

    println!("read rule done");

    let mut scheme = dynafilter::Scheme! {
        keyword: Bytes,
        RESPONSE_STATUS: Bytes,
        RESPONSE_HEADER: Bytes,
        RESPONSE_BODY: Bytes,
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
    for entry in fs::read_dir(sig_dir).unwrap() {
        let e = entry.unwrap();
        let file = e.path();
        if !file.is_dir() {
            let response = fs::read_to_string(file).unwrap();
            let (_, status_line) = parse_status_line(response.as_bytes()).unwrap();
            println!("status: {}", status_line.status);
            let mut feilds = HashMap::new(); 
            feilds.insert("RESPONSE_STATUS".to_string(), status_line.status.to_string());
            match response.find("\r\n\r\n") {
                Some(idx) => {
                    feilds.insert("RESPONSE_HEADER".to_string(), response[0..idx].to_string());
                    feilds.insert("RESPONSE_BODY".to_string(), response[idx..].to_string());
                },
                None => {
                    feilds.insert("RESPONSE_HEADER".to_string(), response);
                }
            }
            let mut mctx = MatchResult::new();
            pf.exec(&feilds, &mut mctx);
            let mut kw_str = String::from("|");
            mctx.get_hit_keyword(&mut kw_str);
            println!("!!!match keywords: {}", kw_str.clone());
            feilds.insert("keyword".to_string(), kw_str.clone());
            
            let mr = rf.exec(&scheme, &feilds, &mctx);
            let mut matched = false;
            for r in mr {
                if OsStr::new(&r.rule_id) == e.file_name() {
                    println!("matched rule: {}", r.rule_id);
                    matched = true;
                }
            }

            if !matched {
                println!("Oops: not match rule: {:?}", e.file_name());
                break;
            }
        }
    }
}
