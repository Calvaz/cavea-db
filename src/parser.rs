use crate::{models::Row, vdbe::Vdbe};

pub struct Parser<'a> {
    pub(crate) tokens: &'a Vec<&'a str>,
}

impl<'a> Parser<'a> {
    pub fn new(sentence: &'a Vec<&str>) -> Parser<'a> {
        Parser { tokens: sentence }
    }

    pub fn parse(&self) -> Result<String, String> {
        let result = match self.tokens[0] {
            "new" => self.parse_new(&self.tokens[1..]).unwrap(),
            "select" => self.parse_select(&self.tokens[1..]).unwrap(),
            _ => String::from("nothing was executed"),
        };
        Ok(result)
    }

    fn parse_new(&self, tokens: &[&str]) -> Result<String, String> {
        let row = Row {
            value: String::from(tokens[0]),
        };
        Vdbe::write(row)
    }

    fn parse_select(&self, tokens: &[&str]) -> Result<String, String> {
        match tokens[0] {
            "*" => Vdbe::read(),
            _ => Ok(format!("'{}' not found", tokens[0])),
        }
    }

    fn peek_token(&self, tokens: Vec<&'a str>, current_index: usize) -> &str {
        &tokens[current_index + 1]
    }
}
