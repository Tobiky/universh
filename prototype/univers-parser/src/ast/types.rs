use super::core::Identity;

pub struct FunctionTypes<'input> {
    pub location: (usize, usize),
    pub parameter_types: Vec<TypeToken<'input>>,
    pub return_type: Box<TypeToken<'input>>,
}

pub enum Type<'input> {
    Function(FunctionTypes<'input>),
    Identity(Identity<'input>)
}

pub struct TypeToken<'input> {
    pub location: (usize, usize),
    pub token: Type<'input>
}
