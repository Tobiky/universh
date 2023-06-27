pub struct Identity<'input> {
    location: (usize, usize),
    id: &'input str,
}

impl<'input> From<(usize, &'input str, usize)> for Identity<'input> {
    fn from(value: (usize, &'input str, usize)) -> Self {
        Identity { 
            location: (value.0, value.2),
            id: value.1
        }
    }
}
