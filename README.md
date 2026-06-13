# canonical-form

A Rust library for **canonicalization of structured data** — transforming key-value maps into deterministic, normalized representations with sorted keys, trimmed whitespace, optional lowercasing, and null removal. Produces stable string encodings suitable for hashing, signing, and content-addressed deduplication.

## Why It Matters

Canonicalization is the bedrock of **deterministic data comparison**. Without a canonical form:

- Two semantically identical JSON objects with different key ordering produce different hashes
- Whitespace differences break signature verification (a real OAuth 1.0a pain point)
- Case mismatches cause cache misses in content-addressed storage

Real-world applications:

- **HTTP signature specs** — draft-cavage-http-signatures requires canonical string construction
- **JSON Canonicalization (RFC 8785)** — JCS for JSON Web Signatures
- **Git tree hashing** — entries must be sorted for reproducible SHA-1s
- **Distributed dedup** — consistent hashing requires identical inputs produce identical keys

## How It Works

### Normalization Pipeline

The canonicalizer applies a configurable sequence of transforms:

```
Input Map → [remove_nulls?] → [trim_whitespace?] → [lowercase_keys?] → [sort_keys?] → Output
```

Each transform is O(n) over the map entries:

| Transform | Complexity | Implementation |
|-----------|------------|----------------|
| `remove_nulls` | O(n) | `BTreeMap::retain` |
| `trim_whitespace` | O(n · L) | In-place `str::trim` per value |
| `lowercase_keys` | O(n · K) | Rebuild map with `String::to_lowercase` |
| `sort_keys` | O(n log n) | `BTreeMap` maintains sort invariant |

### Canonical String Encoding

The `to_canonical_string()` method produces a query-string-style encoding:

$$\text{canon} = \text{sort}(\{k_i = v_i\}) \cdot \text{join}(\text{"&"})$$

Example: `{"b":"2","a":"1"}` → `a=1&b=2`

This is a simplified variant of the **URL query canonicalization** used in AWS Signature Version 4 and OAuth 1.0a.

### Complexity Analysis

| Operation | Time | Space |
|-----------|------|-------|
| `canonicalize(map)` | O(n · L_max) | O(1) in-place |
| `to_canonical_string(map)` | O(n log n + n · L) | O(n · L) |
| Full pipeline | O(n log n) | O(n) |

Where n = number of keys, L = average value length.

### BTreeMap as Implicit Sorting

Using `BTreeMap<String, String>` means keys are always maintained in lexicographic order — `sort_keys` is effectively O(1) on iteration since the data structure already guarantees ordering. This is why `sort_keys` is enabled by default.

## Quick Start

```rust
use std::collections::BTreeMap;
use canonical_form::{CanonicalForm, CanonicalOptions};

let cf = CanonicalForm::default();
let mut map = BTreeMap::new();
map.insert("Name".into(), "  Alice  ".into());
map.insert("Age".into(), "30".into());

cf.canonicalize(&mut map);
assert_eq!(map.get("Name").unwrap(), "Alice"); // trimmed

let encoded = cf.to_canonical_string(&map);
// "Age=30&Name=Alice"
```

## API

| Type | Method | Description |
|------|--------|-------------|
| `CanonicalOptions` | `sort_keys`, `trim_whitespace`, `lowercase_keys`, `remove_nulls` | Configurable flags |
| `CanonicalForm::new(CanonicalOptions)` | | Create with options |
| `canonicalize(&mut BTreeMap)` | | In-place normalization |
| `to_canonical_string(&BTreeMap) → String` | | Deterministic encoding |

## Architecture Notes

The **γ + η = C** link: the normalization transforms (γ) map the input to a reduced representation, while the `BTreeMap` ordering invariant (η) ensures deterministic key emission. Together they conserve the canonical invariant C — for any two semantically equivalent inputs, the canonical string output is byte-identical. This is the mathematical definition of a canonical form: a function $f$ where $x \equiv y \iff f(x) = f(y)$.

## References

- RFC 8785 (2020). *JSON Canonicalization Scheme (JCS).* A. Rundgren, B. Jordan, S. Erdtman.
- RFC 5849 (2010). *The OAuth 1.0 Protocol.* Section 3.4.1.3.2: Parameters Normalization.
- AWS Signature Version 4: *Canonical Request.* AWS Documentation.
- Deutsch, P. (1996). *DEFLATE Compressed Data Format Specification.* RFC 1951. (Canonical Huffman codes.)
- Chaum, D. (1985). *Security Without Identification: Transaction Systems to Make Big Brother Obsolete.* (Digital signatures require canonicalization.)

## License

MIT
