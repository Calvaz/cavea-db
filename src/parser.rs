use crate::{
    tokenizer::{StatementType, ROW, TABLE},
    vdbe::Vdbe,
};

pub struct Parser<'a> {
    pub(crate) tokens: &'a Vec<&'a str>,
}

impl<'a> Parser<'a> {
    pub fn new(sentence: &'a Vec<&str>) -> Parser<'a> {
        Parser { tokens: sentence }
    }

    pub fn parse_statement(&self) -> Result<StatementType, String> {
        let result = match self.tokens[0] {
            "new" => StatementType::Insert, // self.parse_new(&self.tokens[1..]).unwrap(),
            "select" => StatementType::Select, //self.parse_select(&self.tokens[1..]).unwrap(),
            _ => return Err(format!("Unrecognized command: {}.\n", self.tokens[0])),
        };
        Ok(result)
    }

    pub fn parse_args(&self, statement: &StatementType) -> Result<(String, &[&str]), String> {
        match statement {
            StatementType::Select => self.parse_select(&self.tokens[1..]),
            StatementType::Insert => self.parse_new(&self.tokens[1..]),
            _ => return Err(String::from("statement type not valid")),
        }
    }

    fn parse_new(&self, tokens: &'a [&'a str]) -> Result<(String, &[&str]), String> {
        if let "table" = tokens[0] {
            return Err(String::from("inserting new table is not implemented yet"));
        } else {
            Ok((String::from("row"), tokens))
        }
    }

    fn parse_select(&self, tokens: &'a [&'a str]) -> Result<(String, &[&str]), String> {
        match tokens[0] {
            "*" => Ok((String::from("row"), tokens)),
            _ => {
                return Err(String::from(
                    "selecting columns or other is not implemented yet",
                ))
            }
        }
    }

    fn peek_token(&self, tokens: &'a [&'a str], current_index: usize) -> &str {
        &tokens[current_index + 1]
    }
}
