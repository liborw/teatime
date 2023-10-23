use std::{error::Error, fmt::Display, str::FromStr};

use lazy_static::lazy_static;
use regex::Regex;



#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Tag {
    Label(String),
}

impl Tag {

    pub fn new_label(value: String) -> Tag {
        Tag::Label(value)
    }

    /// Find tags in a text
    pub fn find(s: &str) -> impl Iterator<Item=Tag> + '_ {

        lazy_static! {
            static ref RE: Regex = Regex::new(r"@(?P<label>\w+)(\((?P<value>\w+)?\))?").unwrap();
        }

        RE.captures_iter(s)
            .map(|c| {
                Tag::new_label(c.name("label").unwrap().as_str().to_owned())
            })
    }

}

#[derive(Debug, Clone)]
pub enum ParseTagError {
    Unknown
}

impl Error for ParseTagError {}

impl Display for ParseTagError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for Tag {
    type Err = ParseTagError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Tag::new_label(s.to_string()))
    }
}

#[derive(Debug, Clone, Default)]
pub struct Tags(pub Vec<Tag>);


#[derive(Debug, Clone)]
pub enum ParseTagsError {
    Unknown
}

impl Error for ParseTagsError {}

impl Display for ParseTagsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Tags {
    pub fn iter(self: &Self) -> impl Iterator<Item=&Tag> {
        self.0.iter()
    }

    pub fn push(self: &mut Self, tag: Tag) {
        self.0.push(tag)
    }

    pub fn contains(self: &Self, tag: &Tag) -> bool {
        self.0.contains(tag)
    }

    /// Returns true if all elements in Tags is inside given Tags
    pub fn contains_all(self: &Self, tags: &Tags) -> bool {
        self.iter().all(|t| tags.contains(t))
    }

    /// Return true if at least one element in Self is inside given Tags
    pub fn contains_any(self: &Self, tags: &Tags) -> bool {
        self.iter().any(|t| tags.contains(t))
    }

    /// Cterate Tags from Vec<Tag>
    pub fn from_vec(tags: Vec<Tag>) -> Tags {
        Tags(tags)
    }
}

impl FromIterator<Tag> for Tags {

    fn from_iter<I: IntoIterator<Item=Tag>>(iter: I) -> Self {
        Tags(iter.into_iter().collect())
    }

}


impl FromStr for Tags {
    type Err = ParseTagsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Tags(s.split(",").map(|l| Tag::from_str(l).unwrap()).collect()))
    }
}
