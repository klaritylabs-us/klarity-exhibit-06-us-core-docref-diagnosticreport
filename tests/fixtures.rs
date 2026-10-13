use std::fs;
use std::path::PathBuf;

use us_core_clinical_docs::validate_resource_json;

fn fixture(name: &str) -> String {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("examples");
    path.push(name);
    fs::read_to_string(path).expect("fixture")
}

#[test]
fn valid_document_reference() {
    let report = validate_resource_json(&fixture("valid-document-reference.json")).unwrap();
    assert!(report.is_valid(), "{:?}", report.issues);
}

#[test]
fn valid_diagnostic_report() {
    let report = validate_resource_json(&fixture("valid-diagnostic-report.json")).unwrap();
    assert!(report.is_valid(), "{:?}", report.issues);
}

#[test]
fn docref_without_content_fails() {
    let report = validate_resource_json(&fixture("invalid-docref-no-content.json")).unwrap();
    assert!(!report.is_valid());
}
