use har::{Error, HarVersion, Spec, from_path, from_slice, from_str};
use std::fs;
use std::path::{Path, PathBuf};

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

fn fixture_paths() -> Vec<PathBuf> {
    let mut fixtures = fs::read_dir(fixtures_dir())
        .expect("failed to read fixture directory")
        .map(|entry| entry.expect("failed to read fixture entry").path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "har"))
        .collect::<Vec<_>>();

    fixtures.sort();
    fixtures
}

fn read_file(path: &Path) -> String {
    fs::read_to_string(path).expect("failed to read fixture file")
}

fn write_to_file(path: &Path, filename: &str, data: &str) {
    fs::create_dir_all(path).expect("failed to create test output directory");
    fs::write(path.join(filename), data).expect("failed to write test output");
}

fn normalize_json_str(json_str: &str) -> String {
    let json: serde_json::Value = serde_json::from_str(json_str).expect("fixture should be JSON");
    serde_json::to_string_pretty(&json).expect("failed to normalize fixture JSON")
}

fn compare_har_through_json(input_file: &Path, save_path_base: &Path) -> (String, String, String) {
    let source_json_str = normalize_json_str(&read_file(input_file));

    let parsed_har = from_path(input_file).expect("fixture should deserialize");
    let parsed_har_json: serde_json::Value =
        serde_json::to_value(parsed_har).expect("failed to convert HAR to JSON value");
    let parsed_har_json_str =
        serde_json::to_string_pretty(&parsed_har_json).expect("failed to pretty-print HAR JSON");

    let fixture_filename = input_file
        .file_name()
        .expect("fixture should have a file name")
        .to_string_lossy()
        .replace(".har", ".json");

    write_to_file(
        &save_path_base.join("json_to_json"),
        &fixture_filename,
        &source_json_str,
    );
    write_to_file(
        &save_path_base.join("json_to_har_to_json"),
        &fixture_filename,
        &parsed_har_json_str,
    );

    (fixture_filename, parsed_har_json_str, source_json_str)
}

fn expected_version(path: &Path) -> HarVersion {
    match path
        .file_name()
        .expect("fixture should have a file name")
        .to_string_lossy()
        .as_ref()
    {
        "someapi123.har" => HarVersion::V1_2,
        "someapi13.har" => HarVersion::V1_3,
        other => panic!("unexpected fixture {other}"),
    }
}

#[test]
fn can_find_test_fixtures() {
    let fixtures = fixture_paths();
    assert!(
        !fixtures.is_empty(),
        "expected at least one .har fixture in {}",
        fixtures_dir().display()
    );
}

#[test]
fn fixtures_deserialize_to_expected_versions() {
    for fixture in fixture_paths() {
        let har = from_path(&fixture).expect("fixture should deserialize");
        let expected_version = expected_version(&fixture);

        assert_eq!(
            har.version(),
            expected_version,
            "fixture {}",
            fixture.display()
        );

        match (expected_version, &har.log) {
            (HarVersion::V1_2, Spec::V1_2(_)) | (HarVersion::V1_3, Spec::V1_3(_)) => {}
            _ => panic!(
                "fixture {} deserialized to the wrong spec",
                fixture.display()
            ),
        }
    }
}

#[test]
fn from_slice_matches_from_path() {
    for fixture in fixture_paths() {
        let bytes = fs::read(&fixture).expect("failed to read fixture bytes");
        let from_bytes = from_slice(&bytes).expect("fixture bytes should deserialize");
        let from_disk = from_path(&fixture).expect("fixture path should deserialize");
        assert_eq!(from_bytes, from_disk, "fixture {}", fixture.display());
    }
}

#[test]
fn from_str_matches_from_path() {
    for fixture in fixture_paths() {
        let content = read_file(&fixture);
        let from_text = from_str(&content).expect("fixture string should deserialize");
        let from_disk = from_path(&fixture).expect("fixture path should deserialize");
        assert_eq!(from_text, from_disk, "fixture {}", fixture.display());
    }
}

#[test]
fn rejects_missing_log_version() {
    let input = r#"{
      "log": {
        "creator": { "name": "example", "version": "1.0" },
        "entries": []
      }
    }"#;

    let error = from_str(input).expect_err("missing version should fail");
    assert!(matches!(error, Error::MissingVersion));
}

#[test]
fn rejects_unsupported_log_version() {
    let input = r#"{
      "log": {
        "version": "1.4",
        "creator": { "name": "example", "version": "1.0" },
        "entries": []
      }
    }"#;

    let error = from_str(input).expect_err("unsupported version should fail");
    assert!(matches!(error, Error::UnsupportedVersion(version) if version == "1.4"));
}

#[test]
fn can_deserialize_and_reserialize() {
    let save_path_base =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("target/tests/can_deserialize_and_reserialize");
    let mut invalid_diffs = Vec::new();

    for fixture in fixture_paths() {
        let (fixture_filename, parsed_har_json_str, source_json_str) =
            compare_har_through_json(&fixture, &save_path_base);

        if parsed_har_json_str != source_json_str {
            invalid_diffs.push((fixture_filename, parsed_har_json_str, source_json_str));
        }
    }

    for invalid_diff in &invalid_diffs {
        println!("File {} failed JSON comparison!", invalid_diff.0);
    }

    assert_eq!(invalid_diffs.len(), 0);
}
