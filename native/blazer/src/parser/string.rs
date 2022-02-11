use rustler::{OwnedBinary, Term, Binary, env::Env, TermType, types::Atom};

pub fn string_to_term<'a>(string: String, output_type: &TermType, env: Env<'a>) -> Term<'a> {
    match output_type {
        TermType::Binary => convert_to_string(string, env),
        TermType::Atom => Atom::from_str(env, &string[..])
            .unwrap()
            .to_term(env),
        _ => panic!("")
    }
}

fn convert_to_string<'a>(string: String, env: Env<'a>) -> Term<'a> {
        let parsed_slice = string.as_bytes();
        let mut owned_binary = OwnedBinary::new(parsed_slice.len()).unwrap();
        owned_binary
            .as_mut_slice()
            .copy_from_slice(&parsed_slice[..]);

        Binary::from_owned(owned_binary, env).to_term(env)

}
