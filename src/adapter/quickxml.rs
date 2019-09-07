use quick_xml::Reader;
use quick_xml::events::Event;

use super::*;
use std::io::BufRead;
use error::{ErrorKind, Result};

// FIXME: unwraps should be proper errors

impl<R: BufRead> GenericEventReader for Reader<R> {
    fn next(&mut self) -> Result<GenericXmlEvent> {
        loop {
            let mut buf = Vec::new();
            let event = self.read_event(&mut buf)
                .map_err(|e| ErrorKind::Custom(e.to_string()))?; // FIXME

            return match event {
                Event::Eof => Ok(GenericXmlEvent::EndDocument),
                Event::Start(bytes_start) => Ok(GenericXmlEvent::StartElement {
                    name: GenericXmlName { local_name: self.decode(bytes_start.local_name()).unwrap().to_string() },
                    attributes: bytes_start.attributes()
                        .map(|x| x.unwrap())
                        .map(|x| GenericXmlAttribute {
                            name: GenericXmlName { local_name: self.decode(x.key).unwrap().to_string() },
                            value: x.unescape_and_decode_value(self).unwrap(),
                        }).collect(),
                }),
                Event::End(bytes_end) => Ok(GenericXmlEvent::EndElement {
                    name: GenericXmlName { local_name: self.decode(bytes_end.local_name()).unwrap().to_string() },
                }),
                Event::Text(bytes_text) => {
                    let characters = bytes_text.unescape_and_decode(self).unwrap();
                    let word_count = characters.split_whitespace().count(); // FIXME
                    match word_count {
                        0 => continue,
                        _ => Ok(GenericXmlEvent::Characters(characters)),
                    }
                },
                _ => continue,
            }
        }
    }
}
