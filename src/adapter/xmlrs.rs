use std::io::Read;

use xml::reader::{EventReader, XmlEvent};
use xml::name::{Name, OwnedName};
use xml::attribute::{Attribute, OwnedAttribute};

pub use super::*;
use error::Result;


impl From<OwnedName> for GenericXmlName {
    fn from(other: OwnedName) -> GenericXmlName {
        return GenericXmlName {
            local_name: other.local_name.clone(),
        }
    }
}

impl<'a> From<Name<'a>> for GenericXmlName {
    fn from(other: Name) -> GenericXmlName {
        return GenericXmlName {
            local_name: other.local_name.to_string(),
        }
    }
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

impl<R: Read> GenericEventReader for EventReader<R> {
    fn next(&mut self) -> Result<GenericXmlEvent> {
        loop {
            let event = self.next().map_err(ErrorKind::Syntax)?;
            return match event {
                XmlEvent::EndDocument => Ok(GenericXmlEvent::EndDocument),
                XmlEvent::EndElement { name } => Ok(GenericXmlEvent::EndElement { name: name.into() }),
                XmlEvent::Characters(characters) => Ok(GenericXmlEvent::Characters(characters)),
                XmlEvent::StartElement { name, attributes, namespace: _namespace } => Ok(GenericXmlEvent::StartElement {
                    name: name.into(),
                    attributes: attributes.into_iter().map(|attr| attr.into()).collect(),
                }),
                _ => continue,
            }
        }
    }
}

//pub trait GenericEventReaderBuilder<R: Read, C> {
//    fn new_with_config(source: R, config: C) -> Self;
//}
//
//impl<R: Read> GenericEventReaderBuilder<R, ParserConfig> for EventReader<R> {
//    fn new_with_config(source: R, config: ParserConfig) -> Self {
//        EventReader::new_with_config(source, config)
//    }
//}
//
//
