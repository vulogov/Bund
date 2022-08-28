
extern crate pest;
use pest::{Parser};
use pest_derive::*;

#[derive(Parser)]
#[grammar = "bund.pest"]
struct BUNDParser;

pub fn parse(s: &String) {
    let pairs = BUNDParser::parse(Rule::program, s);
    for pair in pairs {
        for p in pair {
            print_pair(0, p);
        }
    }
}

fn print_pair(n: i32, p: pest::iterators::Pair<Rule>) {
    let r = &p.as_rule();
    println!("{} {} {:#?}", n, &p, &r);
    let mut childs = p.into_inner();
    let n = childs.next().unwrap();
    println!("{:#?}", n);
    // for i in childs {
    //     print_pair(n+1, i);
    // }
}
