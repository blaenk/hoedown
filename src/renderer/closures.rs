//! Contains the Closures renderer
#![allow(non_camel_case_types)]
use buffer::Buffer;
use super::Render;

mod types {
    use buffer::Buffer;

    pub type code_block<'a> = Box<FnMut(&mut Buffer, &Buffer, &Buffer) + 'a>;
    pub type quote_block<'a> = Box<FnMut(&mut Buffer, &Buffer) + 'a>;
    pub type header<'a> = Box<FnMut(&mut Buffer, &Buffer, i32) + 'a>;
    pub type horizontal_rule<'a> = Box<FnMut(&mut Buffer) + 'a>;
    pub type list<'a> = Box<FnMut(&mut Buffer, &Buffer, ::renderer::list::List) + 'a>;
    pub type list_item<'a> = Box<FnMut(&mut Buffer, &Buffer, ::renderer::list::List) + 'a>;
    pub type paragraph<'a> = Box<FnMut(&mut Buffer, &Buffer) + 'a>;
    pub type table<'a> = Box<FnMut(&mut Buffer, &Buffer) + 'a>;
    pub type table_header<'a> = Box<FnMut(&mut Buffer, &Buffer) + 'a>;
    pub type table_body<'a> = Box<FnMut(&mut Buffer, &Buffer) + 'a>;
    pub type table_row<'a> = Box<FnMut(&mut Buffer, &Buffer) + 'a>;
    pub type table_cell<'a> = Box<FnMut(&mut Buffer, &Buffer, ::renderer::Table) + 'a>;
    pub type footnotes<'a> = Box<FnMut(&mut Buffer, &Buffer) + 'a>;
    pub type footnote_definition<'a> = Box<FnMut(&mut Buffer, &Buffer, u32) + 'a>;
    pub type html_block<'a> = Box<FnMut(&mut Buffer, &Buffer) + 'a>;

    pub type autolink<'a> = Box<FnMut(&mut Buffer, &Buffer, ::renderer::AutoLink) -> bool + 'a>;
    pub type code_span<'a> = Box<FnMut(&mut Buffer, &Buffer) -> bool + 'a>;
    pub type double_emphasis<'a> = Box<FnMut(&mut Buffer, &Buffer) -> bool + 'a>;
    pub type emphasis<'a> = Box<FnMut(&mut Buffer, &Buffer) -> bool + 'a>;
    pub type underline<'a> = Box<FnMut(&mut Buffer, &Buffer) -> bool + 'a>;
    pub type highlight<'a> = Box<FnMut(&mut Buffer, &Buffer) -> bool + 'a>;
    pub type quote<'a> = Box<FnMut(&mut Buffer, &Buffer) -> bool + 'a>;
    pub type image<'a> = Box<FnMut(&mut Buffer, &Buffer, &Buffer, &Buffer) -> bool + 'a>;
    pub type line_break<'a> = Box<FnMut(&mut Buffer) -> bool + 'a>;
    pub type link<'a> = Box<FnMut(&mut Buffer, &Buffer, &Buffer, &Buffer) -> bool + 'a>;
    pub type triple_emphasis<'a> = Box<FnMut(&mut Buffer, &Buffer) -> bool + 'a>;
    pub type strikethrough<'a> = Box<FnMut(&mut Buffer, &Buffer) -> bool + 'a>;
    pub type superscript<'a> = Box<FnMut(&mut Buffer, &Buffer) -> bool + 'a>;
    pub type footnote_reference<'a> = Box<FnMut(&mut Buffer, u32) -> bool + 'a>;
    pub type math<'a> = Box<FnMut(&mut Buffer, &Buffer, i32) -> bool + 'a>;
    pub type html_span<'a> = Box<FnMut(&mut Buffer, &Buffer) -> bool + 'a>;

    pub type entity<'a> = Box<FnMut(&mut Buffer, &Buffer) + 'a>;
    pub type normal_text<'a> = Box<FnMut(&mut Buffer, &Buffer) + 'a>;

    pub type before_render<'a> = Box<FnMut(&mut Buffer, i32) + 'a>;
    pub type after_render<'a> = Box<FnMut(&mut Buffer, i32) + 'a>;
}

