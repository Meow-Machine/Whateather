use std::fs::File;
use std::io::{Read, Write};
use CacheReadingStatus::*;
const CACHE_BUFFER_SIZE: usize = 64;

pub struct Cache {
    cache_file_path: String,
    cache_reading: CacheReading,
}

pub struct CacheReading {
    status: CacheReadingStatus,
    read_size: usize,
    buffer: [u8; CACHE_BUFFER_SIZE],
}

enum CacheReadingStatus {
    NotRead,
    CouldntRead,
    SuccessfullyRead,
}

impl Cache {
    /// Creates a Cache instance
    pub fn create(file_path: &str) -> Self {
        Self {
            cache_file_path: String::from(file_path),
            cache_reading: CacheReading::create_empty(),
        }
    }

    /// Reads data from Cache-File to Cache
    pub fn read_cache_file(&mut self) {
        let mut cache_file = self.open_cache(false, true, false);
        let possible_read_size = cache_file.read(&mut self.cache_reading.buffer);
        self.validate_reading(possible_read_size);
    }

    /// Writes new data to Cache-File and Cache
    pub fn write_data_to_cache(&mut self, data: &[u8]) {
        {
            let mut cache_file = self.open_cache(true, false,true);

            // Should only Fail on Access-right Issues
            cache_file.write_all(data).unwrap();

        } // File not needed after this point

        self.set_data_manually(data);
    }

    fn set_data_manually(&mut self, data: &[u8]) {
        self.cache_reading.read_size = data.len();
        self.cache_reading.buffer[..data.len()].copy_from_slice(data);
    }

    pub fn print_data(&self) {
        if !self.has_data() {
            eprintln!("Cache is empty");
        } else {
            for i in 0..self.cache_reading.read_size {
                let data = self.cache_reading.buffer[i];
                print!("{}", data as char);
            }
            print!("\n");
        }
    }

    fn has_data(&self) -> bool {
        match self.cache_reading.status {
            SuccessfullyRead => { self.cache_reading.read_size > 0 },

            CouldntRead|NotRead => false
        }
    }

    fn open_cache(&mut self, remove_old_data: bool, read: bool, write: bool) -> File {
        // Should only Fail on Access-Right Issues
        File::options()
            .truncate(remove_old_data)
            .read(read)
            .write(write)
            .open(self.cache_file_path.as_str())
            .unwrap()
    }

    /// Validates the reading from the Cache-File and updates the Status
    fn validate_reading(&mut self, possible_read_size: std::io::Result<usize>) {
        match possible_read_size {
            Ok(size) => {
                println!("Read {} Bytes successfully from Cache-File", size);
                self.cache_reading.read_size = size;
                self.cache_reading.status = SuccessfullyRead;
            },

            Err(e) => {
                eprintln!("Couldn't read data from Cache: {}", e);
                self.cache_reading.status = CouldntRead;
            },
        }
    }
}

impl CacheReading {
   pub fn create_empty() -> Self {
       Self {
           read_size: 0,
           buffer: [0; CACHE_BUFFER_SIZE],
           status: NotRead,
       }
   }
}