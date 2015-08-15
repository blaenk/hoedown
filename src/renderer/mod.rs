//! Contains Render behavior and stock renderers

use wrappers;
use buffer::Buffer;
use ffi::hoedown_renderer;

use markdown::Markdown;
use document::Document;

/// Represents render behavior
///
/// Types that implement this trait can be used to render a `Buffer` document.
///
/// All methods have default implementations which may be implemented by the
/// implementing type as required, depending on which callbacks it's interested in.
///
/// The default implementations attempt to be as neutral as possible, with the
/// exception of the block callbacks. In the underlying library, hoedown skips
/// the block if the handler is not registered. This behavior can be confusing
/// when creating a custom renderer, so the default implementations for block
/// handlers is to output a message into the output buffer to make it clearer.
///
///| Type  | Action                          |
///| :---- | :------                         |
///| block | ignore the block                |
///| span  | pass through markdown to output |
///| rest  | pass through content to output  |
///
///
/// Below is an example of a custom renderer that collects emphasis elements
/// into a vector that can then be inspected after rendering.
///
///``` rust
///# use hoedown::{Markdown, Buffer, Render};
///# use hoedown::renderer::html;
///struct EmphCollector {
///    // html renderer to delegate to
///    html: html::Html,
///
///    // collection of emphasis elements
///    emphs: Vec<String>,
///}
///
///impl EmphCollector {
///    fn new() -> EmphCollector {
///        EmphCollector {
///            html: html::Html::new(html::Flags::empty(), ::std::i32::MAX),
///            emphs: vec![],
///        }
///    }
///}
///
///impl Render for EmphCollector {
///    // pass the content straight to the output buffer
///    fn paragraph(&mut self, ob: &mut Buffer, content: &Buffer) {
///        ob.pipe(content);
///    }
///
///    fn emphasis(&mut self, ob: &mut Buffer, content: &Buffer) -> bool {
///        // collect the emphasis element
///        self.emphs.push(String::from(content.to_str().unwrap()));
///
///        // delegate rendering the emphasis element to the html renderer
///        self.html.emphasis(ob, content)
///    }
///}
///
///let doc = Markdown::new("this _one_ that _two_ another _three_ pass it _around_");
///let mut collector = EmphCollector::new();
///
///let output = collector.render(&doc);
///
///assert_eq!(
///    collector.emphs,
///    vec![
///        String::from("one"),
///        String::from("two"),
///        String::from("three"),
///        String::from("around")]);
///
///assert_eq!(
///    "this <em>one</em> that <em>two</em> another <em>three</em> pass it <em>around</em>",
///    output.to_str().unwrap());
///```

#[allow(unused_variables)]
pub trait Render: Sized {
    /// Render the document to a buffer that is returned
    fn render(&mut self, input: &Markdown) -> Buffer {
        let mut output = Buffer::new(64);
        self.render_to(input, &mut output);
        output
    }

    /// Render the document into the given buffer
    fn render_to(&mut self, input: &Markdown, output: &mut Buffer) {
        let renderer = unsafe { self.to_hoedown() };
        let doc = Document::new(&renderer, input.extensions.clone(), input.max_nesting);
        doc.render(&input.contents, output);
    }

    /// Render the document as inline to a buffer that is returned
    fn render_inline(&mut self, input: &Markdown) -> Buffer {
        let mut output = Buffer::new(64);
        self.render_inline_to(input, &mut output);
        output
    }

    /// Render the document as inline into the given buffer
    fn render_inline_to(&mut self, input: &Markdown, output: &mut Buffer) {
        let renderer = unsafe { self.to_hoedown() };
        let doc = Document::new(&renderer, input.extensions.clone(), input.max_nesting);
        doc.render_inline(&input.contents, output);
    }

