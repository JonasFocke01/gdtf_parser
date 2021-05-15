#![cfg(test)]

use std::convert::TryInto;

use crate::gdtf::fixture_type::attribute_definitions::attribute::Attribute;
use crate::units::physical_unit::PhysicalUnit;

pub fn expect() -> Vec<Attribute> {
    vec![
        Attribute {
            name: "Pan".try_into().unwrap(),
            pretty: "P".to_string(),
            activation_group: Some("PanTilt".to_string()),
            feature: "Position.PanTilt".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::Angle,
            color: None,
        },
        Attribute {
            name: "Tilt".try_into().unwrap(),
            pretty: "T".to_string(),
            activation_group: Some("PanTilt".to_string()),
            feature: "Position.PanTilt".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::Angle,
            color: None,
        },
        Attribute {
            name: "Gobo1".try_into().unwrap(),
            pretty: "G1".to_string(),
            activation_group: Some("Gobo1".to_string()),
            feature: "Gobo.Gobo".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: "Prism1".try_into().unwrap(),
            pretty: "Prism1".to_string(),
            activation_group: Some("Prism".to_string()),
            feature: "Beam.Beam".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: "Prism1Pos".try_into().unwrap(),
            pretty: "Prism1 Pos".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::Angle,
            color: None,
        },
        Attribute {
            name: "Shutter1".try_into().unwrap(),
            pretty: "Sh1".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: "Dimmer".try_into().unwrap(),
            pretty: "Dim".to_string(),
            activation_group: None,
            feature: "Dimmer.Dimmer".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::LuminousIntensity,
            color: None,
        },
        Attribute {
            name: "LampControl".try_into().unwrap(),
            pretty: "Lamp Ctrl".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: "Function".try_into().unwrap(),
            pretty: "Function".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: "Gobo1SelectShake".try_into().unwrap(),
            pretty: "Select Shake".to_string(),
            activation_group: Some("Gobo1".to_string()),
            feature: "Gobo.Gobo".to_string(),
            main_attribute: Some("Gobo1".to_string()),
            physical_unit: PhysicalUnit::Frequency,
            color: None,
        },
        Attribute {
            name: "Prism1PosRotate".try_into().unwrap(),
            pretty: "Rotate1".to_string(),
            activation_group: Some("Prism".to_string()),
            feature: "Beam.Beam".to_string(),
            main_attribute: Some("Prism1Pos".to_string()),
            physical_unit: PhysicalUnit::AngularSpeed,
            color: None,
        },
        Attribute {
            name: "Shutter1Strobe".try_into().unwrap(),
            pretty: "Strobe1".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: Some("Shutter1".to_string()),
            physical_unit: PhysicalUnit::Frequency,
            color: None,
        },
        Attribute {
            name: "Shutter1StrobeRandom".try_into().unwrap(),
            pretty: "Random1".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: Some("Shutter1".to_string()),
            physical_unit: PhysicalUnit::Frequency,
            color: None,
        },
        Attribute {
            name: "Color1".try_into().unwrap(),
            pretty: "C1".to_string(),
            activation_group: Some("ColorRGB".to_string()),
            feature: "Color.Color".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: "Color1WheelSpin".try_into().unwrap(),
            pretty: "Wheel Spin".to_string(),
            activation_group: Some("ColorRGB".to_string()),
            feature: "Color.Color".to_string(),
            main_attribute: Some("Color1".to_string()),
            physical_unit: PhysicalUnit::AngularSpeed,
            color: None,
        },
        Attribute {
            name: "PositionReset".try_into().unwrap(),
            pretty: "Pos Reset".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: "NoFeature".try_into().unwrap(),
            pretty: "NoFeature".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: "Gobo1WheelSpin".try_into().unwrap(),
            pretty: "Wheel Spin".to_string(),
            activation_group: Some("Gobo1".to_string()),
            feature: "Gobo.Gobo".to_string(),
            main_attribute: Some("Gobo1".to_string()),
            physical_unit: PhysicalUnit::AngularSpeed,
            color: None,
        },
        Attribute {
            name: "PT Speed".try_into().unwrap(),
            pretty: "PT Speed".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: Some("0.312700,0.329000,100.000000".try_into().unwrap()),
        },
        Attribute {
            name: "ShutterReset".try_into().unwrap(),
            pretty: "Shutter Reset".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: "Lamp On".try_into().unwrap(),
            pretty: "Lamp On".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: Some("0.312700,0.329000,100.000000".try_into().unwrap()),
        },
        Attribute {
            name: "Lamp Off".try_into().unwrap(),
            pretty: "Lamp Off".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: Some("0.312700,0.329000,100.000000".try_into().unwrap()),
        },
        Attribute {
            name: "Shutter1StrobePulseClose".try_into().unwrap(),
            pretty: "Pulse Close1".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: Some("Shutter1".to_string()),
            physical_unit: PhysicalUnit::Frequency,
            color: None,
        },
        Attribute {
            name: "Shutter1StrobePulseOpen".try_into().unwrap(),
            pretty: "Pulse Open1".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: Some("Shutter1".to_string()),
            physical_unit: PhysicalUnit::Frequency,
            color: None,
        },
        Attribute {
            name: "ColorMixReset".try_into().unwrap(),
            pretty: "Color Mix Reset".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: "GoboWheelReset".try_into().unwrap(),
            pretty: "G Reset".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: "FixtureGlobalReset".try_into().unwrap(),
            pretty: "Fixture Global Reset".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: "Sound".try_into().unwrap(),
            pretty: "Sound".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: Some("0.312700,0.329000,100.000000".try_into().unwrap()),
        }
    ]
}