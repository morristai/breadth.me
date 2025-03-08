use serde::{Deserialize, Deserializer};

#[derive(Debug)]
pub struct InfoParams {
    pub sector: Option<Vec<String>>,
    pub industry: Option<Vec<String>>,
}

fn deserialize_vec<'de, D>(deserializer: D) -> crate::error::Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => {
            let cleaned = s.trim_matches(|c| c == '{' || c == '}');
            Ok(Some(cleaned.split(',').map(|s| s.trim().to_string()).collect()))
        }
        None => Ok(None),
    }
}

impl<'de> Deserialize<'de> for InfoParams {
    fn deserialize<D>(deserializer: D) -> crate::error::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawInfoParams {
            #[serde(default, deserialize_with = "deserialize_vec")]
            sector: Option<Vec<String>>,
            #[serde(default, deserialize_with = "deserialize_vec")]
            industry: Option<Vec<String>>,
        }

        let raw = RawInfoParams::deserialize(deserializer)?;
        Ok(InfoParams {
            sector: raw.sector,
            industry: raw.industry,
        })
    }
}
