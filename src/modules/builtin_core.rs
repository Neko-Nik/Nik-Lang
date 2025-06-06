//! Built-in functions for the interpreter
//! These functions are available in the interpreter environment
//! and can be called directly from the user code

use std::io::{self, Write};
use regex::Regex;
use crate::interpreter::value::Value;


/// Unescapes a string by replacing escape sequences with their corresponding characters
fn unescape_string(s: &str) -> String {
    // Regex to match escape sequences like \n, \t, \\, \r, \", \'
    let re = Regex::new(r#"\\([ntr\\"'])"#).unwrap();

    // Replace each escape sequence with the corresponding char
    re.replace_all(s, |caps: &regex::Captures| {
        match &caps[1] {
            "n" => "\n",
            "t" => "\t",
            "r" => "\r",
            "\\" => "\\",
            "\"" => "\"",
            "'" => "'",
            _ => &caps[0], // fallback (shouldn't happen)
        }.to_string()
    }).to_string()
}


/// Built-in function to print values to the console
/// It accepts any number of arguments and prints them in a single line
pub fn builtin_print(args: Vec<Value>) -> Result<Value, String> {
    let output: Vec<String> = args.into_iter().map(|v| {
        match v {
            Value::String(s) => unescape_string(&s),
            _ => v.to_string(),
        }
    }).collect();

    println!("{}", output.join(" "));
    Ok(Value::Null)
}


/// Built-in function to get the length of any possible type
/// Currently only works on strings
pub fn builtin_len(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("len() takes exactly one argument".to_string());
    }

    match &args[0] {
        Value::String(s) => Ok(Value::Integer(s.len() as i64)),
        Value::Array(a) => Ok(Value::Integer(a.len() as i64)),
        Value::Tuple(t) => Ok(Value::Integer(t.len() as i64)),
        Value::HashMap(h) => Ok(Value::Integer(h.len() as i64)),
        _ => Err(format!("len() expects a string, array, tuple, or hashmap, but got {:?}", args[0])),
    }
}


/// Built-in function to convert a value to a string
/// Currently only works on strings, integers, floats, and booleans
pub fn builtin_str(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("str() takes exactly one argument".to_string());
    }

    match &args[0] {
        Value::String(s) => Ok(Value::String(s.clone())),
        Value::Integer(i) => Ok(Value::String(i.to_string())),
        Value::Float(f) => Ok(Value::String(f.to_string())),
        Value::Bool(b) => Ok(Value::String(b.to_string())),
        Value::Null => Ok(Value::String("None".to_string())),
        Value::Array(a) => Ok(Value::String(format!("{:?}", a))),
        Value::Tuple(t) => Ok(Value::String(format!("{:?}", t))),
        Value::HashMap(h) => Ok(Value::String(format!("{:?}", h))),
        _ => Err(format!("str() expects a string, integer, float, boolean, array, tuple, or hashmap, but got {:?}", args[0])),
    }
}


/// Built-in function to convert a value to an integer
/// Currently only works on strings, integers, and floats
/// Strings are converted to integers if they are valid integer representations
/// Floats are truncated to integers
pub fn builtin_int(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("int() takes exactly one argument".to_string());
    }

    match &args[0] {
        Value::String(s) => s.parse::<i64>()
            .map(Value::Integer)
            .map_err(|_| format!("Invalid string for int conversion: {}", s)),
        Value::Integer(i) => Ok(Value::Integer(*i)),
        Value::Float(f) => Ok(Value::Integer(*f as i64)),
        _ => Err(format!("int() expects a string, integer, or float, but got {:?}", args[0])),
    }
}


/// Built-in function to convert a value to a float
/// Currently only works on strings, integers, and floats
/// Strings are converted to floats if they are valid float representations
/// Integers are converted to floats by adding .0
pub fn builtin_float(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("float() takes exactly one argument".to_string());
    }

    match &args[0] {
        Value::String(s) => s.parse::<f64>()
            .map(Value::Float)
            .map_err(|_| format!("Invalid string for float conversion: {}", s)),
        Value::Integer(i) => Ok(Value::Float(*i as f64)),
        Value::Float(f) => Ok(Value::Float(*f)),
        _ => Err(format!("float() expects a string, integer, or float, but got {:?}", args[0])),
    }
}


/// Built-in function to convert a value to a boolean
/// Currently only works on strings, integers, and floats
/// Strings are converted to booleans if there is even one character
/// Integers, Floats are converted to booleans if they are not 0
pub fn builtin_bool(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("bool() takes exactly one argument".to_string());
    }

    match &args[0] {
        Value::String(s) => Ok(Value::Bool(!s.is_empty())),
        Value::Integer(i) => Ok(Value::Bool(*i != 0)),
        Value::Float(f) => Ok(Value::Bool(*f != 0.0)),
        _ => Err(format!("bool() expects a string, integer, or float, but got {:?}", args[0])),
    }
}


/// Built-in function to exit the interpreter
/// Only works with single argument and of type integer
/// The integer is the exit code
pub fn builtin_exit(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("exit() takes exactly one argument".to_string());
    }

    match &args[0] {
        Value::Integer(i) => {
            std::process::exit(*i as i32);
        }
        _ => Err(format!("exit() only works with integer argument, got {:?}", args[0])),
    }
}


/// Built-in function to get the type of a value
/// Currently only works on strings, integers, floats, and booleans
pub fn builtin_type(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("type() takes exactly one argument".to_string());
    }

    match &args[0] {
        Value::String(_) => Ok(Value::String("String".to_string())),
        Value::Integer(_) => Ok(Value::String("Integer".to_string())),
        Value::Float(_) => Ok(Value::String("Float".to_string())),
        Value::Bool(_) => Ok(Value::String("Boolean".to_string())),
        Value::Null => Ok(Value::String("None".to_string())),
        Value::Array(_) => Ok(Value::String("Array".to_string())),
        Value::Tuple(_) => Ok(Value::String("Tuple".to_string())),
        Value::HashMap(_) => Ok(Value::String("HashMap".to_string())),
        // _ => Err(format!("type() does not support this type: {:?}", args[0])),
        _ => Err(format!("type() only works with strings, integers, floats, booleans, none, arrays, tuples, and hashmaps, but got {:?}", args[0])),
    }
}


/// Built-in function to get input from the user
/// Currently only works with strings
/// Returns the input as a string
pub fn builtin_input(args: Vec<Value>) -> Result<Value, String> {
    let prompt = match args.len() {
        0 => "> ",
        1 => {
            if let Value::String(s) = &args[0] {
                s.as_str()
            } else {
                return Err("input() argument must be a string".to_string());
            }
        }
        _ => return Err(format!("input() takes at most one argument, but got {}", args.len())),
    };

    print!("{}", prompt);
    io::stdout().flush().map_err(|e| format!("Failed to flush stdout: {}", e))?;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("Failed to read input: {}", e))?;

    Ok(Value::String(input.trim().to_string()))
}
