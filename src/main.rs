extern crate pretty_env_logger;
extern crate regex;
#[macro_use] extern crate log;

#[macro_use] mod parser;

use rustyline::error::ReadlineError;
use crate::parser::*;

fn main() {
    pretty_env_logger::init();
    debug!("starting");

    let whitespace = || {
        repeat(regex("[\\s]"))
    };

    let number = || {
      repeat1(digit())
    };

    let operator = || {
        regex("[+/*-]")
    };

    let mut expr = Parser::new();
    let inner_expr = Box::new(
        one_of!(number(),
            seq!(ch('('), operator(), whitespace(), repeat1(expr.make()), whitespace(), ch(')')))
    );

    expr.update(inner_expr);

    let parser = expr.make();

    let mut rl = rustyline::Editor::<()>::new();
    loop {
        let readline = rl.readline("lisp> ");
        match readline {
            Ok(line) => {
                let result = parser(&line);
                match result {
                    ParseResult::Value {value: _, remaining_input: _} => {
                        println!("success!");
                    }
                    ParseResult::Error{text} => {
                        println!("error: {}", text);
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
                break;
            },
            Err(ReadlineError::Eof) => {
                break;
            },
            Err(err) => {
                error!("error: {:?}", err);
                break;
            }
        }
    }
}
