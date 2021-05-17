use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GuardRule {
    pub ty: RuleType,
    pub level: RuleLevel,
    pub scope: RuleScope,
    pub expr: Expr,
    pub ops: Vec<Operator>,
    pub assert: RuleAssert
}

impl Default for GuardRule {
    fn default() -> Self {
        GuardRule {
            ty: RuleType::Normal,
            level: RuleLevel::Class,
            scope: RuleScope::All,
            expr: Expr::Identifier("".to_string()),
            ops: vec![],
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
    Package,
    Module,
    Function,
    Class,
    File,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuleScope {
    All,
    PathDefine(String),
    Extend(String),
    Assignable(String),
    Implementation(String),
    MatchRegex(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expr {
    Call(FunctionCall),
    PropsCall(Vec<String>),
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

impl FunctionCall {
    pub fn new(name: String) -> FunctionCall {
        FunctionCall {
            name,
            args: Default::default()
        }
    }
}

impl Default for FunctionCall {
    fn default() -> Self {
        FunctionCall {
            name: "".to_string(),
            args: Default::default()
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Operator {
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
    /// !
    /// not
    Not,

    // string assert operator
    StartsWith,
    Endswith,
    Contains,

    // package operators
    ResideIn,
    Accessed,
    DependBy
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuleAssert {
    Empty,
    Stringed(String),
    Leveled(RuleLevel, String),
    Sized(usize),
}