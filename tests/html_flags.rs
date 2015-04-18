extern crate hoedown;

use hoedown::Markdown;
use hoedown::renderer::html;

macro_rules! html_test {
    ($flag:ident: $left:expr, $right:expr) => ({
        let doc = Markdown::new($left);
        let html_renderer = html::Html::new(html::$flag, 0);

        let output = doc.render_to_buffer(html_renderer);

        assert_eq!(output.to_str().unwrap(), $right);
    });
}

#[test]
fn test_skip_html() {
    html_test!(SKIP_HTML:
        "It <blink>must be</blink> allergies.",
        "<p>It must be allergies.</p>\n");
}

#[test]
fn test_escaping() {
    html_test!(ESCAPE:
        "JavaScript is pure <strong style=\"color: red;\">evil</strong>.",
        "<p>JavaScript is pure &lt;strong style=&quot;color: red;&quot;&gt;evil&lt;/strong&gt;.</p>\n");
}

#[test]
fn test_hard_wrap() {
    html_test!(HARD_WRAP:
        "One.\nAt.\nA.\nTime.",
        "<p>One.<br>\nAt.<br>\nA.<br>\nTime.</p>\n");
}

#[test]
fn test_xhtml() {
    html_test!(USE_XHTML:
        "![spacer](spacer.gif)",
        "<p><img src=\"spacer.gif\" alt=\"spacer\"/></p>\n");
}
