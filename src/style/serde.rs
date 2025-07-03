use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::Error;

impl Serialize for NodeAttr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for NodeAttr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(D::Error::custom)
    }
}

impl Serialize for EdgeAttr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for EdgeAttr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(D::Error::custom)
    }
}

impl Serialize for GraphAttr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for GraphAttr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(D::Error::custom)
    }
}

impl Serialize for NodeAttribute {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        match self {
            NodeAttribute::Common(attr) => serializer.serialize_str(&attr.to_string()),
            NodeAttribute::NodeAttr(attr) => serializer.serialize_str(&attr.to_string()),
        }
    }
}

impl<'de> Deserialize<'de> for NodeAttribute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?;

        if let Ok(attr) = s.parse::<CommonAttr>() {
            Ok(NodeAttribute::Common(attr))
        } else if let Ok(attr) = s.parse::<NodeAttr>() {
            Ok(NodeAttribute::NodeAttr(attr))
        } else {
            Err(D::Error::custom("Invalid NodeAttribute string"))
        }
    }
}

impl Serialize for EdgeAttribute {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        match self {
            EdgeAttribute::Common(attr) => serializer.serialize_str(&attr.to_string()),
            EdgeAttribute::EdgeAttr(attr) => serializer.serialize_str(&attr.to_string()),
        }
    }
}

impl<'de> Deserialize<'de> for EdgeAttribute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?;

        if let Ok(attr) = s.parse::<CommonAttr>() {
            Ok(EdgeAttribute::Common(attr))
        } else if let Ok(attr) = s.parse::<EdgeAttr>() {
            Ok(EdgeAttribute::EdgeAttr(attr))
        } else {
            Err(D::Error::custom("Invalid EdgeAttribute string"))
        }
    }
}