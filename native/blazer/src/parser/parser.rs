use core::panic;
use rustler::Term;
use convert_case::{Casing,Case};
use super::string::{string_to_term};

pub fn return_for_map<'a>(map: &Term<'a>, case: Case) -> Term<'a> {
	let iterator = rustler::MapIterator::new(*map).unwrap(); 
    let env = Term::get_env(&map);

	let (keys, values): (Vec<Term<'a>>, Vec<Term<'a>>) = iterator.map(|(key, value)| {
		let transform_fn: for <'i> fn(Term<'i>, Case) -> Term<'i> = match key.get_type() {
			rustler::TermType::Atom => parse_atom,
			rustler::TermType::Binary  => parse_string,
			_ => panic!("can't parse any other type")
		};

        (transform_fn(key, case), value)

	}).unzip();
    Term::map_from_arrays(env, &keys.as_slice(), &values.as_slice()).unwrap()
}

pub fn parse_string<'a>(term: Term<'a>, case: Case) -> Term<'a> {
    let env = Term::get_env(&term);
    let key = binary_term_to_string(&term);
    let parsed_key = key.to_case(case);

    string_to_term(parsed_key, env)
}

fn parse_atom<'a>(term: Term<'a>, case: Case) -> Term<'a> {
    let env = Term::get_env(&term);
    
    let key = Term::atom_to_string(&term).unwrap();
    let parsed_key = key.to_case(case);
    rustler::types::Atom::from_str(env, &parsed_key[..]).unwrap().to_term(env)
}

fn binary_term_to_string<'a>(term: &Term<'a>) -> String {
	let binary: rustler::Binary = match term.into_binary() {
		Ok(bin) => bin,
		Err(_) => panic!("cannot parse binary key")
	};
	String::from_utf8_lossy(binary.as_slice()).into_owned()
}
