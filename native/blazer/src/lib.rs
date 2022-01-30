mod parser;
mod cases;

#[rustler::nif]
fn convert_map<'a>(term: rustler::Term<'a>, case: rustler::Term<'a>) -> rustler::Term<'a> {
    parser::parser::return_for_map(&term, case)
}

#[rustler::nif]
fn convert_binary<'a>(term: rustler::Term<'a>, case: rustler::Term<'a>) -> rustler::Term<'a> {
    parser::parser::parse_string(term, case)
}

rustler::init!("Elixir.Blazer.Native", [convert_binary, convert_map]);
