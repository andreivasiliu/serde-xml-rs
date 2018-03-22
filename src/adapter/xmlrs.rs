use std::io::Read;

use xml::reader::{EventReader, XmlEvent};
use xml::name::{Name, OwnedName};
use xml::attribute::{Attribute, OwnedAttribute};

use error::{ErrorKind, Result};


#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct GenericXmlName {
    name: String,
}

impl From<OwnedName> for GenericXmlName {
    fn from(other: OwnedName) -> GenericXmlName {
        return GenericXmlName {
            name: other.local_name.clone(),
        }
    }
}

impl<'a> From<Name<'a>> for GenericXmlName {
    fn from(other: Name) -> GenericXmlName {
        return GenericXmlName {
            name: other.local_name.to_string(),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct GenericXmlAttribute {
    name: GenericXmlName,
    value: String,
}

impl<'a> From<Attribute<'a>> for GenericXmlAttribute {
    fn from(other: Attribute) -> GenericXmlAttribute {
        return GenericXmlAttribute {
            name: other.name.into(),
            value: other.value.to_string(),
        };
    }
}

impl From<OwnedAttribute> for GenericXmlAttribute {
    fn from(other: OwnedAttribute) -> GenericXmlAttribute {
        return GenericXmlAttribute {
            name: other.name.into(),
            value: other.value.to_string(),
        };
    }
}

#[derive(PartialEq, Clone)]
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

pub trait GenericEventReader {
    fn next(&mut self) -> Result<GenericXmlEvent>;
}

impl<R: Read> GenericEventReader for EventReader<R> {
    fn next(&mut self) -> Result<GenericXmlEvent> {
        loop {
            let event = self.next();
            return match event {
                Ok(XmlEvent::EndDocument) => Ok(GenericXmlEvent::EndDocument),
                Ok(XmlEvent::EndElement {name}) => Ok(GenericXmlEvent::EndElement {name: name.into()}),
                Ok(XmlEvent::Characters(characters)) => Ok(GenericXmlEvent::Characters(characters)),
                Ok(XmlEvent::StartElement {name, attributes, namespace: _namespace}) => Ok(GenericXmlEvent::StartElement {
                    name: name.into(),
                    attributes: attributes.into_iter().map(|attr| attr.into()).collect(),
                }),
                Ok(_) => continue,
                Err(e) => Err(ErrorKind::Custom(e.to_string()).into()),
            }
        }
    }
}


