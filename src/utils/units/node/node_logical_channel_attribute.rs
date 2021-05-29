//!Module for Node used in LogicalChannel.attribute
use std::borrow::Borrow;
use std::convert::{TryFrom, TryInto};

use quick_xml::events::attributes::Attribute;

#[cfg(test)]
use crate::utils::partial_eq_allow_empty::PartialEqAllowEmpty;
use crate::utils::partial_eq_option::partial_eq_option;
use crate::utils::units::attribute_name::AttributeName;
use crate::utils::units::node::{GDTFNodeError, NodeHelper};
use crate::utils::units::node::node_option::NodeOption;

#[derive(Debug)]
///Node used in LogicalChannel.attribute. Link to the channel function that will be activated by default for this DMXChannel;
pub struct NodeLogicalChannelAttribute(Option<Vec<AttributeName>>);

impl NodeHelper for NodeLogicalChannelAttribute {}

///Parses an xml attribute directly to a Node. In case of an error, the function will return a Node with None. This function will allow invalid chars for Name due to GDTF specs because Rust can handle it.
impl From<Attribute<'_>> for NodeLogicalChannelAttribute {
    fn from(attr: Attribute<'_>) -> Self {
        NodeLogicalChannelAttribute::new_from_str_unchecked(std::str::from_utf8(attr.value.borrow()).unwrap_or_else(|_| ""))
    }
}


impl NodeLogicalChannelAttribute {
    pub fn new_from_str(value: &str) -> Result<Self, GDTFNodeError> {
        if value == "" {
            return Ok(Self(None));
        }
        let value = value.split(".");
        let mut tree: Vec<AttributeName> = vec![];
        for value in value.into_iter() {
            tree.push(value.try_into()?);
        }
        Ok(Self(Some(tree)))
    }


    pub fn new_from_str_unchecked(value: &str) -> Self {
        if value == "" {
            return Self(None);
        }
        let value = value.split(".");
        let mut tree: Vec<AttributeName> = vec![];
        for value in value.into_iter() {
            tree.push(AttributeName::new_unchecked(value));
        }
        Self(Some(tree))
    }


    pub fn new_from_attribute_names(names: Vec<AttributeName>) -> Result<Self, GDTFNodeError> {
        Ok(Self(Some(names)))
    }

    ///Creates a new instance with None content
    pub fn none() -> Self {
        Self(None)
    }

    pub fn new_from_strs(names: Vec<&str>) -> Result<Self, GDTFNodeError> {
        let mut vec = vec![];
        for name in names.into_iter() {
            vec.push(AttributeName::new(name)?);
        }
        Ok(Self(Some(vec)))
    }

    pub fn new_from_strs_unchecked(names: Vec<&str>) -> Self {
        let mut vec = vec![];
        for name in names.into_iter() {
            vec.push(AttributeName::new_unchecked(name));
        }
        Self(Some(vec))
    }
}

#[cfg(test)]
impl PartialEqAllowEmpty for NodeLogicalChannelAttribute {
    fn is_eq_allow_empty_impl(&self, other: &Self, log: bool) -> bool {
        Self::is_eq_option_of_vec_allow_empty(&self.0, &other.0, log)
    }
}

///Parses a str directly to a Node. This function will not allow invalid chars due to GDTF specs.
impl TryFrom<&str> for NodeLogicalChannelAttribute {
    type Error = GDTFNodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self::new_from_str(value)?)
    }
}

///Partial eq returns false if one is None, otherwise it compares the node value
impl PartialEq for NodeLogicalChannelAttribute {
    fn eq(&self, other: &Self) -> bool {
        partial_eq_option(&self.0, &other.0)
    }
}

///Default value is None
impl Default for NodeLogicalChannelAttribute {
    fn default() -> Self {
        NodeLogicalChannelAttribute(None)
    }
}

///Implements helper trait for Option<Node> to prevent code redundancy
impl NodeOption for NodeLogicalChannelAttribute {}

#[cfg(test)]
mod tests {
    use std::convert::{TryFrom, TryInto};

