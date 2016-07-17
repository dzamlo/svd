use std::io;

#[derive(Debug)]
pub enum CodegenError {
    /// An error occured while writing to the output file
    IoError(io::Error),
    /// An unsupported feature was encountered in the svd
    UnsupportedFeature,
}

impl From<io::Error> for CodegenError {
    fn from(e: io::Error) -> CodegenError {
        CodegenError::IoError(e)
    }
}
