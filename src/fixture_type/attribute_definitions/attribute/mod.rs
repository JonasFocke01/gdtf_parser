use std::convert::TryInto;

use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::utils::deparse::{DeparseSingle, DeparseVec};
use crate::utils::deparse;
#[cfg(test)]
use crate::utils::deparse::TestDeparseSingle;
#[cfg(test)]
use crate::utils::deparse::TestDeparseVec;
use crate::utils::errors::GdtfError;
use crate::utils::units::attribute_name::AttributeName;
use crate::utils::units::color_cie::ColorCIE;
use crate::utils::units::physical_unit::PhysicalUnit;

///Describes a singular mutual exclusive control function
#[derive(Debug, PartialEq)]
pub struct Attribute {
    pub name: AttributeName,
    pub pretty: String,
    pub activation_group: Option<String>,
    pub feature: String,
    pub main_attribute: Option<String>,
    pub physical_unit: PhysicalUnit,
    pub color: Option<ColorCIE>,
}

impl DeparseSingle for Attribute {
    fn single_from_event(_reader: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<Self, GdtfError> where
        Self: Sized {
        let mut name = Default::default();
        let mut pretty = String::new();
        let mut activation_group = None;
        let mut feature = String::new();
        let mut main_attribute = None;
        let mut physical_unit: PhysicalUnit = PhysicalUnit::None;
        let mut color: Option<ColorCIE> = None;

        for attr in e.attributes().into_iter() {
            let attr = attr?;
            match attr.key {
                b"Name" => name = attr.into(),
                b"Pretty" => pretty = deparse::attr_to_string(&attr),
                b"ActivationGroup" => activation_group = deparse::attr_to_string_option(&attr),
                b"Feature" => feature = deparse::attr_to_string(&attr),
                b"MainAttribute" => main_attribute = deparse::attr_to_string_option(&attr),
                b"PhysicalUnit" => physical_unit = deparse::attr_to_str(&attr).into(),
                b"Color" => color = attr.try_into().ok(),
                _ => {}
            }
        }

        Ok(Attribute {
            feature,
            pretty,
            name,
            color,
            activation_group,
            main_attribute,
            physical_unit,
        })
    }

    fn is_single_event_name(event_name: &[u8]) -> bool {
        event_name == b"Attribute"
    }

    fn single_event_name() -> String {
        "Attribute".to_string()
    }
}

impl DeparseVec for Attribute {
    fn is_group_event_name(event_name: &[u8]) -> bool {
        event_name == b"Attributes"
    }
}

#[cfg(test)]
impl TestDeparseSingle for Attribute {}

#[cfg(test)]
impl TestDeparseVec for Attribute {}

#[cfg(test)]
mod tests {
    use crate::fixture_type::attribute_definitions::attribute::Attribute;
    use crate::utils::deparse::TestDeparseSingle;
    use crate::utils::units::attribute_name::AttributeName;
    use crate::utils::units::color_cie::ColorCIE;
    use crate::utils::units::name::Name;
    use crate::utils::units::physical_unit::PhysicalUnit;

    #[test]
    fn test_attribute_all() {
        Attribute {
            name: AttributeName::UserDefined(Name::new_unchecked("Sound")),
            pretty: "SoundP".to_string(),
            activation_group: Some("Gobo1".to_string()),
            feature: "Control.Control".to_string(),
            main_attribute: Some("Gobo1M".to_string()),
            physical_unit: PhysicalUnit::Angle,
            color: Some(ColorCIE {
                x: 0.312700,
                y: 0.329000,
                Y: 100.000000,
            }),
        }.test(
            r#"<Attribute Color="0.312700,0.329000,100.000000" Feature="Control.Control" Name="Sound" PhysicalUnit="Angle" Pretty="SoundP" ActivationGroup="Gobo1"  MainAttribute="Gobo1M" />"#
        )
    }

    #[test]
    fn test_attribute_all_2() {
        Attribute {
            name: AttributeName::Effects_n_Adjust_m_(2, 4),
            pretty: "SoundP".to_string(),
            activation_group: Some("Gobo1".to_string()),
            feature: "Control.Control".to_string(),
            main_attribute: Some("Gobo1M".to_string()),
            physical_unit: PhysicalUnit::Angle,
            color: Some(ColorCIE {
                x: 0.312700,
                y: 0.329000,
                Y: 100.000000,
            }),
        }.test(
            r#"<Attribute Color="0.312700,0.329000,100.000000" Feature="Control.Control" Name="Effects2Adjust4" PhysicalUnit="Angle" Pretty="SoundP" ActivationGroup="Gobo1"  MainAttribute="Gobo1M" />"#
        )
    }

    #[test]
    fn test_attribute_min() {
        Attribute {
            name: AttributeName::UserDefined(Name::new_unchecked("Sound")),
            pretty: "SoundP".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::Angle,
            color: None,
        }.test(
            r#"<Attribute Feature="Control.Control" Name="Sound" PhysicalUnit="Angle" Pretty="SoundP"/>"#
        )
    }

    #[test]
    fn test_attribute_min_2() {
        Attribute {
            name: AttributeName::UserDefined(Name::new_unchecked("")),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        }.test(
            r#"<Attribute Feature="" Name="" MainAttribute="" ActivationGroup="" PhysicalUnit="" Pretty=""/>"#
        )
    }

    #[test]
    fn test_attribute_empty() {
        Attribute {
            name: AttributeName::UserDefined(Name::new_unchecked("")),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        }.test(
            r#"<Attribute/>"#
        )
    }

    #[test]
    fn test_faulty() {
        match Attribute::single_from_xml(r#"<ttribute"#) {
            Ok(_) => { panic!("test_faulty should return an error"); }
            Err(_) => {}
        }
    }
}