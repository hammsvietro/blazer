use super::super::cases::case::get_case;
use super::super::parser::string::string_to_term;
use rustler::{Term, TermType};

#[derive(Debug)]
pub struct BlazerOptions<'a> {
    pub case: convert_case::Case,
    pub keys: &'a TermType,
}

impl<'b> BlazerOptions<'b> {
    pub fn from_term<'a>(term: Term<'a>) -> Result<BlazerOptions, Term<'a>> {
        let mut options = BlazerOptions {
            keys: &TermType::Binary,
            case: convert_case::Case::Camel,
        };

        let env = term.get_env();

        let list_iterator: Vec<(Term<'a>, Term<'a>)> = match term.decode() {
            Ok(iterator) => iterator,
            Err(_) => return Err(string_to_term(
                    "couldn't iterate through opts".into(),
                    &rustler::TermType::Binary,
                    env)),
        };
        for (key, value) in list_iterator.into_iter() {
            let string_key = atom_to_string(&key)?;
            match &string_key[..] {
                "case" => options.case = get_case(&value)?,
                "keys" => options.keys = resolve_keys(&value)?,
                _ => (),
            };
        }
        Ok(options)
    }
}

fn atom_to_string<'a>(term: &Term<'a>) -> Result<String, Term<'a>> {
    match Term::atom_to_string(term) {
        Ok(string) => Ok(string),
        Err(_) => Err(string_to_term(
            "couldn't parse atom into string".into(),
            &rustler::TermType::Binary,
            term.get_env(),
        )),
    }
}

fn resolve_keys<'a, 'b>(term: &Term<'a>) -> Result<&'b TermType, Term<'a>> {
    let string_value: String = atom_to_string(term)?;
    match &string_value[..] {
        "atoms" | "atoms!" => Ok(&TermType::Atom),
        "strings" => Ok(&TermType::Binary),
        _ => Err(string_to_term(
            "invalid :keys option".into(),
            &rustler::TermType::Binary,
            term.get_env(),
        )),
    }
}
