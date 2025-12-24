use std::collections::HashMap;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    NULL,
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
    Bool(bool),
    DateTime(DateTime<Utc>),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer, 
    {
        match self {
            Value::NULL => serializer.serialize_none(),
            Value::Int(v) => serializer.serialize_i32(*v),
            Value::Long(v) => serializer.serialize_i64(*v),
            Value::Float(v) => serializer.serialize_f32(*v),
            Value::Double(v) => serializer.serialize_f64(*v),
            Value::String(v) => serializer.serialize_str(v),
            Value::Bool(v) => serializer.serialize_bool(*v),
            Value::DateTime(v) => serializer.serialize_str(&v.to_rfc3339()),
            Value::Array(v) => v.serialize(serializer),
            Value::Object(v) => v.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ValueVisitor;
        
        impl<'de> de::Visitor<'de> for ValueVisitor {
            type Value = Value;
            
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("any valid JSON value")
            }
            
            fn visit_none<E>(self) -> Result<Value, E>
            where
                E: de::Error,
            {
                Ok(Value::NULL)
            }
            
            fn visit_unit<E>(self) -> Result<Value, E>
            where
                E: de::Error,
            {
                Ok(Value::NULL)
            }
            
            fn visit_bool<E>(self, v: bool) -> Result<Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Bool(v))
            }
            
            fn visit_i64<E>(self, v: i64) -> Result<Value, E>
            where
                E: de::Error,
            {
                if v >= i32::MIN.into() && v <= i32::MAX.into() {
                    Ok(Value::Int(v as i32))
                } else {
                    Ok(Value::Long(v))
                }
            }
            
            fn visit_u64<E>(self, v: u64) -> Result<Value, E>
            where
                E: de::Error,
            {
                if v <= i32::MAX as u64 {
                    Ok(Value::Int(v as i32))
                } else if v <= i64::MAX as u64 {
                    Ok(Value::Long(v as i64))
                } else {
                    Err(de::Error::custom("integer too large"))
                }
            }
            
            fn visit_f32<E>(self, v: f32) -> Result<Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Float(v))
            }
            
            fn visit_f64<E>(self, v: f64) -> Result<Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Double(v))
            }
            
            fn visit_str<E>(self, v: &str) -> Result<Value, E>
            where
                E: de::Error,
            {
                // 尝试解析为日期时间
                if let Ok(dt) = DateTime::parse_from_rfc3339(v).map(|dt| dt.with_timezone(&Utc)) {
                    Ok(Value::DateTime(dt))
                } else {
                    Ok(Value::String(v.to_string()))
                }
            }
            
            fn visit_string<E>(self, v: String) -> Result<Value, E>
            where
                E: de::Error,
            {
                // 尝试解析为日期时间
                if let Ok(dt) = DateTime::parse_from_rfc3339(&v).map(|dt| dt.with_timezone(&Utc)) {
                    Ok(Value::DateTime(dt))
                } else {
                    Ok(Value::String(v))
                }
            }
            
            fn visit_seq<A>(self, mut seq: A) -> Result<Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let mut vec = Vec::new();
                
                while let Some(element) = seq.next_element()? {
                    vec.push(element);
                }
                
                Ok(Value::Array(vec))
            }
            
            fn visit_map<A>(self, mut map: A) -> Result<Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                let mut hash_map = HashMap::new();
                
                while let Some((key, value)) = map.next_entry()? {
                    hash_map.insert(key, value);
                }
                
                Ok(Value::Object(hash_map))
            }
        }
        
        deserializer.deserialize_any(ValueVisitor)
    }
}
