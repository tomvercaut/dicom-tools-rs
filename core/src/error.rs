use dicom_core::value::CastValueError;
use dicom_object::{AccessError, ReadError};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ReadError(#[from] ReadError),
    #[error(transparent)]
    AccessError(#[from] AccessError),
    #[error(transparent)]
    CastValueError(#[from] CastValueError),
}