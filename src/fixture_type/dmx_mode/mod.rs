//!This section is describes all DMX modes of the device
use std::fmt::Debug;

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

use crate::fixture_type::dmx_mode::dmx_channel::DmxChannel;
use crate::utils::deparse::{DeparseHashMap, DeparseSingle, DeparseVec};
#[cfg(test)]
use crate::utils::deparse::TestDeparseSingle;
use crate::utils::errors::GdtfError;
use crate::utils::units::name::Name;

pub mod dmx_channel;

///Each DMX mode describes logical control a part of the device in a specific mode
#[derive(Debug, PartialEq, Clone)]
pub struct DmxMode {
    ///Name of the first geometry in the device; Only top level geometries are allowed to be linked.
    pub geometry: Name,
    ///Description of all DMX channels used in the mode
    pub dmx_channels: Vec<DmxChannel>,

    //TODO relations

    //TODO ftmacros
}

impl DeparseSingle for DmxMode {
    type PrimaryKey = Name;

    fn single_from_event(reader: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<(Self, Option<Self::PrimaryKey>), GdtfError> where
        Self: Sized {
        let mut name: Name = Default::default();
        let mut geometry: Name = Default::default();
        let mut dmx_channels: Vec<DmxChannel> = Vec::new();

        for attr in e.attributes().into_iter() {
            let attr = attr?;
            match attr.key {
                b"Name" => name = attr.into(),
                b"Geometry" => geometry = attr.into(),
                _ => {}
            }
        }


        let mut buf: Vec<u8> = Vec::new();
        let mut tree_down = 0;
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                    match e.name() {
                        b"DMXChannels" => dmx_channels = DmxChannel::vec_from_event(reader, e)?,
                        _ => { tree_down += 1; }
                    }
                }
                Ok(Event::End(_)) => {
                    tree_down -= 1;
                    if tree_down <= 0 { break; }
                }
                Ok(Event::Eof) => {
                    break;
                }
                _ => {}
            }
        }
        buf.clear();

        Ok((Self {
            geometry,
            dmx_channels,
        }, Some(name)))
    }

    fn is_single_event_name(event_name: &[u8]) -> bool {
        event_name == b"DMXMode"
    }

    fn single_event_name() -> String {
        "DMXMode".to_string()
    }
}

impl DeparseHashMap for DmxMode {
    fn is_group_event_name(event_name: &[u8]) -> bool {
        event_name == b"DMXModes"
    }
}

#[cfg(test)]
impl TestDeparseSingle for DmxMode {}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use crate::fixture_type::dmx_mode::dmx_channel::DmxChannel;
    use crate::fixture_type::dmx_mode::DmxMode;
    use crate::utils::deparse::TestDeparseSingle;
    use crate::utils::errors::GdtfError;
    use crate::utils::units::dmx_break::DmxBreak;
    use crate::utils::units::highlight::Highlight;
    use crate::utils::units::name::Name;
    use crate::utils::units::offset::Offset;

    #[test]
    fn test_normal() -> Result<(), GdtfError> {
        DmxMode {
            geometry: Name::new("Base")?,
            dmx_channels: vec![
                DmxChannel {
                    dmx_break: DmxBreak::Overwrite,
                    offset: Some(Offset(vec![1, 2])),
                    initial_function: Default::default(),
                    highlight: Highlight::None,
                    geometry: "Yoke".try_into().unwrap(),
                    logical_channels: vec![],
                }, DmxChannel {
                    dmx_break: DmxBreak::Value(1),
                    offset: Some(Offset(vec![3, 4])),
                    initial_function: Default::default(),
                    highlight: Highlight::None,
                    geometry: "Head".try_into().unwrap(),
                    logical_channels: vec![],
                }
            ],
        }.test(Some(Name::new("Mode 1 12 DMX")?),
               r#"
      <DMXMode Geometry="Base" Name="Mode 1 12 DMX">
        <DMXChannels>
          <DMXChannel DMXBreak="Overwrite" Default="32768/2" Geometry="Yoke" Highlight="None" Offset="1,2">
          </DMXChannel>
          <DMXChannel DMXBreak="1" Default="32767/2" Geometry="Head" Highlight="None" Offset="3,4">
          </DMXChannel>
        </DMXChannels>
       </DMXMode>
            "#,
        );
        Ok(())
    }
}