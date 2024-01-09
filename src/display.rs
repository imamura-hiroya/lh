use {
    crate::lang::{
        E,
        T,
        V,
    },
    std::{
        fmt::{
            Display,
            Formatter,
            Result as FmtResult,
        },
        ops::Deref as _,
    },
};

impl Display for V {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        self.0.fmt(formatter)?;

        for _ in 0..self.1 {
            '\''.fmt(formatter)?;
        }

        Result::Ok(())
    }
}

impl Display for E {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        self.0.fmt(formatter)
    }
}

impl Display for T {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Var(x) => x.fmt(formatter),
            Self::Abs(x, t) => {
                write!(formatter, "\\{x}. ")?;

                match t.deref() {
                    Self::Imp(_, _, _) => write!(formatter, "({t})"),
                    _ => t.fmt(formatter),
                }
            },
            Self::App(t1, t2) => {
                match t1.deref() {
                    Self::Var(_) | Self::App(_, _) | Self::Per(_, _) => t1.fmt(formatter),
                    Self::Abs(_, _) | Self::Imp(_, _, _) | Self::Han(_, _, _) => {
                        write!(formatter, "({t1})")
                    },
                }?;

                ' '.fmt(formatter)?;

                match t2.deref() {
                    Self::Var(_) | Self::Abs(_, _) | Self::Per(_, _) | Self::Han(_, _, _) => {
                        t2.fmt(formatter)
                    },
                    Self::App(_, _) | Self::Imp(_, _, _) => write!(formatter, "({t2})"),
                }?;

                Result::Ok(())
            },
            Self::Per(e, t) => write!(formatter, "{e}<{t}>"),
            Self::Imp(e, t1, t2) => write!(formatter, "{e}<{t1}> >>> {t2}"),
            Self::Han(e, t1, t2) => write!(formatter, "{e} ~> {t1} |> {t2}"),
        }
    }
}
