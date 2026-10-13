use serde_json::Value;

use crate::outcome::{ValidationIssue, ValidationReport};
use crate::profile::{
    LOINC_SYSTEM, US_CORE_DIAGNOSTIC_REPORT_PROFILE, US_CORE_DOCREF_PROFILE,
};

pub fn validate_resource_json(json: &str) -> Result<ValidationReport, String> {
    let value: Value = serde_json::from_str(json).map_err(|e| format!("JSON parse: {e}"))?;
    Ok(validate_resource_value(&value))
}

pub fn validate_resource_value(value: &Value) -> ValidationReport {
    match value.get("resourceType").and_then(Value::as_str) {
        Some("DocumentReference") => validate_document_reference(value),
        Some("DiagnosticReport") => validate_diagnostic_report(value),
        Some(other) => {
            let mut report = ValidationReport::ok();
            report.push(ValidationIssue::error(
                "resourceType",
                format!("expected DocumentReference or DiagnosticReport, got {}", other),
                "resource-type",
            ));
            report
        }
        None => {
            let mut report = ValidationReport::ok();
            report.push(ValidationIssue::error(
                "resourceType",
                "missing resourceType",
                "resource-type",
            ));
            report
        }
    }
}

fn validate_document_reference(doc: &Value) -> ValidationReport {
    let mut report = ValidationReport::ok();

    let has_profile = doc
        .pointer("/meta/profile")
        .and_then(Value::as_array)
        .is_some_and(|a| {
            a.iter()
                .filter_map(Value::as_str)
                .any(|p| p == US_CORE_DOCREF_PROFILE)
        });
    if !has_profile {
        report.push(ValidationIssue::error(
            "DocumentReference.meta.profile",
            format!("meta.profile should include {}", US_CORE_DOCREF_PROFILE),
            "us-core-meta-profile",
        ));
    }

    if doc.get("status").and_then(Value::as_str).is_none() {
        report.push(ValidationIssue::error(
            "DocumentReference.status",
            "status is required",
            "status-required",
        ));
    }

    if doc
        .pointer("/subject/reference")
        .and_then(Value::as_str)
        .is_none_or(|r| r.is_empty())
    {
        report.push(ValidationIssue::error(
            "DocumentReference.subject",
            "subject.reference is required",
            "subject-required",
        ));
    }

    let has_type = doc
        .pointer("/type/coding")
        .and_then(Value::as_array)
        .is_some_and(|c| !c.is_empty());
    if !has_type {
        report.push(ValidationIssue::error(
            "DocumentReference.type",
            "type.coding is required for document indexing",
            "type-required",
        ));
    }

    let content = doc.get("content").and_then(Value::as_array);
    if content.is_none() || content.is_some_and(|c| c.is_empty()) {
        report.push(ValidationIssue::error(
            "DocumentReference.content",
            "content with attachment or reference is required",
            "content-required",
        ));
    } else if let Some(items) = content {
        let any_payload = items.iter().any(|item| {
            item.get("attachment").is_some() || item.pointer("/attachment/url").is_some()
        });
        if !any_payload {
            report.push(ValidationIssue::error(
                "DocumentReference.content",
                "content should include attachment metadata or URL",
                "content-attachment",
            ));
        }
    }

    report
}

fn validate_diagnostic_report(dr: &Value) -> ValidationReport {
    let mut report = ValidationReport::ok();

    let has_profile = dr
        .pointer("/meta/profile")
        .and_then(Value::as_array)
        .is_some_and(|a| {
            a.iter()
                .filter_map(Value::as_str)
                .any(|p| p == US_CORE_DIAGNOSTIC_REPORT_PROFILE)
        });
    if !has_profile {
        report.push(ValidationIssue::error(
            "DiagnosticReport.meta.profile",
            format!("meta.profile should include {}", US_CORE_DIAGNOSTIC_REPORT_PROFILE),
            "us-core-meta-profile",
        ));
    }

    if dr.get("status").and_then(Value::as_str).is_none() {
        report.push(ValidationIssue::error(
            "DiagnosticReport.status",
            "status is required",
            "status-required",
        ));
    }

    let has_loinc = dr
        .pointer("/code/coding")
        .and_then(Value::as_array)
        .is_some_and(|arr| {
            arr.iter().any(|c| {
                c.get("system").and_then(Value::as_str) == Some(LOINC_SYSTEM)
                    && c.get("code").and_then(Value::as_str).is_some()
            })
        });
    if !has_loinc {
        report.push(ValidationIssue::error(
            "DiagnosticReport.code",
            "code needs LOINC for US Core DiagnosticReport",
            "code-loinc",
        ));
    }

    if dr
        .pointer("/subject/reference")
        .and_then(Value::as_str)
        .is_none_or(|r| r.is_empty())
    {
        report.push(ValidationIssue::error(
            "DiagnosticReport.subject",
            "subject.reference is required",
            "subject-required",
        ));
    }

    if dr.get("effectiveDateTime").is_none() && dr.get("effectivePeriod").is_none() {
        report.push(ValidationIssue::warning(
            "DiagnosticReport.effective",
            "effectiveDateTime or effectivePeriod expected for lab/report context",
            "effective-recommended",
        ));
    }

    if dr.get("result").and_then(Value::as_array).is_none() {
        report.push(ValidationIssue::warning(
            "DiagnosticReport.result",
            "result references to Observations are Must Support when results are exchanged",
            "result-recommended",
        ));
    }

    report
}
