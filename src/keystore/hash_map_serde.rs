use serde::Serializer;
use super::*;
use anyhow::Result;

pub fn serialize<S>(map: &HashMap<String, TKey<ChaCha20Poly1305>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    let converted: Vec<_> = map
        .iter()
        .map(|(k, v)| (k, BASE64_STANDARD.encode(v).to_string()))
        .collect();
    converted.serialize(serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<HashMap<String, TKey<ChaCha20Poly1305>>, D::Error>
    where
        D: Deserializer<'de>,
{
    let vec: Vec<(String, String)> = Deserialize::deserialize(deserializer)?;
    let mut map: HashMap<String, TKey<ChaCha20Poly1305>> = HashMap::new();
    for (key, value) in vec {
        let bytes = BASE64_STANDARD.decode(value).unwrap();
        let mut arr: TKey<ChaCha20Poly1305> = Default::default();
        for (place, element) in arr.iter_mut().zip(bytes) {
            *place = element;
        }
        map.insert(key, arr);
    }
    return Ok(map);
}