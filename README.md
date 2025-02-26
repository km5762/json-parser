# JSON Parser in Rust

This is a simple JSON parser implemented in Rust. It supports parsing JSON values, including strings, numbers, booleans, null, arrays, and objects.

## Features
- Parses JSON strings into a `Value` enum
- Supports numbers, booleans, null, arrays, and objects
- Provides error handling for invalid JSON inputs
- Includes unit tests for validation

## Usage

### Parsing JSON Strings
```rust
use json_parser::Value;

fn main() {
    let json = "{\"key\": \"value\"}";
    if let Some(parsed) = Value::from_str(json) {
        println!("Parsed JSON: {:?}", parsed);
    } else {
        println!("Invalid JSON");
    }
}
```

### Supported JSON Types
The parser supports the following JSON structures:
- **String**: `"hello"`
- **Number**: `42`, `3.14`, `-123`
- **Boolean**: `true`, `false`
- **Null**: `null`
- **Array**: `[1, 2, 3]`, `["a", null, true]`
- **Object**: `{ "key": "value" }`, `{ "nested": { "inner": 42 } }`

## Implementation
The parser consists of two main modules:
- **Scanner**: Tokenizes the input string into JSON tokens.
- **Parser**: Converts tokens into the `Value` enum representation.
