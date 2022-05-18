#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    Str,
    Uint
}

#[derive(Debug, Clone)]
pub struct Row<V> {
    value_type: ValueType,
    key: String,
    value: V
}

impl<V> Row<V> {
    pub fn value_type(&self) -> &ValueType {
        &self.value_type
    }

    pub fn key(&self) -> &String {
        &self.key
    }

    pub fn value(&self) -> &V {
        &self.value
    }
}

impl<V> From<(ValueType, String, V)> for Row<V> {
    fn from((value_type, key, value): (ValueType, String, V)) -> Self {
        Self {
            value_type,
            key,
            value
        }
    }
}
