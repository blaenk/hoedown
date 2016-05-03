#[macro_use]
extern crate hoedown;
extern crate glob;

use std::result::Result;
use std::fs::File;
use std::process::{Command, Stdio};
use glob::glob;

use std::path::PathBuf;

use std::io::{Read, Write};

use hoedown::{Markdown, Render, Wrapper};
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
        let mut html = html::Html::new(html::Flags::empty(), 0);

        let target = source.with_extension("html");
        let mut target_contents = String::new();

        File::open(&target).unwrap().read_to_string(&mut target_contents).unwrap();

        let output = html.render(&doc);

        let rendered_tidy = tidy(output.to_str().unwrap());
        let target_tidy = tidy(&target_contents[..]);

        assert_eq!(rendered_tidy, target_tidy);
    }
}

#[test]
fn test_wrapper() {
    struct HtmlWrapper {
        html: html::Html,
    }

    wrap!(HtmlWrapper);

    impl Wrapper for HtmlWrapper {
        type Base = html::Html;

        #[inline(always)]
        fn base(&mut self) -> &mut html::Html {
            &mut self.html
        }
    }

    for source in
        glob("libhoedown/test/MarkdownTest_1.0.3/Tests/*.text").unwrap()
        .filter_map(Result::ok)
        .chain(Some(PathBuf::from("libhoedown/test/Tests/Escape character.text")).into_iter())
        .chain(Some(PathBuf::from("tests/fixtures/unicode.txt")).into_iter()) {
        let doc = Markdown::read_from(File::open(&source).unwrap());
        let mut wrapper = HtmlWrapper { html: html::Html::new(html::Flags::empty(), 0) };

        let target = source.with_extension("html");
        let mut target_contents = String::new();

        File::open(&target).unwrap().read_to_string(&mut target_contents).unwrap();

        let output = wrapper.render(&doc);

        let rendered_tidy = tidy(output.to_str().unwrap());
        let target_tidy = tidy(&target_contents[..]);

        assert_eq!(rendered_tidy, target_tidy);
    }
}

#[test]
fn test_math() {
    let source = PathBuf::from("libhoedown/test/Tests/Math.text");
    let doc =
        Markdown::read_from(File::open(&source).unwrap())
        .extensions(hoedown::MATH);
    let mut html = html::Html::new(html::Flags::empty(), 0);

    let target = source.with_extension("html");
    let mut target_contents = String::new();

    File::open(&target).unwrap().read_to_string(&mut target_contents).unwrap();

    let output = html.render(&doc);

    let rendered_tidy = tidy(output.to_str().unwrap());
    let target_tidy = tidy(&target_contents[..]);

    assert_eq!(rendered_tidy, target_tidy);
}

#[test]
fn test_toc() {
    let source = PathBuf::from("libhoedown/test/Tests/Formatting in Table of Contents.text");
    let doc = Markdown::read_from(File::open(&source).unwrap());
    let mut renderer = html::Html::toc(3);

    let target = source.with_extension("html");
    let mut target_contents = String::new();

    File::open(&target).unwrap().read_to_string(&mut target_contents).unwrap();

    let output = renderer.render(&doc);

    let rendered_tidy = tidy(output.to_str().unwrap());
    let target_tidy = tidy(&target_contents);

    assert_eq!(rendered_tidy, target_tidy);
}
