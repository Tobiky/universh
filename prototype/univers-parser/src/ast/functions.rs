use super::{types::Type, core::Identity, statements::Statement};

pub type Parameter<'input> = (Identity<'input>, Option<Type<'input>>);

pub struct Function<'input> {
    return_type: Option<Type<'input>>,
    parameters: Vec<Parameter<'input>>,
    name: Identity<'input>,
    body: Vec<Statement>
}

pub struct FunctionToken<'input> {
    location: (usize, usize),
    token: Function<'input>,
}

pub struct Flag<'input> {
    name: Identity<'input>,
    settings: Vec<(Identity<'input>, )
}

pub struct FlagToken<'input> {
    location: (usize, usize),
    token: Flag,
}

pub struct Command<'input> {

}

pub struct CommandToken<'input> {
    location: (usize, usize),
    token: Command<'input>,
}
