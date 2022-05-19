use std::marker::PhantomData;

pub trait ValueToType<T> {
    fn convert(value: &Value) -> T;
}

impl ValueToType<String> for String {
    fn convert(data: &Value) -> String {
        String::from_utf8(data.raw_data.clone()).unwrap()
    }
}

impl ValueToType<u32> for u32 {
    fn convert(data: &Value) -> u32 {
        u32::from_be_bytes(<[u8; 4]>::try_from(data.raw_data.clone()).unwrap())
    }
}

#[derive(Debug, Clone)]
pub struct Value {
    raw_data: Vec<u8>
}

impl From<Vec<u8>> for Value {
    fn from(data: Vec<u8>) -> Self {
        Self::new(data)
    }
}

impl Value {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            raw_data: data
        }
    }
    pub fn to_type<T: ValueToType<T>>(&self) -> T {
        T::convert(&self)
    }
}

#[derive(Debug, Clone)]
pub struct Record<T> {
    key: String,
    value: Value,
    _marker: PhantomData<T>
}

impl<T: ValueToType<T>> Record<T> {
    pub fn key(&self) -> &String {
        &self.key
    }

    pub fn value(&self) -> T {
        T::convert(&self.value)
    }

    pub fn value_raw(&self) -> &Value {
        &self.value
    }
}

impl<T: ValueToType<T>> From<(String, Vec<u8>)> for Record<T> {
    fn from((key, value_data): (String, Vec<u8>)) -> Self {
        Self {
            key,
            value: Value::from(value_data),
            _marker: PhantomData
        }
    }
}
