use chrono::NaiveDateTime;
use serde::{Serialize, Serializer};

pub fn serialize_naive_datetime<S>(dt: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    dt.format("%Y-%m-%d %H:%M:%S")
        .to_string()
        .serialize(serializer)
}
