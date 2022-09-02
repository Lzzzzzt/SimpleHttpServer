use json::{object, JsonValue};

pub fn parse_kv(str: &str) -> JsonValue {
    let mut obj = object! {};

    if str.len() > 3 {
        str.split('&').for_each(|kv| {
            let mut kv = kv.split('=');
            let key = kv.next().unwrap();
            let value = kv.next().unwrap();

            obj.insert(key, value).unwrap();
        });
    }

    obj
}

#[cfg(test)]
mod test {
    use crate::utils::parse_kv;
    use json::object;

    #[test]
    fn parse_kv_empty_str() {
        let obj = parse_kv("");
        assert_eq!(obj, object! {});
    }

    #[test]
    fn parse_kv_single_character() {
        let obj = parse_kv("&");
        assert_eq!(obj, object! {});
    }

    #[test]
    fn parse_kv_with_one_key_value_pair() {
        let obj = parse_kv("test=1");
        assert_eq!(obj.to_string(), object! {test: "1"}.to_string())
    }

    #[test]
    fn parse_kv_with_many_key_value_pair() {
        let obj = parse_kv("test1=1&test2=12nd&test3=hhh");
        assert_eq!(
            obj.to_string(),
            object! {test1: "1", test2: "12nd", test3: "hhh"}.to_string()
        )
    }
}
