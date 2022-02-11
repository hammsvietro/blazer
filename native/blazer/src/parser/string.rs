pub fn string_to_term<'a>(string: String, env: rustler::env::Env<'a>) -> rustler::Term<'a> {
	let parsed_slice = string.as_bytes();
	let mut owned_binary = rustler::OwnedBinary::new(parsed_slice.len()).unwrap();
	owned_binary .as_mut_slice() .copy_from_slice(&parsed_slice[..]);

	rustler::Binary::from_owned(owned_binary, env).to_term(env)
}
