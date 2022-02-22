mod cases;
mod model;
mod parser;

use parser::parser::{parse_string, parse_map};
use rustler::{types::atom, Atom, Encoder, Term};
use model::options::BlazerOptions;

#[rustler::nif]
fn convert_map<'a>(term: Term<'a>, opts: Term<'a>) -> Term<'a> {
    match BlazerOptions::from_term(opts) {
        Ok(options) => encode_return((atom::ok(), parse_map(term, &options))),
        Err(err) => encode_return((atom::error(), err)),
    }
}

#[rustler::nif]
fn convert_binary<'a>(term: Term<'a>, opts: Term<'a>) -> Term<'a> {
    match BlazerOptions::from_term(opts) {
        Ok(options) => encode_return((atom::ok(), parse_string(term, &options))),
        Err(err) => encode_return((atom::error(), err)),
    }
}

rustler::init!("Elixir.Blazer.Native", [convert_binary, convert_map]);

fn encode_return<'a>(native_tuple: (Atom, Term<'a>)) -> Term<'a> {
    let env = native_tuple.1.get_env();
    native_tuple.encode(env)
}
