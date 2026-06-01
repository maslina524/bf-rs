#[macro_export]
macro_rules! get_string {
    ($map:expr, $key:expr) => {{
        let _map: &Map<String, Value> = $map;
        let _key: &str = &$key;

        _map.get(_key).and_then(|v| v.as_str()).ok_or("Strange Response")
    }};
}

#[macro_export]
macro_rules! get_i64 {
    ($map:expr, $key:expr) => {{
        let _map: &Map<String, Value> = $map;
        let _key: &str = &$key;

        _map.get(_key).and_then(|v| v.as_number().and_then(|v| v.as_i64())).ok_or("Strange Response")
    }};
}

#[macro_export]
macro_rules! get_array {
    ($map:expr, $key:expr) => {{
        let _map: &Map<String, Value> = $map;
        let _key: &str = &$key;

        _map.get(_key).and_then(|v| v.as_array()).ok_or("Strange Response")
    }};
}

#[macro_export]
macro_rules! get_object {
    ($map:expr, $key:expr) => {{
        let _map: &Map<String, Value> = $map;
        let _key: &str = &$key;

        _map.get(_key).and_then(|v| v.as_object()).ok_or("Strange Response")
    }};
}

#[macro_export]
macro_rules! get_bool {
    ($map:expr, $key:expr) => {{
        let _map: &Map<String, Value> = $map;
        let _key: &str = &$key;

        _map.get(_key).and_then(|v| v.as_bool()).ok_or("Strange Response")
    }};
}