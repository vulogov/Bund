extern crate pest;
use crate::lang::Rule;

use crate::vm::vm;
use crate::vm::unknown;
use crate::vm::eoi;
use crate::vm::lfb;
use crate::vm::rfb;
use crate::vm::ltb;
use crate::vm::rtb;
use crate::vm::literal;
use crate::vm::string;
use crate::vm::ident;
use crate::vm::tag;
use crate::vm::prefix;
use crate::vm::suffix;
use crate::vm::integer;
use crate::vm::float;
use crate::vm::token;



pub fn parse_pair(b: &mut vm::VM, p: pest::iterators::Pair<Rule>) {
    let rule  = &p.as_rule();
    let token = &p.as_span();
    match rule {
        Rule::term => {
            token::process_token(&b, &p, &token.as_str().to_string());
            for inner in p.into_inner() {
                parse_pair(b, inner);
            }
            token::post_process_token(&b, &rule, &token.as_str().to_string());
        }
        Rule::integer => {
            integer::process_token(&b, &p, &token.as_str().to_string());
        }
        Rule::float => {
            float::process_token(b, &p, &token.as_str().to_string());
        }
        Rule::ident => {
            ident::process_token(&b, &p, &token.as_str().to_string());
        }
        Rule::prefix => {
            prefix::process_token(&b, &p, &token.as_str().to_string());
        }
        Rule::suffix => {
            suffix::process_token(&b, &p, &token.as_str().to_string());
        }
        Rule::literal => {
            literal::process_token(&b, &p, &token.as_str().to_string());
        }
        Rule::string => {
            string::process_token(&b, &p, &token.as_str().to_string());
        }
        Rule::multi_line_string => {
            string::process_token(&b, &p, &token.as_str().to_string());
        }
        Rule::left_function_bracket => {
            lfb::process_token(&b, &p, &token.as_str().to_string());
        }
        Rule::right_function_bracket => {
            rfb::process_token(&b, &p, &token.as_str().to_string());
        }
        Rule::tag => {
            tag::process_token(&b, &p, &token.as_str().to_string());
        }
        Rule::left_tag_bracket => {
            ltb::process_token(&b, &p, &token.as_str().to_string());
        }
        Rule::right_tag_bracket => {
            rtb::process_token(&b, &p, &token.as_str().to_string());
        }
        Rule::EOI => {
            eoi::process_token(&b, &p, &token.as_str().to_string());
        }
        _ => {
            unknown::process_token(&b, &p, &token.as_str().to_string());
        }
    }
}