    /// Converts the type into an underlying `hoedown_renderer` structure.
    ///
    /// The default implementation of this should suffice for the majority of implementations,
    /// unless you know what you're doing.
    ///
    /// The default implementation is unsafe because it stores a pointer to `self` in the
    /// underlying library. Problems could arise if that pointer is used after `self` has
    /// stopped existing.
    ///
    /// This library avoids this issue by tightly controlling the the render process
    /// to ensure that the renderer outlives the document (the document is what
    /// requires `to_hoedown`)
    unsafe fn to_hoedown(&mut self) -> hoedown_renderer {
        use libc::c_void;

        let renderer = hoedown_renderer {
            opaque: self as *mut _ as *mut c_void,

            // block-level handlers are unconditionally registered
            blockcode: Some(wrappers::blockcode::<Self>),
            blockquote: Some(::wrappers::blockquote::<Self>),
            header: Some(wrappers::header::<Self>),
            hrule: Some(wrappers::hrule::<Self>),
            list: Some(wrappers::list::<Self>),
            listitem: Some(wrappers::listitem::<Self>),
            paragraph: Some(wrappers::paragraph::<Self>),
            table: Some(wrappers::table::<Self>),
            table_header: Some(wrappers::table_header::<Self>),
            table_body: Some(wrappers::table_body::<Self>),
            table_row: Some(wrappers::table_row::<Self>),
            table_cell: Some(wrappers::table_cell::<Self>),
            footnotes: Some(wrappers::footnotes::<Self>),
            footnote_def: Some(wrappers::footnote_def::<Self>),
            blockhtml: Some(wrappers::blockhtml::<Self>),

            autolink: Some(wrappers::autolink::<Self>),
            codespan: Some(wrappers::codespan::<Self>),
            double_emphasis: Some(wrappers::double_emphasis::<Self>),
            emphasis: Some(wrappers::emphasis::<Self>),
            underline: Some(wrappers::underline::<Self>),
            highlight: Some(wrappers::highlight::<Self>),
            quote: Some(wrappers::quote::<Self>),
            image: Some(wrappers::image::<Self>),
            linebreak: Some(wrappers::linebreak::<Self>),
            link: Some(wrappers::link::<Self>),
            triple_emphasis: Some(wrappers::triple_emphasis::<Self>),
            strikethrough: Some(wrappers::strikethrough::<Self>),
            superscript: Some(wrappers::superscript::<Self>),
            footnote_ref: Some(wrappers::footnote_ref::<Self>),
            math: Some(wrappers::math::<Self>),
            raw_html: Some(wrappers::raw_html::<Self>),

            entity: Some(wrappers::entity::<Self>),
            normal_text: Some(wrappers::normal_text::<Self>),

            doc_header: Some(wrappers::doc_header::<Self>),
            doc_footer: Some(wrappers::doc_footer::<Self>),
        };

        return renderer;
    }

    // block-level: not registered = skip the block

    /// Runs when a codeblock is encountered
    ///
    /// The `lang` parameter will be empty if it's an indented codeblock
    /// or if no language was specified in a fenced codeblock.
    ///
    /// The default implementation outputs an error string.
    ///
    /// Not run if the `DISABLE_INDENTED_CODE` extension is enabled.
    fn code_block(&mut self, output: &mut Buffer, text: &Buffer, lang: &Buffer) {}

    /// Runs when a block quote is encountered
    ///
    /// The default implementation outputs an error string.
    fn quote_block(&mut self, output: &mut Buffer, content: &Buffer) {}

    /// Runs when a header is encountered
    ///
    /// The default implementation outputs an error string.
    fn header(&mut self, output: &mut Buffer, content: &Buffer, level: i32) {}

    /// Runs when a horizontal rule is encountered
    ///
    /// The default implementation outputs an error string.
    fn horizontal_rule(&mut self, output: &mut Buffer) {}

    /// Runs when a list is encountered.
    ///
    /// The default implementation outputs an error string.
    fn list(&mut self, output: &mut Buffer, content: &Buffer, flags: ::renderer::list::List) {}

    /// Runs when a list item is encountered.
    ///
    /// The default implementation outputs an error string.
    fn list_item(&mut self, output: &mut Buffer, content: &Buffer, flags: ::renderer::list::List) {}

