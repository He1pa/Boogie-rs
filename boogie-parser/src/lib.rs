pub mod ast;

use lalrpop_util::lalrpop_mod;

#[cfg(test)]
pub mod tests;

lalrpop_mod!(pub boogie); // synthesized by LALRPOP