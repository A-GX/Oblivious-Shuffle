#![allow(dead_code)]
#![allow(unused)]

use std::io;
use std::{fmt,io::{Result, Error, ErrorKind}};

/// Stop and waits for user input before conitnuing the execution.
/// User input is then thrown away.
/// It is intended to help in debugging function.
/// 
/// 
/// # Example Utilisation:
/// 
/// 
/// fn to_debug() {
///     ...
///     println!("Message you need to be able to read");
///     wait_for_user();
///     ...
/// }
/// 
pub fn wait_for_user()
{
    let mut user_input = String::new();
    let _stdin = io::stdin();
    let _err = _stdin.read_line(&mut user_input).unwrap();
}
