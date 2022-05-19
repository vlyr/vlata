use std::marker::PhantomData;
use crate::core::generate_padding_bytes;

pub trait ValueToType<T> {
    fn convert(value: &Value) -> T;
    fn to_bytes(value: T) -> Vec<u8>;
}

impl ValueToType<String> for String {
    fn convert(data: &Value) -> String {
        String::from_utf8(data.raw_data.clone()).unwrap()
    }

    fn to_bytes(value: String) -> Vec<u8> {
        value.as_bytes().to_vec()
    }
}

impl ValueToType<u32> for u32 {
    fn convert(data: &Value) -> u32 {
        u32::from_be_bytes(<[u8; 4]>::try_from(data.raw_data.clone()).unwrap())
    }

    fn to_bytes(data: u32) -> Vec<u8> {
        data.to_be_bytes().to_vec()
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

    pub fn to_bytes(&self) -> [u8; 32] {
        let data = &mut [0x0u8; 32];

        let key_bytes = self.key.as_bytes();
        let padding_bytes = generate_padding_bytes(&key_bytes);
        data[0..16].clone_from_slice(&[key_bytes, &padding_bytes].concat());

        let value_bytes = &self.value.raw_data.as_slice();
        let padding_bytes = generate_padding_bytes(&value_bytes);
        data[16..32].clone_from_slice(&[value_bytes, padding_bytes.as_slice()].concat());

        data.clone()
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
