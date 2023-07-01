use super::statements::StatementToken;

#[derive(Clone, Copy)]
pub struct Identity<'input> {
    pub location: (usize, usize),
    pub id: &'input str,
}

impl<'input> Identity<'input> {
    fn from(left: usize, id: &'input str, right: usize) -> Self {
        Identity { location: (left, right), id }
    }
}

pub type Program<'input> = Vec<StatementToken<'input>>;
