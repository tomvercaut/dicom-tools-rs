use crate::Error;
use dicom_core::DataDictionary;
use dicom_dictionary_std::tags;
use dicom_object::InMemDicomObject;
use std::convert::TryFrom;

/// This module defines the Attributes that are required for proper
/// functioning and identification of the associated SOP Instances.
/// They do not specify any semantics about the Real-World Object represented by the IOD.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SOPCommon {
    /// Uniquely identifies the SOP Class. See Section C.12.1.1.1 for further explanation. See also PS3.4.
    pub class_uid: String,
    /// Uniquely identifies the SOP Instance. See Section C.12.1.1.1 for further explanation. See also PS3.4.
    pub instance_uid: String,
}

impl<D> TryFrom<&InMemDicomObject<D>> for SOPCommon
where
    D: DataDictionary + Clone,
{
    type Error = Error;

    fn try_from(obj: &InMemDicomObject<D>) -> Result<Self, Self::Error> {
        let class_uid = obj.element(tags::SOP_CLASS_UID)?.string()?.to_string();
        let instance_uid = obj.element(tags::SOP_INSTANCE_UID)?.string()?.to_string();
        Ok(SOPCommon {
            class_uid,
            instance_uid,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dicom_core::VR;
    use dicom_dictionary_std::tags::{SOP_CLASS_UID, SOP_INSTANCE_UID};
    use dicom_dictionary_std::uids::CT_IMAGE_STORAGE;
    use dicom_object::mem::InMemElement;

    #[test]
    fn sop_common_try_from() {
        let mut obj = InMemDicomObject::new_empty();
        let _ = obj.put_element(InMemElement::new(SOP_CLASS_UID, VR::UI, CT_IMAGE_STORAGE));
        let _ = obj.put_element(InMemElement::new(SOP_INSTANCE_UID, VR::UI, "1.2.3.4"));
        let res = SOPCommon::try_from(&obj);
        assert!(res.is_ok());
        let expected = SOPCommon {
            class_uid: CT_IMAGE_STORAGE.to_string(),
            instance_uid: "1.2.3.4".to_string(),
        };
        let sop = res.unwrap();
        assert_eq!(expected, sop);
    }
}
