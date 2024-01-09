use {
    crate::lang::{
        E,
        T,
        V,
    },
    ecow::EcoString,
    std::rc::Rc,
    unscanny::Scanner,
};

impl T {
    pub fn parse(s: &str) -> Option<Rc<Self>> {
        Option::None
    }
}

fn parse_in_scope<T>(
    scanner: &mut Scanner,
    parser: impl FnOnce(&mut Scanner) -> Option<T>,
) -> Option<T> {
    let backup = *scanner;
    let result = parser(scanner);

    if result.is_none() {
        *scanner = backup;
    }

    result
}

fn parse_iden(scanner: &mut Scanner) -> Option<EcoString> {
    parse_in_scope(scanner, |scanner| {
        let var = scanner.eat_while(|c: char| c.is_alphanumeric() || c == '_');
        (!var.is_empty()).then_some(())?;
        Option::Some(var.into())
    })
}

fn parse_var(scanner: &mut Scanner) -> Option<V> {
    parse_iden(scanner).map(V::new)
}

fn parse_eff(scanner: &mut Scanner) -> Option<E> {
    parse_iden(scanner).map(E)
}

fn parse_term_var(scanner: &mut Scanner) -> Option<Rc<T>> {
    parse_var(scanner).map(T::new_var)
}

fn parse_term_abs(scanner: &mut Scanner) -> Option<Rc<T>> {
    parse_in_scope(scanner, |scanner| {
        scanner.eat_if('\\').then_some(())?;
        let x = parse_var(scanner)?;
        scanner.eat_if(". ").then_some(())?;

        let t = parse_term_app(scanner)
            .or_else(|| parse_term_var(scanner))
            .or_else(|| parse_term_abs(scanner))
            .or_else(|| parse_term_per(scanner))
            .or_else(|| parse_term_han(scanner))?;

        Option::Some(T::new_abs(x, t))
    })
}

fn parse_term_app(scanner: &mut Scanner) -> Option<Rc<T>> {
    todo!()
}

fn parse_term_per(scanner: &mut Scanner) -> Option<Rc<T>> {
    parse_in_scope(scanner, |scanner| {
        let e = parse_eff(scanner)?;
        scanner.eat_if('<').then_some(())?;

        let t = parse_term_app(scanner)
            .or_else(|| parse_term_var(scanner))
            .or_else(|| parse_term_abs(scanner))
            .or_else(|| parse_term_per(scanner))
            .or_else(|| parse_term_han(scanner))?;

        scanner.eat_if('>').then_some(())?;
        Option::Some(T::new_per(e, t))
    })
}

fn parse_term_han(scanner: &mut Scanner) -> Option<Rc<T>> {
    todo!()
}
