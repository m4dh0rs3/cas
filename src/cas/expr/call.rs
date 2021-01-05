use {super::Expr, op::Op};

pub(crate) mod op;

#[derive(Clone, PartialEq)]
pub(crate) struct Call {
    pub(crate) op: Op,
    pub(crate) args: Vec<Expr>,
}

impl Call {
    pub(crate) fn new(op: Op, args: Vec<Expr>) -> Call {
        Call { op, args }
    }
}
