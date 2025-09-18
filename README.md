# democracy-index

Machine readable version of the [The Economist Democracy
Index](https://en.wikipedia.org/wiki/The_Economist_Democracy_Index) Wikipedia
article.

## Usage

```rust
let democracy = democracy_index::get("NPL").expect("Country not found");
println!("{democracy:?}");

if democracy.regime_type == democracy_index::RegimeType::Authoritarian {
    println!("welp 😿");
}
```

## License

The wikidata data is licensed `CC0-1.0`, since this license is uncommon for
code, the code is licensed `0BSD` instead.
