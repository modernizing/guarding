#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rule {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuleType {
    Normal,
    Layer,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expr {}