    use NodeLogicalChannelAttribute as T;

    use crate::utils::errors::GdtfError;
    use crate::utils::partial_eq_allow_empty::PartialEqAllowEmpty;
    use crate::utils::testdata;
    use crate::utils::units::attribute_name::AttributeName;
    use crate::utils::units::node::node_logical_channel_attribute::NodeLogicalChannelAttribute;

    #[test]
    fn test_partial_eq_allow_empty() -> Result<(), GdtfError> {
        assert!(T::new_from_str("")?.is_eq_allow_empty(&T::new_from_str("")?, true));
        assert!(T(Some(vec![])).is_eq_allow_empty(&T(Some(vec![])), true));
        assert!(T::new_from_str("test")?.is_eq_allow_empty(&T::new_from_str("test")?, true));
        assert!(T::new_from_strs(vec!["some", "test"])?.is_eq_allow_empty(&T::new_from_strs(vec!["some", "test"])?, true));
        assert!(T::new_from_strs(vec!["", "test"])?.is_eq_allow_empty(&T::new_from_strs(vec!["", "test"])?, true));
        assert!(T::new_from_strs(vec!["some", ""])?.is_eq_allow_empty(&T::new_from_strs(vec!["some", ""])?, true));

        assert!(!T::new_from_str("")?.is_eq_allow_empty(&T::new_from_str("test")?, false));
        assert!(!T::new_from_str("some")?.is_eq_allow_empty(&T::new_from_str("test")?, false));
        assert!(!T::new_from_str("test")?.is_eq_allow_empty(&T::new_from_str("")?, false));
        assert!(!T(Some(vec![])).is_eq_allow_empty(&T::new_from_str("")?, false));
        assert!(!T::new_from_str("")?.is_eq_allow_empty(&T(Some(vec![])), false));
        Ok(())
    }

    #[test]
    fn test_new_from_strs() -> Result<(), GdtfError> {
        T(Some(vec![AttributeName::new("test")?])).assert_eq_allow_empty(&T::new_from_strs(vec!["test"])?, true);
        T(Some(vec![AttributeName::new("test")?, AttributeName::new("other")?])).assert_eq_allow_empty(&T::new_from_strs(vec!["test", "other"])?, true);
        T(Some(vec![AttributeName::new("")?])).assert_eq_allow_empty(&T::new_from_strs(vec![""])?, true);
        T(Some(vec![])).assert_eq_allow_empty(&T::new_from_strs(vec![])?, true);
        assert!(T::new_from_strs(vec!["asdf{"]).is_err());
        assert!(T::new_from_strs(vec!["test", "asdf{"]).is_err());
        Ok(())
    }

    #[test]
    fn test_new_from_str() -> Result<(), GdtfError> {
        T(Some(vec![AttributeName::new("test")?])).assert_eq_allow_empty(&T::new_from_str("test")?, true);
        T(None).assert_eq_allow_empty(&T::new_from_str("")?, true);
        assert!(T::new_from_str("asdf{").is_err());
        Ok(())
    }

    #[test]
    fn test_new_from_strs_unchecked() -> Result<(), GdtfError> {
        T(Some(vec![AttributeName::new("test")?])).assert_eq_allow_empty(&T::new_from_strs_unchecked(vec!["test"]), true);
        T(Some(vec![AttributeName::new("test")?, AttributeName::new("other")?])).assert_eq_allow_empty(&T::new_from_strs_unchecked(vec!["test", "other"]), true);
        T(Some(vec![AttributeName::new("")?])).assert_eq_allow_empty(&T::new_from_strs_unchecked(vec![""]), true);
        T(Some(vec![])).assert_eq_allow_empty(&T::new_from_strs_unchecked(vec![]), true);
        T(Some(vec![AttributeName::new_unchecked("asdf{")])).assert_eq_allow_empty(&T::new_from_strs_unchecked(vec!["asdf{"]), true);
        T(Some(vec![AttributeName::new("test")?, AttributeName::new_unchecked("asdf{")])).assert_eq_allow_empty(&T::new_from_strs_unchecked(vec!["test", "asdf{"]), true);
        Ok(())
    }

