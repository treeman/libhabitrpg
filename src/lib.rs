#![feature(macro_rules)]
#![feature(phase)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![feature(globs)]
#![feature(slicing_syntax)]

#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

extern crate serialize;
extern crate time;

pub use data::*;

mod data;
mod json_helpers;
