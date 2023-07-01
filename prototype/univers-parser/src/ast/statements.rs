use super::{expressions::ExpressionToken, core::Identity, functions::{FunctionToken, CommandToken}};

pub enum Statement<'input> {
    Expression(ExpressionToken<'input>),
    Assign(Identity<'input>, ExpressionToken<'input>),
    FunctionDeclaration(FunctionToken<'input>),
    CommandDeclaration(CommandToken<'input>),
}

pub struct StatementToken<'input> {
    pub location: (usize, usize),
    pub token: Statement<'input>,
}
