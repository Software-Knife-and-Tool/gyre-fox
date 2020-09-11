/* mu/print.rs */
// use std::io;

use std::char::from_u32;

use crate::mu::r#type::Type;
use crate::mu::r#type::SysClass;
use crate::mu::fixnum::*;

pub fn _print(src: Type) {
    match src.type_of() {
        SysClass::String => {
            let str = &Type::string_from_type(&src);
            println!("\"{}\"", &str._value)
        },
        SysClass::Symbol =>
            if src.type_keyword() {
                println!(":{}", "wahoo");
            } else {
                let sym = Type::symbol_from_type(&src);
                let name = &sym._name;
                let str = Type::string_from_type(name);
                println!("{}", &str._value);
            },
        SysClass::Fixnum =>
            match _Fixnum::_from_type(&src) {
                Some(fx) => fx._print(),
                None => println!("isn't a fixnum")
            },
        SysClass::Char =>
            println!("#\\{}",
                     from_u32(src.immediate_data() as u32).unwrap()),
        SysClass::Function => println!("is a function"),
        SysClass::Cons => println!("is a cons"),
        SysClass::T => println!("is a T"),
        _ => println!("undecoded")
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    /*
    #[test]
    fn test_type() {
        assert!(fixnum(0).type_fixnum());
    }

    #[test]
    fn test_u64() {
        assert!(fixnum(0).u64_from_fixnum() == 0);
        assert!(fixnum(1).u64_from_fixnum() == 1);
    }

    #[test]
    fn test_eq() {
        assert!(fixnum(0).eq(fixnum(0)));
    }
    
    #[test]
    fn test_add() {
        assert!(fixnum(1).fixnum_add(fixnum(2)).eq(fixnum(3)));
    }
     */
}
