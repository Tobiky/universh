use std::borrow::Cow;

use super::core::Identity;

pub struct FunctionTypes<'input> {
    pub location: (usize, usize),
    pub parameter_types: Vec<TypeToken<'input>>,
    pub return_type: Box<TypeToken<'input>>,
}

pub enum Type<'input> {
    Function(FunctionTypes<'input>),
    Nested(Box<TypeToken<'input>>, Vec<TypeToken<'input>>),
    Named(Identity<'input>)
}

pub struct TypeToken<'input> {
    pub location: (usize, usize),
    pub token: Type<'input>
}

impl<'input> TypeToken<'input> {
    pub fn named_at(
        left: usize,
        name: &'input str,
        right: usize)
        -> TypeToken<'input>
    {
        TypeToken { 
            location: (left, right),
            token: Type::Named(
                Identity { 
                    location: (left, right),
                    id: Cow::from(name)
                }
            )
        }
    }

    pub fn named(name: &'input str) -> Self {
        Self::named_at(0, name, 0)
    }
}
