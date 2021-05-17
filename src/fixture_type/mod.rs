//! Holds the GDTF FixtureType and it's children
use std::convert::TryInto;

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

use crate::fixture_type::attribute_definitions::AttributeDefinitions;
use crate::utils::deparse::DeparseSingle;
use crate::utils::errors::GdtfError;
use crate::utils::errors::GdtfError::QuickXMLError;
use crate::utils::units::guid::GUID;
use crate::utils::units::name::Name;

pub mod attribute_definitions;
pub mod dmx_mode;

///The FixtureType node is the starting point of the description of the fixture type
#[derive(Debug)]
pub struct FixtureType {
    ///Name of the fixture type.
    pub name: Name,
    /// Shortened name of the fixture type.
    pub short_name: String,
    ///Detailed name of the fixture type.
    pub long_name: String,
    ///Manufacturer of the fixture type.
    pub manufacturer: String,
    /// Description of the fixture type.
    pub description: String,
    ///Unique number of the fixture type.
    pub fixture_type_id: GUID,
    /// File name without extension containing description of the thumbnail.Use the following as a resource file:
    pub thumbnail: Option<String>,
    ///GUID of the referenced fixture type
    pub ref_ft: Option<GUID>,
    ///This section defines all attributes that are used in the fixture type.
    pub attribute_definitions: AttributeDefinitions,
    //Defines the physical or virtual color wheels, gobo wheels, media server content and others.
    // pub wheels: Option<Wheels>,
    //Contains additional physical descriptions.
    // pub physical_descriptions: Option<PhysicalDescriptions>,
    //Contains models of physically separated parts of the device.
    // pub models: Option<Models>,
    //Describes physically separated parts of the device.
    //pub geometries: Geometries,
    //Contains descriptions of the DMX modes.
    //pub dmx_modes: Vec<DMXModes>,
    //Describe the history of the fixture type.
    //pub revisions: Option<Revisions>,
    //Is used to transfer user - defined and fixture type specific presets to other show files.
    //pub ft_presets: Option<FTPresets>,
    //Specifies supported protocols.
    //pub protocols: Option<Protocols>,
}


impl DeparseSingle for FixtureType {
    #[cfg(test)]
    fn is_same_item_identifier(&self, _: &Self) -> bool {
        false
    }

    fn single_from_event_unchecked(reader: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<Self, GdtfError> where
        Self: Sized {
        let mut name = Name::new();
        let mut short_name = String::new();
        let mut long_name = String::new();
        let mut manufacturer = String::new();
        let mut description = String::new();
        let mut fixture_type_id = GUID::new();
        let mut thumbnail = None;
        let mut ref_ft: Option<GUID> = None;

        for attr in e.attributes().into_iter() {
            let attr = attr?;
            match attr.key {
                b"Name" => name = Self::attr_to_name(&attr)?,
                b"ShortName" => short_name = Self::attr_to_string(&attr)?,
                b"LongName" => long_name = Self::attr_to_string(&attr)?,
                b"Manufacturer" => manufacturer = Self::attr_to_string(&attr)?,
                b"Description" => description = Self::attr_to_string(&attr)?,
                b"FixtureTypeID" => fixture_type_id = Self::attr_to_str(&attr)?.try_into()?,
                b"Thumbnail" => thumbnail = Self::attr_to_string_option(&attr)?,
                b"RefFT" => ref_ft = match Self::attr_to_str_option(&attr)? {
                    None => { None }
                    Some(v) => { Some(v.try_into()?) }
                },

                _ => {}
            }
        }

        if name.is_empty() {
            return Err(GdtfError::RequiredValueNotFoundError("Name not found in FixtureType".to_string()));
        }

        if fixture_type_id.is_empty() {
            return Err(GdtfError::RequiredValueNotFoundError("FixtureTypeId not found in FixtureType".to_string()));
        }

        let mut buf: Vec<u8> = Vec::new();
        let mut attribute_definitions: Option<AttributeDefinitions> = None;
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                    if e.name() == b"AttributeDefinitions" {
                        attribute_definitions = Some(AttributeDefinitions::single_from_event(reader, e)?);
                    }
                }
                Ok(Event::Eof) | Ok(Event::End(_)) => {
                    break;
                }
                Err(e) => return Err(QuickXMLError(e)),
                _ => {}
            }
        }
        buf.clear();

        if attribute_definitions.is_none() {
            return Err(GdtfError::RequiredValueNotFoundError("AttributeDefinitions not found in FixtureType".to_string()));
        }
        Ok(Self {
            name,
            short_name,
            long_name,
            manufacturer,
            description,
            fixture_type_id,
            thumbnail,
            ref_ft,
            attribute_definitions: attribute_definitions.unwrap(),
        })
    }

    fn is_single_event_name(event_name: &[u8]) -> bool {
        event_name == b"FixtureType"
    }

    fn single_event_name() -> String {
        "FixtureType".to_string()
    }
    #[cfg(test)]
    fn is_single_eq_no_log(&self, other: &Self) -> bool {
        self.name == other.name &&
            self.short_name == other.short_name &&
            self.long_name == other.long_name &&
            self.manufacturer == other.manufacturer &&
            self.description == other.description &&
            self.fixture_type_id == other.fixture_type_id &&
            self.thumbnail == other.thumbnail &&
            self.ref_ft == other.ref_ft &&
            AttributeDefinitions::is_single_eq_log(&self.attribute_definitions, &other.attribute_definitions)
    }
}


