fn main() {
    generate_tests_from_spec()
}

// If the "gen-tests" feature is absent,
// this function will be compiled down to nothing
#[cfg(not(feature = "gen-tests"))]
fn generate_tests_from_spec() {}

// If the feature is present, generate tests
// from any .txt file present in the specs/ directory
//
// Test cases are present in the files in the
// following format:
//
// ```````````````````````````````` example
// markdown
// .
// expected html output
// ````````````````````````````````
#[cfg(feature = "gen-tests")]
fn generate_tests_from_spec() {
    use std::fs::{self, File};
    use std::io::{Read, Write};
    use std::path::PathBuf;

    // This is a hardcoded path to the CommonMark spec because it is not situated in
    // the specs/ directory. It's in an array to easily chain it to the other iterator
    // and make it easy to eventually add other hardcoded paths in the future if needed
    let hardcoded = [
        "./third_party/CommonMark/spec.txt",
        "./third_party/CommonMark/smart_punct.txt",
        "./third_party/GitHub/gfm_table.txt",
        "./third_party/GitHub/gfm_strikethrough.txt",
        "./third_party/GitHub/gfm_tasklist.txt",
    ];
    let hardcoded_iter = hardcoded.iter().map(PathBuf::from);

    // Create an iterator over the files in the specs/ directory that have a .txt extension
    let mut spec_files = fs::read_dir("./specs")
        .expect("Could not find the 'specs' directory")
        .filter_map(Result::ok)
        .map(|d| d.path())
        .filter(|p| p.extension().map(|e| e.to_owned()).is_some())
        .chain(hardcoded_iter)
        .collect::<Vec<_>>();
    // Sort by spec names
    spec_files.sort_by(|p, q| p.file_stem().cmp(&q.file_stem()));
    let spec_files = spec_files;

    for file_path in &spec_files {
        let mut raw_spec = String::new();

        File::open(&file_path)
            .and_then(|mut f| f.read_to_string(&mut raw_spec))
            .expect("Could not read the spec file");

        let rs_test_file = PathBuf::from("./tests/suite/")
            .join(file_path.file_name().expect("Invalid filename"))
            .with_extension("rs");

        let mut spec_rs =
            File::create(&rs_test_file).expect(&format!("Could not create {:?}", rs_test_file));

        let spec_name = file_path.file_stem().unwrap().to_str().unwrap();

        let spec = Spec::new(&raw_spec);
        let mut n_tests = 0;

        spec_rs
            .write_all(b"// This file is auto-generated by the build script\n")
            .unwrap();
        spec_rs
            .write_all(b"// Please, do not modify it manually\n")
            .unwrap();
        spec_rs
            .write_all(b"\nuse super::test_markdown_html;\n")
            .unwrap();

        for (i, testcase) in spec.enumerate() {
            spec_rs
                .write_fmt(format_args!(
                    r###"
#[test]
fn {}_test_{i}() {{
    let original = r##"{original}"##;
    let expected = r##"{expected}"##;

    test_markdown_html(original, expected, {smart_punct}, {metadata_blocks}, {old_footnotes}, {subscript}, {wikilinks});
}}
"###,
                    spec_name,
                    i = i + 1,
                    original = testcase.original,
                    expected = testcase.expected,
                    smart_punct = testcase.smart_punct,
                    metadata_blocks = testcase.metadata_blocks,
                    old_footnotes = testcase.old_footnotes,
                    subscript = testcase.subscript,
                    wikilinks = testcase.wikilinks,
                ))
                .unwrap();

            n_tests += 1;
        }

        println!(
            "cargo:warning=Generated {} tests in {:?}",
            n_tests, rs_test_file
        );
    }

    // write mods to suite/mod.rs
    let suite_mod_file = PathBuf::from("./tests/suite/mod").with_extension("rs");

    let mut mod_rs =
        File::create(&suite_mod_file).expect(&format!("Could not create {:?}", &suite_mod_file));

    mod_rs
        .write_all(b"// This file is auto-generated by the build script\n")
        .unwrap();
    mod_rs
        .write_all(b"// Please, do not modify it manually\n")
        .unwrap();
    mod_rs
        .write_all(b"\npub use super::test_markdown_html;\n\n")
        .unwrap();

    for file_path in &spec_files {
        let mod_name = file_path.file_stem().unwrap().to_str().unwrap();
        mod_rs.write_all(b"mod ").unwrap();
        mod_rs.write_all(mod_name.as_bytes()).unwrap();
        mod_rs.write_all(b";\n").unwrap();
    }
}

