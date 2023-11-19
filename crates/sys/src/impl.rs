use super::napi_value;

pub trait NapiValue<T> {
    fn get_value_from_raw(&self) -> T;
    fn try_into_raw(&self) -> napi_value;
}