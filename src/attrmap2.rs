//!
//! Defines the type AttrMap as container for different attribute-sets.
//! And there are a number of traits working with AttrMap to set
//! related families of attributes.
//!

use std::collections::{hash_map, HashMap};
use string_cache::DefaultAtom;

/// Container type for attributes.
#[derive(Default, Clone, Debug)]
pub struct AttrMap2 {
    map: Option<HashMap<DefaultAtom, String>>,
}

impl AttrMap2 {
    #[allow(dead_code)]
    pub fn new() -> Self {
        AttrMap2 {
            map: Default::default(),
        }
    }

    /// Are there any attributes?
    pub fn is_empty(&self) -> bool {
        self.map.is_none()
    }

    /// Add from Slice
    pub fn add_all(&mut self, data: &[(&str, String)]) {
        let attr = self.map.get_or_insert_with(HashMap::new);
        for (name, value) in data {
            attr.insert(DefaultAtom::from(*name), value.to_string());
        }
    }

    /// Adds an attribute.
    pub fn set_attr(&mut self, name: &str, value: String) {
        self.map
            .get_or_insert_with(HashMap::new)
            .insert(DefaultAtom::from(name), value);
    }

    /// Removes an attribute.
    pub fn clear_attr(&mut self, name: &str) -> Option<String> {
        if let Some(ref mut attr) = self.map {
            attr.remove(&DefaultAtom::from(name))
        } else {
            None
        }
    }

    /// Returns the attribute.
    pub fn attr(&self, name: &str) -> Option<&String> {
        if let Some(ref prp) = self.map {
            prp.get(&DefaultAtom::from(name))
        } else {
            None
        }
    }

    /// Returns a property or a default.
    pub fn attr_def<'a, 'b, S>(&'a self, name: &'b str, default: S) -> &'a str
    where
        S: Into<&'a str>,
    {
        if let Some(ref prp) = self.map {
            if let Some(value) = prp.get(&DefaultAtom::from(name)) {
                value.as_ref()
            } else {
                default.into()
            }
        } else {
            default.into()
        }
    }

    pub fn iter(&self) -> AttrMapIter<'_> {
        From::from(self)
    }
}

/// Iterator for an AttrMap.
#[derive(Debug)]
pub struct AttrMapIter<'a> {
    it: Option<hash_map::Iter<'a, DefaultAtom, String>>,
}

impl<'a> From<&'a AttrMap2> for AttrMapIter<'a> {
    fn from(attrmap: &'a AttrMap2) -> Self {
        if let Some(ref attrmap) = attrmap.map {
            Self {
                it: Some(attrmap.iter()),
            }
        } else {
            Self { it: None }
        }
    }
}

impl<'a> Iterator for AttrMapIter<'a> {
    type Item = (&'a DefaultAtom, &'a String);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(it) = &mut self.it {
            it.next()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::attrmap2::AttrMap2;

    #[test]
    fn test_attrmap2() {
        let mut m = AttrMap2::new();

        m.add_all(&[
            ("foo", "baz".to_string()),
            ("lol", "now".to_string()),
            ("ful", "uuu".to_string()),
        ]);
        assert_eq!(m.attr("foo").unwrap(), "baz");

        m.set_attr("lol", "loud!".to_string());
        assert_eq!(m.attr("lol").unwrap(), "loud!");

        m.clear_attr("ful");
        assert_eq!(m.attr("ful"), None);
    }
}