#[cfg(feature = "gen-tests")]
pub struct Spec<'a> {
    spec: &'a str,
}

#[cfg(feature = "gen-tests")]
impl<'a> Spec<'a> {
    pub fn new(spec: &'a str) -> Self {
        Spec { spec }
    }
}

#[cfg(feature = "gen-tests")]
pub struct TestCase {
    pub original: String,
    pub expected: String,
    pub smart_punct: bool,
    pub metadata_blocks: bool,
    pub old_footnotes: bool,
    pub subscript: bool,
    pub wikilinks: bool,
}

#[cfg(feature = "gen-tests")]
impl<'a> Iterator for Spec<'a> {
    type Item = TestCase;

    fn next(&mut self) -> Option<TestCase> {
        let spec = self.spec;
        let prefix = "```````````````````````````````` example";

        let (i_start, smart_punct, metadata_blocks, old_footnotes, subscript, wikilinks) =
            self.spec.find(prefix).and_then(|pos| {
                let smartpunct_suffix = "_smartpunct\n";
                let metadata_blocks_suffix = "_metadata_blocks\n";
                let old_footnotes_suffix = "_old_footnotes\n";
                let super_sub_suffix = "_super_sub\n";
                let wikilinks_suffix = "_wikilinks\n";
                if spec[(pos + prefix.len())..].starts_with(smartpunct_suffix) {
                    Some((
                        pos + prefix.len() + smartpunct_suffix.len(),
                        true,
                        false,
                        false,
                        false,
                        false,
                    ))
                } else if spec[(pos + prefix.len())..].starts_with(metadata_blocks_suffix) {
                    Some((
                        pos + prefix.len() + metadata_blocks_suffix.len(),
                        false,
                        true,
                        false,
                        false,
                        false,
                    ))
                } else if spec[(pos + prefix.len())..].starts_with(old_footnotes_suffix) {
                    Some((
                        pos + prefix.len() + old_footnotes_suffix.len(),
                        false,
                        false,
                        true,
                        false,
                        false,
                    ))
                } else if spec[(pos + prefix.len())..].starts_with(super_sub_suffix) {
                    Some((
                        pos + prefix.len() + super_sub_suffix.len(),
                        false,
                        false,
                        false,
                        true,
                        false,
                    ))
                } else if spec[(pos + prefix.len())..].starts_with(wikilinks_suffix) {
                    Some((
                        pos + prefix.len() + wikilinks_suffix.len(),
                        false,
                        false,
                        false,
                        false,
                        true,
                    ))
                } else if spec[(pos + prefix.len())..].starts_with('\n') {
                    Some((pos + prefix.len() + 1, false, false, false, false, false))
                } else {
                    None
                }
            })?;

        let i_end = self.spec[i_start..]
            .find("\n.\n")
            .map(|pos| (pos + 1) + i_start)?;

        let e_end = self.spec[i_end + 2..]
            .find("````````````````````````````````\n")
            .map(|pos| pos + i_end + 2)?;

        self.spec = &self.spec[e_end + 33..];

        let test_case = TestCase {
            original: spec[i_start..i_end].to_string().replace("→", "\t"),
            expected: spec[i_end + 2..e_end].to_string().replace("→", "\t"),
            smart_punct,
            metadata_blocks,
            old_footnotes,
            subscript,
            wikilinks,
        };

        Some(test_case)
    }
}
