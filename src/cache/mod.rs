use std::fs::File;
use std::io::{Read, Write};
const CACHE_BUFFER_SIZE: usize = 64;

pub struct Cache {
    cache_file_path: String,
    cache_reading: CacheReading,
}

pub struct CacheReading {
    read_size: Option<usize>,
    buffer: [u8; CACHE_BUFFER_SIZE],
}


impl Cache {
    /// Creates a Cache instance
    pub fn new(file_path: &str) -> Self {
        Self {
            cache_file_path: String::from(file_path),
            cache_reading: CacheReading::create_empty(),
        }
    }

    /// Reads data from Cache-File to Cache
    pub fn read_cache_file(&mut self) -> std::io::Result<()> {
        let mut cache_file = self.open_cache(false, true, false)?;
        let possible_read_size = cache_file.read(&mut self.cache_reading.buffer);
        self.validate_reading(possible_read_size);

        Ok(())
    }

    /// Writes new data to Cache-File and Cache
    pub fn write_data_to_cache(&mut self, data: &[u8]) -> std::io::Result<()> {
        {
            let mut cache_file = self.open_cache(true, false,true)?;

            // Should only Fail on Access-right Issues
            cache_file.write_all(data)?;

        } // File not needed after this point

        self.set_data_manually(data);

        Ok(())
    }

    fn set_data_manually(&mut self, data: &[u8]) {
        self.cache_reading.read_size = Some(data.len());
        self.cache_reading.buffer[..data.len()].copy_from_slice(data);
    }

    pub fn print_data(&self) {
        match self.cache_reading.read_size {
            Some(0) | None => eprintln!("Cache is empty"),
            Some(size) => {
                let output = String::from_utf8_lossy(&self.cache_reading.buffer[..size]);
                println!("{}", output);
            }
        }
    }

    fn has_data(&self) -> bool {
        self.cache_reading.read_size.unwrap_or(0) > 0
    }

    fn open_cache(&mut self, remove_old_data: bool, read: bool, write: bool) -> std::io::Result<File> {
        // Should only Fail on Access-Right Issues
        File::options()
            .truncate(remove_old_data)
            .read(read)
            .write(write)
            .open(self.cache_file_path.as_str())
    }

    /// Validates the reading from the Cache-File and updates the Status
    fn update_reading_status(&mut self, possible_read_size: std::io::Result<usize>) {
        match possible_read_size {
            Ok(size) => {
                println!("Read {} Bytes successfully from Cache-File", size);
                self.cache_reading.read_size = Some(size);
            },

            Err(e) => {
                eprintln!("Couldn't read data from Cache: {}", e);
                self.cache_reading.read_size = None;
            },
        }
    }
}

impl CacheReading {
   pub fn create_empty() -> Self {
       Self {
           read_size: None,
           buffer: [0; CACHE_BUFFER_SIZE],
       }
   }
}