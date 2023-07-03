use std::borrow::Cow;

use super::statements::StatementToken;

#[derive(Clone)]
pub struct Identity<'input> {
    pub location: (usize, usize),
    pub id: Cow<'input, str>,
}

impl<'input> Identity<'input> {
    pub fn new(left: usize, id: &'input str, right: usize) -> Self {
        Identity { location: (left, right), id: Cow::from(id) }
    }

    pub fn inject_at(left: usize, id: String, right: usize) -> Self {
        Identity { location: (left, right), id: Cow::from(id) }
    }

    pub fn inject(id: String) -> Self {
        Self::inject_at(0, id, 0)
    }
}

pub type Program<'input> = Vec<StatementToken<'input>>;