    /// Runs when a paragraph is encountered.
    ///
    /// The default implementation outputs an error string.
    fn paragraph(&mut self, output: &mut Buffer, content: &Buffer) {}

    /// Runs when a table is encountered.
    ///
    /// Only runs if the `TABLES` extension is enabled.
    ///
    /// The default implementation outputs an error string.
    fn table(&mut self, output: &mut Buffer, content: &Buffer) {}

    /// Runs when a table header is encountered.
    ///
    /// Only runs if the `TABLES` extension is enabled.
    ///
    /// The default implementation outputs an error string.
    fn table_header(&mut self, output: &mut Buffer, content: &Buffer) {}

    /// Runs when a table body is encountered.
    ///
    /// Only runs if the `TABLES` extension is enabled.
    ///
    /// The default implementation outputs an error string.
    fn table_body(&mut self, output: &mut Buffer, content: &Buffer) {}

    /// Runs when a table row is encountered.
    ///
    /// Only runs if the `TABLES` extension is enabled.
    ///
    /// The default implementation outputs an error string.
    fn table_row(&mut self, output: &mut Buffer, content: &Buffer) {}

    /// Runs when a table cell is encountered.
    ///
    /// Only runs if the `TABLES` extension is enabled.
    ///
    /// The default implementation outputs an error string.
    fn table_cell(&mut self, output: &mut Buffer, content: &Buffer, flags: ::renderer::Table) {}

    /// Runs when footnotes are encountered.
    ///
    /// Only runs if the `FOOTNOTES` extension is enabled.
    ///
    /// The default implementation outputs an error string.
    fn footnotes(&mut self, output: &mut Buffer, content: &Buffer) {}

    /// Runs when a footnote definition is encountered.
    ///
    /// Only runs if the `FOOTNOTES` extension is enabled.
    ///
    /// The default implementation outputs an error string.
    fn footnote_definition(&mut self, output: &mut Buffer, content: &Buffer, num: u32) {}

    /// Runs when a raw html block is encountered.
    ///
    /// The default implementation outputs an error string.
    fn html_block(&mut self, output: &mut Buffer, text: &Buffer) {}

    // span-level: not registered = pass-through

    /// Runs when an autolink candidate is encountered.
    ///
    /// Only runs if the `AUTOLINK` extension is enabled.
    ///
    /// The default implementation passes the context markdown to the buffer verbatim.
    fn autolink(&mut self, output: &mut Buffer, link: &Buffer, link_type: AutoLink) -> bool {
        false
    }

    /// Runs when a code span is encountered.
    ///
    /// The default implementation passes the context markdown to the buffer verbatim.
    fn code_span(&mut self, output: &mut Buffer, text: &Buffer) -> bool {
        false
    }

    /// Runs when double emphasis is encountered.
    ///
    /// e.g. `**double emphasis**`
    ///
    /// The default implementation passes the context markdown to the buffer verbatim.
    fn double_emphasis(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        false
    }

    /// Runs when emphasis is encountered.
    ///
    /// The default implementation passes the context markdown to the buffer verbatim.
    fn emphasis(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        false
    }

    /// Runs when underline is encountered.
    ///
    /// Only runs if the `UNDERLINE` extension is enabled.
    ///
    /// The default implementation passes the context markdown to the buffer verbatim.
    fn underline(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        false
    }

    /// Runs when highlight is encountered.
    ///
    /// Only runs if the `HIGHLIGHT` extension is enabled.
    ///
    /// The default implementation passes the context markdown to the buffer verbatim.
    fn highlight(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        false
    }

    /// Runs when a quote is encountered.
    ///
    /// Only runs if the `QUOTE` extension is enabled.
    ///
    /// The default implementation passes the context markdown to the buffer verbatim.
    fn quote_span(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        false
    }

    /// Runs when an image is encountered.
    ///
    /// e.g. `![alt](link title)`
    ///
    /// The default implementation passes the context markdown to the buffer verbatim.
    fn image(&mut self, output: &mut Buffer, link: &Buffer, title: &Buffer, alt: &Buffer) -> bool {
        false
    }

