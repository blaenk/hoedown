extern crate hoedown;

use hoedown::Markdown;
use hoedown::renderer::{self, Render};
use hoedown::buffer::Buffer;

use std::io::Write;

struct BlockRenderer;

impl Render for BlockRenderer {
    fn paragraph(&mut self, ob: &mut Buffer, content: &Buffer) {
        ob.pipe(content);
    }

    fn code_block(&mut self, output: &mut Buffer, input: &Buffer, language: &Buffer) {
        let s = format!("[CODE_BLOCK language={}] {}",
                        language.as_str().unwrap(),
                        input.as_str().unwrap());
        output.write(s.as_bytes()).unwrap();
    }

    fn html_block(&mut self, output: &mut Buffer, input: &Buffer) {
        let s = format!("[HTML_BLOCK] {}", input.as_str().unwrap());
        output.write(s.as_bytes()).unwrap();
    }

    fn quote_block(&mut self, output: &mut Buffer, content: &Buffer) {
        let s = format!("[QUOTE_BLOCK] {}", content.as_str().unwrap());
        output.write(s.as_bytes()).unwrap();
    }

    fn header(&mut self, output: &mut Buffer, input: &Buffer, level: i32) {
        let s = format!("[HEADER level={}] {}", level, input.as_str().unwrap());
        output.write(s.as_bytes()).unwrap();
    }

    fn horizontal_rule(&mut self, output: &mut Buffer) {
        output.write("[HORIZONTAL_RULE]".as_bytes()).unwrap();
    }

    fn list(&mut self, output: &mut Buffer, input: &Buffer, list_flags: renderer::list::List) {
        let is_ordered = list_flags.intersects(renderer::list::ORDERED);
        let s = format!("[LIST ordered={}]\n{}", is_ordered, input.as_str().unwrap());
        output.write(s.as_bytes()).unwrap();
    }

    fn list_item(&mut self, output: &mut Buffer, input: &Buffer, list_flags: renderer::list::List) {
        let is_ordered = list_flags.intersects(renderer::list::ORDERED);
        let s = format!("[LISTITEM ordered={}] {}", is_ordered, input.as_str().unwrap());
        output.write(s.as_bytes()).unwrap();
    }

    fn table(&mut self, output: &mut Buffer, content: &Buffer) {
        let s = format!("[TABLE]\n{}", content.as_str().unwrap());
        output.write(s.as_bytes()).unwrap();
    }

    fn table_header(&mut self, output: &mut Buffer, content: &Buffer) {
        let s = format!("[TABLE_HEADER]{}", content.as_str().unwrap());
        output.write(s.as_bytes()).unwrap();
    }

    fn table_body(&mut self, output: &mut Buffer, content: &Buffer) {
        let s = format!("\n[TABLE_BODY]{}", content.as_str().unwrap());
        output.write(s.as_bytes()).unwrap();
    }

    fn table_row(&mut self, output: &mut Buffer, content: &Buffer) {
        let s = format!("\n[TABLE_ROW]\n{}", content.as_str().unwrap());
        output.write(s.as_bytes()).unwrap();
    }

    fn table_cell(&mut self, output: &mut Buffer, content: &Buffer, _flags: renderer::Table) {
        let s = format!("[TABLE_CELL text={}]", content.as_str().unwrap());
        output.write(s.as_bytes()).unwrap();
    }
}

struct DocRenderer;

impl Render for DocRenderer {
    fn paragraph(&mut self, ob: &mut Buffer, content: &Buffer) {
        ob.pipe(content);
    }

    fn before_render(&mut self, output: &mut Buffer, _inline_render: bool) {
        output.write("One.\n".as_bytes()).unwrap();
    }

    fn after_render(&mut self, output: &mut Buffer, _inline_render: bool) {
        output.write("\nFive.".as_bytes()).unwrap();
    }
}

struct FootnotesRenderer;

impl Render for FootnotesRenderer {
    fn paragraph(&mut self, ob: &mut Buffer, content: &Buffer) {
        ob.pipe(content);
    }

