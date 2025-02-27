use std::fs::File;
use std::io;
use std::io::Read;
use crate::cache::CacheReadingState::*;

const CACHE_READING_BUFFER_SIZE: usize = 64;

pub struct Cache {
    cache_file: File,
    cache_reading: CacheReading,
}
pub struct CacheReading{
    status: CacheReadingState,
    read_size: usize,
    buffer : [u8; CACHE_READING_BUFFER_SIZE],
}

enum CacheReadingState {
    NotRead,
    CouldntRead,
    SuccessfullyRead,
}

impl Cache {

    /// Creates a Cache instance by opening a local Cache file.
    /// If the File at the given Path does not exist it will be created.
    pub fn open(file_path: &str) -> Self {
        // If this fails, there is an access-right error
        let cache_file = File::options()
            .read(true) // Needed to read the cache
            .write(true)// Needed to write to cache
            .create(true)// Open or create cache
            .open(file_path) // execute
            .unwrap();

        Self {
            cache_file,
            cache_reading: CacheReading::create_empty(),
        }
    }

    /// Reads the data from the cache File
    pub fn update_data(&mut self ) {
        let possible_read_size = self.cache_file.read(&mut self.cache_reading.buffer);
        self.validate_reading(possible_read_size);
    }

    /// Validates the reading from the File and updates the Status
    fn validate_reading(&mut self, possible_read_size: io::Result<usize>) {
        match possible_read_size {
            Ok(size) => {
                println!("Success, {}", size);
                self.cache_reading.read_size = size;
                self.cache_reading.status = SuccessfullyRead;
            },

            Err(e) => {
                eprintln!("Couldn't read: {}", e);
                self.cache_reading.status = CouldntRead;
            }
        }
    }

    /// Returns whether the Reading was a Success and the cache was not empty or not
    fn reading_was_success(&self) -> bool {
        match self.cache_reading.status {

            SuccessfullyRead => { self.cache_reading.read_size > 0 },

            CouldntRead|NotRead => false
        }
    }

}

impl CacheReading {
    pub fn create_empty() -> Self {
        Self {read_size: 0, buffer: [0; CACHE_READING_BUFFER_SIZE], status: NotRead }
    }
}