/// A renderer whose behavior is defined by closures.
///
/// This renderer wraps another renderer and enables its behavior
/// to be overridden using closures.
///
/// This might be removed in the future since it's not possible to
/// access the base renderer from within a provided closure. The use
/// of the `Wrapper` trait and `wrap!` macro is encouraged instead.
///
///``` rust
///# use std::io::Write;
///# use hoedown::{Markdown, Buffer};
///# use hoedown::renderer::html;
///# use hoedown::renderer::closures::Closures;
///let html = html::Html::new(html::Flags::empty(), 0);
///let mut closures = Closures::new(html);
///
///closures.on_emphasis(|output: &mut Buffer, content: &Buffer| -> bool {
///    output.write(b"~~").unwrap();
///    output.pipe(content);
///    output.write(b"~~").unwrap();
///    true
///});
///
///let doc = Markdown::new("this _requires_ emphasis");
///let output = doc.render_to_buffer(closures);
///
///assert_eq!(output.to_str().unwrap(), "<p>this ~~requires~~ emphasis</p>\n");
///```
pub struct Closures<'a, R> where R: Render {
    base: R,

    code_block: Option<types::code_block<'a>>,
    quote_block: Option<types::quote_block<'a>>,
    header: Option<types::header<'a>>,
    horizontal_rule: Option<types::horizontal_rule<'a>>,
    list: Option<types::list<'a>>,
    list_item: Option<types::list_item<'a>>,
    paragraph: Option<types::paragraph<'a>>,
    table: Option<types::table<'a>>,
    table_header: Option<types::table_header<'a>>,
    table_body: Option<types::table_body<'a>>,
    table_row: Option<types::table_row<'a>>,
    table_cell: Option<types::table_cell<'a>>,
    footnotes: Option<types::footnotes<'a>>,
    footnote_definition: Option<types::footnote_definition<'a>>,
    html_block: Option<types::html_block<'a>>,

    autolink: Option<types::autolink<'a>>,
    code_span: Option<types::code_span<'a>>,
    double_emphasis: Option<types::double_emphasis<'a>>,
    emphasis: Option<types::emphasis<'a>>,
    underline: Option<types::underline<'a>>,
    highlight: Option<types::highlight<'a>>,
    quote: Option<types::quote<'a>>,
    image: Option<types::image<'a>>,
    line_break: Option<types::line_break<'a>>,
    link: Option<types::link<'a>>,
    triple_emphasis: Option<types::triple_emphasis<'a>>,
    strikethrough: Option<types::strikethrough<'a>>,
    superscript: Option<types::superscript<'a>>,
    footnote_reference: Option<types::footnote_reference<'a>>,
    math: Option<types::math<'a>>,
    html_span: Option<types::html_span<'a>>,

    entity: Option<types::entity<'a>>,
    normal_text: Option<types::normal_text<'a>>,

    before_render: Option<types::before_render<'a>>,
    after_render: Option<types::after_render<'a>>,
}

impl <'a, R> Closures<'a, R> where R: Render {
    pub fn new(renderer: R) -> Closures<'a, R> {
        Closures {
            base: renderer,

            code_block: None,
            quote_block: None,
            header: None,
            horizontal_rule: None,
            list: None,
            list_item: None,
            paragraph: None,
            table: None,
            table_header: None,
            table_body: None,
            table_row: None,
            table_cell: None,
            footnotes: None,
            footnote_definition: None,
            html_block: None,

            autolink: None,
            code_span: None,
            double_emphasis: None,
            emphasis: None,
            underline: None,
            highlight: None,
            quote: None,
            image: None,
            line_break: None,
            link: None,
            triple_emphasis: None,
            strikethrough: None,
            superscript: None,
            footnote_reference: None,
            math: None,
            html_span: None,

            entity: None,
            normal_text: None,

            before_render: None,
            after_render: None,
        }
    }
}