    fn footnotes(&mut self, output: &mut Buffer, content: &Buffer) {
        let s = format!("[FOOTNOTES]\n{}", content.as_str().unwrap());
        output.write(s.as_bytes()).unwrap();
    }

    fn footnote_definition(&mut self, output: &mut Buffer, content: &Buffer, number: u32) {
        let s = format!("[FOOTNOTE_DEFINITION #{}] {}", number, content.as_str().unwrap());
        output.write(s.as_bytes()).unwrap();
    }

    fn footnote_reference(&mut self, output: &mut Buffer, number: u32) -> bool {
        let s = format!("[FOOTNOTE_REFERENCE #{}]", number);
        output.write(s.as_bytes()).unwrap();
        true
    }
}

struct LowLevelRenderer;

impl Render for LowLevelRenderer {
    fn paragraph(&mut self, ob: &mut Buffer, content: &Buffer) {
        ob.pipe(content);
    }

    fn entity(&mut self, output: &mut Buffer, content: &Buffer) {
        let s = format!("[ENTITY] {}", content.as_str().unwrap());
        output.write(s.as_bytes()).unwrap();
    }

    fn normal_text(&mut self, output: &mut Buffer, content: &Buffer) {
        let s = format!("[NORMAL_TEXT] {}", content.as_str().unwrap());
        output.write(s.as_bytes()).unwrap();
    }
}
struct ParagraphRenderer;

impl Render for ParagraphRenderer {
    fn paragraph(&mut self, output: &mut Buffer, content: &Buffer) {
        let s = format!("[PARAGRAPH] {}", content.as_str().unwrap());
        output.write(s.as_bytes()).unwrap();
    }
}

struct SpanRenderer;

impl Render for SpanRenderer {
    fn paragraph(&mut self, ob: &mut Buffer, content: &Buffer) {
        ob.pipe(content);
    }

    fn line_break(&mut self, output: &mut Buffer) -> bool {
        output.write("[LINE_BREAK]".as_bytes()).unwrap();
        true
    }

    fn autolink(&mut self, output: &mut Buffer, content: &Buffer, link_type: renderer::AutoLink) -> bool {
        let s = format!("[AUTOLINK type={:?}] {}", link_type, content.as_str().unwrap());
        output.write(s.as_bytes()).unwrap();
        true
    }

    fn code_span(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        let s = format!("[CODE_SPAN] {}", content.as_str().unwrap());
        output.write(s.as_bytes()).unwrap();
        true
    }

    fn double_emphasis(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        let s = format!("[DOUBLE_EMPHASIS] {}", content.as_str().unwrap());
        output.write(s.as_bytes()).unwrap();
        true
    }

    fn emphasis(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        let s = format!("[EMPHASIS] {}", content.as_str().unwrap());
        output.write(s.as_bytes()).unwrap();
        true
    }

    fn highlight(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        let s = format!("[HIGHLIGHT] {}", content.as_str().unwrap());
        output.write(s.as_bytes()).unwrap();
        true
    }

    fn image(&mut self, output: &mut Buffer, link: &Buffer, title: &Buffer, alt: &Buffer) -> bool {
        let s = format!("[IMAGE link={} title={} alt={}]",
                        link.as_str().unwrap(),
                        title.as_str().unwrap(),
                        alt.as_str().unwrap());
        output.write(s.as_bytes()).unwrap();
        true
    }

    fn link(&mut self, output: &mut Buffer, content: &Buffer, link: &Buffer, title: &Buffer) -> bool {
        let s = format!("[LINK link={} title={}] {}",
                        link.as_str().unwrap(),
                        title.as_str().unwrap(),
                        content.as_str().unwrap());
        output.write(s.as_bytes()).unwrap();
        true
    }

    fn quote_span(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        let s = format!("[QUOTE] {}", content.as_str().unwrap());
        output.write(s.as_bytes()).unwrap();
        true
    }

