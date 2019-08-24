pub mod xmlrs;
pub mod quickxml;

use error::{ErrorKind, Result};


pub trait GenericEventReader {
    fn next(&mut self) -> Result<GenericXmlEvent>;
}

#[derive(PartialEq, Clone, Debug)]
pub enum GenericXmlEvent {
    EndDocument,

    StartElement {
        name: GenericXmlName,
        attributes: Vec<GenericXmlAttribute>,
    },

    EndElement {
        name: GenericXmlName,
    },

    Characters(String),
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct GenericXmlAttribute {
    pub name: GenericXmlName,
    pub value: String,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct GenericXmlName {
    pub local_name: String,
}

