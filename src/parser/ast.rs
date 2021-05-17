#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rule {
    pub ty: RuleType,
    pub level: RuleLevel,
    pub scope: RuleScope,
    pub expr: Expr,
    pub ops: Operation,
    pub assert: RuleAssert
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuleType {
    Normal,
    Layer,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuleLevel {
    Class,
    Package,
    Module,
    Function,
    File,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuleScope {
    PathDefine(String),
    Extend(String),
    Implementation(String),
    MatchRegex(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expr {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Operation {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuleAssert {}