use std::{
    fs::File,
    io::{BufReader, Error},
};

pub fn open_file(filename: &str) -> Result<BufReader<File>, Error> {
    let file = File::open(filename);
    match file {
        Ok(file) => Ok(BufReader::new(file)),
        Err(e) => Err(Error::new(
            e.kind(),
            format!("Unable to open file '{}', cause: {}", filename, e),
        )),
    }
}
