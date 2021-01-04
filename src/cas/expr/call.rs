pub(crate) mod op;

use {super::Expr, op::Op};

#[derive(Clone)]
pub(crate) struct Call {
    pub(crate) op: Op,
    pub(crate) args: Vec<Expr>,
}

impl Call {
    pub(crate) fn new(op: Op, args: Vec<Expr>) -> Call {
        Call { op, args }
    }
}
