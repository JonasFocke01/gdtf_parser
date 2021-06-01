//!Module for Node used in ChannelFunction.attribute
use std::borrow::Borrow;
use std::convert::TryFrom;

use quick_xml::events::attributes::Attribute;

use crate::utils::units::name::Name;
use crate::utils::units::node::{GDTFNodeError, Node};

#[derive(Debug, PartialEq)]
///Node used in ChannelFunction.attribute. Link to attribute; Starting point is the attributes node. Default value: “NoFeature”.
pub enum NodeChannelFunctionAttribute {
    ///Used when a reference to a node is present
    Node(Vec<Name>),
    ///Used for special value NoFeature
    NoFeature,
}


impl NodeChannelFunctionAttribute {
    ///New Node from str defined in GDTF-XML with checking if chars are valid for GDTF-Names
    pub fn new_from_str(value: &str) -> Result<Self, GDTFNodeError> {
        if value == "" || value == "NoFeature" {
            return Ok(Self::NoFeature);
        }
        Ok(Self::Node(Self::str_to_names_vec(value)?))
    }
    ///New Node from str defined in GDTF-XML without checking if chars are valid for GDTF-Names
    pub fn new_from_str_unchecked(value: &str) -> Self {
        if value == "" || value == "NoFeature" {
            return Self::NoFeature;
        }
        Self::Node(Name::str_to_names_vec_unchecked(value))
    }
    #[cfg(test)]
    ///New Node from a vec of str defined in GDTF-XML without checking if charse are valid for GDTF-Names
    pub fn new_from_strs_unchecked(value: Vec<&str>) -> Self {
        Self::Node(Name::strs_to_names_vec_unchecked(value))
    }
    ///New Node with None content
    pub fn no_feature() -> Self {
        Self::NoFeature
    }
}

impl Node for NodeChannelFunctionAttribute {}

///Parses a str directly to a Node. This function will not allow invalid chars due to GDTF specs.
impl TryFrom<&str> for NodeChannelFunctionAttribute {
    type Error = GDTFNodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new_from_str(value)
    }
}

///Parses an xml attribute directly to a Node. In case of an error, the function will return a Node with None. This function will allow invalid chars for Name due to GDTF specs because Rust can handle it.
impl From<Attribute<'_>> for NodeChannelFunctionAttribute {
    fn from(attr: Attribute<'_>) -> Self {
        Self::new_from_str_unchecked(std::str::from_utf8(attr.value.borrow()).unwrap_or_else(|_| ""))
    }
}

///Default value is NoFeature
impl Default for NodeChannelFunctionAttribute {
    fn default() -> Self {
        NodeChannelFunctionAttribute::NoFeature
    }
}


#[cfg(test)]
mod tests {
    use std::convert::{TryFrom, TryInto};

    use NodeChannelFunctionAttribute as T;

    use crate::utils::errors::GdtfError;
    use crate::utils::testdata;
    use crate::utils::units::name::Name;
    use crate::utils::units::node::node_channel_function_attribute::NodeChannelFunctionAttribute;

    #[test]
    fn test_new_from_str() -> Result<(), GdtfError> {
        assert_eq!(T::Node(vec![Name::new("test")?]), T::new_from_str("test")?);
        assert_eq!(T::Node(vec![Name::new("test")?, Name::new("test2")?]), T::new_from_str("test.test2")?);
        assert_eq!(T::NoFeature, T::new_from_str("")?);
        assert!(T::new_from_str("asdf{").is_err());
        assert!(T::new_from_str("Test.asdf{").is_err());
        assert!(T::new_from_str("asdf{.Test").is_err());
        Ok(())
    }

