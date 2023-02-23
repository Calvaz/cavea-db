use std::error::Error;
use std::fmt;

use crate::models::Table;
use crate::pager::Pager;
use crate::parser::Parser;
use crate::vdbe::Vdbe;

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

pub const ROW: &str = "row";
pub const TABLE: &str = "table";

pub enum StatementType {
    Select,
    Insert,
    Update,
}

pub struct Statement<'a>(StatementType, String, &'a [&'a str]);

pub fn process_tokens(tokens: &str) -> Result<MetaCommand, String> {
    if tokens.starts_with(".") {
        let _meta_command: MetaCommand = match apply_meta_command(tokens) {
            Err(e) => return Err(e),
            Ok(r) if r == MetaCommand::Exit => return Ok(r),
            Ok(r) => r,
        };
    }

    let tokens_list: Vec<&str> = tokens.split(" ").collect::<Vec<&str>>();
    let parser = Parser::new(&tokens_list);
    let statement_type = match prepare_statement(&parser) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    let args = parser.parse_args(&statement_type).unwrap();

    // implement statement details based on statement type
    execute_statement(Statement(statement_type, args.0, args.1));
    Ok(MetaCommand::Success)
}

fn apply_meta_command(tokens: &str) -> Result<MetaCommand, String> {
    match tokens {
        ".exit" => return Ok(MetaCommand::Exit),
        ".specs" => return Ok(MetaCommand::Specs),
        _ => Err(format!("Unrecognized command: {}.\n", tokens)),
    }
}

pub fn prepare_statement(parser: &Parser) -> Result<StatementType, String> {
    parser.parse_statement()
}

pub fn execute_statement(statement: Statement) -> Result<(), Box<dyn Error>> {
    let table = Table {
        pager: Pager::new(),
        num_pages: 0,
        root_node: 0,
    };

    match statement.0 {
        StatementType::Select => {
            println!("selecting");
            table.pager.read_page(0)?;
        }
        StatementType::Insert => {
            println!("inserting");
            table.pager.append(statement.2)?;
        }
        StatementType::Update => println!("updating"),
    };

    Ok(())
}
