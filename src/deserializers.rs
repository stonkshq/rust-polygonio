//! Custom deserializers for Polygon.io API responses
//!
//! This module contains custom serde deserializers to handle various data format
//! inconsistencies in the Polygon.io API, such as:
//! - Volume fields returned in scientific notation (e.g., 3.7659058e+07)
//! - Mixed integer/float representations
//! - Optional fields with different semantics

use serde::{Deserialize, Deserializer};

/// Custom deserializer for volume fields that may come as scientific notation
///
/// Polygon.io sometimes returns integer volume values in scientific notation
/// (e.g., 3.7659058e+07 for 37,659,058). This deserializer handles both
/// regular integers and scientific notation by deserializing as f64 first
/// then converting to i64.
///
/// # Usage
/// ```rust
/// use serde::Deserialize;
/// use polygon_io::deserializers::deserialize_volume_as_i64;
///
/// #[derive(Deserialize)]
/// struct VolumeData {
///     #[serde(deserialize_with = "deserialize_volume_as_i64")]
///     volume: i64,
/// }
/// ```
pub fn deserialize_volume_as_i64<'de, D>(deserializer: D) -> std::result::Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let value: f64 = f64::deserialize(deserializer)?;
    Ok(value as i64)
}

/// Custom deserializer for optional volume fields that may come as scientific notation
///
/// Similar to `deserialize_volume_as_i64` but handles optional fields.
pub fn deserialize_optional_volume_as_i64<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<f64> = Option::deserialize(deserializer)?;
    Ok(value.map(|v| v as i64))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use serde_json;

    #[derive(Deserialize, Debug, PartialEq)]
    struct TestVolumeStruct {
        #[serde(deserialize_with = "deserialize_volume_as_i64")]
        volume: i64,
        #[serde(deserialize_with = "deserialize_optional_volume_as_i64")]
        optional_volume: Option<i64>,
    }

    #[test]
    fn test_scientific_notation_volume() {
        let json = r#"{"volume": 3.7659058e+07, "optional_volume": 1.5e+06}"#;
        let result: TestVolumeStruct = serde_json::from_str(json).unwrap();

        assert_eq!(result.volume, 37659058);
        assert_eq!(result.optional_volume, Some(1500000));
    }

    #[test]
    fn test_regular_volume() {
        let json = r#"{"volume": 1000000, "optional_volume": 500000}"#;
        let result: TestVolumeStruct = serde_json::from_str(json).unwrap();

        assert_eq!(result.volume, 1000000);
        assert_eq!(result.optional_volume, Some(500000));
    }

    #[test]
    fn test_null_optional_volume() {
        let json = r#"{"volume": 1000000, "optional_volume": null}"#;
        let result: TestVolumeStruct = serde_json::from_str(json).unwrap();

        assert_eq!(result.volume, 1000000);
        assert_eq!(result.optional_volume, None);
    }
}
