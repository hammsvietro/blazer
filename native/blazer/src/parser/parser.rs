use core::panic;
use rustler::Term;
use std::mem;
use convert_case::{Case, Casing};

pub fn return_for_map<'a>(map: &Term<'a>) -> Term<'a> {
	let iterator = rustler::MapIterator::new(*map).unwrap(); 
    let env = Term::get_env(&map);

	let (keys, values): (Vec<Term<'a>>, Vec<Term<'a>>) = iterator.map(|(key, value)| {
		let transformed_key = match key.get_type() {
			rustler::TermType::Atom => panic!("can't parse any other type"),
			rustler::TermType::Binary  => parse_string(key),
			_ => panic!("can't parse any other type")
		};

        (transformed_key, value)

	}).unzip();
    Term::map_from_arrays(env, &keys.as_slice(), &values.as_slice()).unwrap()
}


fn parse_atom(term: Term) -> () {

}

fn parse_string(term: Term) -> Term {
    let env = Term::get_env(&term);

    let string = binary_term_to_string(&term);
    let parsed_string = string.to_case(Case::Camel);
    let string_size = parsed_string.as_bytes();
    let mut owned_binary = rustler::OwnedBinary::new(string_size.len()).unwrap();
    owned_binary.as_mut_slice().copy_from_slice(string_size);

    rustler::Binary::from_owned(owned_binary, env).to_term(env)
}

fn binary_term_to_string<'a>(term: &Term<'a>) -> String {
	let binary: rustler::Binary = match term.into_binary() {
		Ok(bin) => bin,
		Err(_) => panic!("cannot parse binary key")
	};
	String::from_utf8_lossy(binary.as_slice()).into_owned()
}
