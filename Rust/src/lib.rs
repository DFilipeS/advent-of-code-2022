use std::io::Read;

#[derive(Debug)]
pub enum Error {
    FailedToReadInput,
}

pub fn read_input(reader: &mut impl Read) -> Result<String, Error> {
    let mut buffer = String::new();

    reader
        .read_to_string(&mut buffer)
        .map_err(|_| Error::FailedToReadInput)?;

    Ok(buffer)
}
