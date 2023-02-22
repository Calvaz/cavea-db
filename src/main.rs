use std::io;
use std::io::Write;

mod btree;
mod cursor;
mod models;
mod pager;
mod parser;
mod tokenizer;
mod vdbe;

fn main() {
    loop {
        print!("db > ");
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).is_ok();
        let token = match buffer.trim_end() {
            "" => "Command not found".into(),
            word => word,
        };

        match tokenizer::process_tokens(token) {
            Err(e) => {
                println!("{e}");
                std::process::exit(1);
            }
            Ok(r) if r == tokenizer::MetaCommand::Exit => {
                println!("{r}");
                std::process::exit(1);
            }
            Ok(r) => {
                println!("{r}")
            }
        }
    }
}
