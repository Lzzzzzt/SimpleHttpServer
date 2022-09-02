use json::{JsonValue, object};

pub fn parse_kv(str: &str) -> JsonValue {
    let mut obj = object! {};

    str.split('&').for_each(|kv| {
        let mut kv = kv.split('=');
        let key = kv.next().unwrap();
        let value = kv.next().unwrap();

        obj.insert(key, value).unwrap();
    });

    obj
}