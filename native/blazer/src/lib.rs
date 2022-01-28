mod parser;

#[rustler::nif]
fn convert_map<'a>(term: rustler::Term<'a>) -> rustler::Term<'a> {
    parser::parser::return_for_map(&term)
}

rustler::init!("Elixir.Blazer.Native", [convert_map]);
