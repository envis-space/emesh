use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Field does not match type")]
    VertexDuplicates,
}
