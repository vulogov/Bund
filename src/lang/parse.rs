extern crate pest;
use crate::lang::Rule;

pub fn parse_pair(p: pest::iterators::Pair<Rule>) {
    let r = &p.as_rule();
    let s = &p.as_span();
    println!("{:#?} {:#?}", &r, &s.as_str());
}
