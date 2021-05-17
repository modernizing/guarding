use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GuardRule {
    pub ty: RuleType,
    pub level: RuleLevel,
    pub scope: RuleScope,
    pub expr: Expr,
    pub ops: Operation,
    pub assert: RuleAssert
}

impl Default for GuardRule {
    fn default() -> Self {
        GuardRule {
            ty: RuleType::Normal,
            level: RuleLevel::Class,
            scope: RuleScope::All,
            expr: Expr::Identifier("".to_string()),
            ops: Operation::Gt,
            assert: RuleAssert::Empty
        }
    }
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
    All,
    PathDefine(String),
    Extend(String),
    Implementation(String),
    MatchRegex(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expr {
    Call(FunctionCall),
    Identifier(String)
}

/// A function call, can be a filter or a global function
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FunctionCall {
    /// The name of the function
    pub name: String,
    /// The args of the function: key -> value
    pub args: HashMap<String, Expr>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Operation {
    /// >
    Gt,
    /// >=
    Gte,
    /// <
    Lt,
    /// <=
    Lte,
    /// ==
    Eq,
    /// !=
    NotEq,
    /// and
    And,
    /// or
    Or,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuleAssert {
    Empty,
    Int(usize),
}