#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use crate::fixture_type::attribute_definitions::activation_group::ActivationGroup;
    use crate::fixture_type::attribute_definitions::attribute::Attribute;
    use crate::fixture_type::attribute_definitions::AttributeDefinitions;
    use crate::fixture_type::attribute_definitions::feature_group::feature::Feature;
    use crate::fixture_type::attribute_definitions::feature_group::FeatureGroup;
    use crate::fixture_type::FixtureType;
    use crate::utils::deparse::DeparseSingle;
    use crate::utils::units::physical_unit::PhysicalUnit;

    #[test]
    fn test_fixture_type() {
        FixtureType {
            description: "ACME AE-610 BEAM".to_string(),
            fixture_type_id: "E62F2ECF-2A08-491D-BEEC-F5C491B89784".try_into().unwrap(),
            long_name: "ACME AE 610 BEAM".to_string(),
            manufacturer: "ACME".to_string(),
            name: "ACME AE-610 BEAM".try_into().unwrap(),
            ref_ft: Some("8F54E11C-4C91-11E9-80BC-F1DFE217E634".try_into().unwrap()),
            short_name: "ACME AE 610 BEAM".to_string(),
            thumbnail: Some("AE-610 BEAM".to_string()),
            attribute_definitions: AttributeDefinitions {
                activation_groups: vec![ActivationGroup {
                    name: "PanTilt".try_into().unwrap()
                }],
                feature_groups: vec![FeatureGroup {
                    name: "Position".try_into().unwrap(),
                    pretty: "PositionP".to_string(),
                    features: vec![Feature {
                        name: "PanTilt".try_into().unwrap()
                    }],
                }],
                attributes: vec![Attribute {
                    activation_group: Some("PanTilt".to_string()),
                    feature: "Position.PanTilt".to_string(),
                    name: "Pan".try_into().unwrap(),
                    physical_unit: PhysicalUnit::Angle,
                    pretty: "P".to_string(),
                    main_attribute: None,
                    color: None,
                }],
            },
        }.test(
            r#"
        <FixtureType Description="ACME AE-610 BEAM" FixtureTypeID="E62F2ECF-2A08-491D-BEEC-F5C491B89784" LongName="ACME AE 610 BEAM" Manufacturer="ACME" Name="ACME AE-610 BEAM" RefFT="8F54E11C-4C91-11E9-80BC-F1DFE217E634" ShortName="ACME AE 610 BEAM" Thumbnail="AE-610 BEAM">
            <AttributeDefinitions>
                    <ActivationGroups>
                        <ActivationGroup Name="PanTilt"/>
                    </ActivationGroups>
                <FeatureGroups>
                    <FeatureGroup Name="Position" Pretty="PositionP">
                        <Feature Name="PanTilt"/>
                    </FeatureGroup>
                </FeatureGroups>
                <Attributes>
                    <Attribute ActivationGroup="PanTilt" Feature="Position.PanTilt" Name="Pan" PhysicalUnit="Angle" Pretty="P"/>
                </Attributes>
            </AttributeDefinitions>
    "#)
    }
}