#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;
#[cfg(test)]
extern crate glob;
extern crate semver;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate url;
extern crate url_serde;

use std::fs::File;
use std::io::Read;
use std::path::Path;

pub mod v1_2;
pub mod v1_3;
pub use crate::errors::{Result, ResultExt};

const MINIMUM_HAR12_VERSION: &str = ">= 1.2";

/// Errors that HAR functions may return
pub mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
            Yaml(::serde_yaml::Error);
            Serialize(::serde_json::Error);
            SemVerError(::semver::SemVerError);
        }

        errors {
            /// Deprecated - not generated anymore.
            UnsupportedSpecFileVersion(version: ::semver::Version) {
                description("Unsupported HAR file version")
                display("Unsupported HAR file version ({}). Expected {}", version, crate::MINIMUM_HAR12_VERSION)
            }
        }
    }
}

/// Supported versions of HAR.
///
/// Note that point releases require adding here (as they must other wise they wouldn't need a new version)
/// Using untagged can avoid that but the errors on incompatible documents become super hard to debug.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "version")]
pub enum Spec {
    /// Version 1.2 of the HAR specification.
    ///
    /// Refer to the official
    /// [specification](https://w3c.github.io/web-performance/specs/HAR/Overview.html)
    /// for more information.
    #[allow(non_camel_case_types)]
    #[serde(rename = "1.2")]
    V1_2(v1_2::Log),

