use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
struct FooBar {
    foo: u32,
    bar: String,
    map: std::collections::HashMap<String, String>,
}

fn main() {
    let mut f1 = FooBar {
        foo: 42,
        bar: "The answer".into(),
        map: std::collections::HashMap::new(),
    };
    f1.map.insert("foo".into(), "first".into());
    f1.map.insert("baz".into(), "third".into());
    f1.map.insert("bar".into(), "second".into());
    let f1_json = serde_json::to_string_pretty(&f1).unwrap();
    let f2: FooBar = serde_json::from_str(&f1_json).unwrap();
    println!("{:?}", f2);
}
