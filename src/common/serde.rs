use std::{fmt::Display, str::FromStr};

use serde::Deserialize;

#[allow(unused)]
#[derive(serde::Deserialize)]
#[serde(untagged)]
enum StringOrNumber<T> {
    String(String),
    Number(T),
}

// deserialize number from string or number<T>
#[allow(unused)]
pub fn deserialize_string_or_number<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr + serde::Deserialize<'de>,
    T::Err: Display,
    D: serde::Deserializer<'de>,
{
    match StringOrNumber::<T>::deserialize(deserializer)? {
        StringOrNumber::String(s) => s.parse().map_err(serde::de::Error::custom),
        StringOrNumber::Number(n) => Ok(n),
    }
}
