extern crate hoedown;

use hoedown::Markdown;
use hoedown::renderer::html;

macro_rules! extensions_test {
    ($extension:ident: $left:expr, $right:expr) => ({
        let doc = Markdown::new($left.as_bytes()).with_extensions(hoedown::$extension);
        let renderer = html::Html::new(html::Flags::empty(), 0);

        assert_eq!(doc.render_to_buffer(renderer).as_str().unwrap(), $right);
    });
}

#[test]
fn test_hard_wrap() {
    extensions_test!(AUTOLINK:
        "https://github.com/",
        "<p><a href=\"https://github.com/\">https://github.com/</a></p>\n");
}

#[test]
fn test_fenced_code() {
    extensions_test!(FENCED_CODE:
        "```\n$ :(){ :|:& };:\n```",
        "<pre><code>$ :(){ :|:&amp; };:\n</code></pre>\n");
}

#[test]
fn test_fenced_code_lang() {
    extensions_test!(FENCED_CODE:
        "```bash\n$ :(){ :|:& };:\n```",
        "<pre><code class=\"language-bash\">$ :(){ :|:&amp; };:\n</code></pre>\n");
}

#[test]
fn test_footnotes() {
    extensions_test!(FOOTNOTES:
        "What you looking at? [^1]\n\n[^1]: Yeah, I\'m talking to you pal.",
"<p>What you looking at? <sup id=\"fnref1\"><a href=\"#fn1\" rel=\"footnote\">1</a></sup></p>

<div class=\"footnotes\">
<hr>
<ol>

<li id=\"fn1\">
<p>Yeah, I&#39;m talking to you pal.&nbsp;<a href=\"#fnref1\" rev=\"footnote\">&#8617;</a></p>
</li>

</ol>
</div>\n");
}

#[test]
fn test_highlight() {
    extensions_test!(HIGHLIGHT:
        "I\'m ==special==.",
        "<p>I&#39;m <mark>special</mark>.</p>\n");
}

#[test]
fn test_no_indented_code() {
    extensions_test!(DISABLE_INDENTED_CODE:
        "    $ :(){ :|:& };:",
        "<p>$ :(){ :|:&amp; };:</p>\n");
}

#[test]
fn test_no_intra_emphasis() {
    extensions_test!(NO_INTRA_EMPHASIS:
        "Sorry,_my_space_key_is_broken.",
        "<p>Sorry,_my_space_key_is_broken.</p>\n");
}

#[test]
fn test_quote() {
    extensions_test!(QUOTE:
        r#""Air quotes are obnoxious.""#,
        "<p><q>Air quotes are obnoxious.</q></p>\n");
}

#[test]
fn test_space_headers() {
    extensions_test!(SPACE_HEADERS:
        "#Are you listening to me?!",
        "<p>#Are you listening to me?!</p>\n");
}

#[test]
fn test_strikethrough() {
    extensions_test!(STRIKETHROUGH:
        "I\'m ~~running~~ out of ideas.",
        "<p>I&#39;m <del>running</del> out of ideas.</p>\n");
}

#[test]
fn test_superscript() {
    extensions_test!(SUPERSCRIPT:
        "^bro",
        "<p><sup>bro</sup></p>\n");
}

#[test]
fn test_tables() {
    extensions_test!(TABLES:
"|  1  |  2  |  3  |
| --- | --- | --- |
|  X  |  X  |  O  |
|  O  |  O  |  X  |
|  X  |  O  |  X  |
",
"<table>
<thead>
<tr>
<th>1</th>
<th>2</th>
<th>3</th>
</tr>
</thead>

<tbody>
<tr>
<td>X</td>
<td>X</td>
<td>O</td>
</tr>
<tr>
<td>O</td>
<td>O</td>
<td>X</td>
</tr>
<tr>
<td>X</td>
<td>O</td>
<td>X</td>
</tr>
</tbody>
</table>\n");
}

#[test]
fn test_underline() {
    extensions_test!(UNDERLINE:
        "What do you _mean_?",
        "<p>What do you <u>mean</u>?</p>\n");
}