    #[test]
    fn test_new_from_str_unchecked() -> Result<(), GdtfError> {
        T(Some(vec![AttributeName::new("test")?])).assert_eq_allow_empty(&T::new_from_str_unchecked("test"), true);
        T(None).assert_eq_allow_empty(&T::new_from_str_unchecked(""), true);
        T(Some(vec![AttributeName::new_unchecked("asdf{")])).assert_eq_allow_empty(&T::new_from_str_unchecked("asdf{"), true);
        Ok(())
    }

    #[test]
    fn test_none() {
        T(None).assert_eq_allow_empty(&T::none(), true)
    }

    #[test]
    fn test_from_attr_borrowed() -> Result<(), GdtfError> {
        T(None).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"").into(), true);
        T(Some(vec!["One".try_into()?])).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"One").into(), true);
        T(Some(vec!["One".try_into()?, "Two".try_into()?])).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"One.Two").into(), true);
        T(Some(vec![AttributeName::new_unchecked("Some{Invalid"), "Two".try_into()?])).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"Some{Invalid.Two").into(), true);
        T(Some(vec![AttributeName::new_unchecked("Some{Invalid"), AttributeName::new_unchecked("T{wo")])).assert_eq_allow_empty(&testdata::to_attr_borrowed(b"Some{Invalid.T{wo").into(), true);
        Ok(())
    }

    #[test]
    fn test_from_attr_owned() -> Result<(), GdtfError> {
        T(None).assert_eq_allow_empty(&testdata::to_attr_owned(b"").into(), true);
        T(Some(vec!["One".try_into()?])).assert_eq_allow_empty(&testdata::to_attr_owned(b"One").into(), true);
        T(Some(vec!["One".try_into()?, "Two".try_into()?])).assert_eq_allow_empty(&testdata::to_attr_owned(b"One.Two").into(), true);
        T(Some(vec![AttributeName::new_unchecked("Some{Invalid"), "Two".try_into()?])).assert_eq_allow_empty(&testdata::to_attr_owned(b"Some{Invalid.Two").into(), true);
        T(Some(vec![AttributeName::new_unchecked("Some{Invalid"), AttributeName::new_unchecked("T{wo")])).assert_eq_allow_empty(&testdata::to_attr_owned(b"Some{Invalid.T{wo").into(), true);
        Ok(())
    }

    #[test]
    fn test_try_from_str() -> Result<(), GdtfError> {
        T(None).assert_eq_allow_empty(&"".try_into()?, true);
        T(Some(vec!["One".try_into()?])).assert_eq_allow_empty(&"One".try_into()?, true);
        T(Some(vec!["One".try_into()?, "Two".try_into()?])).assert_eq_allow_empty(&"One.Two".try_into()?, true);
        assert!(T::try_from("Some{Invalid").is_err());
        Ok(())
    }

    #[test]
    fn test_partial_eq() -> Result<(), GdtfError> {
        assert_ne!(T(None), T(None));
        assert_ne!(T(None), T(Some(vec![])));
        assert_ne!(T(Some(vec![])), T(None));
        assert_eq!(T(Some(vec![])), T(Some(vec![])));
        assert_eq!(T(Some(vec!["One".try_into()?])), T(Some(vec!["One".try_into()?])));
        assert_eq!(T(Some(vec!["One".try_into()?, "Two".try_into()?])), T(Some(vec!["One".try_into()?, "Two".try_into()?])));
        assert_ne!(T(Some(vec!["One".try_into()?, "Two".try_into()?])), T(Some(vec!["One".try_into()?])));
        assert_ne!(T(Some(vec!["Two".try_into()?, "One".try_into()?])), T(Some(vec!["One".try_into()?, "Two".try_into()?])));
        Ok(())
    }

    #[test]
    fn test_default() {
        T(None).assert_eq_allow_empty(&Default::default(), true)
    }
}