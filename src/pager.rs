use std::fs;

const DB_PATH: &str = "data";
const DB_FILE_PATH: &str = "./data/cavea.db";
const MAX_PAGE_SIZE: usize = 4096;

struct Pager {
    pub num_pages: u32,
    file_length: u64,
}

impl Pager {
    pub fn new() -> Pager {
        let file_length = fs::metadata(DB_FILE_PATH)
            .unwrap_or_else(|_| create_dir_and_file())
            .len();
    }

    pub fn read_page(page_num: usize) {
        if self.file_length / num_pages < page_num {
            return Err(format!(
                "Tried to read a page out of range. Page number: {}",
                page_num
            ));
        }

        read(page_num);
    }

    pub fn read(page_num: usize) -> Result<String, String> {
        let mut file = Self::open_file_at(false, page_num * MAX_PAGE_SIZE);
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

    pub fn write(insert_type: String, value: &[&str]) -> Result<String, String> {
        let mut file = Self::open_file_at(true, 0);

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
            .unwrap_or_else(|error| self.create_dir_and_file());

        file.seek(SeekFrom::Start(position)).unwrap();
        file
    }

    fn create_dir_and_file() {
        File::create(DB_FILE_PATH).unwrap_or_else(|error| {
            fs::create_dir(DB_PATH)
                .unwrap_or_else(|err| panic!("could not create data folder because: {err}"));

            File::create(DB_FILE_PATH)
                .unwrap_or_else(|err| panic!("could not create folder because {err}"))
        })
    }
}
