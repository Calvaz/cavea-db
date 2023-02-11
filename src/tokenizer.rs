use std::error::Error;
use std::fmt;

use crate::parser::Parser;

#[derive(Debug, PartialEq, Eq)]
pub enum MetaCommand {
    Success,
    Specs,
    Exit,
}

impl fmt::Display for MetaCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MetaCommand::Success => write!(f, ""),
            MetaCommand::Specs => write!(f, ""),
            MetaCommand::Exit => write!(f, "exiting ..."),
        }
    }
}

pub enum StatementType {
    Select,
    Insert,
    Update,
}

pub fn process_tokens(tokens: &str) -> Result<MetaCommand, String> {
    if tokens.starts_with(".") {
        let _meta_command: MetaCommand = match apply_meta_command(tokens) {
            Err(e) => return Err(e),
            Ok(r) if r == MetaCommand::Exit => return Ok(r),
            Ok(r) => r,
        };
    }

    let _statement = match prepare_statement(tokens) {
        Ok(statement) => statement,
        Err(e) => return Err(e),
    };

    // execute_statement(statement);

    Ok(MetaCommand::Success)
}

fn apply_meta_command(tokens: &str) -> Result<MetaCommand, String> {
    match tokens {
        ".exit" => return Ok(MetaCommand::Exit),
        ".specs" => return Ok(MetaCommand::Specs),
        _ => Err(format!("Unrecognized command: {}.\n", tokens)),
    }
}

pub fn prepare_statement(sentence: &str) -> Result<StatementType, String> {
    let tokens: Vec<&str> = sentence.split(" ").collect::<Vec<&str>>();
    let parser = Parser::new(&tokens);
    println!("{}", &parser.parse().unwrap());
    Ok(StatementType::Select)
}

pub fn execute_statement(statement: StatementType) -> Result<(), Box<dyn Error>> {
    match statement {
        StatementType::Select => println!("selecting"),
        StatementType::Insert => println!("inserting"),
        StatementType::Update => println!("updating"),
    }

    Ok(())
}
