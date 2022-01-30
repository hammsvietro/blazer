use rustler::Term;
use super::key_type::KeyType;
use convert_case::Case;

pub fn get_case(term: &Term, key_type: KeyType) -> Case {
    match (term.atom_to_string().unwrap().as_str(), key_type) {
       ("camel", _)  => Case::Camel,
       ("pascal", KeyType::String) => Case::Pascal,
       ("snake", _) => Case::Snake,
       ("upper", _) => Case::Upper,
       ("title", _) => Case::Title,
       _ => panic!("There's no such type for this key type")
    }
} 