    /// Runs when a line break is encountered.
    ///
    /// The default implementation passes the context markdown to the buffer verbatim.
    fn line_break(&mut self, output: &mut Buffer) -> bool {
        false
    }

    /// Runs when a link is encountered.
    ///
    /// e.g. `[content](link title)`
    ///
    /// The default implementation passes the context markdown to the buffer verbatim.
    fn link(&mut self, output: &mut Buffer, content: &Buffer, link: &Buffer, title: &Buffer) -> bool {
        false
    }

    /// Runs when triple emphasis is encountered.
    ///
    /// e.g. `***strongly emphasized***`
    ///
    /// The default implementation passes the context markdown to the buffer verbatim.
    fn triple_emphasis(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        false
    }

    /// Runs when strikethrough is encountered.
    ///
    /// Only runs if the `STRIKETHROUGH` extension is enabled.
    ///
    /// The default implementation passes the context markdown to the buffer verbatim.
    fn strikethrough(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        false
    }

    /// Runs when superscript is encountered.
    ///
    /// Only runs if the `SUPERSCRIPT` extension is enabled.
    ///
    /// The default implementation passes the context markdown to the buffer verbatim.
    fn superscript(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        false
    }

    /// Runs when a footnote reference is encountered.
    ///
    /// Only runs if the `FOOTNOTES` extension is enabled.
    ///
    /// The default implementation passes the context markdown to the buffer verbatim.
    fn footnote_reference(&mut self, output: &mut Buffer, num: u32) -> bool {
        false
    }

    /// Runs when math is encountered.
    ///
    /// Only runs if the `MATH` extension is enabled.
    ///
    /// The default implementation passes the context markdown to the buffer verbatim.
    fn math(&mut self, output: &mut Buffer, text: &Buffer, displaymode: i32) -> bool {
        false
    }

    /// Runs when raw html span is encountered.
    ///
    /// The default implementation passes the context markdown to the buffer verbatim.
    fn html_span(&mut self, output: &mut Buffer, text: &Buffer) -> bool {
        false
    }

    // low-level: not registered = pass-through

    /// Runs when an html entity is encountered.
    ///
    /// The default implementation passes the entity to the output buffer verbatim.
    fn entity(&mut self, output: &mut Buffer, text: &Buffer) {
        output.pipe(text);
    }

    /// Runs when plain text is encountered.
    ///
    /// The default implementation passes the entity to the output buffer verbatim.
    fn normal_text(&mut self, output: &mut Buffer, text: &Buffer) {
        output.pipe(text);
    }

    // misc callbacks

    /// Runs before the document is processed.
    ///
    /// The default implementation does nothing.
    fn before_render(&mut self, output: &mut Buffer, inline_render: bool) {}

    /// Runs after the document has been processed.
    ///
    /// The default implementation does nothing.
    fn after_render(&mut self, output: &mut Buffer, inline_render: bool) {}
}

impl<'a, R> Render for &'a mut R where R: Render {
    unsafe fn to_hoedown(&mut self) -> hoedown_renderer {
        (**self).to_hoedown()
    }

