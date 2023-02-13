use crate::models::Row;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::mem;
use std::str;

const DB_PATH: &str = "data";
const DB_FILE_PATH: &str = "./data/cavea.db";
const MAX_PAGE_SIZE: usize = 4096;

pub struct Pager {
    pub num_pages: u32,
    file_length: u64,
    root_page: u8,
}

impl Pager {
    pub fn new() -> Self {
        let file_length = fs::metadata(DB_FILE_PATH)
            .unwrap_or_else(|_| {
                Self::create_dir_and_file();
                fs::metadata(DB_FILE_PATH).unwrap()
            })
            .len();

        let mut root_page = 0;
        if file_length > 1 {
            // root page is in the first byte of the file
            let buffer = Self::read_bytes(1, 0);
            root_page = buffer[0];
        }

        Pager {
            num_pages: Self::get_num_pages(file_length) as u32,
            file_length,
            root_page,
        }
    }

    fn get_num_pages(file_length: u64) -> usize {
        file_length as usize / MAX_PAGE_SIZE
    }

    pub fn read_page(&self, page_num: usize) {
        if Self::get_num_pages(self.file_length) < page_num {
            // return Err(format!(
            //     "Tried to read a page out of range. Page number: {}",
            //     page_num
            // ));
        }

        Self::read(page_num);
    }

    fn read_bytes(bytes: usize, from: usize) -> Vec<u8> {
        let mut file = Self::open_file_at(false, from as u64);
        let mut buffer = vec![0; bytes];
        file.read_exact(&mut buffer).unwrap();
        buffer
    }

    pub fn read(page_num: usize) -> Result<String, String> {
        let mut file = Self::open_file_at(false, (page_num * MAX_PAGE_SIZE) as u64);
        let mut buffer = vec![0; 4096];

        let len = file.metadata().unwrap().len();
        if len == 0 {
            return Ok(String::from("no record found"));
        } else if len < 4096 {
            // otherwise we might hit end of file
            buffer = vec![0; len as usize];
        }

        file.read_exact(&mut buffer).unwrap();
        let row_value = str::from_utf8(&buffer).unwrap();
        let row = Row {
            value: String::from(row_value),
        };
        println!("{:?}", row);
        Ok(row.value)
    }

    pub fn write(mut value: &[&str]) -> Result<String, String> {
        let mut file = Self::open_file_at(true, 0);

        // insert root_page number if I have not inserted anything
        if file.metadata().unwrap().len() == 0 {
            let root_page = [0u8];
            file.write(&root_page[..]);
            file.seek(SeekFrom::Start(1)).unwrap();
        }

        println!("size is {}", mem::size_of_val(value[0]));

        // write on file
        file.write_all(&value[0].as_bytes()).unwrap();

        let result = format!("added string {:?}", value[0]);
        Ok(result)
    }

    fn open_file_at(write_permission: bool, position: u64) -> File {
        let mut file = OpenOptions::new()
            .read(true)
            .write(write_permission)
            .open(DB_FILE_PATH)
            .unwrap_or_else(|error| Self::create_dir_and_file());

        file.seek(SeekFrom::Start(position)).unwrap();
        file
    }

    fn create_dir_and_file() -> File {
        File::create(DB_FILE_PATH).unwrap_or_else(|error| {
            fs::create_dir(DB_PATH)
                .unwrap_or_else(|err| panic!("could not create data folder because: {err}"));

            File::create(DB_FILE_PATH)
                .unwrap_or_else(|err| panic!("could not create folder because {err}"))
        })
    }
}
