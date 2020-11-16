// mu/read.rs
use std::io::{self, BufRead};

use crate::mu::r#type::Type;
use crate::mu::r#type::NIL;

use crate::mu::char::Char;
use crate::mu::fixnum::Fixnum;
use crate::mu::string::String;
use crate::mu::symbol::Symbol;

use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_until, take_while},
    character::{is_alphanumeric, is_space},
    combinator::{map_res, opt},
    multi::many0,
    sequence::tuple,
    IResult,
};

/*
#          non-terminating macro char
\          single escape
|          multiple escape
 */

// terminating macro
/*
"          terminating macro char
'          terminating macro char
(          terminating macro char
)          terminating macro char
,          terminating macro char
;          terminating macro char  
`          terminating macro char
 */

// constituent
fn is_constituent(ch: char) -> bool {
    const CONSTITUENT: &str = "0123456789\
                               !$%&*+-./:<=>?@[]^_{}~\
                               ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                               abcdefghijklmnopqrstuvwxyz";

    // println!("{} {}", ch, CONSTITUENT.contains(ch));
    CONSTITUENT.contains(ch)
}

// whitespace

/*
Linefeed   whitespace[2]
Newline    whitespace[2]
Page       whitespace[2]
Return     whitespace[2]
Space      whitespace[2]
Tab        whitespace[2]
*/

fn not_parsed(input: &str) -> IResult<&str, Type> {

    println!("not parsed: {} ", input);
    assert!(false);
    Ok((input, NIL))
}

// numbers
fn read_hexadecimal(input: &str) -> IResult<&str, Type> {
    let (input, _) = tag("#x")(input)?;
    let (input, hex) = || -> IResult<&str, i64> {
        map_res(take_while(|c: char| c.is_digit(16)), |input: &str| {
            i64::from_str_radix(input, 16)
        })(input)
    }()?;

    Ok((input, Fixnum::make_type(hex)))
}

fn read_decimal(input: &str) -> IResult<&str, Type> {

    println!("testing fixnum {}", input);
    
    let (input, dec) = || -> IResult<&str, i64> {
        map_res(take_while(|c: char| c.is_digit(10)), |input: &str| {
            i64::from_str_radix(input, 10)
        })(input)
    }()?;

    Ok((input, Fixnum::make_type(dec)))
}

// string/char
fn read_string(input: &str) -> IResult<&str, Type> {
    let (input, _) = tag("\"")(input)?;
    let (input, str) = take_until("\"")(input)?;

    Ok((input, String::make_type(str)))
}

fn read_char(input: &str) -> IResult<&str, Type> {
    
    println!("testing {} for char", input);
    
    let (input, _) = tag("#\\")(input)?;
    let (input, ch) = take(1 as usize)(input)?;

    Ok((input, Char::make_type(ch.chars().nth(0).unwrap())))
}

// special forms
fn read_quote(input: &str) -> IResult<&str, Type> {
    let (input, _) = tag("'")(input)?;
    let (input, form) = alt((
        read_char,
        read_hexadecimal,
        read_list,
        read_quote,
        read_string,
        read_vector,
        read_decimal,
        read_symbol,
        not_parsed
    ))(input)?;

    Ok((input, form))
}

// lists/vectors
fn vec_to_list(list: Type, i: usize, v: &Vec<Type>) -> Type {
    println!("vec_to_list {}", i);
    if i == v.len() {
        list
    } else {
        println!("vect_to_list: {}", i);
        vec_to_list(v[i].cons(list), i + 1, v)
    }
}

fn read_list(input: &str) -> IResult<&str, Type> {

    println!("testing {} for list", input);
    
    let (input, (_, v, _)) = tuple((tag("("), many0(read_form), tag(")")))(input)?;
    
    println!("we got a vec len {}", v.len());
    Ok((input, vec_to_list(NIL, 0, &v)))
}

fn read_vector(input: &str) -> IResult<&str, Type> {

    println!("testing {} for vector", input);
    
    let (input, (_, v, _)) = tuple((tag("#("), many0(read_form), tag(")")))(input)?;
    
    Ok((input, vec_to_list(NIL, 0, &v)))
}

// symbols
fn read_symbol(input: &str) -> IResult<&str, Type> {
    let (input, str) = take_while(|ch: char| is_constituent(ch))(input)?;

    println!("symbol: {}", str);
    
    Ok((input, Symbol::make_type(String::make_type(str), NIL)))
}

// reader
fn read_form(input: &str) -> IResult<&str, Type> {
    let (input, _) = take_while(|ch: char| ch.is_ascii_whitespace())(input)?;

    println!("read_form: {}", input);
    
    alt((
        read_char,
        read_hexadecimal,
        read_list,
        read_quote,
        read_string,
        read_vector,
        read_decimal,
        read_symbol,
        not_parsed
    ))(input)
}

pub fn read_from_stdin(stream: Type) -> Type {
    let input = io::stdin().lock().lines().next().unwrap().unwrap();

    match read_form(&input) {
        Ok((_, t)) => t,
        Err(err) => {
            println!("unread {:?}", err);
            NIL
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex() {
        assert!(match read_hexadecimal("#x2F14DF") {
            Ok(("", fx)) => match fx.i64_from_fixnum() {
                Some(ival) => ival == 0x2f14df,
                _ => false,
            },
            _ => false,
        })
    }

    #[test]
    fn test_dec() {
        assert!(match read_decimal("123456") {
            Ok(("", fx)) => match fx.i64_from_fixnum() {
                Some(ival) => ival == 123456,
                _ => false,
            },
            _ => false,
        })
    }

    #[test]
    fn test_string() {
        assert!(match read_string("\"abc123\"") {
            Ok(("", str)) => str.typep_string(),
            _ => false,
        })
    }

    #[test]
    fn test_char() {
        assert!(match read_char("#\\a") {
            Ok(("", ch)) => ch.typep_char(),
            _ => false,
        })
    }

    #[test]
    fn test_symbol() {
        assert!(match read_symbol("abc123") {
            Ok(("", sym)) => sym.typep_symbol(),
            _ => false,
        })
    }

    #[test]
    fn test_keyword() {
        assert!(match read_symbol(":abc123") {
            Ok(("", kwd)) => kwd.typep_keyword(),
            _ => false,
        })
    }

    /*
    #[test]
    fn test_string() {
        assert!(match string_(b"\"abc123\" ") {
            Ok((_, (_, str, _))) => {
                let _st = string(str);
                true
            }
            Err(_) => false,
        })
    }

    #[test]
    fn test_char() {
        assert!(match char_(b"#\\a ") {
            Ok((_, (_, _ch))) => true,
            Err(_) => false,
        })
    }

    #[test]
    fn test_dotted() {
        assert!(match dotted_(b"( 123 . 456 ) ") {
            Ok((_, (_, _car, _, _, _, _cdr, _, _))) => _car.typep_fixnum(),
            Err(_) => false,
        })
    }

    #[test]
    fn test_list() {
        assert!(match list_(b"( 1234 5678 ) ") {
            Ok((_, (_, _vec, _, _))) => true,
            Err(_) => false,
        })
    }

    #[test]
    fn test_nil() {
        assert!(match nil_(b"( ) ") {
            Ok((_, (_, _, _))) => true,
            Err(_) => false,
        })
    }
     */
}
