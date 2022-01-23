mod parser;

#[rustler::nif]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

#[rustler::nif]
fn change<'a>(term: rustler::Term<'a>) -> Vec<String> {
    match term.get_type() {
        rustler::TermType::Map => parser::parser::return_for_map(&term),
        rustler::TermType::List | rustler::TermType::EmptyList => vec![String::from("b")],
        _ => vec![String::from("3")]
    }
}


#[rustler::nif]
fn map_new_test<'a>(env: rustler::Env<'a>) -> rustler::Term<'a> {
    let new_map: rustler::Term<'a> = rustler::Term::map_new(env);
    new_map
}

rustler::init!("Elixir.Blazer.Native", [add, change]);
