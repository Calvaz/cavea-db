use crate::btree::BtreeNode;
use crate::models::Row;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::mem;
use std::str;

const DB_PATH: &str = "data";
const DB_FILE_PATH: &str = "./data/cavea.db";
pub const MAX_PAGE_SIZE: usize = 4096;
const ROOT_NODE_SIZE: usize = 1;
const PARENT_KEY_SIZE: usize = 4;
const NODE_KEY_SIZE: usize = 2;
const NODE_VALUE_SIZE: usize = 20;

pub struct Pager {
    pub num_pages: u32,
    pub file_length: u64,
}

impl Pager {
    pub fn new() -> Self {
        let mut file_length = fs::metadata(DB_FILE_PATH)
            .unwrap_or_else(|_| {
                Self::create_dir_and_file();
                fs::metadata(DB_FILE_PATH).unwrap()
            })
            .len();

        // let mut root_node = 0;
        // if file_length > 1 {
        //     // root page is in the first byte of the file
        //     let buffer = Self::read_bytes(1, 0);
        //     root_node = buffer[0];
        // }

        // file has just been created
        if file_length == 0 {
            Self::init_page();
            file_length = MAX_PAGE_SIZE as u64;
        }

        Pager {
            num_pages: Self::get_num_pages(file_length) as u32,
            file_length,
        }
    }

    fn get_num_pages(file_length: u64) -> usize {
        file_length as usize / MAX_PAGE_SIZE
    }

    fn get_header_size() -> usize {
        ROOT_NODE_SIZE + PARENT_KEY_SIZE
    }

    fn get_node_size() -> usize {
        NODE_KEY_SIZE + NODE_VALUE_SIZE
    }

    fn read_bytes(bytes: usize, from: usize) -> Vec<u8> {
        let mut file = Self::open_file_at(false, from as u64);
        let mut buffer = vec![0; bytes];
        file.read_exact(&mut buffer).unwrap();
        buffer
    }

    pub fn read_btree(&self, root_page: u8) -> Result<Vec<BtreeNode>, String> {
        let root_page = self.read_page(root_page)?;
        let nodes = BtreeNode::get(root_page);
        Ok(nodes)
    }

    pub fn read_page(&self, page_num: u8) -> Result<[u8; MAX_PAGE_SIZE], String> {
        let mut buffer = [0u8; 4096];
        let page_num_offset = (page_num as usize * MAX_PAGE_SIZE) as u64;
        let last_page_length = self.file_length % MAX_PAGE_SIZE as u64;

        if self.file_length == 0 {
            return Ok(buffer);
        } else if self.file_length < page_num_offset {
            return Err(String::from("argument out of range exception"));
        }

        let mut file = Self::open_file_at(false, page_num_offset);
        file.read_exact(&mut buffer).unwrap();
        Ok(buffer)
    }

    pub fn write(&self, offset: u64, mut value: &[&str]) -> Result<String, String> {
        let mut file = Self::open_file_at(true, offset);

        if file.metadata().unwrap().len() == 0 {
            return Err(String::from("can't do this"));
        }

        // write on file
        file.write_all(&value[0].as_bytes()).unwrap();

        let result = format!("added string {:?}", value[0]);
        Ok(result)
    }

    pub fn append_btree(&self, root_page: u8, mut value: &[&str]) -> Result<String, String> {
        let root_page = self.read_page(root_page).unwrap();
        let nodes_count = BtreeNode::get(root_page).len();
        let offset = Self::get_header_size() + (Self::get_node_size() * nodes_count);

        // write on file
        let mut file = Self::open_file_at(true, offset as u64);
        let buffer = [0u8; NODE_KEY_SIZE + NODE_VALUE_SIZE];
        let mut new_key = ((nodes_count + 1) as u8).to_be_bytes().to_vec();
        new_key.extend_from_slice(value[0].as_bytes());
        file.write_all(&new_key[..]).unwrap();

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

    fn init_page() -> File {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(DB_FILE_PATH)
            .unwrap();
        file.write_all(&[0u8; MAX_PAGE_SIZE]).unwrap();
        file
    }
}

pub struct Cursor {
    pub page_num: u32,
    pub cell_num: u32,
    pub is_end: bool,
}
