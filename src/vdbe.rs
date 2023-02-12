use crate::models::Row;
use bincode::{deserialize, serialize};
use std::any::TypeId;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

const DB_PATH: &str = "data";
const DB_FILE_PATH: &str = "./data/cavea.db";

pub struct Vdbe {}

impl Vdbe {
    pub fn read() -> Result<String, String> {
        let mut file = Self::open_file_at(false, 0);
        let mut buffer = vec![0; 4096];

        let len = file.metadata().unwrap().len();
        if len == 0 {
            return Ok(String::from("no record found"));
        } else if len < 4096 {
            // otherwise we might hit end of file
            buffer = vec![0; len as usize];
        }

        println!("{:?}", buffer.len());

        file.read_exact(&mut buffer).unwrap();
        let row = deserialize::<Row>(&mut buffer).unwrap();
        Ok(row.value)
    }

    pub fn write(insert_type: String, value: &[&str]) -> Result<String, String> {
        let mut file = Self::open_file_at(true, 0);

        let typed_value;
        if &insert_type[..] == "row" {
            typed_value = Row {
                value: String::from(value[0]),
            }
        } else {
            return Err(String::from("can't insert this"));
        }

        // write on file
        let bytes = serialize(&typed_value).unwrap();
        file.write_all(bytes.as_slice()).unwrap();

        let result = format!("added string {:?}", typed_value);
        Ok(result)
    }

    fn open_file_at(write_permission: bool, position: u64) -> File {
        let mut file = OpenOptions::new()
            .read(true)
            .write(write_permission)
            .open(DB_FILE_PATH)
            .unwrap_or_else(|error| {
                File::create(DB_FILE_PATH).unwrap_or_else(|error| {
                    fs::create_dir(DB_PATH).unwrap_or_else(|err| {
                        panic!("could not create data folder because: {err}")
                    });

                    File::create(DB_FILE_PATH)
                        .unwrap_or_else(|err| panic!("could not create folder"))
                })
            });

        file.seek(SeekFrom::Start(position)).unwrap();
        file
    }

    fn get_insert(value: &[&str], insert_type: &str) {}
}
