use {
    ecow::EcoString,
    std::{
        collections::HashSet,
        ops::Deref as _,
        rc::Rc,
    },
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct V(pub EcoString, pub u32);

impl V {
    pub const fn new(name: EcoString) -> Self {
        Self(name, 0)
    }

    pub const fn xu() -> Self {
        Self::new(EcoString::inline("@xu"))
    }

    pub fn fresh(mut self, v: &HashSet<Self>) -> Self {
        self.1 = 0;

        while v.contains(&self) {
            self.1 += 1;
        }

        self
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct E(pub EcoString);

pub enum T {
    Var(V),
    Abs(V, Rc<Self>),
    App(Rc<Self>, Rc<Self>),
    Per(E, Rc<Self>),
    Imp(E, Rc<Self>, Rc<Self>),
    Han(E, Rc<Self>, Rc<Self>),
}

impl T {
    pub fn new_var(x: V) -> Rc<Self> {
        Rc::new(Self::Var(x))
    }

    pub fn new_abs(x: V, t: Rc<Self>) -> Rc<Self> {
        Rc::new(Self::Abs(x, t))
    }

    pub fn new_app(t1: Rc<Self>, t2: Rc<Self>) -> Rc<Self> {
        Rc::new(Self::App(t1, t2))
    }

    pub fn new_per(e: E, t: Rc<Self>) -> Rc<Self> {
        Rc::new(Self::Per(e, t))
    }

    pub fn new_imp(e: E, t1: Rc<Self>, t2: Rc<Self>) -> Rc<Self> {
        Rc::new(Self::Imp(e, t1, t2))
    }

    pub fn new_han(e: E, t1: Rc<Self>, t2: Rc<Self>) -> Rc<Self> {
        Rc::new(Self::Han(e, t1, t2))
    }

    pub fn fv(self: &Rc<Self>) -> HashSet<V> {
        match self.deref() {
            Self::Var(x) => HashSet::from([x.clone()]),
            Self::Abs(x, t) => {
                let mut v = t.fv();
                v.remove(x);
                v
            },
            Self::App(t1, t2) => {
                let mut v = t1.fv();
                v.extend(t2.fv());
                v
            },
            Self::Per(_, t) => t.fv(),
            Self::Imp(_, t1, t2) => {
                let mut v = t1.fv();
                v.extend(t2.fv());
                v
            },
            Self::Han(_, t1, t2) => {
                let mut v = t1.fv();
                v.extend(t2.fv());
                v
            },
        }
    }

    pub fn subst(self: &Rc<Self>, x: &V, t: &Rc<Self>) -> Rc<Self> {
        match self.deref() {
            Self::Var(x1) => match x == x1 {
                true => t,
                false => self,
            }
            .clone(),
            Self::Abs(x1, t1) => {
                let mut v = t1.fv();
                v.remove(x1);
                v.insert(x.clone());
                v.extend(t.fv());
                let x2 = x1.clone().fresh(&v);
                let t2 = t1.subst(x1, &Self::new_var(x2.clone())).subst(x, t);
                Self::new_abs(x2, t2)
            },
            Self::App(t1, t2) => Self::new_app(t1.subst(x, t), t2.subst(x, t)),
            Self::Per(e, t1) => Self::new_per(e.clone(), t1.subst(x, t)),
            Self::Imp(e, t1, t2) => Self::new_imp(e.clone(), t1.subst(x, t), t2.subst(x, t)),
            Self::Han(e, t1, t2) => Self::new_han(e.clone(), t1.subst(x, t), t2.subst(x, t)),
        }
    }

    pub fn eval(self: &Rc<Self>) -> Option<Rc<Self>> {
        match self.deref() {
            Self::App(t1, t2) => {
                if let Option::Some(t1) = t1.eval() {
                    Option::Some(Self::new_app(t1, t2.clone()))
                } else if let Self::Imp(e, t11, t12) = t1.deref() {
                    Option::Some(Self::new_imp(
                        e.clone(),
                        t11.clone(),
                        Self::new_abs(
                            V::xu(),
                            Self::new_app(
                                Self::new_app(t12.clone(), Self::new_var(V::xu())),
                                t2.clone(),
                            ),
                        ),
                    ))
                } else if let Option::Some(t2) = t2.eval() {
                    Option::Some(Self::new_app(t1.clone(), t2))
                } else if let Self::Imp(e, t21, t22) = t2.deref() {
                    Option::Some(Self::new_imp(
                        e.clone(),
                        t21.clone(),
                        Self::new_abs(
                            V::xu(),
                            Self::new_app(
                                t1.clone(),
                                Self::new_app(t22.clone(), Self::new_var(V::xu())),
                            ),
                        ),
                    ))
                } else if let Self::Abs(x, t1) = t1.deref() {
                    Option::Some(t1.subst(x, t2))
                } else {
                    Option::None
                }
            },
            Self::Per(e, t) => Option::Some(
                if let Option::Some(t) = t.eval() {
                    Self::new_per(e.clone(), t)
                } else if let Self::Imp(e0, t1, t2) = t.deref() {
                    Self::new_imp(
                        e0.clone(),
                        t1.clone(),
                        Self::new_abs(
                            V::xu(),
                            Self::new_per(
                                e.clone(),
                                Self::new_app(t2.clone(), Self::new_var(V::xu())),
                            ),
                        ),
                    )
                } else {
                    Self::new_imp(
                        e.clone(),
                        t.clone(),
                        Self::new_abs(V::xu(), Self::new_var(V::xu())),
                    )
                },
            ),
            Self::Han(e, t1, t2) => Option::Some(
                if let Option::Some(t2) = t2.eval() {
                    Self::new_han(e.clone(), t1.clone(), t2.clone())
                } else if let Self::Imp(e20, t21, t22) = t2.deref() {
                    match e == e20 {
                        true => Self::new_app(
                            Self::new_app(t1.clone(), t21.clone()),
                            Self::new_abs(
                                V::xu(),
                                Self::new_han(
                                    e.clone(),
                                    t1.clone(),
                                    Self::new_app(t22.clone(), Self::new_var(V::xu())),
                                ),
                            ),
                        ),
                        false => Self::new_imp(
                            e20.clone(),
                            t21.clone(),
                            Self::new_abs(
                                V::xu(),
                                Self::new_han(
                                    e.clone(),
                                    t1.clone(),
                                    Self::new_app(t22.clone(), Self::new_var(V::xu())),
                                ),
                            ),
                        ),
                    }
                } else {
                    t2.clone()
                },
            ),
            _ => Option::None,
        }
    }

    pub fn eval_star(self: &Rc<Self>) -> Rc<Self> {
        match self.eval() {
            Option::Some(mut t) => {
                while let Option::Some(t1) = t.eval() {
                    t = t1
                }

                t
            },
            Option::None => self.clone(),
        }
    }
}
