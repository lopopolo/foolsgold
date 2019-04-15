use mruby_sys::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Rust {
    Bool,
    SignedInt,
    UnsignedInt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Ruby {
    Bool,
    Class,
    CPointer,
    Exception,
    Fixnum,
    Float,
    Module,
    Nil,
    Object,
    String,
    Symbol,
}

impl From<mrb_value> for Ruby {
    #[allow(non_upper_case_globals)]
    fn from(value: mrb_value) -> Self {
        // `nil` is implemented with the FALSE type tag in mruby (since both
        // values are falsy). The difference is that booleans are non-zero
        // `Fixnum`s.
        if unsafe { mrb_sys_value_is_nil(value) } {
            return Ruby::Nil;
        }

        // switch on the type tag in the `mrb_value`
        match value.tt {
            mrb_vtype_MRB_TT_ARRAY => unimplemented!("mruby type array"),
            mrb_vtype_MRB_TT_FALSE | mrb_vtype_MRB_TT_TRUE => Ruby::Bool,
            mrb_vtype_MRB_TT_BREAK => unimplemented!("mruby type break"), // TODO: what is this?
            mrb_vtype_MRB_TT_CLASS => Ruby::Class,
            mrb_vtype_MRB_TT_CPTR => Ruby::CPointer,
            mrb_vtype_MRB_TT_DATA => unimplemented!("mruby type data"), // TODO: what is this?
            mrb_vtype_MRB_TT_ENV => unimplemented!("mruby type env"),   // TODO: what is this?
            mrb_vtype_MRB_TT_EXCEPTION => Ruby::Exception,
            mrb_vtype_MRB_TT_FIBER => unimplemented!("mruby type fiber"), // WONTFIX
            mrb_vtype_MRB_TT_FILE => unimplemented!("mruby type file"),
            mrb_vtype_MRB_TT_FIXNUM => Ruby::Fixnum,
            mrb_vtype_MRB_TT_FLOAT => Ruby::Float,
            mrb_vtype_MRB_TT_HASH => unimplemented!("mruby type hash"),
            mrb_vtype_MRB_TT_ICLASS => unimplemented!("mruby type iclass"),
            mrb_vtype_MRB_TT_ISTRUCT => unimplemented!("mruby type istruct"),
            mrb_vtype_MRB_TT_MAXDEFINE => unimplemented!("mruby type maxdefine"),
            mrb_vtype_MRB_TT_MODULE => Ruby::Module,
            mrb_vtype_MRB_TT_PROC => unimplemented!("mruby type proc"),
            mrb_vtype_MRB_TT_OBJECT => Ruby::Object,
            mrb_vtype_MRB_TT_RANGE => unimplemented!("mruby type range"),
            mrb_vtype_MRB_TT_SCLASS => unimplemented!("mruby type sclass"),
            mrb_vtype_MRB_TT_STRING => Ruby::String,
            mrb_vtype_MRB_TT_SYMBOL => Ruby::Symbol,
            mrb_vtype_MRB_TT_UNDEF => unimplemented!("mruby type undef"), // TODO: what is this?
            _ => unreachable!(
                "Unknown mruby type. See include/mruby/value.h in vendored mruby source."
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use mruby_sys::*;
    use std::ffi::CString;

    use crate::value::types::*;

    #[test]
    fn nil_type() {
        unsafe {
            let value = mrb_sys_nil_value();
            assert_eq!(Ruby::from(value), Ruby::Nil);
        }
    }

    #[test]
    fn bool_type() {
        unsafe {
            let value = mrb_sys_false_value();
            assert_eq!(Ruby::from(value), Ruby::Bool);
            let value = mrb_sys_true_value();
            assert_eq!(Ruby::from(value), Ruby::Bool);
        }
    }

    #[test]
    fn fixnum_type() {
        unsafe {
            let value = mrb_sys_fixnum_value(17);
            assert_eq!(Ruby::from(value), Ruby::Fixnum);
        }
    }

    #[test]
    fn string_type() {
        unsafe {
            let mrb = mrb_open();
            let literal = "dinner plate";
            let cstr = CString::new(literal).unwrap();
            let value = mrb_str_new_cstr(mrb, cstr.as_ptr());
            assert_eq!(Ruby::from(value), Ruby::String);
            mrb_close(mrb);
        }
    }
}