#[macro_export]
macro_rules! get_string {
    ($map:expr, $key:expr) => {{
        let _map: &Map<String, Value> = $map;
        let _key: &str = &$key;

        _map.get(_key).and_then(|v| v.as_str())
    }};
}

#[macro_export]
macro_rules! get_i64 {
    ($map:expr, $key:expr) => {{
        let _map: &Map<String, Value> = $map;
        let _key: &str = &$key;

        _map.get(_key).and_then(|v| v.as_number().and_then(|v| v.as_i64()))
    }};
}

#[macro_export]
macro_rules! get_array {
    ($map:expr, $key:expr) => {{
        let _map: &Map<String, Value> = $map;
        let _key: &str = &$key;

        _map.get(_key).and_then(|v| v.as_array())
    }};
}