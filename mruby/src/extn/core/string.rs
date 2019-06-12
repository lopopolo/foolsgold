use crate::def::Define;
use crate::eval::MrbEval;
use crate::interpreter::Mrb;
use crate::MrbError;
use log::trace;

pub fn patch(interp: &Mrb) -> Result<(), MrbError> {
    let string = interp
        .borrow_mut()
        .def_class::<RString>("String", None, None);
    string.borrow().define(interp).map_err(|_| MrbError::New)?;
    interp.eval(include_str!("string.rb"))?;
    trace!("Patched String onto interpreter");
    Ok(())
}

#[allow(clippy::module_name_repetitions)]
pub struct RString;

// Tests from String core docs in Ruby 2.6.3
// https://ruby-doc.org/core-2.6.3/String.html
#[cfg(test)]
mod tests {
    use crate::eval::MrbEval;
    use crate::extn::core::string;
    use crate::interpreter::Interpreter;

    #[test]
    fn string_equal_squiggle() {
        let interp = Interpreter::create().expect("mrb init");
        string::patch(&interp).expect("string init");

        let value = interp.eval(r#""cat o' 9 tails" =~ /\d/"#).unwrap();
        assert_eq!(value.try_into::<Option<i64>>(), Ok(Some(7)));
        let value = interp.eval(r#""cat o' 9 tails" =~ 9"#).unwrap();
        assert_eq!(value.try_into::<Option<i64>>(), Ok(None));
    }

    #[test]
    fn string_idx() {
        let interp = Interpreter::create().expect("mrb init");
        string::patch(&interp).expect("string init");

        assert_eq!(
            &interp
                .eval(r"'hello there'[/[aeiou](.)\1/]")
                .unwrap()
                .try_into::<String>()
                .unwrap(),
            "ell"
        );
        assert_eq!(
            &interp
                .eval(r"'hello there'[/[aeiou](.)\1/, 0]")
                .unwrap()
                .try_into::<String>()
                .unwrap(),
            "ell"
        );
        assert_eq!(
            &interp
                .eval(r"'hello there'[/[aeiou](.)\1/, 1]")
                .unwrap()
                .try_into::<String>()
                .unwrap(),
            "l"
        );
        assert_eq!(
            interp
                .eval(r"'hello there'[/[aeiou](.)\1/, 2]")
                .unwrap()
                .try_into::<Option<String>>()
                .unwrap(),
            None
        );
        assert_eq!(
            &interp
                .eval(r"'hello there'[/(?<vowel>[aeiou])(?<non_vowel>[^aeiou])/, 'non_vowel']")
                .unwrap()
                .try_into::<String>()
                .unwrap(),
            "l"
        );
        assert_eq!(
            &interp
                .eval(r"'hello there'[/(?<vowel>[aeiou])(?<non_vowel>[^aeiou])/, 'vowel']")
                .unwrap()
                .try_into::<String>()
                .unwrap(),
            "e"
        );
    }
}