    fn html_span(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        let s = format!("[HTML_SPAN] {}", content.as_str().unwrap());
        output.write(s.as_bytes()).unwrap();
        true
    }

    fn strikethrough(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        let s = format!("[STRIKETHROUGH] {}", content.as_str().unwrap());
        output.write(s.as_bytes()).unwrap();
        true
    }

    fn superscript(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        let s = format!("[SUPERSCRIPT] {}", content.as_str().unwrap());
        output.write(s.as_bytes()).unwrap();
        true
    }

    fn triple_emphasis(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        let s = format!("[TRIPLE_EMPHASIS] {}", content.as_str().unwrap());
        output.write(s.as_bytes()).unwrap();
        true
    }
}

struct UnderlineRenderer;

impl Render for UnderlineRenderer {
    fn paragraph(&mut self, ob: &mut Buffer, content: &Buffer) {
        ob.pipe(content);
    }

    fn underline(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        let s = format!("[UNDERLINE] {}", content.as_str().unwrap());
        output.write(s.as_bytes()).unwrap();
        true
    }
}

macro_rules! renderer_test {
    ($renderer:expr => $left:expr, $right:expr) => ({
        renderer_test!($renderer, ::hoedown::Extension::empty() => $left, $right);
    });

    ($renderer:expr, $flags:expr => $left:expr, $right:expr) => ({
        let mut renderer = $renderer;
        let doc = Markdown::new($left.as_bytes()).with_extensions($flags);

        let output = doc.render_to_buffer(&mut renderer);

        assert_eq!(output.as_str().unwrap(), $right);
    })
}

#[test]
fn test_doc_header_and_footer() {
    renderer_test!(DocRenderer =>
       "Two.",
       "One.\nTwo.\nFive.");
}

#[test]
fn test_blockcode() {
    renderer_test!(BlockRenderer, hoedown::FENCED_CODE =>
        "```bash\n$ :(){ :|:& };:\n```",
        "[CODE_BLOCK language=bash] $ :(){ :|:& };:\n");
}

#[test]
fn test_blockcode_empty_lang() {
    renderer_test!(BlockRenderer, hoedown::FENCED_CODE =>
        "```\n$ :(){ :|:& };:\n```",
        "[CODE_BLOCK language=] $ :(){ :|:& };:\n");
}

#[test]
fn test_blockhtml() {
    renderer_test!(BlockRenderer =>
        "<p>Hi.</p>",
        "[HTML_BLOCK] <p>Hi.</p>\n");
}

#[test]
fn test_blockquote() {
    renderer_test!(BlockRenderer =>
        "> Echo.",
        "[QUOTE_BLOCK] Echo.");
}

