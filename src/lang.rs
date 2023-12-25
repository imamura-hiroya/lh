use {
    ecow::EcoString,
    std::{
        collections::HashSet,
        ops::Deref as _,
        rc::Rc,
    },
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct V {
    pub name: EcoString,
    pub tag: u32,
}

impl V {
    pub fn new(name: EcoString) -> Self {
        Self {
            name,
            tag: 0,
        }
    }

    pub fn fresh(mut self, v: &HashSet<Self>) -> Self {
        self.tag = 0;

        while v.contains(&self) {
            self.tag += 1;
        }

        self
    }
}

pub enum T {
    Var(V),
    Abs(V, Rc<Self>),
    App(Rc<Self>, Rc<Self>),
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
        }
    }

    pub fn eval(self: &Rc<Self>) -> Option<Rc<Self>> {
        match self.deref() {
            Self::App(t1, t2) => {
                if let Option::Some(t1) = t1.eval() {
                    Option::Some(Self::new_app(t1, t2.clone()))
                } else if let Option::Some(t2) = t2.eval() {
                    Option::Some(Self::new_app(t1.clone(), t2))
                } else if let Self::Abs(x, t1) = t1.deref() {
                    Option::Some(t1.subst(x, t2))
                } else {
                    Option::None
                }
            },
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
