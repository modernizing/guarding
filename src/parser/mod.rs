use pest::Parser;
use pest::iterators::{Pairs, Pair};
use crate::parser::ast::{GuardRule, RuleLevel, RuleScope};

pub mod ast;

#[derive(Parser)]
#[grammar = "parser/guarding.pest"]
struct IdentParser;

pub fn parse(code: &str) -> Vec<GuardRule> {
    let pairs = IdentParser::parse(Rule::start, code).unwrap_or_else(|e| panic!("{}", e));
    consume_rules_with_spans(pairs)
}

fn consume_rules_with_spans(pairs: Pairs<Rule>) -> Vec<GuardRule> {
    pairs.filter(|pair| {
        return pair.as_rule() == Rule::declaration;
    }).map(|pair| {
        let mut rule: GuardRule = Default::default();
        for p in pair.into_inner() {
             match p.as_rule() {
                Rule::normal_rule => {
                    rule = return parse_normal_rule(p);
                }
                _ => panic!("unreachable content rule: {:?}", p.as_rule())
            };
        }

        return rule;
    })
        .collect::<Vec<GuardRule>>()
}

fn parse_normal_rule(pair: Pair<Rule>) -> GuardRule {
    let mut guard_rule = GuardRule::default();

    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::rule_level => {
                let level = p.as_span().as_str();
                match level {
                    "module" => { guard_rule.level = RuleLevel::Module }
                    "package" => { guard_rule.level = RuleLevel::Package }
                    "function" => { guard_rule.level = RuleLevel::Function }
                    "file" => { guard_rule.level = RuleLevel::File }
                    "class" => { guard_rule.level = RuleLevel::Class }
                    &_ => {unreachable!("error rule level: {:?}", level)}
                };
            }
            Rule::prop => {}
            Rule::expression => {}
            Rule::operation => {}
            Rule::assert => {}
            Rule::scope => {
                for sc in p.into_inner() {
                    match sc.as_rule() {
                        Rule::string => {
                            let path = sc.as_span().as_str().to_string();
                            println!("path: {:?}", path);
                            guard_rule.scope = RuleScope::PathDefine(path);
                        },
                        _ => {
                            println!("implementing scope: {:?}, text: {:?}", sc.as_rule(), sc.as_span());
                        }
                    }
                }
            }
            Rule::should => {
                // nothing to do
            }
            _ => {
                println!("implementing rule: {:?}, level: {:?}", p.as_rule(), p.as_span());
            }
        }
    }

    guard_rule
}

#[cfg(test)]
mod tests {
    use crate::parser::parse;
    use crate::parser::ast::RuleLevel;

    #[test]
    fn should_parse_rule_level() {
        let code = "class::name contains \"Controller\";";
        let rules = parse(code);
        assert_eq!(1, rules.len());
        assert_eq!(RuleLevel::Class, rules[0].level);
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