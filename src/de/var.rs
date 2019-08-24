use serde::de::{self, Deserializer as SerdeDeserializer, IntoDeserializer};

use de::Deserializer;
use error::{Error, Result};
use adapter::xmlrs::{GenericEventReader, GenericXmlEvent, GenericXmlName};

pub struct EnumAccess<'a, E: 'a + GenericEventReader> {
    de: &'a mut Deserializer<E>,
}

impl<'a, E: 'a + GenericEventReader> EnumAccess<'a, E> {
    pub fn new(de: &'a mut Deserializer<E>) -> Self {
        EnumAccess { de: de }
    }
}

impl<'de, 'a, E: 'a + GenericEventReader> de::EnumAccess<'de> for EnumAccess<'a, E> {
    type Error = Error;
    type Variant = VariantAccess<'a, E>;

    fn variant_seed<V: de::DeserializeSeed<'de>>(
        self,
        seed: V,
    ) -> Result<(V::Value, VariantAccess<'a, E>)> {
        let name = expect!(
            self.de.peek()?,

            &GenericXmlEvent::Characters(ref name) |
            &GenericXmlEvent::StartElement { name: GenericXmlName { local_name: ref name, .. }, .. } => {
                seed.deserialize(name.as_str().into_deserializer())
            }
        )?;
        self.de.set_map_value();
        Ok((name, VariantAccess::new(self.de)))
    }
}

pub struct VariantAccess<'a, E: 'a + GenericEventReader> {
    de: &'a mut Deserializer<E>,
}

impl<'a, E: 'a + GenericEventReader> VariantAccess<'a, E> {
    pub fn new(de: &'a mut Deserializer<E>) -> Self {
        VariantAccess { de: de }
    }
}

impl<'de, 'a, E: 'a + GenericEventReader> de::VariantAccess<'de> for VariantAccess<'a, E> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        self.de.unset_map_value();
        match self.de.next()? {
            GenericXmlEvent::StartElement {
                name, attributes, ..
            } => if attributes.is_empty() {
                self.de.expect_end_element(name)
            } else {
                Err(de::Error::invalid_length(attributes.len(), &"0"))
            },
            GenericXmlEvent::Characters(_) => Ok(()),
            _ => unreachable!(),
        }
    }

    fn newtype_variant_seed<T: de::DeserializeSeed<'de>>(self, seed: T) -> Result<T::Value> {
        seed.deserialize(&mut *self.de)
    }

    fn tuple_variant<V: de::Visitor<'de>>(self, len: usize, visitor: V) -> Result<V::Value> {
        self.de.deserialize_tuple(len, visitor)
    }

    fn struct_variant<V: de::Visitor<'de>>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> {
        self.de.deserialize_map(visitor)
    }
}
