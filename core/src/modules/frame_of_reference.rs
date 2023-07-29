use dicom_core::DataDictionary;
use dicom_dictionary_std::tags::{FRAME_OF_REFERENCE_UID, POSITION_REFERENCE_INDICATOR};
use dicom_object::InMemDicomObject;
use helper::element_opt_to_str;
use std::convert::TryFrom;
use Error;

/// This module specifies the Attributes necessary to uniquely identify a Frame of Reference that
/// ensures the spatial relationship of Images within a Series.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FrameOfReference {
    /// Uniquely identifies the Frame of Reference.
    /// See Section C.7.4.1.1.1 for further explanation.
    pub uid: String,
    /// Part of the imaging target used as a reference.
    /// See Section C.7.4.1.1.2 for further explanation.
    pub position_reference_indicator: String,
}

impl<D> TryFrom<&InMemDicomObject<D>> for FrameOfReference
where
    D: DataDictionary + Clone,
{
    type Error = Error;

    fn try_from(obj: &InMemDicomObject<D>) -> Result<Self, Self::Error> {
        let mut frame = Self::default();
        frame.uid = obj.element(FRAME_OF_REFERENCE_UID)?.string()?.to_string();
        frame.position_reference_indicator =
            element_opt_to_str(obj, POSITION_REFERENCE_INDICATOR, "")?.to_string();
        Ok(frame)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dicom_core::VR;
    use dicom_object::mem::InMemElement;

    #[test]
    fn frame_of_reference_try_from() {
        let mut obj = InMemDicomObject::new_empty();
        let _ = obj.put_element(InMemElement::new(FRAME_OF_REFERENCE_UID, VR::UI, "1.2.3.4"));
        let _ = obj.put_element(InMemElement::new(
            POSITION_REFERENCE_INDICATOR,
            VR::LO,
            "reference indicator",
        ));
        let res = FrameOfReference::try_from(&obj);
        assert!(res.is_ok());
        let expected = FrameOfReference {
            uid: "1.2.3.4".to_string(),
            position_reference_indicator: "reference indicator".to_string(),
        };
        let sop = res.unwrap();
        assert_eq!(expected, sop);
    }
}
