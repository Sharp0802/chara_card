
macro_rules! extensions {
    ($($name:ident($value:ty),)+) => {
        use serde_json::Value;
        use serdev::de::{MapAccess, Visitor};
        use serdev::{Deserialize, Deserializer, Serialize, Serializer};
        use serdev::ser::SerializeMap;
        use std::fmt::Formatter;
        
        impl Serialize for Extensions {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let mut ser = serializer.serialize_map(Some(self.0.len()))?;
                for ext in self.deref() {
                    ext.serialize_entry(&mut ser)?;
                }
                ser.end()
            }
        }

        impl<'de> Deserialize<'de> for Extensions {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_map(ExtensionVisitor)
            }
        }

        struct ExtensionVisitor;

        impl<'de> Visitor<'de> for ExtensionVisitor {
            type Value = Extensions;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                write!(formatter, "an extension map")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut extensions = vec![];

                while let Some(key) = map.next_key::<&str>()? {
                    let value = match key {
                        $(stringify!($name) => paste::paste! { Extension::[< $name:camel >](map.next_value()?) } ,)+
                        _ => Extension::Unknown(key.to_owned(), map.next_value()?),
                    };

                    extensions.push(value);
                }

                Ok(Extensions(extensions))
            }
        }

        paste::paste!{
            #[derive(Debug, Clone)]
            pub enum Extension {
                $([< $name:camel >]($value),)+
                Unknown(String, Value),
            }
        }

        impl Extension {
            fn serialize_entry<T: SerializeMap>(&self, ser: &mut T) -> Result<(), T::Error> {
                match &self {
                    $(paste::paste! { Self::[< $name:camel >](value) } => ser.serialize_entry(stringify!($name), value),)+
                    Self::Unknown(key, value) => ser.serialize_entry(key, value),
                }
            }
        }
    };
}

pub(crate) use extensions;