    // Version 1.3 of the HAR specification.
    //
    // Refer to the draft
    // [specification](https://github.com/ahmadnassri/har-spec/blob/master/versions/1.3.md)
    // for more information.
    #[allow(non_camel_case_types)]
    #[serde(rename = "1.3")]
    V1_3(v1_3::Log),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Har {
    pub log: Spec,
}

/// Deserialize a HAR from a path
pub fn from_path<P>(path: P) -> errors::Result<Har>
where
    P: AsRef<Path>,
{
    from_reader(File::open(path)?)
}

/// Deserialize a HAR from type which implements Read
pub fn from_reader<R>(read: R) -> errors::Result<Har>
where
    R: Read,
{
    Ok(serde_yaml::from_reader::<R, Har>(read)?)
}

/// Serialize HAR spec to a YAML string
pub fn to_yaml(spec: &Har) -> errors::Result<String> {
    Ok(serde_yaml::to_string(spec)?)
}

/// Serialize HAR spec to JSON string
pub fn to_json(spec: &Har) -> errors::Result<String> {
    Ok(serde_json::to_string_pretty(spec)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use glob::glob;
    use std::fs::File;
    use std::io::Write;

    const FIXTURES_GLOB: &str = "tests/fixtures/*.har";

    /// Helper function for reading a file to string.
    fn read_file<P>(path: P) -> String
    where
        P: AsRef<Path>,
    {
        let mut f = File::open(path).unwrap();
        let mut content = String::new();
        f.read_to_string(&mut content).unwrap();
        content
    }

    /// Helper function to write string to file.
    fn write_to_file<P>(path: P, filename: &str, data: &str)
    where
        P: AsRef<Path> + std::fmt::Debug,
    {
        println!("    Saving string to {:?}...", path);
        std::fs::create_dir_all(&path).unwrap();
        let full_filename = path.as_ref().to_path_buf().join(filename);
        let mut f = File::create(&full_filename).unwrap();
        f.write_all(data.as_bytes()).unwrap();
    }

    /// Convert a YAML `&str` to a JSON `String`.
    fn convert_yaml_str_to_json(yaml_str: &str) -> String {
        let yaml: serde_yaml::Value = serde_yaml::from_str(yaml_str).unwrap();
        let json: serde_json::Value = serde_yaml::from_value(yaml).unwrap();
        serde_json::to_string_pretty(&json).unwrap()
    }

    /// Deserialize and re-serialize the input file to a JSON string through two different
    /// paths, comparing the result.
    /// 1. File -> `String` -> `serde_yaml::Value` -> `serde_json::Value` -> `String`
    /// 2. File -> `Spec` -> `serde_json::Value` -> `String`
    /// Both conversion of `serde_json::Value` -> `String` are done
    /// using `serde_json::to_string_pretty`.
    /// Since the first conversion is independent of the current crate (and only
    /// uses serde's json and yaml support), no information should be lost in the final
    /// JSON string. The second conversion goes through our `Har`, so the final JSON
    /// string is a representation of _our_ implementation.
    /// By comparing those two JSON conversions, we can validate our implementation.
    fn compare_spec_through_json(
        input_file: &Path,
        save_path_base: &Path,
    ) -> (String, String, String) {
        // First conversion:
        //     File -> `String` -> `serde_yaml::Value` -> `serde_json::Value` -> `String`

        // Read the original file to string
        let spec_yaml_str = read_file(&input_file);
        // Convert YAML string to JSON string
        let spec_json_str = convert_yaml_str_to_json(&spec_yaml_str);

        // Second conversion:
        //     File -> `Spec` -> `serde_json::Value` -> `String`

        // Parse the input file
        let parsed_spec = from_path(&input_file).unwrap();
        // Convert to serde_json::Value
        let parsed_spec_json: serde_json::Value = serde_json::to_value(parsed_spec).unwrap();
        // Convert to a JSON string
        let parsed_spec_json_str: String = serde_json::to_string_pretty(&parsed_spec_json).unwrap();

        // Save JSON strings to file
        let api_filename = input_file
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .replace(".yaml", ".json");

        let mut save_path = save_path_base.to_path_buf();
        save_path.push("yaml_to_json");
        write_to_file(&save_path, &api_filename, &spec_json_str);

        let mut save_path = save_path_base.to_path_buf();
        save_path.push("yaml_to_spec_to_json");
        write_to_file(&save_path, &api_filename, &parsed_spec_json_str);

        // Return the JSON filename and the two JSON strings
        (api_filename, parsed_spec_json_str, spec_json_str)
    }

    // Makes sure the paths to the test fixtures works on this platform
    #[test]
    fn can_find_test_fixtures() {
        let fixtures: Vec<std::result::Result<std::path::PathBuf, glob::GlobError>> =
            glob(FIXTURES_GLOB)
                .expect("Failed to read glob pattern")
                .filter(|e| e.is_ok())
                .collect();
        assert_ne!(0, fixtures.len());
    }

    // Just tests if the deserialization does not blow up. But does not test correctness
    #[test]
    fn can_deserialize() {
        for entry in glob(FIXTURES_GLOB).expect("Failed to read glob pattern") {
            let entry = entry.unwrap();
            let path = entry.as_path();
            // cargo test -- --nocapture to see this message
            println!("Testing if {:?} is deserializable", path);
            from_path(path).unwrap();
        }
    }

    #[test]
    fn can_deserialize_and_reserialize() {
        let save_path_base: std::path::PathBuf =
            ["target", "tests", "can_deserialize_and_reserialize"]
                .iter()
                .collect();
        let mut invalid_diffs = Vec::new();

        for entry in glob(FIXTURES_GLOB).expect("Failed to read glob pattern") {
            let entry = entry.unwrap();
            let path = entry.as_path();

            println!("Testing if {:?} is deserializable", path);

            let (api_filename, parsed_spec_json_str, spec_json_str) =
                compare_spec_through_json(&path, &save_path_base);

            if parsed_spec_json_str != spec_json_str {
                invalid_diffs.push((
                    api_filename,
                    parsed_spec_json_str.clone(),
                    spec_json_str.clone(),
                ));
                File::create(path.with_extension("parsed"))
                    .unwrap()
                    .write_all(parsed_spec_json_str.as_bytes())
                    .unwrap();
                File::create(path.with_extension("pretty"))
                    .unwrap()
                    .write_all(spec_json_str.as_bytes())
                    .unwrap();
            }
        }

        for invalid_diff in &invalid_diffs {
            println!("File {} failed JSON comparison!", invalid_diff.0);
        }
        assert_eq!(invalid_diffs.len(), 0);
    }
}
