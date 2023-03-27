use chrono::{NaiveDate};
use serde::{self, Deserializer, de, Serializer};
use std::fmt;

const FORMAT: &'static str = "%d.%m.%Y";

pub fn serialize<S>(some_date: &Option<NaiveDate>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
  match some_date {
    Some(date) => {
      let s = format!("{}", date.format(FORMAT));
      serializer.serialize_str(&s)
    },
    None => serializer.serialize_none()
  }
}

pub fn deserialize<'de, D>(d: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
  d.deserialize_option(OptionalDateTimeFromCustomFormatVisitor)
}

struct OptionalDateTimeFromCustomFormatVisitor;
impl<'de> de::Visitor<'de> for OptionalDateTimeFromCustomFormatVisitor {
    type Value = Option<NaiveDate>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "null or a datetime string")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }

    fn visit_some<D>(self, d: D) -> Result<Option<NaiveDate>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        Ok(Some(d.deserialize_str(DateTimeFromCustomFormatVisitor)?))
    }
}

struct DateTimeFromCustomFormatVisitor;

impl<'de> de::Visitor<'de> for DateTimeFromCustomFormatVisitor {
    type Value = NaiveDate;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a datetime string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        NaiveDate::parse_from_str(&value, FORMAT)
            .map_err(serde::de::Error::custom)
            .map(|d| d.into())
    }
}
