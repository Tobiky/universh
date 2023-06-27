use super::core::Identity;

pub struct LiteralToken<'input> {
    location: (usize, usize),
    value: &'input str,
    literal_type: Identity<'input>,
}

pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Xor,
}

pub struct BinOpToken<'input> {
    location: (usize, usize),
    op_location: (usize, usize),
    op: BinOp,
    lhs: Box<Expression<'input>>,
    rhs: Box<Expression<'input>>,
}

pub enum UnOp {
    Not,
    Neg,
}

pub struct UnOpToken<'input> {
    location: (usize, usize),
    op_location: (usize, usize),
    op: UnOp,
    expr: Box<Expression<'input>>
}

pub struct FuncCallToken<'input> {
    location: (usize, usize),
    name: Identity<'input>,
    arguments: Vec<Identity<'input>>,
}

pub enum Expression<'input> {
    Literal(LiteralToken<'input>),
    Variable(Identity<'input>),
    BinOp(BinOpToken<'input>),
    UnOp(UnOpToken<'input>),
    Call(FuncCallToken<'input>),
}
