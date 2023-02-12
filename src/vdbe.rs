use crate::models::Row;
use bincode::{deserialize, serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

const DB_PATH: &str = "data/cavea.db";

pub struct Vdbe {}

impl Vdbe {
    pub fn read() -> Result<String, String> {
        let mut file = Self::open_file_at(false, 0);
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
        let mut file = Self::open_file_at(true, 0);

        // write on file
        let bytes = serialize(&node).unwrap();
        file.write_all(bytes.as_slice()).unwrap();

        let result = format!("added string {:?}", node);
        Ok(result)
    }

    fn open_file_at(write_permission: bool, position: u64) -> File {
        let mut file = OpenOptions::new()
            .read(true)
            .write(write_permission)
            .open(DB_PATH)
            .unwrap_or_else(|error| {
                File::create(DB_PATH)
                    .unwrap_or_else(|error| panic!("was not able to create the file"))
            });

        file.seek(SeekFrom::Start(position)).unwrap();
        file
    }
}
