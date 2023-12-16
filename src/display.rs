use {
    crate::lang::Term,
    std::fmt::{
        Display,
        Formatter,
        Result as FmtResult,
    },
};

impl Display for Term {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        self.display(formatter, false, true)
    }
}

impl Term {
    fn display(&self, formatter: &mut Formatter, is_arg: bool, is_tail: bool) -> FmtResult {
        match self {
            Self::Var(var) => var.fmt(formatter),
            Self::Abs(arg, ret) => scope(formatter, !is_tail, |formatter| {
                write!(formatter, "\\{arg}. ")?;
                ret.display(formatter, false, is_tail)
            }),
            Self::App(fun, arg) => scope(formatter, is_arg, |formatter| {
                fun.display(formatter, false, false)?;
                write!(formatter, " ")?;
                arg.display(formatter, true, is_tail)?;
                Result::Ok(())
            }),
        }
    }
}

fn scope(
    formatter: &mut Formatter,
    in_scope: bool,
    display: impl FnOnce(&mut Formatter) -> FmtResult,
) -> FmtResult {
    if in_scope {
        write!(formatter, "(")?;
    }

    display(formatter)?;

    if in_scope {
        write!(formatter, ")")?;
    }

    Result::Ok(())
}
