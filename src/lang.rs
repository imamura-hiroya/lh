use {
    ecow::EcoString,
    std::{
        collections::HashSet,
        ops::Deref as _,
        rc::Rc,
    },
};

pub enum Term {
    Var(EcoString),
    Abs(EcoString, Rc<Self>),
    App(Rc<Self>, Rc<Self>),
}

impl Term {
    pub fn new_var(name: EcoString) -> Rc<Self> {
        Rc::new(Self::Var(name))
    }

    pub fn new_abs(arg: EcoString, ret: Rc<Self>) -> Rc<Self> {
        Rc::new(Self::Abs(arg, ret))
    }

    pub fn new_app(fun: Rc<Self>, arg: Rc<Self>) -> Rc<Self> {
        Rc::new(Self::App(fun, arg))
    }

    pub fn free_vars(&self) -> HashSet<EcoString> {
        match self {
            Self::Var(var) => HashSet::from([var.clone()]),
            Self::Abs(arg, ret) => {
                let mut vars = ret.free_vars();
                vars.remove(arg);
                vars
            },
            Self::App(fun, arg) => {
                let mut vars = fun.free_vars();
                vars.extend(arg.free_vars());
                vars
            },
        }
    }

    pub fn substitute(self: &Rc<Self>, var: &str, term: &Rc<Self>) -> Rc<Self> {
        match self.deref() {
            Self::Var(name) => match name == var {
                true => term,
                false => self,
            }
            .clone(),
            Self::Abs(arg, ret) => match arg == var {
                true => self.clone(),
                false => {
                    let mut new_arg = arg.clone();
                    let free_vars = term.free_vars();
                    let ret_buffer;

                    let new_ret = match free_vars.contains(&new_arg) {
                        true => {
                            while {
                                new_arg.push('\'');
                                free_vars.contains(&new_arg)
                            } {}

                            ret_buffer = ret.substitute(arg, &Self::new_var(new_arg.clone()));
                            &ret_buffer
                        },
                        false => ret,
                    }
                    .substitute(var, term);

                    Self::new_abs(new_arg, new_ret)
                },
            },
            Self::App(fun, arg) => {
                Self::new_app(fun.substitute(var, term), arg.substitute(var, term))
            },
        }
    }

    pub fn evaluate(self: &Rc<Self>) -> Option<Rc<Self>> {
        match self.deref() {
            Self::App(fun, arg) => {
                if let Option::Some(fun) = fun.evaluate() {
                    Option::Some(Self::new_app(fun, arg.clone()))
                } else if let Option::Some(arg) = arg.evaluate() {
                    Option::Some(Self::new_app(fun.clone(), arg))
                } else if let Self::Abs(arg_var, ret) = fun.deref() {
                    Option::Some(ret.substitute(arg_var, arg))
                } else {
                    Option::None
                }
            },
            _ => Option::None,
        }
    }

    pub fn evaluate_rt(self: &Rc<Self>) -> Rc<Self> {
        match self.evaluate() {
            Option::Some(mut term) => {
                while let Option::Some(next) = term.evaluate() {
                    term = next
                }

                term
            },
            Option::None => self.clone(),
        }
    }
}
