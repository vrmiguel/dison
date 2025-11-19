# dison (DIsplay as jSON)

dison is a tiny crate for zero-copy JSON Display implementation for any type that implements `Serialize`.

```rust
use dison::Json;

#[derive(Serialize)]
struct Object {
   key: String,
   values: Vec<u8>,
}

let obj = Object {
   key: "KEY01",
   values: vec![1, 2, 3],
}

// Displays as JSON
let json = Json(&obj);

// Displays as pretty-printed JSON
let json_pretty = JsonPretty(&obj);

// Displays "{'key':'KEY01', 'values':[1,2,3]}
println!("{json}");

// Displays "{
//    'key': 'KEY01',
//    'values': [1, 2, 3]
// }"
println!("{json_pretty}");
```

This crate uses `serde_json` internally and therefore shall always match whatever `serde_json::to_string` produces, with the added benefit of not having to allocate a temporary `String` for common use cases such as printing a JSON-formatted string to `stdout` or within `format!`.

```rust
use dison::Json;

fn send_message(message: &Message) -> Result {
   query("SELECT from send_message($1::jsonb)", Json(message))
}
```