extern crate hoedown;

use hoedown::Buffer;
use hoedown::renderer::html;

macro_rules! smartypants_test {
    ($left:expr, $right:expr) => ({
        let input = Buffer::from($left);
        let mut output = Buffer::new(64);

        html::smartypants(&input, &mut output);

        assert_eq!(output.to_str().unwrap(), $right);
    });
}

#[test]
fn test_apostrophe() {
    smartypants_test!(
        "What\'s with apostrophes?",
        "What&rsquo;s with apostrophes?");
}

#[test]
fn test_double_quotes() {
    smartypants_test!(
        r#""Air quotes are obnoxious.""#,
        "&ldquo;Air quotes are obnoxious.&rdquo;");
}

#[test]
fn test_ellipsis() {
    smartypants_test!(
        "One of these days...",
        "One of these days&hellip;");
}

#[test]
fn test_em_dash() {
    smartypants_test!(
        "In five minutes the---",
        "In five minutes the&mdash;");
}

#[test]
fn test_en_dash() {
    smartypants_test!(
        "Non--zero.",
        "Non&ndash;zero.");
}
