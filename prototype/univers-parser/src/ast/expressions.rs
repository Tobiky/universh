use super::{core::Identity, statements::{Statement, StatementToken}};

pub struct LiteralToken<'input> {
    pub location: (usize, usize),
    pub value: &'input str,
    // FIXME: injecting a type (e.g. syntax sugar) will not work
    pub literal_type: Identity<'input>,
}

pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    LogicalAnd,
    LogicalOr,
    LogicalXor,
    LeftShift,
    RightShift,
    Equal,
    NotEqual,
    GreaterOrEqual,
    LessOrEqual,
    Less,
    Greater,
    BooleanAnd,
    BooleanOr,
    BooleanXor,
}

impl<'input> BinOp {
    pub fn expression(
        self,
        left: usize,
        lhs: ExpressionToken<'input>,
        op_left: usize,
        op_right: usize,
        rhs: ExpressionToken<'input>,
        right: usize
    ) -> ExpressionToken<'input> {
        ExpressionToken {
            location: (left, right),
            token: Expression::BinOp(
                BinOpToken {
                    location: (left, right),
                    op_location: (op_left, op_right),
                    op: self,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                }
            )
        }
    }

    pub fn assign_statement(
        self,
        left: usize,
        name: Identity<'input>,
        op_left: usize,
        op_right: usize,
        lhs: ExpressionToken<'input>,
        right: usize
    ) -> StatementToken<'input> {
        StatementToken {
            location: (left, right),
            token: Statement::Assign(
                name,
                self.expression(
                    left, 
                    ExpressionToken {
                        location: (left, op_left),
                        token: Expression::Variable(name)
                    },
                    op_left, op_right, lhs, right),
            )
        }
    }
}

pub struct BinOpToken<'input> {
    pub location: (usize, usize),
    pub op_location: (usize, usize),
    pub op: BinOp,
    pub lhs: Box<ExpressionToken<'input>>,
    pub rhs: Box<ExpressionToken<'input>>,
}

pub enum UnOp {
    BooleanNot,
    Negative,
    LogicalInverse,
}

impl<'input> UnOp {
    pub fn expression(
        self,
        left: usize,
        op_right: usize,
        expr: ExpressionToken<'input>,
        right: usize) -> ExpressionToken<'input> {
        ExpressionToken {
            location: (left, right),
            token: Expression::UnOp(
                UnOpToken {
                    location: (left, right),
                    op_location: (left, op_right),
                    op: self,
                    expr: Box::new(expr),
                }
            )
        }
    }
}

pub struct UnOpToken<'input> {
    pub location: (usize, usize),
    pub op_location: (usize, usize),
    pub op: UnOp,
    pub expr: Box<ExpressionToken<'input>>
}

pub struct FunctionCallToken<'input> {
    pub location: (usize, usize),
    pub name: Identity<'input>,
    pub arguments: Vec<ExpressionToken<'input>>,
}

pub struct ProcessExecutionToken<'input> {
    pub location: (usize, usize),
    pub process_name: Identity<'input>,
    // TODO: arguments, modifiers (bg proc, etc.)
}

pub enum MemberAccess<'input> {
    Field(Identity<'input>),
    Function(FunctionCallToken<'input>),
}

pub struct MemberAccessToken<'input> {
    pub location: (usize, usize),
    pub source: Box<ExpressionToken<'input>>,
    pub access: MemberAccess<'input>,
}

pub enum Expression<'input> {
    Literal(LiteralToken<'input>),
    Variable(Identity<'input>),
    BinOp(BinOpToken<'input>),
    UnOp(UnOpToken<'input>),
    Call(FunctionCallToken<'input>),
    Execute(ProcessExecutionToken<'input>),
    MemberAccess(MemberAccessToken<'input>)
}

pub struct ExpressionToken<'input> {
    pub location: (usize, usize),
    pub token: Expression<'input>
}
