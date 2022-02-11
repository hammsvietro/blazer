mod cases;
mod model;
mod parser;

use parser::parser::{parse_string, return_for_map};
use rustler::{types::atom, Atom, Encoder, Term};

#[rustler::nif]
fn convert_map<'a>(term: Term<'a>, case: Term<'a>) -> Term<'a> {
    match cases::case::get_case(&case) {
        Ok(case) => encode_return((atom::ok(), return_for_map(&term, case))),
        Err(err) => encode_return((atom::error(), err)),
    }
}

#[rustler::nif]
fn convert_binary<'a>(term: Term<'a>, case: Term<'a>) -> Term<'a> {
    let _options = model::options::BlazerOptions::from_term(case);
    match cases::case::get_case(&case) {
        Ok(case) => encode_return((atom::ok(), parse_string(term, case))),
        Err(err) => encode_return((atom::error(), err)),
    }
}

rustler::init!("Elixir.Blazer.Native", [convert_binary, convert_map]);

fn encode_return<'a>(native_tuple: (Atom, Term<'a>)) -> Term<'a> {
    let env = native_tuple.1.get_env();
    native_tuple.encode(env)
}