impl<'a, R> Render for Closures<'a, R>
where R: Render {
    fn code_block(&mut self, output: &mut Buffer, text: &Buffer, lang: &Buffer) {
        if let Some(ref mut func) = self.code_block {
            func(output, text, lang);
        } else {
            self.base.code_block(output, text, lang);
        }
    }

    fn quote_block(&mut self, output: &mut Buffer, content: &Buffer) {
        if let Some(ref mut func) = self.quote_block {
            func(output, content);
        } else {
            self.base.quote_block(output, content);
        }
    }

    fn header(&mut self, output: &mut Buffer, content: &Buffer, level: i32) {
        if let Some(ref mut func) = self.header {
            func(output, content, level);
        } else {
            self.base.header(output, content, level);
        }
    }

    fn horizontal_rule(&mut self, output: &mut Buffer) {
        if let Some(ref mut func) = self.horizontal_rule {
            func(output);
        } else {
            self.base.horizontal_rule(output);
        }
    }

    fn list(&mut self, output: &mut Buffer, content: &Buffer, flags: ::renderer::list::List) {
        if let Some(ref mut func) = self.list {
            func(output, content, flags);
        } else {
            self.base.list(output, content, flags);
        }
    }

    fn list_item(&mut self, output: &mut Buffer, content: &Buffer, flags: ::renderer::list::List) {
        if let Some(ref mut func) = self.list_item {
            func(output, content, flags);
        } else {
            self.base.list_item(output, content, flags);
        }
    }

    fn paragraph(&mut self, output: &mut Buffer, content: &Buffer) {
        if let Some(ref mut func) = self.paragraph {
            func(output, content);
        } else {
            self.base.paragraph(output, content);
        }
    }

    fn table(&mut self, output: &mut Buffer, content: &Buffer) {
        if let Some(ref mut func) = self.table {
            func(output, content);
        } else {
            self.base.table(output, content);
        }
    }

    fn table_header(&mut self, output: &mut Buffer, content: &Buffer) {
        if let Some(ref mut func) = self.table_header {
            func(output, content);
        } else {
            self.base.table_header(output, content);
        }
    }

    fn table_body(&mut self, output: &mut Buffer, content: &Buffer) {
        if let Some(ref mut func) = self.table_body {
            func(output, content);
        } else {
            self.base.table_body(output, content);
        }
    }

    fn table_row(&mut self, output: &mut Buffer, content: &Buffer) {
        if let Some(ref mut func) = self.table_row {
            func(output, content);
        } else {
            self.base.table_row(output, content);
        }
    }

    fn table_cell(&mut self, output: &mut Buffer, content: &Buffer, flags: ::renderer::Table) {
        if let Some(ref mut func) = self.table_cell {
            func(output, content, flags);
        } else {
            self.base.table_cell(output, content, flags);
        }
    }

    fn footnotes(&mut self, output: &mut Buffer, content: &Buffer) {
        if let Some(ref mut func) = self.footnotes {
            func(output, content);
        } else {
            self.base.footnotes(output, content);
        }
    }

    fn footnote_definition(&mut self, output: &mut Buffer, content: &Buffer, num: u32) {
        if let Some(ref mut func) = self.footnote_definition {
            func(output, content, num);
        } else {
            self.base.footnote_definition(output, content, num);
        }
    }

    fn html_block(&mut self, output: &mut Buffer, text: &Buffer) {
        if let Some(ref mut func) = self.html_block {
            func(output, text);
        } else {
            self.base.html_block(output, text);
        }
    }

    fn autolink(&mut self, output: &mut Buffer, link: &Buffer, ty: ::renderer::AutoLink) -> bool {
        if let Some(ref mut func) = self.autolink {
            func(output, link, ty)
        } else {
            self.base.autolink(output, link, ty)
        }
    }

    fn code_span(&mut self, output: &mut Buffer, text: &Buffer) -> bool {
        if let Some(ref mut func) = self.code_span {
            func(output, text)
        } else {
            self.base.code_span(output, text)
        }
    }

    fn double_emphasis(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        if let Some(ref mut func) = self.double_emphasis {
            func(output, content)
        } else {
            self.base.double_emphasis(output, content)
        }
    }

    fn emphasis(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        if let Some(ref mut func) = self.emphasis {
            func(output, content)
        } else {
            self.base.emphasis(output, content)
        }
    }

    fn underline(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        if let Some(ref mut func) = self.underline {
            func(output, content)
        } else {
            self.base.underline(output, content)
        }
    }

    fn highlight(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        if let Some(ref mut func) = self.highlight {
            func(output, content)
        } else {
            self.base.highlight(output, content)
        }
    }

    fn quote_span(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        if let Some(ref mut func) = self.quote {
            func(output, content)
        } else {
            self.base.quote_span(output, content)
        }
    }

    fn image(&mut self, output: &mut Buffer, link: &Buffer, title: &Buffer, alt: &Buffer) -> bool {
        if let Some(ref mut func) = self.image {
            func(output, link, title, alt)
        } else {
            self.base.image(output, link, title, alt)
        }
    }

    fn line_break(&mut self, output: &mut Buffer) -> bool {
        if let Some(ref mut func) = self.line_break {
            func(output)
        } else {
            self.base.line_break(output)
        }
    }

    fn link(&mut self, output: &mut Buffer, content: &Buffer, link: &Buffer, title: &Buffer) -> bool {
        if let Some(ref mut func) = self.link {
            func(output, content, link, title)
        } else {
            self.base.link(output, content, link, title)
        }
    }

    fn triple_emphasis(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        if let Some(ref mut func) = self.triple_emphasis {
            func(output, content)
        } else {
            self.base.triple_emphasis(output, content)
        }
    }

    fn strikethrough(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        if let Some(ref mut func) = self.strikethrough {
            func(output, content)
        } else {
            self.base.strikethrough(output, content)
        }
    }

    fn superscript(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        if let Some(ref mut func) = self.superscript {
            func(output, content)
        } else {
            self.base.superscript(output, content)
        }
    }

    fn footnote_reference(&mut self, output: &mut Buffer, num: u32) -> bool {
        if let Some(ref mut func) = self.footnote_reference {
            func(output, num)
        } else {
            self.base.footnote_reference(output, num)
        }
    }

    fn math(&mut self, output: &mut Buffer, text: &Buffer, displaymode: i32) -> bool {
        if let Some(ref mut func) = self.math {
            func(output, text, displaymode)
        } else {
            self.base.math(output, text, displaymode)
        }
    }

    fn html_span(&mut self, output: &mut Buffer, text: &Buffer) -> bool {
        if let Some(ref mut func) = self.html_span {
            func(output, text)
        } else {
            self.base.html_span(output, text)
        }
    }

    fn entity(&mut self, output: &mut Buffer, text: &Buffer) {
        if let Some(ref mut func) = self.entity {
            func(output, text);
        } else {
            self.base.entity(output, text)
        }
    }

    fn normal_text(&mut self, output: &mut Buffer, text: &Buffer) {
        if let Some(ref mut func) = self.normal_text {
            func(output, text);
        } else {
            self.base.normal_text(output, text)
        }
    }

    fn before_render(&mut self, output: &mut Buffer, inline_render: bool) {
        if let Some(ref mut func) = self.before_render {
            func(output, inline_render as i32);
        }

        self.base.before_render(output, inline_render);
    }

    fn after_render(&mut self, output: &mut Buffer, inline_render: bool) {
        if let Some(ref mut func) = self.after_render {
            func(output, inline_render as i32);
        }

        self.base.after_render(output, inline_render);
    }
}

