use super::core::Identity;

pub struct FunctionTypes<'input> {
    location: (usize, usize),
    parameter_types: Vec<Type<'input>>,
    return_type: Box<Type<'input>>,
}

pub enum Type<'input> {
    Function(FunctionTypes<'input>),
    Identity(Identity<'input>)
}

pub struct TypeToken<'input> {
    location: (usize, usize),
    token: Type<'input>
}
