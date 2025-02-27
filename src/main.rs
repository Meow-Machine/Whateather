mod cache;
mod weather;

const CACHE_FILE_PATH: &str = "./cache.txt";


fn main () {
    // get cache
    let mut cache = cache::Cache::open(CACHE_FILE_PATH);
    // Read cache
    cache.update_data();
    // Check Reading
    // println!("{}", cache.reading_was_success());
    // if cache.reading_was_success() {
    //
    // }
    // retrieve Data or generate Data

}