    #[test]
    fn test_new_from_str_unchecked() -> Result<(), GdtfError> {
        assert_eq!(T::Node(vec![Name::new("test")?]), T::new_from_str_unchecked("test"));
        assert_eq!(T::Node(vec![Name::new("test")?, Name::new("test2")?]), T::new_from_str_unchecked("test.test2"));
        assert_eq!(T::NoFeature, T::new_from_str_unchecked(""));
        assert_eq!(T::Node(vec![Name::new_unchecked("asdf{")]), T::new_from_str_unchecked("asdf{"));
        assert_eq!(T::Node(vec![Name::new("Test")?, Name::new_unchecked("asdf{")]), T::new_from_str_unchecked("Test.asdf{"));
        assert_eq!(T::Node(vec![Name::new_unchecked("asdf{"), Name::new("Test")?]), T::new_from_str_unchecked("asdf{.Test"));
        Ok(())
    }

    #[test]
    fn test_new_from_strs_unchecked() -> Result<(), GdtfError> {
        assert_eq!(T::Node(vec![Name::new("test")?]), T::new_from_strs_unchecked(vec!["test"]));
        assert_eq!(T::Node(vec![Name::new("test")?, Name::new("test2")?]), T::new_from_strs_unchecked(vec!["test", "test2"]));
        assert_eq!(T::Node(vec![Name::new("")?]), T::new_from_strs_unchecked(vec![""]));
        assert_eq!(T::Node(vec![Name::new_unchecked("asdf{")]), T::new_from_strs_unchecked(vec!["asdf{"]));
        assert_eq!(T::Node(vec![Name::new("Test")?, Name::new_unchecked("asdf{")]), T::new_from_strs_unchecked(vec!["Test", "asdf{"]));
        assert_eq!(T::Node(vec![Name::new_unchecked("asdf{"), Name::new("Test")?]), T::new_from_strs_unchecked(vec!["asdf{", "Test"]));
        Ok(())
    }


    #[test]
    fn test_try_from_str() -> Result<(), GdtfError> {
        assert_eq!(T::NoFeature, "".try_into()?);
        assert_eq!(T::Node(vec!["One".try_into()?]), "One".try_into()?);
        assert_eq!(T::Node(vec!["One".try_into()?, "Two".try_into()?]), "One.Two".try_into()?);
        assert!(T::try_from("Some{Invalid").is_err());
        Ok(())
    }

    #[test]
    fn test_from_attr_owned() -> Result<(), GdtfError> {
        assert_eq!(T::NoFeature, testdata::to_attr_owned(b"").into());
        assert_eq!(T::Node(vec!["One".try_into()?]), testdata::to_attr_owned(b"One").into());
        assert_eq!(T::Node(vec!["One".try_into()?, "Two".try_into()?]), testdata::to_attr_owned(b"One.Two").into());
        assert_eq!(T::Node(vec![Name::new_unchecked("Some{Invalid"), "Two".try_into()?]), testdata::to_attr_owned(b"Some{Invalid.Two").into());
        assert_eq!(T::Node(vec![Name::new_unchecked("Some{Invalid"), Name::new_unchecked("T{wo")]), testdata::to_attr_owned(b"Some{Invalid.T{wo").into());
        Ok(())
    }

    #[test]
    fn test_from_attr_borrowed() -> Result<(), GdtfError> {
        assert_eq!(T::NoFeature, testdata::to_attr_borrowed(b"").into());
        assert_eq!(T::Node(vec!["One".try_into()?]), testdata::to_attr_borrowed(b"One").into());
        assert_eq!(T::Node(vec!["One".try_into()?, "Two".try_into()?]), testdata::to_attr_borrowed(b"One.Two").into());
        assert_eq!(T::Node(vec![Name::new_unchecked("Some{Invalid"), "Two".try_into()?]), testdata::to_attr_borrowed(b"Some{Invalid.Two").into());
        assert_eq!(T::Node(vec![Name::new_unchecked("Some{Invalid"), Name::new_unchecked("T{wo")]), testdata::to_attr_borrowed(b"Some{Invalid.T{wo").into());
        Ok(())
    }

    #[test]
    fn test_no_feature() -> Result<(), GdtfError> {
        assert_eq!(T::NoFeature, T::no_feature());
        Ok(())
    }

    #[test]
    fn test_default() -> Result<(), GdtfError> {
        assert_eq!(T::NoFeature, Default::default());
        Ok(())
    }
}
