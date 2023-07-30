use data::UsSs;
use dicom_core::{DataDictionary, Tag, VR};
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

pub(crate) trait DicomValueOrNone<D, R> {
    fn optional_value(&self, tag: Tag) -> Result<Option<R>, Error>;
}

impl<'a, D> DicomValueOrNone<D, &'a str> for &'a InMemDicomObject<D>
where
    D: DataDictionary + Clone,
{
    fn optional_value(&self, tag: Tag) -> Result<Option<&'a str>, Error> {
        match self.element_opt(tag)? {
            None => Ok(None),
            Some(ime) => Ok(Some(ime.string()?)),
        }
    }
}

impl<'a, D> DicomValueOrNone<D, String> for &'a InMemDicomObject<D>
where
    D: DataDictionary + Clone,
{
    fn optional_value(&self, tag: Tag) -> Result<Option<String>, Error> {
        match self.element_opt(tag)? {
            None => Ok(None),
            Some(ime) => Ok(Some(ime.string()?.to_string())),
        }
    }
}

impl<'a, D> DicomValueOrNone<D, Vec<String>> for &'a InMemDicomObject<D>
where
    D: DataDictionary + Clone,
{
    fn optional_value(&self, tag: Tag) -> Result<Option<Vec<String>>, Error> {
        match self.element_opt(tag)? {
            None => Ok(None),
            Some(ime) => Ok(Some(ime.strings()?.to_vec())),
        }
    }
}

impl<'a, D> DicomValueOrNone<D, f64> for &'a InMemDicomObject<D>
where
    D: DataDictionary + Clone,
{
    fn optional_value(&self, tag: Tag) -> Result<Option<f64>, Error> {
        match self.element_opt(tag)? {
            None => Ok(None),
            Some(ime) => Ok(Some(ime.to_float64()?)),
        }
    }
}

impl<'a, D> DicomValueOrNone<D, UsSs> for &'a InMemDicomObject<D>
where
    D: DataDictionary + Clone,
{
    fn optional_value(&self, tag: Tag) -> Result<Option<UsSs>, Error> {
        match self.element_opt(tag)? {
            None => Ok(None),
            Some(ime) => {
                let vr = ime.vr();
                match vr {
                    VR::SS => Ok(Some(UsSs {
                        us: None,
                        ss: Some(ime.int16()?),
                    })),
                    VR::US => Ok(Some(UsSs {
                        us: Some(ime.uint16()?),
                        ss: None,
                    })),
                    _ => Err(Error::ExpectedVrEqualToUsSs(vr)),
                }
            }
        }
    }
}
