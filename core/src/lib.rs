extern crate dicom_core;
extern crate dicom_dictionary_std;
extern crate dicom_object;

use dicom_object::{open_file, DefaultDicomObject};
use std::path::Path;
use std::result::Result;

mod error;
pub use error::Error;

pub(crate) mod helper;
pub mod modules;

/// Read a DICOM object from a file.
///
/// # Arguments
///
/// * `p` - path to a DICOM file
pub fn read_dicom_file<P: AsRef<Path>>(path: P) -> Result<DefaultDicomObject, Error> {
    open_file(path).map_err(|e| Error::from(e))
}
