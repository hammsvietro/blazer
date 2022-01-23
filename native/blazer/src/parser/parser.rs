use core::panic;

use rustler::Term;

pub fn return_for_map<'a>(map: &Term<'a>) -> Vec<String> {
	let iterator = rustler::MapIterator::new(*map).unwrap(); 

	let term = Term::get_env(map);
	let map = Term::map_new(term);
	Term::map_put(map, "zas", 123);

	iterator.map(|(key, _value)| {
		match key.get_type() {
			rustler::TermType::Atom => rustler::Term::atom_to_string(&key).unwrap(),
			rustler::TermType::Binary  => binary_term_to_string(&key),
			_ => panic!("can't any other type")
		}

	}).collect::<Vec<String>>()
}


fn binary_term_to_string<'a>(term: &Term<'a>) -> String {
	let binary: rustler::Binary = match term.into_binary() {
		Ok(bin) => bin,
		Err(_) => panic!("cannot parse binary key")
	};
	String::from_utf8_lossy(binary.as_slice()).into_owned()

}