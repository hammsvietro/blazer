use super::super::model::options::BlazerOptions;
use super::string::string_to_term;
use convert_case::Casing;
use core::panic;
use rustler::Term;

pub fn parse_map<'a>(map: &Term<'a>, options: BlazerOptions) -> Term<'a> {
    let iterator = rustler::MapIterator::new(*map).unwrap();
    let env = Term::get_env(&map);

    let (keys, values): (Vec<Term<'a>>, Vec<Term<'a>>) = iterator
        .map(|(key, value)| {
            let transform_fn: for<'i> fn(Term<'i>, &BlazerOptions) -> Term<'i> = match key.get_type() {
                rustler::TermType::Atom => parse_atom,
                rustler::TermType::Binary => parse_string,
                _ => panic!("can't parse any other type"),
            };
            (transform_fn(key, &options), value)
        })
        .unzip();
    Term::map_from_arrays(env, &keys.as_slice(), &values.as_slice()).unwrap()
}

pub fn parse_string<'a>(term: Term<'a>, options: &BlazerOptions) -> Term<'a> {
    let env = Term::get_env(&term);
    let key = binary_term_to_string(&term);
    let parsed_key = key.to_case(options.case);

    string_to_term(parsed_key, options.keys, env)
}

fn parse_atom<'a>(term: Term<'a>, options: &BlazerOptions) -> Term<'a> {
    let env = Term::get_env(&term);

    let key = Term::atom_to_string(&term).unwrap();
    let parsed_key = key.to_case(options.case);
    string_to_term(parsed_key, options.keys, env)
}

fn binary_term_to_string<'a>(term: &Term<'a>) -> String {
    let binary: rustler::Binary = match term.into_binary() {
        Ok(bin) => bin,
        Err(_) => panic!("cannot parse binary key"),
    };
    String::from_utf8_lossy(binary.as_slice()).into_owned()
}
