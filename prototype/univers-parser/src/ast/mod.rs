use lalrpop_util::{self, lalrpop_mod};

pub mod core;
pub mod types;
pub mod statements;
pub mod expressions;
pub mod functions;

lalrpop_mod!(pub parser);
