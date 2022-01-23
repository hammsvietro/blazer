mod parser;

#[rustler::nif]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

#[rustler::nif]
fn convert<'a>(term: rustler::Term<'a>) -> rustler::Term<'a> {
    parser::parser::return_for_map(&term)
}

rustler::init!("Elixir.Blazer.Native", [add, convert]);
