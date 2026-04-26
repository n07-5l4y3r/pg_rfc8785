use pgrx::prelude::*;
use serde_json::Value;

pgrx::pg_module_magic!();

/// Canonicalize a JSON text document according to RFC 8785 / JCS.
///
/// This function intentionally accepts text rather than json/jsonb so callers can decide
/// their own input path, e.g. `payload::jsonb::text`.
#[pg_extern(immutable, strict, parallel_safe, name = "rfc8785_canonicalize")]
fn rfc8785_canonicalize(input: &str) -> Result<String, String> {
    canonicalize(input)
}

/// Returns true if the input JSON text is already byte-for-byte RFC 8785 canonical.
#[pg_extern(immutable, strict, parallel_safe, name = "rfc8785_is_canonical")]
fn rfc8785_is_canonical(input: &str) -> Result<bool, String> {
    let canonical = canonicalize(input)?;
    Ok(input == canonical)
}

fn canonicalize(input: &str) -> Result<String, String> {
    let value: Value = serde_json::from_str(input)
        .map_err(|e| format!("invalid JSON input: {e}"))?;

    serde_json_canonicalizer::to_string(&value)
        .map_err(|e| format!("RFC 8785 canonicalization failed: {e}"))
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use super::*;

    #[pg_test]
    fn canonicalizes_object_key_order() {
        let got = rfc8785_canonicalize(r#"{"b":2,"a":1}"#).unwrap();
        assert_eq!(got, r#"{"a":1,"b":2}"#);
    }

    #[pg_test]
    fn canonicalizes_nested_values() {
        let got = rfc8785_canonicalize(r#"{"z":[3,2,1],"a":{"b":true,"a":null}}"#).unwrap();
        assert_eq!(got, r#"{"a":{"a":null,"b":true},"z":[3,2,1]}"#);
    }

    #[pg_test]
    fn detects_canonical_text() {
        assert!(rfc8785_is_canonical(r#"{"a":1,"b":2}"#).unwrap());
        assert!(!rfc8785_is_canonical(r#"{"b":2,"a":1}"#).unwrap());
    }
}
