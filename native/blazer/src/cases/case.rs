use rustler::Term;
use convert_case::Case;
use super::super::parser::string::string_to_term;

pub fn get_case<'a>(term: &Term<'a>) -> Result<Case, Term<'a>> {
    match term.atom_to_string().unwrap().as_str() {
       "camel"  => Ok(Case::Camel),
       "pascal" => Ok(Case::Pascal),
       "snake" => Ok(Case::Snake),
       "upper" => Ok(Case::Upper),
       "title" => Ok(Case::Title),
       _ => {
           let env = term.get_env();
           Err(string_to_term("this case is not supported, refer to the documentation for more info.".into(), env))
       }
    }
} 
