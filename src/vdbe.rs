use crate::models::Row;
use bincode::{deserialize, serialize};
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};

pub struct Vdbe {}

impl Vdbe {
    pub fn read() -> Result<String, String> {
        let mut file = match OpenOptions::new().read(true).write(false).open("db.cavea") {
            Ok(f) => f,
            Err(e) => {
                return Err(format!("could not read file because {e}"));
            }
        };
        file.seek(SeekFrom::Start(0)).unwrap();
        let mut buffer = vec![0; 4096];
        let len = file.metadata().unwrap().len();

        // otherwise we might hit end of file
        if len < 4096 {
            buffer = vec![0; len as usize];
        }
        println!("{:?}", buffer.len());

        file.read_exact(&mut buffer).unwrap();
        let row = deserialize::<Row>(&mut buffer).unwrap();
        Ok(row.value)
    }

    pub fn write(node: Row) -> Result<String, String> {
        let mut file = match OpenOptions::new().read(true).write(true).open("db.cavea") {
            Ok(f) => f,
            Err(e) => return Err(format!("could not read file because {e}")),
        };
        let bytes = serialize(&node).unwrap();

        // write on file
        file.seek(SeekFrom::Start(0)).unwrap();
        file.write_all(bytes.as_slice()).unwrap();

        let result = format!("added string {:?}", node);
        Ok(result)
    }
}
