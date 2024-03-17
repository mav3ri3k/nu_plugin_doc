use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]

pub struct MdParser;

fn find_rule(pair: &pest::iterators::Pair<Rule>  ,rule: Rule) -> Option<String> {
    if pair.as_rule() == rule {
        return Some(pair.as_span().as_str().to_string());
    }
    for into_pair in pair.clone().into_inner().into_iter() {
        match find_rule(&into_pair, rule) {
            Some(str) => { return Some(str) },
            _ => ()
        };
    }

    None
}

pub fn tokens(input: &String) -> Option<(String, String)> {

    let parse = MdParser::parse(Rule::markdown, input);
    match parse {
        Ok(pairs) => {
            let mut itr = pairs.into_iter(); 
            let pair = itr.next().unwrap();

            let title = find_rule(&pair, Rule::title).unwrap();
            let category = find_rule(&pair, Rule::category).unwrap();
            Some((title, category))
        },
        Err(_error) => None,
    }
}
