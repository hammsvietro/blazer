use rustler::Term;
use super::super::parser::string::string_to_term;
use super::super::cases::case::get_case;

pub enum Keys {
  Atoms,
  Strings,
}

pub struct BlazerOptions {
  case: convert_case::Case,
  keys: Keys
}

impl BlazerOptions {
  pub fn from_term<'a>(term: Term<'a>) -> Result<BlazerOptions, Term<'a>> {
    let mut options = BlazerOptions {
      keys: Keys::Strings,
      case: convert_case::Case::Camel
    };

    let env = term.get_env();

    let list_iterator: Vec<(Term<'a>, Term<'a>)> = match term.decode() {
        Ok(iterator) => iterator,
        Err(_) => return Err(string_to_term("couldn't iterate through opts".into(), env))
    };
    for (key, value) in list_iterator.into_iter() {
      let string_key = atom_to_string(&key);
      match &string_key[..] {
        "case" => {
            options.case = match get_case(&value) {
                Ok(case) => case,
                Err(err) => return Err(err)
            };
        },
        "keys" => {
            options.keys = match resolve_keys(&value) {
                Ok(keys) => keys,
                Err(error) => return Err(error)
            };
        },
        _ => {},
      };
    };
    Ok(options)
  }
}

fn atom_to_string<'a>(term: &Term<'a>) -> String {
    Term::atom_to_string(term).unwrap()
}

fn resolve_keys<'a>(term: &Term<'a>) -> Result<Keys, Term<'a>> {
    let string_value: String = atom_to_string(term);
    match &string_value[..] {
      "atoms"|"atoms!" => Ok(Keys::Atoms),
      "strings" => Ok(Keys::Strings),
      _ => {
        let env = term.get_env();
        Err(string_to_term("invalid :keys option".into(), env))
      }
    }
}