#[test]
fn test_footnotes() {
    renderer_test!(FootnotesRenderer, hoedown::FOOTNOTES =>
        "What you looking at? [^1]\n\n[^1]: Yeah, I'm talking to you pal.",
"What you looking at? [FOOTNOTE_REFERENCE #1][FOOTNOTES]
[FOOTNOTE_DEFINITION #1] Yeah, I'm talking to you pal.");
}

#[test]
fn test_header() {
    renderer_test!(BlockRenderer =>
        "## One more to go.",
        "[HEADER level=2] One more to go.");
}

#[test]
fn test_horizontal_rule() {
    renderer_test!(BlockRenderer =>
        "---",
        "[HORIZONTAL_RULE]");
}

#[test]
fn test_list_ordered() {
    renderer_test!(BlockRenderer =>
        "1. Ehh\n2. Bee\n3. Eee",
"[LIST ordered=true]
[LISTITEM ordered=true] Ehh
[LISTITEM ordered=true] Bee
[LISTITEM ordered=true] Eee\n");
}

#[test]
fn test_list_unordered() {
    renderer_test!(BlockRenderer =>
        "+ One\n+ Two\n+ Five",
"[LIST ordered=false]
[LISTITEM ordered=false] One
[LISTITEM ordered=false] Two
[LISTITEM ordered=false] Five\n");
}

#[test]
fn test_paragraph() {
    renderer_test!(ParagraphRenderer =>
        "One might say this is soul sucking...",
        "[PARAGRAPH] One might say this is soul sucking...");
}

#[test]
fn test_table() {
    renderer_test!(BlockRenderer, hoedown::TABLES =>
"|  1  |  2  |  3  |
| --- | --- | --- |
|  X  |  X  |  O  |
|  O  |  O  |  X  |
|  X  |  O  |  X  |\n",

"[TABLE]
[TABLE_HEADER]
[TABLE_ROW]
[TABLE_CELL text=1][TABLE_CELL text=2][TABLE_CELL text=3]
[TABLE_BODY]
[TABLE_ROW]
[TABLE_CELL text=X][TABLE_CELL text=X][TABLE_CELL text=O]
[TABLE_ROW]
[TABLE_CELL text=O][TABLE_CELL text=O][TABLE_CELL text=X]
[TABLE_ROW]
[TABLE_CELL text=X][TABLE_CELL text=O][TABLE_CELL text=X]");
}

#[test]
fn test_autolink() {
    renderer_test!(SpanRenderer, hoedown::AUTOLINK =>
        "https://github.com/",
        "[AUTOLINK type=Normal] https://github.com/");
}

#[test]
fn test_code_span() {
    renderer_test!(SpanRenderer =>
        "`$ rm -Rf tests/`",
        "[CODE_SPAN] $ rm -Rf tests/");
}

#[test]
fn test_double_emphasis() {
    renderer_test!(SpanRenderer =>
        "__strong__",
        "[DOUBLE_EMPHASIS] strong");
}

#[test]
fn test_emphasis() {
    renderer_test!(SpanRenderer =>
        "_wat_",
        "[EMPHASIS] wat");
}

#[test]
fn test_highlight() {
    renderer_test!(SpanRenderer, hoedown::HIGHLIGHT =>
        "==blink==",
        "[HIGHLIGHT] blink");
}

#[test]
fn test_image() {
    renderer_test!(SpanRenderer =>
        "![spacer](spacer.gif \"the spacer\")",
        "[IMAGE link=spacer.gif title=the spacer alt=spacer]");
}


#[test]
fn test_line_break() {
    renderer_test!(SpanRenderer =>
        "So.  \nTired.",
        "So.[LINE_BREAK]Tired.");
}
#[test]
fn test_link() {
    renderer_test!(SpanRenderer =>
        "[GitHub](https://github.com/ \"repo\")",
        "[LINK link=https://github.com/ title=repo] GitHub");
}

#[test]
fn test_quote_span() {
    renderer_test!(SpanRenderer, hoedown::QUOTE =>
        "\"Air quotes are obnoxious.\"",
        "[QUOTE] Air quotes are obnoxious.");
}

#[test]
fn test_html_span() {
    renderer_test!(SpanRenderer =>
        "<halp/>",
        "[HTML_SPAN] <halp/>");
}

#[test]
fn test_strikethrough() {
    renderer_test!(SpanRenderer, hoedown::STRIKETHROUGH =>
        "I'm ~~running~~ out of ideas.",
        "I'm [STRIKETHROUGH] running out of ideas.");
}

#[test]
fn test_superscript() {
    renderer_test!(SpanRenderer, hoedown::SUPERSCRIPT =>
        "^bro",
        "[SUPERSCRIPT] bro");
}

#[test]
fn test_triple_emphasis() {
    renderer_test!(SpanRenderer =>
        "Triple emphasis? That's ___absurd___.",
        "Triple emphasis? That's [TRIPLE_EMPHASIS] absurd.");
}

#[test]
fn test_underline() {
    renderer_test!(UnderlineRenderer, hoedown::UNDERLINE =>
        "That's _it_?",
        "That's [UNDERLINE] it?");
}

#[test]
fn test_low_level() {
    renderer_test!(LowLevelRenderer =>
        "&#9731;",
        "[NORMAL_TEXT] [ENTITY] &#9731;");
}
