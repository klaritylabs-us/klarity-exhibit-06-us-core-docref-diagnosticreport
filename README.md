# US Core DocumentReference & DiagnosticReport (FHIR R4) — Rust

Small library and CLI that sanity-check **US Core** clinical document JSON: **DocumentReference** (`status`, `type`, `subject`, `content` attachment) and **DiagnosticReport** (LOINC `code`, `subject`, effective time, `result` references).

Synthetic fixtures only. Not ONC certification, not a substitute for the HL7 FHIR Validator or Inferno, and not legal/clinical advice.

Maintained by [Klarity Labs](https://klaritylabs.ai/) as open reference code for U.S. healthcare FHIR interoperability work.

**Exhibit 06** — [FHIR exhibit catalog](https://github.com/klaritylabs-us/klarity-fhir-exhibits) · [klaritylabs-us](https://github.com/klaritylabs-us)


## How this relates to other tools

| Tool class | Role |
|------------|------|
| **HL7 FHIR Validator**, **HAPI**, **Firely** | Full validation, servers, SDKs, IG packages |
| **Inferno**, **Touchstone**, vendor cert suites | **Conformance** against test plans |
| **This exhibit** | **Pre-flight construction** on synthetic fixtures—catch shape mistakes before wire traffic |

Conformance tools test whether your **API behaves**; exhibits focus on whether your **payloads and pipes are built** to survive real partners.

## Specifications cited

- [US Core DocumentReference](http://hl7.org/fhir/us/core/STU6.1/StructureDefinition-us-core-documentreference.html)
- [US Core DiagnosticReport](http://hl7.org/fhir/us/core/STU6.1/StructureDefinition-us-core-diagnosticreport.html)

## What it checks

**DocumentReference**

- `meta.profile` includes [US Core DocumentReference](http://hl7.org/fhir/us/core/STU6.1/StructureDefinition-us-core-documentreference.html)
- `status`, `subject.reference`, `type.coding`, and non-empty `content` with attachment metadata or URL

**DiagnosticReport**

- `meta.profile` includes [US Core DiagnosticReport](http://hl7.org/fhir/us/core/STU6.1/StructureDefinition-us-core-diagnosticreport.html)
- `status`, LOINC on `code`, `subject.reference`
- Warnings if `effectiveDateTime` / `effectivePeriod` or `result` missing when exchanging lab context

| Problem | What breaks |
|--------|-------------|
| **DocRef without content** | Index points at nothing — common in C-CDA conversion |
| **Missing document type** | Cannot route imaging vs clinical note |
| **DiagnosticReport without LOINC** | US Core binding failure on lab/report panels |
| **Results not linked** | Observations exist but `result` references omitted |

## Run it

Needs [Rust](https://www.rust-lang.org/tools/install).

```bash
cargo test
cargo run --bin validate-clinical-doc -- examples/valid-document-reference.json
cargo run --bin validate-clinical-doc -- examples/invalid-docref-no-content.json
```

Exit `0` if no errors, `1` if validation failed, `2` for bad usage or JSON.

## Sample output

One command:

```bash
cargo run --bin validate-clinical-doc -- examples/valid-diagnostic-report.json
```

**Pass** (`examples/valid-diagnostic-report.json`) — exit code `0`:

```json
[]
```

**Fail** (`examples/invalid-docref-no-content.json`) — exit code `1` (issues array; more entries may appear):

```json
[
  {
    "severity": "error",
    "path": "DocumentReference.content",
    "detail": "content with attachment or reference is required",
    "rule_id": "content-required"
  }
]
```


## Layout

- `src/validate.rs` — rules
- `examples/` — valid and broken payloads for tests and demos

## Inferno vs this repo

Inferno exercises your **API** against CMS-style scenarios. This repo is about **document pipeline shape** — USCDI labs and imaging narrative gaps that persist after a green API run.

## License

MIT — see [LICENSE](./LICENSE).

**Need this on your payloads or API?** 20-minute FHIR Scan — sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)

## Contact

sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)
