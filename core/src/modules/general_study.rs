use crate::Error;
use dicom_core::DataDictionary;
use dicom_dictionary_std::tags::{
    ACCESSION_NUMBER, REFERRING_PHYSICIAN_NAME, STUDY_DATE, STUDY_ID, STUDY_INSTANCE_UID,
    STUDY_TIME,
};
use dicom_object::InMemDicomObject;
use helper::element_opt_to_str;
use std::convert::TryFrom;

///This module specifies the Attributes that describe and identify the
/// Study performed upon the Patient.
///
/// Only a limited set of Attributes are supported.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GeneralStudy {
    /// Date study was performed
    pub study_date: String,
    /// Time study was performed
    pub study_time: String,
    ///A RIS generated number that identifies the order for the Study.
    pub accession_number: String,
    /// Name of the Patient's referring physician
    pub referring_physicians_name: String,
    /// Unique identifier for the Study.
    pub study_instance_uid: String,
    /// User or equipment generated Study identifier.
    pub study_id: String,
}

impl<D> TryFrom<&InMemDicomObject<D>> for GeneralStudy
where
    D: DataDictionary + Clone,
{
    type Error = Error;

    fn try_from(obj: &InMemDicomObject<D>) -> Result<Self, Self::Error> {
        let mut study = GeneralStudy::default();
        study.study_date = element_opt_to_str(obj, STUDY_DATE, "")?.to_string();
        study.study_time = element_opt_to_str(obj, STUDY_TIME, "")?.to_string();
        study.accession_number = element_opt_to_str(obj, ACCESSION_NUMBER, "")?.to_string();
        study.referring_physicians_name =
            element_opt_to_str(obj, REFERRING_PHYSICIAN_NAME, "")?.to_string();
        study.study_instance_uid = obj.element(STUDY_INSTANCE_UID)?.string()?.to_string();
        study.study_id = element_opt_to_str(obj, STUDY_ID, "")?.to_string();
        Ok(study)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dicom_core::VR;
    use dicom_object::mem::InMemElement;

    #[test]
    fn general_study_try_from() {
        let mut obj = InMemDicomObject::new_empty();
        let _ = obj.put_element(InMemElement::new(STUDY_DATE, VR::DA, "20230729"));
        let _ = obj.put_element(InMemElement::new(STUDY_TIME, VR::TM, "21:59:59"));
        let _ = obj.put_element(InMemElement::new(ACCESSION_NUMBER, VR::SH, "12"));
        let _ = obj.put_element(InMemElement::new(
            REFERRING_PHYSICIAN_NAME,
            VR::PN,
            "Doe^John",
        ));
        let _ = obj.put_element(InMemElement::new(STUDY_INSTANCE_UID, VR::UI, "1.2.3.4"));
        let _ = obj.put_element(InMemElement::new(STUDY_ID, VR::SH, "13"));
        let res = GeneralStudy::try_from(&obj);
        assert!(res.is_ok());
        let expected = GeneralStudy {
            study_date: "20230729".to_string(),
            study_time: "21:59:59".to_string(),
            accession_number: "12".to_string(),
            referring_physicians_name: "Doe^John".to_string(),
            study_instance_uid: "1.2.3.4".to_string(),
            study_id: "13".to_string(),
        };
        let sop = res.unwrap();
        assert_eq!(expected, sop);
    }

    #[test]
    fn general_study_try_from_no_optional() {
        let mut obj = InMemDicomObject::new_empty();
        let _ = obj.put_element(InMemElement::new(STUDY_INSTANCE_UID, VR::UI, "1.2.3.4"));
        let res = GeneralStudy::try_from(&obj);
        assert!(res.is_ok());
        let expected = GeneralStudy {
            study_date: "".to_string(),
            study_time: "".to_string(),
            accession_number: "".to_string(),
            referring_physicians_name: "".to_string(),
            study_instance_uid: "1.2.3.4".to_string(),
            study_id: "".to_string(),
        };
        let sop = res.unwrap();
        assert_eq!(expected, sop);
    }
}
