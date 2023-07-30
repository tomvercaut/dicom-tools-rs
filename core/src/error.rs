use dicom_core::value::{CastValueError, ConvertValueError};
use dicom_core::VR;
use dicom_object::{AccessError, ReadError};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ReadError(#[from] ReadError),
    #[error(transparent)]
    AccessError(#[from] AccessError),
    #[error(transparent)]
    CastValueError(#[from] CastValueError),
    #[error(transparent)]
    ConvertValueError(#[from] ConvertValueError),
    #[error("Expected a value representation SS or US but detected: {0}")]
    ExpectedVrEqualToUsSs(VR),
}
