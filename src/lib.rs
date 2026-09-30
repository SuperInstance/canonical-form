//! canonical-form
//! Utilities for transforming data into canonical representations — normalization, sorting, hashing.

use std::collections::BTreeMap;

/// Options controlling canonicalization behavior.
#[derive(Debug, Clone)]
pub struct CanonicalOptions {
    pub sort_keys: bool,
    pub trim_whitespace: bool,
    pub lowercase_keys: bool,
    pub remove_nulls: bool,
}

impl Default for CanonicalOptions {
    fn default() -> Self {
        Self {
            sort_keys: true,
            trim_whitespace: true,
            lowercase_keys: false,
            remove_nulls: false,
        }
    }
}

/// A canonical form processor for key-value maps.
pub struct CanonicalForm {
    options: CanonicalOptions,
}

impl CanonicalForm {
    pub fn new(options: CanonicalOptions) -> Self {
        Self { options }
    }

    /// Canonicalize a map of string key-value pairs.
    pub fn canonicalize(&self, input: &mut BTreeMap<String, String>) {
        if self.options.remove_nulls {
            input.retain(|_, v| !v.is_empty());
        }
        if self.options.trim_whitespace {
            for v in input.values_mut() {
                *v = v.trim().to_string();
            }
        }
        if self.options.lowercase_keys {
            let updated: BTreeMap<String, String> = std::mem::take(input)
                .into_iter()
                .map(|(k, v)| (k.to_lowercase(), v))
                .collect();
            *input = updated;
        }
    }

    /// Produce a deterministic string representation.
    pub fn to_canonical_string(&self, input: &BTreeMap<String, String>) -> String {
        let mut pairs: Vec<String> = input
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();
        if self.options.sort_keys {
            pairs.sort();
        }
        pairs.join("&")
    }
}

impl Default for CanonicalForm {
    fn default() -> Self {
        Self::new(CanonicalOptions::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_canonicalization() {
        let cf = CanonicalForm::default();
        let mut map = BTreeMap::new();
        map.insert("Name".into(), "  Alice  ".into());
        map.insert("Age".into(), "30".into());
        cf.canonicalize(&mut map);
        assert_eq!(map.get("Name").unwrap(), "Alice");
    }
}

/// FNV-1a 64 — the digest every substrate in the SuperInstance fleet agrees on.
pub const FNV_OFFSET: u64 = 0xcbf29ce484222325;
pub const FNV_PRIME: u64 = 0x100000001b3;

#[inline]
pub fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h = FNV_OFFSET;
    for &b in bytes {
        h = (h ^ b as u64).wrapping_mul(FNV_PRIME);
    }
    h
}

/// True if this crate's FNV-1a still agrees with the rest of the fleet.
pub fn canary_holds() -> bool {
    fnv1a64("café Δ 日本語".as_bytes()) == 0x024a555471370b18d
}
