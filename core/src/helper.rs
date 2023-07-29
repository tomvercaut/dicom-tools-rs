use dicom_core::{DataDictionary, Tag};
use dicom_object::InMemDicomObject;
use Error;

/// Read the string value of an optional DICOM tag in a DICOM object.
///
/// # Arguments
///
/// * `obj` - DICOM object
/// * `tag` - DICOM tag
/// * `default_value` - default string value returned if the DICOM tag is not present in the DICOM object
pub(crate) fn element_opt_to_str<'a, D>(
    obj: &'a InMemDicomObject<D>,
    tag: Tag,
    default_value: &'a str,
) -> Result<&'a str, Error>
where
    D: DataDictionary + Clone,
{
    match obj.element_opt(tag)? {
        None => Ok(default_value),
        Some(elem) => Ok(elem.string()?),
    }
}
