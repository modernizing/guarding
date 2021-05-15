use pest::Parser;

#[derive(Parser)]
#[grammar = "parser/ident.pest"]
struct IdentParser;

pub fn parse(code: &str) {
    // online parser: [https://pest.rs/](https://pest.rs/)
    let pairs = IdentParser::parse(Rule::start, code).unwrap_or_else(|e| panic!("{}", e));
    for pair in pairs {
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());

        for inner_pair in pair.into_inner() {
            println!("{:?}", inner_pair)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::parse;

    #[test]
    fn should_parse_ident() {
        let code = "class::name contains \"Controller\";";
        parse(code);
    }
}