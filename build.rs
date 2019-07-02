use skeptic;
use std::path::PathBuf;

fn main() {
    // generates doc tests for guide.
    // skeptic::generate_doc_tests(&[
    //     "docs/guides/HelloGgez.md",
    // ]);

    let mut mdbook_files = skeptic::markdown_files_of_directory("docs/guides/");
    let other_files: Vec<PathBuf> = vec![
        "README.md".into(),
    ];
    mdbook_files.extend(other_files);
    skeptic::generate_doc_tests(&mdbook_files);
}
