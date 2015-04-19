extern crate hoedown;
extern crate glob;

use std::result::Result;
use std::fs::File;
use std::process::{Command, Stdio};
use glob::glob;

use std::path::PathBuf;

use std::io::{Read, Write};

use hoedown::Markdown;
use hoedown::renderer::html;

fn tidy(input: &str) -> String {
    let mut process =
        Command::new("tidy")
        .arg("--show-body-only").arg("1")
        .arg("--quiet").arg("1")
        .arg("--show-warnings").arg("0")
        .arg("-utf8")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    process.stdin.as_mut().unwrap().write(input.as_bytes()).unwrap();

    let output = process.wait_with_output().unwrap();
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn test_markdown() {
    for source in
        glob("libhoedown/test/MarkdownTest_1.0.3/Tests/*.text").unwrap()
        .filter_map(Result::ok)
        .chain(Some(PathBuf::from("libhoedown/test/Tests/Escape character.text")).into_iter())
        .chain(Some(PathBuf::from("tests/fixtures/unicode.txt")).into_iter()) {
        let doc = Markdown::read_from(File::open(&source).unwrap());
        let html = html::Html::new(html::Flags::empty(), 0);

        let target = source.with_extension("html");
        let mut target_contents = String::new();

        File::open(&target).unwrap().read_to_string(&mut target_contents).unwrap();

        let output = doc.render(html);

        let rendered_tidy = tidy(output.to_str().unwrap());
        let target_tidy = tidy(&target_contents[..]);

        assert_eq!(rendered_tidy, target_tidy);
    }
}

#[test]
fn test_math() {
    let source = PathBuf::from("libhoedown/test/Tests/Math.text");
    let doc = Markdown::read_from(File::open(&source).unwrap()).extensions(hoedown::MATH);
    let html = html::Html::new(html::Flags::empty(), 0);

    let target = source.with_extension("html");
    let mut target_contents = String::new();

    File::open(&target).unwrap().read_to_string(&mut target_contents).unwrap();

    let output = doc.render(html);

    let rendered_tidy = tidy(output.to_str().unwrap());
    let target_tidy = tidy(&target_contents[..]);

    assert_eq!(rendered_tidy, target_tidy);
}

#[test]
fn test_toc() {
    let source = PathBuf::from("libhoedown/test/Tests/Formatting in Table of Contents.text");
    let doc = Markdown::read_from(File::open(&source).unwrap());
    let renderer = html::Html::toc(3);

    let target = source.with_extension("html");
    let mut target_contents = String::new();

    File::open(&target).unwrap().read_to_string(&mut target_contents).unwrap();

    let output = doc.render(renderer);

    let rendered_tidy = tidy(output.to_str().unwrap());
    let target_tidy = tidy(&target_contents);

    assert_eq!(rendered_tidy, target_tidy);
}
