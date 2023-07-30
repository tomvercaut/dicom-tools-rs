use data::UsSs;
use dicom_core::DataDictionary;
use dicom_dictionary_std::tags::{
    DEVICE_SERIAL_NUMBER, DEVICE_UID, GANTRY_ID, INSTITUTIONAL_DEPARTMENT_NAME,
    INSTITUTION_ADDRESS, INSTITUTION_NAME, MANUFACTURER, MANUFACTURER_DEVICE_CLASS_UID,
    MANUFACTURER_MODEL_NAME, PIXEL_PADDING_VALUE, SOFTWARE_VERSIONS, SPATIAL_RESOLUTION,
};
use dicom_object::InMemDicomObject;
use helper::{element_opt_to_str, DicomValueOrNone};
use std::convert::TryFrom;
use Error;

/// This module specifies the Attributes that identify and describe the piece of equipment that
/// produced a Series of Composite Instances.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GeneralEquipment {
    /// Manufacturer of the equipment that produced the data.
    pub manufacturer: String,
    /// Name of the institution that produced the data.
    pub institution_name: Option<String>,
    /// Address of the institution where the equipment that produced data is located.
    pub institution_address: Option<String>,
    /// Name of the department where the equipment that produced the data is located.
    pub institutional_department_name: Option<String>,
    /// Manufacturer's model name of the equipment
    pub manufacturer_model_name: Option<String>,
    /// Manufacturer's serial number of the equipment
    pub device_serial_number: Option<String>,
    /// Unique identifier of the equipment that produced the data.
    pub device_uid: Option<String>,
    /// Identifies the gantry
    pub gantry_id: Option<String>,
    /// Manufacturer's Unique Identifier (UID) for the class of the device
    pub manufacturer_device_class_uid: Option<String>,
    ///  Manufacturer's designation of software version of the equipment that produced the
    /// Composite Instances. See Section C.7.5.1.1.3.
    pub software_versions: Option<Vec<String>>,
    /// The limiting resolution in mm of the acquired data.
    /// If the value varies across the images in the series, this value is at the image center.
    pub spatial_resolution: Option<f64>,
    /// Single pixel value or one limit (inclusive) of a range of pixel values used in an image to
    /// pad to rectangular format or to signal background that may be suppressed.
    /// See Section C.7.5.1.1.2 for further explanation.
    pub pixel_padding_value: Option<UsSs>,
}

impl<D> TryFrom<&InMemDicomObject<D>> for GeneralEquipment
where
    D: DataDictionary + Clone,
{
    type Error = Error;

    fn try_from(obj: &InMemDicomObject<D>) -> Result<Self, Self::Error> {
        let manufacturer = element_opt_to_str(obj, MANUFACTURER, "")?.to_string();
        let institution_name = obj.optional_value(INSTITUTION_NAME)?;
        let institution_address = obj.optional_value(INSTITUTION_ADDRESS)?;
        let institutional_department_name = obj.optional_value(INSTITUTIONAL_DEPARTMENT_NAME)?;
        let manufacturer_model_name = obj.optional_value(MANUFACTURER_MODEL_NAME)?;
        let device_serial_number = obj.optional_value(DEVICE_SERIAL_NUMBER)?;
        let device_uid = obj.optional_value(DEVICE_UID)?;
        let gantry_id = obj.optional_value(GANTRY_ID)?;
        let manufacturer_device_class_uid = obj.optional_value(MANUFACTURER_DEVICE_CLASS_UID)?;
        let software_versions = obj.optional_value(SOFTWARE_VERSIONS)?;
        let spatial_resolution = obj.optional_value(SPATIAL_RESOLUTION)?;
        let pixel_padding_value = obj.optional_value(PIXEL_PADDING_VALUE)?;
        Ok(Self {
            manufacturer,
            institution_name,
            institution_address,
            institutional_department_name,
            manufacturer_model_name,
            device_serial_number,
            device_uid,
            gantry_id,
            manufacturer_device_class_uid,
            software_versions,
            spatial_resolution,
            pixel_padding_value,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dicom_core::{PrimitiveValue, VR};
    use dicom_object::mem::InMemElement;

    #[test]
    fn general_equipment_try_from() {
        let mut obj = InMemDicomObject::new_empty();
        let _ = obj.put_element(InMemElement::new(MANUFACTURER, VR::LO, "manufacturer"));
        let _ = obj.put_element(InMemElement::new(
            INSTITUTION_NAME,
            VR::LO,
            "institution name",
        ));
        let _ = obj.put_element(InMemElement::new(
            INSTITUTION_ADDRESS,
            VR::SH,
            "institution address",
        ));
        let _ = obj.put_element(InMemElement::new(
            INSTITUTIONAL_DEPARTMENT_NAME,
            VR::LO,
            "institutional department name",
        ));
        let _ = obj.put_element(InMemElement::new(
            MANUFACTURER_MODEL_NAME,
            VR::LO,
            "manufacturer model name",
        ));
        let _ = obj.put_element(InMemElement::new(
            DEVICE_SERIAL_NUMBER,
            VR::LO,
            "device serial number",
        ));
        let _ = obj.put_element(InMemElement::new(DEVICE_UID, VR::UI, "device UID"));
        let _ = obj.put_element(InMemElement::new(GANTRY_ID, VR::LO, "gantry id"));
        let _ = obj.put_element(InMemElement::new(
            MANUFACTURER_DEVICE_CLASS_UID,
            VR::UI,
            "manufacturer device class UID",
        ));
        let _ = obj.put_element(InMemElement::new(SOFTWARE_VERSIONS, VR::LO, "1.2.3.4"));
        let _ = obj.put_element(InMemElement::new(SPATIAL_RESOLUTION, VR::DS, "12.34"));
        let _ = obj.put_element(InMemElement::new(
            PIXEL_PADDING_VALUE,
            VR::SS,
            PrimitiveValue::from(-1234i16),
        ));
        let res = GeneralEquipment::try_from(&obj);
        assert!(res.is_ok());
        let expected = GeneralEquipment {
            manufacturer: "manufacturer".to_string(),
            institution_name: Some("institution name".to_string()),
            institution_address: Some("institution address".to_string()),
            institutional_department_name: Some("institutional department name".to_string()),
            manufacturer_model_name: Some("manufacturer model name".to_string()),
            device_serial_number: Some("device serial number".to_string()),
            device_uid: Some("device UID".to_string()),
            gantry_id: Some("gantry id".to_string()),
            manufacturer_device_class_uid: Some("manufacturer device class UID".to_string()),
            software_versions: Some(vec!["1.2.3.4".to_string()]),
            spatial_resolution: Some(12.34),
            pixel_padding_value: Some(UsSs {
                us: None,
                ss: Some(-1234),
            }),
        };
        let sop = res.unwrap();
        assert_eq!(expected, sop);
    }
}
