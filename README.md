# Canonical Form

**A Rust library for transforming data into canonical representations** — normalizes key-value maps with configurable rules (sort, trim, lowercase, null-removal) and produces deterministic string output for hashing, signing, and comparison.

## Why It Matters

Canonicalization — reducing data to a single, unambiguous form — is essential wherever you need to compare, hash, or sign structured data:

- **Digital signatures** — signing `{"a":1,"b":2}` must produce the same signature as `{"b":2,"a":1}`. Without canonicalization, JSON key order makes signatures non-deterministic.
- **HTTP request signing** — AWS SigV4, OAuth 1.0a, and JWTs all require canonical request strings
- **Deduplication** — detecting that two configs are equivalent despite formatting differences
- **Cache keys** — ensuring `?b=2&a=1` and `?a=1&b=2` hit the same cache entry

The most common pitfall: JSON doesn't guarantee key order. Two semantically identical JSON objects can have different byte representations, breaking hashing and signature verification. Canonical form solves this.

## How It Works

The `CanonicalForm` processor applies transformations to `BTreeMap<String, String>` in a configurable pipeline:

1. **Remove nulls** — drop entries with empty values (`remove_nulls` option)
2. **Trim whitespace** — strip leading/trailing spaces from values (`trim_whitespace`)
3. **Lowercase keys** — normalize key casing (`lowercase_keys`)
4. **Sort keys** — `BTreeMap` maintains sorted order by default, but `to_canonical_string()` re-sorts for output

**Canonical string output**: The `to_canonical_string()` method produces `key1=value1&key2=value2&...` — sorted, trimmed, and joined with `&`. This format is compatible with URL query strings and HTTP signing canonical requests.

The use of `BTreeMap` (which maintains sorted key order) rather than `HashMap` ensures deterministic iteration order by default.

## Quick Start

```rust
use canonical_form::{CanonicalForm, CanonicalOptions};
use std::collections::BTreeMap;

let cf = CanonicalForm::default();
let mut map = BTreeMap::new();
map.insert("Name".into(), "  Alice  ".into());
map.insert("Age".into(), "30".into());

cf.canonicalize(&mut map);
// map["Name"] is now "Alice" (trimmed)

let canonical = cf.to_canonical_string(&map);
// "Age=30&Name=Alice" (sorted, trimmed)
```

## API

- **`CanonicalOptions`** — sort_keys, trim_whitespace, lowercase_keys, remove_nulls
- **`CanonicalForm`** — Processor with `canonicalize(&mut map)` and `to_canonical_string(&map)`

## Architecture Notes

Provides the normalization layer for SuperInstance request signing and config deduplication. Used in API gateway middleware to produce canonical request representations for signature verification. See the [architecture overview](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## License

MIT
