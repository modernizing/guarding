use pest::Parser;
use pest::iterators::{Pairs, Pair};
use crate::parser::ast::Operation::Gt;
use crate::parser::ast::{GuardRule, RuleLevel};

pub mod ast;

#[derive(Parser)]
#[grammar = "parser/guarding.pest"]
struct IdentParser;

pub fn parse(code: &str) {
    let pairs = IdentParser::parse(Rule::start, code).unwrap_or_else(|e| panic!("{}", e));
    consume_rules_with_spans(pairs);
}

fn consume_rules_with_spans(pairs: Pairs<Rule>) {
    pairs.filter(|pair| {
        return pair.as_rule() == Rule::declaration;
    }).map(|pair| {
        for p in pair.into_inner() {
            match p.as_rule() {
                Rule::normal_rule => {
                    parse_normal_rule(p);
                },
                _ => println!("unreachable content rule: {:?}", p.as_rule())
            };
        }

        return GuardRule::default();
    })
        .collect::<Vec<GuardRule>>();
}

fn parse_normal_rule(pair: Pair<Rule>) -> GuardRule {
    let mut rule_level = RuleLevel::Class;
    for p in pair.into_inner() {
        println!("rule: {:?}, level: {:?}", p.as_rule(), p.as_span());
        match p.as_rule() {
            Rule::rule_level => {

            },
            Rule::prop => {

            },
            Rule::expression => {

            },
            Rule::operation => {

            },
            Rule::assert => {

            },
            _ => {},

        }
    }

    GuardRule::default()
}

#[cfg(test)]
mod tests {
    use crate::parser::parse;

    #[test]
    fn should_parse_ident() {
        let code = "class::name contains \"Controller\";";
        parse(code);
    }

    #[test]
    fn should_parse_package_asset() {
        let code = "class(\"..myapp..\")::function.name should contains(\"\");";
        parse(code);
    }

    #[test]
    fn should_parse_package_extends() {
        let code = "class(extends \"Connection.class\")::name endsWith \"Connection\";";
        parse(code);
    }

    #[test]
    fn should_parse_package_container_scope() {
        let code = "class(assignable \"EntityManager.class\") resideIn package(\"..persistence.\");";
        parse(code);
    }

    #[test]
    fn should_parse_package_regex() {
        let code = "package(match(\"^/app\")) endsWith \"Connection\";";
        parse(code);
    }

    #[test]
    fn should_parse_class_compare() {
        let code = "class(\"..myapp..\")::function.name should not contains(\"\");
class(\"..myapp..\")::function.name !contains(\"\");

class(\"..myapp..\")::vars.len should <= 20;
class(\"..myapp..\")::function.vars.len should <= 20;
";
        parse(code);
    }

    #[test]
    fn should_parse_simple_usage() {
        let code = "class::name.len should < 20;
function::name.len should < 30;
module::package.len should <= 20;
";
        parse(code);
    }

    #[test]
    fn should_parse_arrow_usage() {
        let code = "class -> name.len should < 20;
function -> name.len should < 30;
module -> package.len should <= 20;
";
        parse(code);
    }

    #[test]
    fn should_parse_layer() {
        let code = "layer(\"onion\")
    ::domainModel(\"\")
    ::domainService(\"\")
    ::applicationService(\"\")
    ::adapter(\"com.phodal.com\", \"zero\");

";
        parse(code);
    }
}