impl<'a, R> Closures<'a, R>
where R: Render {
    pub fn on_code_block<F>(&mut self, closure: F)
    where F: FnMut(&mut Buffer, &Buffer, &Buffer), F: 'a {
        self.code_block = Some(Box::new(closure));
    }

    pub fn on_quote_block<F>(&mut self, closure: F)
    where F: FnMut(&mut Buffer, &Buffer), F: 'a {
        self.quote_block = Some(Box::new(closure));
    }

    pub fn on_header<F>(&mut self, closure: F)
    where F: FnMut(&mut Buffer, &Buffer, i32), F: 'a {
        self.header = Some(Box::new(closure));
    }

    pub fn on_horizontal_rule<F>(&mut self, closure: F)
    where F: FnMut(&mut Buffer), F: 'a {
        self.horizontal_rule = Some(Box::new(closure));
    }

    pub fn on_list<F>(&mut self, closure: F)
    where F: FnMut(&mut Buffer, &Buffer, ::renderer::list::List), F: 'a {
        self.list = Some(Box::new(closure));
    }

    pub fn on_list_item<F>(&mut self, closure: F)
    where F: FnMut(&mut Buffer, &Buffer, ::renderer::list::List), F: 'a {
        self.list_item = Some(Box::new(closure));
    }

    pub fn on_paragraph<F>(&mut self, closure: F)
    where F: FnMut(&mut Buffer, &Buffer), F: 'a {
        self.paragraph = Some(Box::new(closure));
    }

    pub fn on_table<F>(&mut self, closure: F)
    where F: FnMut(&mut Buffer, &Buffer), F: 'a {
        self.table = Some(Box::new(closure));
    }

    pub fn on_table_header<F>(&mut self, closure: F)
    where F: FnMut(&mut Buffer, &Buffer), F: 'a {
        self.table_header = Some(Box::new(closure));
    }

    pub fn on_table_body<F>(&mut self, closure: F)
    where F: FnMut(&mut Buffer, &Buffer), F: 'a {
        self.table_body = Some(Box::new(closure));
    }

    pub fn on_table_row<F>(&mut self, closure: F)
    where F: FnMut(&mut Buffer, &Buffer), F: 'a {
        self.table_row = Some(Box::new(closure));
    }

    pub fn on_table_cell<F>(&mut self, closure: F)
    where F: FnMut(&mut Buffer, &Buffer, ::renderer::Table), F: 'a {
        self.table_cell = Some(Box::new(closure));
    }

    pub fn on_footnotes<F>(&mut self, closure: F)
    where F: FnMut(&mut Buffer, &Buffer), F: 'a {
        self.footnotes = Some(Box::new(closure));
    }

    pub fn on_footnote_definition<F>(&mut self, closure: F)
    where F: FnMut(&mut Buffer, &Buffer, u32), F: 'a {
        self.footnote_definition = Some(Box::new(closure));
    }

    pub fn on_html_block<F>(&mut self, closure: F)
    where F: FnMut(&mut Buffer, &Buffer), F: 'a {
        self.html_block = Some(Box::new(closure));
    }

    pub fn on_autolink<F>(&mut self, closure: F)
    where F: FnMut(&mut Buffer, &Buffer, ::renderer::AutoLink) -> bool, F: 'a {
        self.autolink = Some(Box::new(closure));
    }

    pub fn on_code_span<F>(&mut self, closure: F)
    where F: FnMut(&mut Buffer, &Buffer) -> bool, F: 'a {
        self.code_span = Some(Box::new(closure));
    }

    pub fn on_double_emphasis<F>(&mut self, closure: F)
    where F: FnMut(&mut Buffer, &Buffer) -> bool, F: 'a {
        self.double_emphasis = Some(Box::new(closure));
    }

    pub fn on_emphasis<F>(&mut self, closure: F)
    where F: FnMut(&mut Buffer, &Buffer) -> bool, F: 'a {
        self.emphasis = Some(Box::new(closure));
    }

    pub fn on_underline<F>(&mut self, closure: F)
    where F: FnMut(&mut Buffer, &Buffer) -> bool, F: 'a {
        self.underline = Some(Box::new(closure));
    }

    pub fn on_highlight<F>(&mut self, closure: F)
    where F: FnMut(&mut Buffer, &Buffer) -> bool, F: 'a {
        self.highlight = Some(Box::new(closure));
    }

    pub fn on_quote<F>(&mut self, closure: F)
    where F: FnMut(&mut Buffer, &Buffer) -> bool, F: 'a {
        self.quote = Some(Box::new(closure));
    }

    pub fn on_image<F>(&mut self, closure: F)
    where F: FnMut(&mut Buffer, &Buffer, &Buffer, &Buffer) -> bool, F: 'a {
        self.image = Some(Box::new(closure));
    }

    pub fn on_line_break<F>(&mut self, closure: F)
    where F: FnMut(&mut Buffer) -> bool, F: 'a {
        self.line_break = Some(Box::new(closure));
    }

    pub fn on_link<F>(&mut self, closure: F)
    where F: FnMut(&mut Buffer, &Buffer, &Buffer, &Buffer) -> bool, F: 'a {
        self.link = Some(Box::new(closure));
    }

    pub fn on_triple_emphasis<F>(&mut self, closure: F)
    where F: FnMut(&mut Buffer, &Buffer) -> bool, F: 'a {
        self.triple_emphasis = Some(Box::new(closure));
    }

    pub fn on_strikethrough<F>(&mut self, closure: F)
    where F: FnMut(&mut Buffer, &Buffer) -> bool, F: 'a {
        self.strikethrough = Some(Box::new(closure));
    }

    pub fn on_superscript<F>(&mut self, closure: F)
    where F: FnMut(&mut Buffer, &Buffer) -> bool, F: 'a {
        self.superscript = Some(Box::new(closure));
    }

    pub fn on_footnote_reference<F>(&mut self, closure: F)
    where F: FnMut(&mut Buffer, u32) -> bool, F: 'a {
        self.footnote_reference = Some(Box::new(closure));
    }

    pub fn on_math<F>(&mut self, closure: F)
    where F: FnMut(&mut Buffer, &Buffer, i32) -> bool, F: 'a {
        self.math = Some(Box::new(closure));
    }

    pub fn on_html_span<F>(&mut self, closure: F)
    where F: FnMut(&mut Buffer, &Buffer) -> bool, F: 'a {
        self.html_span = Some(Box::new(closure));
    }

    pub fn on_entity<F>(&mut self, closure: F)
    where F: FnMut(&mut Buffer, &Buffer), F: 'a {
        self.entity = Some(Box::new(closure));
    }

    pub fn on_normal_text<F>(&mut self, closure: F)
    where F: FnMut(&mut Buffer, &Buffer), F: 'a {
        self.normal_text = Some(Box::new(closure));
    }

    pub fn on_before_render<F>(&mut self, closure: F)
    where F: FnMut(&mut Buffer, i32), F: 'a {
        self.before_render = Some(Box::new(closure));
    }

    pub fn on_after_render<F>(&mut self, closure: F)
    where F: FnMut(&mut Buffer, i32), F: 'a {
        self.after_render = Some(Box::new(closure));
    }
}

