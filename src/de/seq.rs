use serde::de;

use de::Deserializer;
use error::{Error, Result};
use adapter::xmlrs::{GenericEventReader, GenericXmlEvent};

pub struct SeqAccess<'a, E: 'a + GenericEventReader> {
    de: &'a mut Deserializer<E>,
    max_size: Option<usize>,
    expected_name: Option<String>,
}

impl<'a, E: 'a + GenericEventReader> SeqAccess<'a, E> {
    pub fn new(de: &'a mut Deserializer<E>, max_size: Option<usize>) -> Self {
        let expected_name = if de.unset_map_value() {
            debug_expect!(de.peek(), Ok(&GenericXmlEvent::StartElement { ref name, .. }) => {
                Some(name.local_name.clone())
            })
        } else {
            None
        };
        SeqAccess {
            de: de,
            max_size: max_size,
            expected_name: expected_name,
        }
    }
}

impl<'de, 'a, E: 'a + GenericEventReader> de::SeqAccess<'de> for SeqAccess<'a, E> {
    type Error = Error;

    fn next_element_seed<T: de::DeserializeSeed<'de>>(
        &mut self,
        seed: T,
    ) -> Result<Option<T::Value>> {
        match self.max_size.as_mut() {
            Some(&mut 0) => {
                return Ok(None);
            },
            Some(max_size) => {
                *max_size -= 1;
            },
            None => {},
        }
        let more = match (self.de.peek()?, self.expected_name.as_ref()) {
            (&GenericXmlEvent::StartElement { ref name, .. }, Some(expected_name)) => {
                &name.local_name == expected_name
            },
            (&GenericXmlEvent::EndElement { .. }, None) |
            (_, Some(_)) |
            (&GenericXmlEvent::EndDocument { .. }, _) => false,
            (_, None) => true,
        };
        if more {
            if self.expected_name.is_some() {
                self.de.set_map_value();
            }
            seed.deserialize(&mut *self.de).map(Some)
        } else {
            Ok(None)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        self.max_size
    }
}
