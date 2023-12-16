use {
    crate::lang::Term,
    ecow::EcoString,
    std::rc::Rc,
    unscanny::Scanner,
};

impl Term {
    pub fn parse(s: &str) -> Option<Rc<Self>> {
        let mut scanner = Scanner::new(s);
        let result = parse_apps(&mut scanner)?;
        scanner.done().then_some(())?;
        Option::Some(result)
    }
}

fn scope<T>(scanner: &mut Scanner, parser: impl FnOnce(&mut Scanner) -> Option<T>) -> Option<T> {
    let backup = *scanner;
    let result = parser(scanner);

    if result.is_none() {
        *scanner = backup;
    }

    result
}

fn parse_var(scanner: &mut Scanner) -> Option<EcoString> {
    scope(scanner, |scanner| {
        let var = scanner.eat_while(|c: char| c.is_alphanumeric() || c == '_');
        (!var.is_empty()).then_some(())?;
        Option::Some(var.into())
    })
}

fn parse_chunk(scanner: &mut Scanner) -> Option<Rc<Term>> {
    parse_var(scanner)
        .map(Term::new_var)
        .or_else(|| {
            scope(scanner, |scanner| {
                scanner.eat_if('\\').then_some(())?;
                let arg = parse_var(scanner)?;
                scanner.eat_if('.').then_some(())?;
                let ret = parse_apps(scanner)?;
                Option::Some(Term::new_abs(arg, ret))
            })
        })
        .or_else(|| {
            scope(scanner, |scanner| {
                scanner.eat_if('(').then_some(())?;
                let apps = parse_apps(scanner)?;
                scanner.eat_if(')').then_some(())?;
                Option::Some(apps)
            })
        })
}

fn parse_apps(scanner: &mut Scanner) -> Option<Rc<Term>> {
    scope(scanner, |scanner| {
        scanner.eat_whitespace();
        let mut result = parse_chunk(scanner)?;

        while let Option::Some(arg) = scope(scanner, |scanner| {
            (!scanner.eat_whitespace().is_empty()).then_some(())?;
            parse_chunk(scanner)
        }) {
            result = Term::new_app(result, arg);
        }

        scanner.eat_whitespace();
        Option::Some(result)
    })
}
