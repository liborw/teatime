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



impl FromStr for Tags {
    type Err = ParseTagsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Tags(s.split(",").map(|l| Tag::from_str(l).unwrap()).collect()))
    }
}