    // block-level: not registered = skip the block
    fn code_block(&mut self, output: &mut Buffer, text: &Buffer, lang: &Buffer) {
        (**self).code_block(output, text, lang)
    }
    fn quote_block(&mut self, output: &mut Buffer, content: &Buffer) {
        (**self).quote_block(output, content)
    }
    fn header(&mut self, output: &mut Buffer, content: &Buffer, level: i32) {
        (**self).header(output, content, level)
    }
    fn horizontal_rule(&mut self, output: &mut Buffer) {
        (**self).horizontal_rule(output)
    }
    fn list(&mut self, output: &mut Buffer, content: &Buffer, flags: list::List) {
        (**self).list(output, content, flags)
    }
    fn list_item(&mut self, output: &mut Buffer, content: &Buffer, flags: list::List) {
        (**self).list_item(output, content, flags)
    }
    fn paragraph(&mut self, output: &mut Buffer, content: &Buffer) {
        (**self).paragraph(output, content)
    }
    fn table(&mut self, output: &mut Buffer, content: &Buffer) {
        (**self).table(output, content)
    }
    fn table_header(&mut self, output: &mut Buffer, content: &Buffer) {
        (**self).table_header(output, content)
    }
    fn table_body(&mut self, output: &mut Buffer, content: &Buffer) {
        (**self).table_body(output, content)
    }
    fn table_row(&mut self, output: &mut Buffer, content: &Buffer) {
        (**self).table_row(output, content)
    }
    fn table_cell(&mut self, output: &mut Buffer, content: &Buffer, flags: Table) {
        (**self).table_cell(output, content, flags)
    }
    fn footnotes(&mut self, output: &mut Buffer, content: &Buffer) {
        (**self).footnotes(output, content)
    }
    fn footnote_definition(&mut self, output: &mut Buffer, content: &Buffer, num: u32) {
        (**self).footnote_definition(output, content, num)
    }
    fn html_block(&mut self, output: &mut Buffer, text: &Buffer) {
        (**self).html_block(output, text)
    }

    // span-level: not registered = pass-through
    fn autolink(&mut self, output: &mut Buffer, link: &Buffer, link_type: AutoLink) -> bool {
        (**self).autolink(output, link, link_type)
    }
    fn code_span(&mut self, output: &mut Buffer, text: &Buffer) -> bool {
        (**self).code_span(output, text)
    }
    fn double_emphasis(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        (**self).double_emphasis(output, content)
    }
    fn emphasis(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        (**self).emphasis(output, content)
    }
    fn underline(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        (**self).underline(output, content)
    }
    fn highlight(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        (**self).highlight(output, content)
    }
    fn quote_span(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        (**self).quote_span(output, content)
    }
    fn image(&mut self, output: &mut Buffer, link: &Buffer, title: &Buffer, alt: &Buffer) -> bool {
        (**self).image(output, link, title, alt)
    }
    fn line_break(&mut self, output: &mut Buffer) -> bool {
        (**self).line_break(output)
    }
    fn link(&mut self, output: &mut Buffer, content: &Buffer, link: &Buffer, title: &Buffer) -> bool {
        (**self).link(output, content, link, title)
    }
    fn triple_emphasis(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        (**self).triple_emphasis(output, content)
    }
    fn strikethrough(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        (**self).strikethrough(output, content)
    }
    fn superscript(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
        (**self).superscript(output, content)
    }
    fn footnote_reference(&mut self, output: &mut Buffer, num: u32) -> bool {
        (**self).footnote_reference(output, num)
    }
    fn math(&mut self, output: &mut Buffer, text: &Buffer, displaymode: i32) -> bool {
        (**self).math(output, text, displaymode)
    }
    fn html_span(&mut self, output: &mut Buffer, text: &Buffer) -> bool {
        (**self).html_span(output, text)
    }

    // low-level: not registered = pass-through
    fn entity(&mut self, output: &mut Buffer, text: &Buffer) {
        (**self).entity(output, text)
    }
    fn normal_text(&mut self, output: &mut Buffer, text: &Buffer) {
        (**self).normal_text(output, text)
    }

    // misc callbacks
    fn before_render(&mut self, output: &mut Buffer, inline_render: bool) {
        (**self).before_render(output, inline_render)
    }
    fn after_render(&mut self, output: &mut Buffer, inline_render: bool) {
        (**self).after_render(output, inline_render)
    }
}

/// Flags that describe a list or list item
pub mod list {
    bitflags! {
        #[doc="Flags that describe a list or list item"]
        #[repr(C)]
        flags List: u32 {
            #[doc="An ordered list or list item"]
            const ORDERED = 1 << 0,

            #[doc="A list item that contains a block"]
            const BLOCK   = 1 << 1,
        }
    }
}

/// The table alignment or position
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub enum Table {
    Left = 1,
    Right,
    Center,
    Mask,
    Header,
}

/// The type of an autolink candidate
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub enum AutoLink {
    Normal = 1,
    Email,
}

pub mod wrapper;
pub mod html;
pub mod trace;

