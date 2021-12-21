use serde::de::Error;

use super::Version;

impl<'de> serde::Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let version_str = <&str>::deserialize(deserializer)?;
        version_str.parse().map_err(Error::custom)
    }
}
