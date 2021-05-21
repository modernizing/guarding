extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate serde;

pub mod identify;
pub mod parser;
pub mod rule_executor;
pub mod domain;

#[cfg(test)]
mod tests;