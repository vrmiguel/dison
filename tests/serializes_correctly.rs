use dison::{Json, JsonPretty};
use quickcheck::{Arbitrary, Gen};
use quickcheck_macros::quickcheck;

#[quickcheck]
fn serializes_correctly(value: ArbValue) {
    let ArbValue(value) = value;

    assert_eq!(
        Json(&value).to_string(),
        serde_json::to_string(&value).unwrap()
    );
    assert_eq!(
        JsonPretty(&value).to_string(),
        serde_json::to_string_pretty(&value).unwrap()
    );
}

#[derive(Debug, Clone)]
struct ArbValue(serde_json::Value);

impl Arbitrary for ArbValue {
    fn arbitrary(g: &mut Gen) -> Self {
        fn arbitrary_value(g: &mut Gen, depth: u32) -> serde_json::Value {
            if depth > 5 {
                match u8::arbitrary(g) % 4 {
                    0 => serde_json::Value::Null,
                    1 => serde_json::Value::Bool(bool::arbitrary(g)),
                    2 => serde_json::Value::Number(
                        serde_json::Number::from_f64(f64::arbitrary(g))
                            .unwrap_or_else(|| serde_json::Number::from(0)),
                    ),
                    _ => serde_json::Value::String(String::arbitrary(g)),
                }
            } else {
                match u8::arbitrary(g) % 6 {
                    0 => serde_json::Value::Null,
                    1 => serde_json::Value::Bool(bool::arbitrary(g)),
                    2 => {
                        if bool::arbitrary(g) {
                            serde_json::Value::Number(i64::arbitrary(g).into())
                        } else {
                            serde_json::Value::Number(
                                serde_json::Number::from_f64(f64::arbitrary(g))
                                    .unwrap_or_else(|| serde_json::Number::from(0)),
                            )
                        }
                    }
                    3 => serde_json::Value::String(String::arbitrary(g)),
                    4 => {
                        let size = usize::arbitrary(g) % 10;
                        let array: Vec<serde_json::Value> =
                            (0..size).map(|_| arbitrary_value(g, depth + 1)).collect();
                        serde_json::Value::Array(array)
                    }
                    _ => {
                        let size = usize::arbitrary(g) % 8;
                        let mut map = serde_json::Map::new();
                        for _ in 0..size {
                            let key = String::arbitrary(g);
                            let value = arbitrary_value(g, depth + 1);
                            map.insert(key, value);
                        }
                        serde_json::Value::Object(map)
                    }
                }
            }
        }

        ArbValue(arbitrary_value(g, 0))
    }
}
