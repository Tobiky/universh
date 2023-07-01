use super::{types::Type, core::Identity, statements::Statement, expressions::ExpressionToken};

pub type Parameter<'input> = (Identity<'input>, Option<Type<'input>>);

pub struct FunctionToken<'input> {
    pub location: (usize, usize),
    pub return_type: Option<Type<'input>>,
    pub parameters: Vec<Parameter<'input>>,
    pub name: Identity<'input>,
    pub body: Vec<Statement<'input>>
}

pub enum CommandOptionType {
    Flag,
    Argument,
}

pub struct CommandOptionTypeToken {
    pub location: (usize, usize),
    pub token: CommandOptionType,
}

pub struct CommandOptionToken<'input> {
    pub location: (usize, usize),
    pub name: Identity<'input>,
    pub settings: Vec<(Identity<'input>, ExpressionToken<'input>)>,
    pub option_type: CommandOptionTypeToken
}

pub struct CommandToken<'input> {
    pub location: (usize, usize),
    pub name: Identity<'input>,
    pub options: Vec<CommandOptionToken<'input>>,
    pub rest_variable: Option<Identity<'input>>,
    pub body: Vec<Statement<'input>>,
}
