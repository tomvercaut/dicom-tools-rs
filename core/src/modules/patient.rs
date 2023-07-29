use dicom_core::DataDictionary;
use dicom_dictionary_std::tags::{
    PATIENT_BIRTH_DATE, PATIENT_BIRTH_TIME, PATIENT_ID, PATIENT_NAME, PATIENT_SEX,
};
use dicom_object::InMemDicomObject;
use helper::element_opt_to_str;
use std::convert::TryFrom;
use Error;

/// This module specifies the Attributes of the Patient that describe and identify the
/// Patient who is the subject of a Study.
///
/// Only a limited set of Attributes are supported.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Patient {
    /// Patient's full name
    pub name: String,
    /// Primary identifier of the patient.
    pub id: String,
    /// Birth date of the patient
    pub birth_date: String,
    /// Birth time of the patient
    pub birth_time: String,
    /// Sec of the patient
    pub sex: String,
}

impl<D> TryFrom<&InMemDicomObject<D>> for Patient
where
    D: DataDictionary + Clone,
{
    type Error = Error;

    fn try_from(obj: &InMemDicomObject<D>) -> Result<Self, Self::Error> {
        Ok(Patient {
            name: element_opt_to_str(obj, PATIENT_NAME, "")?.to_string(),
            id: element_opt_to_str(obj, PATIENT_ID, "")?.to_string(),
            birth_date: element_opt_to_str(obj, PATIENT_BIRTH_DATE, "")?.to_string(),
            birth_time: element_opt_to_str(obj, PATIENT_BIRTH_TIME, "")?.to_string(),
            sex: element_opt_to_str(obj, PATIENT_SEX, "")?.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dicom_core::VR;
    use dicom_object::mem::InMemElement;

    #[test]
    fn patient_try_from() {
        let mut obj = InMemDicomObject::new_empty();
        let _ = obj.put_element(InMemElement::new(PATIENT_ID, VR::LO, "X_ID"));
        let _ = obj.put_element(InMemElement::new(PATIENT_NAME, VR::PN, "Doe^John"));
        let _ = obj.put_element(InMemElement::new(PATIENT_BIRTH_DATE, VR::DA, "20230729"));
        let _ = obj.put_element(InMemElement::new(PATIENT_BIRTH_TIME, VR::TM, "23:59:59"));
        let _ = obj.put_element(InMemElement::new(PATIENT_SEX, VR::TM, "F"));
        let res = Patient::try_from(&obj);
        assert!(res.is_ok());
        let expected = Patient {
            name: "Doe^John".to_string(),
            id: "X_ID".to_string(),
            birth_date: "20230729".to_string(),
            birth_time: "23:59:59".to_string(),
            sex: "F".to_string(),
        };
        let sop = res.unwrap();
        assert_eq!(expected, sop);
    }
}
