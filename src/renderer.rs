//! Contains Render behavior and stock renderers

use buffer::Buffer;
use wrappers;
use ffi::hoedown_renderer;

/// Represents render behavior
///
/// Types that implement this trait can be used to render a `Markdown` document.
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
///| Type  | Action                                       |
///| :---- | :------                                      |
///| block | write "MISSING <callback> HANDLER" to output |
///| span  | pass through markdown to output              |
///| rest  | pass through content to output               |
///
///
/// Below is an example of a custom renderer that collects emphasis elements
/// into a vector that can then be inspected after rendering.
///
///``` rust
///# use hoedown::Markdown;
///# use hoedown::renderer::{Render, html};
///# use hoedown::buffer::Buffer;
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
///        self.emphs.push(content.as_str().unwrap().to_string());
///
///        // delegate rendering the emphasis element to the html renderer
///        self.html.emphasis(ob, content)
///    }
///}
///
///let doc = Markdown::new("this _one_ that _two_ another _three_ pass it _around_".as_bytes());
///let mut collector = EmphCollector::new();
///
///let output = doc.render_to_buffer(&mut collector);
///
///assert_eq!(
///    collector.emphs,
///    vec![
///        "one".to_string(),
///        "two".to_string(),
///        "three".to_string(),
///        "around".to_string()]);
///
///assert_eq!(
///    "this <em>one</em> that <em>two</em> another <em>three</em> pass it <em>around</em>",
///    output.as_str().unwrap());
///```

#[allow(unused_variables)]
pub trait Render: Sized {
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
    fn code_block(&mut self, output: &mut Buffer, text: &Buffer, lang: &Buffer) {
        output.write_str("MISSING CODE_BLOCK HANDLER\n").unwrap();
    }

    /// Runs when a block quote is encountered
    ///
    /// The default implementation outputs an error string.
    fn quote_block(&mut self, output: &mut Buffer, content: &Buffer) {
        output.write_str("MISSING QUOTE_BLOCK HANDLER\n").unwrap();
    }

    /// Runs when a header is encountered
    ///
    /// The default implementation outputs an error string.
    fn header(&mut self, output: &mut Buffer, content: &Buffer, level: i32) {
        output.write_str("MISSING HEADER HANDLER\n").unwrap();
    }

    /// Runs when a horizontal rule is encountered
    ///
    /// The default implementation outputs an error string.
    fn horizontal_rule(&mut self, output: &mut Buffer) {
        output.write_str("MISSING HORIZONTAL_RULE HANDLER\n").unwrap();
    }

    /// Runs when a list is encountered.
    ///
    /// The default implementation outputs an error string.
    fn list(&mut self, output: &mut Buffer, content: &Buffer, flags: ::renderer::list::List) {
        output.write_str("MISSING LIST HANDLER\n").unwrap();
    }

    /// Runs when a list item is encountered.
    ///
    /// The default implementation outputs an error string.
    fn list_item(&mut self, output: &mut Buffer, content: &Buffer, flags: ::renderer::list::List) {
        output.write_str("MISSING LIST_ITEM HANDLER\n").unwrap();
    }

    /// Runs when a paragraph is encountered.
    ///
    /// The default implementation outputs an error string.
    fn paragraph(&mut self, output: &mut Buffer, content: &Buffer) {
        output.write_str("MISSING PARAGRAPH HANDLER\n").unwrap();
    }

    /// Runs when a table is encountered.
    ///
    /// Only runs if the `TABLES` extension is enabled.
    ///
    /// The default implementation outputs an error string.
    fn table(&mut self, output: &mut Buffer, content: &Buffer) {
        output.write_str("MISSING TABLE HANDLER\n").unwrap();
    }

    /// Runs when a table header is encountered.
    ///
    /// Only runs if the `TABLES` extension is enabled.
    ///
    /// The default implementation outputs an error string.
    fn table_header(&mut self, output: &mut Buffer, content: &Buffer) {
        output.write_str("MISSING TABLE_HEADER HANDLER\n").unwrap();
    }

    /// Runs when a table body is encountered.
    ///
    /// Only runs if the `TABLES` extension is enabled.
    ///
    /// The default implementation outputs an error string.
    fn table_body(&mut self, output: &mut Buffer, content: &Buffer) {
        output.write_str("MISSING TABLE_BODY HANDLER\n").unwrap();
    }

    /// Runs when a table row is encountered.
    ///
    /// Only runs if the `TABLES` extension is enabled.
    ///
    /// The default implementation outputs an error string.
    fn table_row(&mut self, output: &mut Buffer, content: &Buffer) {
        output.write_str("MISSING TABLE_ROW HANDLER\n").unwrap();
    }

    /// Runs when a table cell is encountered.
    ///
    /// Only runs if the `TABLES` extension is enabled.
    ///
    /// The default implementation outputs an error string.
    fn table_cell(&mut self, output: &mut Buffer, content: &Buffer, flags: ::renderer::Table) {
        output.write_str("MISSING TABLE_CELL HANDLER\n").unwrap();
    }

    /// Runs when footnotes are encountered.
    ///
    /// Only runs if the `FOOTNOTES` extension is enabled.
    ///
    /// The default implementation outputs an error string.
    fn footnotes(&mut self, output: &mut Buffer, content: &Buffer) {
        output.write_str("MISSING FOOTNOTES HANDLER\n").unwrap();
    }

    /// Runs when a footnote definition is encountered.
    ///
    /// Only runs if the `FOOTNOTES` extension is enabled.
    ///
    /// The default implementation outputs an error string.
    fn footnote_definition(&mut self, output: &mut Buffer, content: &Buffer, num: u32) {
        output.write_str("MISSING FOOTNOTE_DEFINITION HANDLER\n").unwrap();
    }

    /// Runs when a raw html block is encountered.
    ///
    /// The default implementation outputs an error string.
    fn html_block(&mut self, output: &mut Buffer, text: &Buffer) {
        output.write_str("MISSING HTML_BLOCK HANDLER\n").unwrap();
    }

    // span-level: not registered = pass-through

    /// Runs when an autolink candidate is encountered.
    ///
    /// Only runs if the `AUTOLINK` extension is enabled.
    ///
    /// The default implementation passes the context markdown to the buffer verbatim.
    fn autolink(&mut self, output: &mut Buffer, link: &Buffer, link_type: ::renderer::AutoLink) -> bool {
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
    fn list(&mut self, output: &mut Buffer, content: &Buffer, flags: ::renderer::list::List) {
        (**self).list(output, content, flags)
    }
    fn list_item(&mut self, output: &mut Buffer, content: &Buffer, flags: ::renderer::list::List) {
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
    fn table_cell(&mut self, output: &mut Buffer, content: &Buffer, flags: ::renderer::Table) {
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
    fn autolink(&mut self, output: &mut Buffer, link: &Buffer, link_type: ::renderer::AutoLink) -> bool {
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
        flags List: u32 {
            #[doc="An ordered list or list item"]
            const ORDERED = 1 << 0,

            #[doc="A list item that contains a block"]
            const BLOCK   = 1 << 1,
        }
    }
}

/// The table alignment or position
#[derive(Debug, Copy)]
pub enum Table {
    Left = 1,
    Right,
    Center,
    Mask,
    Header,
}

/// The type of an autolink candidate
#[derive(Debug, Copy)]
pub enum AutoLink {
    Normal = 1,
    Email,
}

/// Contains the html renderer and utilities
pub mod html {
    use std::ptr::Unique;
    use libc::{c_void, c_int};

    use buffer::Buffer;
    use ffi::{
        hoedown_renderer,
        hoedown_html_renderer_new,
        hoedown_html_toc_renderer_new,
        hoedown_html_smartypants,
        hoedown_html_renderer_free
    };

    use super::Render;

    /// Performs "smartypants" processing of the provided buffer.
    ///
    /// This turns, for example, straight quotes `"test"` into curly quotes `“test”`
    pub fn smartypants(output: &mut Buffer, content: &Buffer) {
        unsafe {
            hoedown_html_smartypants(
                output.get_mut(),
                content.get().data,
                content.get().size);
        }
    }

    /// Flags to control the behavior of the html renderer
    bitflags! {
        #[doc="Information about a list item"]
        flags Flags: u32 {
            #[doc="Ignore raw html"]
            const SKIP_HTML = 1 << 0,

            #[doc="Ignore raw html blocks and escape html spans"]
            const ESCAPE    = 1 << 1,

            #[doc="Insert breaks inside paragraphs for every newline"]
            const HARD_WRAP = 1 << 2,

            #[doc="Output XHTML"]
            const USE_XHTML = 1 << 3,
        }
    }

    /// HTML renderer
    pub struct Html {
        renderer: Unique<hoedown_renderer>,
    }

    impl Html {
        /// Construct a new html renderer given the provided html flags
        /// and table of contents nesting level.
        ///
        /// The `toc` method can be used to construct a table of contents renderer
        /// which renders _only_ the table of contents. The `nesting_level` on this
        /// method determines the maximum depth of headers to associate with the
        /// table of contents.
        ///
        /// For this reason, if a table of contents is going to be rendered, this
        /// method's `nesting_level` argument should be the same as the one passed
        /// to the `toc` method.
        pub fn new(flags: Flags, nesting_level: i32) -> Html {
            let renderer = unsafe {
                hoedown_html_renderer_new(flags.bits(), nesting_level as c_int)
            };

            Html {
                renderer: Unique(renderer),
            }
        }

        /// Construct a table of contents renderer.
        ///
        /// This renderer will _only_ render the table of contents.
        /// If you want to have the headers of the document specify `id` attributes
        /// so that the table of contents items link to the correct header, you should
        /// render the document with the renderer returned by the `new` method with the
        /// same value for the `nesting_level` parameter.
        pub fn toc(nesting_level: i32) -> Html {
            let renderer = unsafe {
               hoedown_html_toc_renderer_new(nesting_level as c_int)
            };

            Html {
                renderer: Unique(renderer),
            }
        }

        /// Get a reference to the underlying hoedown renderer
        pub fn get(&self) -> &hoedown_renderer {
            unsafe { &*self.renderer.0 }
        }

        /// Get a mutable reference to the underlying hoedown renderer
        pub fn get_mut(&mut self) -> &mut hoedown_renderer {
            unsafe { &mut *self.renderer.0 }
        }
    }

    impl Render for Html {
        unsafe fn to_hoedown(&mut self) -> hoedown_renderer {
            *self.renderer.0
        }

        fn code_block(&mut self, ob: &mut Buffer, text: &Buffer, lang: &Buffer) {
            let data = self.renderer.0 as *mut c_void;
            let func = unsafe { (*self.renderer.0).blockcode.unwrap() };
            func(ob.get_mut(), text.get(), lang.get(), data);
        }

        fn quote_block(&mut self, ob: &mut Buffer, content: &Buffer) {
            let data = self.renderer.0 as *mut c_void;
            let func = unsafe { (*self.renderer.0).blockquote.unwrap() };
            func(ob.get_mut(), content.get(), data);
        }

        fn header(&mut self, ob: &mut Buffer, content: &Buffer, level: i32) {
            let data = self.renderer.0 as *mut c_void;
            let func = unsafe { (*self.renderer.0).header.unwrap() };
            func(ob.get_mut(), content.get(), level, data);
        }

        fn horizontal_rule(&mut self, ob: &mut Buffer) {
            let data = self.renderer.0 as *mut c_void;
            let func = unsafe { (*self.renderer.0).hrule.unwrap() };
            func(ob.get_mut(), data);
        }

        fn list(&mut self, ob: &mut Buffer, content: &Buffer, flags: ::renderer::list::List) {
            let data = self.renderer.0 as *mut c_void;
            let func = unsafe { (*self.renderer.0).list.unwrap() };
            func(ob.get_mut(), content.get(), flags, data);
        }

        fn list_item(&mut self, ob: &mut Buffer, content: &Buffer, flags: ::renderer::list::List) {
            let data = self.renderer.0 as *mut c_void;
            let func = unsafe { (*self.renderer.0).listitem.unwrap() };
            func(ob.get_mut(), content.get(), flags, data);
        }

        fn paragraph(&mut self, ob: &mut Buffer, content: &Buffer) {
            let data = self.renderer.0 as *mut c_void;
            let func = unsafe { (*self.renderer.0).paragraph.unwrap() };
            func(ob.get_mut(), content.get(), data);
        }

        fn table(&mut self, ob: &mut Buffer, content: &Buffer) {
            let data = self.renderer.0 as *mut c_void;
            let func = unsafe { (*self.renderer.0).table.unwrap() };
            func(ob.get_mut(), content.get(), data);
        }

        fn table_header(&mut self, ob: &mut Buffer, content: &Buffer) {
            let data = self.renderer.0 as *mut c_void;
            let func = unsafe { (*self.renderer.0).table_header.unwrap() };
            func(ob.get_mut(), content.get(), data);
        }

        fn table_body(&mut self, ob: &mut Buffer, content: &Buffer) {
            let data = self.renderer.0 as *mut c_void;
            let func = unsafe { (*self.renderer.0).table_body.unwrap() };
            func(ob.get_mut(), content.get(), data);
        }

        fn table_row(&mut self, ob: &mut Buffer, content: &Buffer) {
            let data = self.renderer.0 as *mut c_void;
            let func = unsafe { (*self.renderer.0).table_row.unwrap() };
            func(ob.get_mut(), content.get(), data);
        }

        fn table_cell(&mut self, ob: &mut Buffer, content: &Buffer, flags: ::renderer::Table) {
            let data = self.renderer.0 as *mut c_void;
            let func = unsafe { (*self.renderer.0).table_cell.unwrap() };
            func(ob.get_mut(), content.get(), flags, data);
        }

        fn footnotes(&mut self, ob: &mut Buffer, content: &Buffer) {
            let data = self.renderer.0 as *mut c_void;
            let func = unsafe { (*self.renderer.0).footnotes.unwrap() };
            func(ob.get_mut(), content.get(), data);
        }

        fn footnote_definition(&mut self, ob: &mut Buffer, content: &Buffer, num: u32) {
            let data = self.renderer.0 as *mut c_void;
            let func = unsafe { (*self.renderer.0).footnote_def.unwrap() };
            func(ob.get_mut(), content.get(), num as u32, data);
        }

        fn html_block(&mut self, ob: &mut Buffer, text: &Buffer) {
            let data = self.renderer.0 as *mut c_void;
            let func = unsafe { (*self.renderer.0).blockhtml.unwrap() };
            func(ob.get_mut(), text.get(), data);
        }

        fn autolink(&mut self, ob: &mut Buffer, link: &Buffer, ty: ::renderer::AutoLink) -> bool {
            let data = self.renderer.0 as *mut c_void;
            let func = unsafe { (*self.renderer.0).autolink.unwrap() };
            func(ob.get_mut(), link.get(), ty, data) != 0
        }

        fn code_span(&mut self, ob: &mut Buffer, text: &Buffer) -> bool {
            let data = self.renderer.0 as *mut c_void;
            let func = unsafe { (*self.renderer.0).codespan.unwrap() };
            func(ob.get_mut(), text.get(), data) != 0
        }

        fn double_emphasis(&mut self, ob: &mut Buffer, content: &Buffer) -> bool {
            let data = self.renderer.0 as *mut c_void;
            let func = unsafe { (*self.renderer.0).double_emphasis.unwrap() };
            func(ob.get_mut(), content.get(), data) != 0
        }

        fn emphasis(&mut self, ob: &mut Buffer, content: &Buffer) -> bool {
            let data = self.renderer.0 as *mut c_void;
            let func = unsafe { (*self.renderer.0).emphasis.unwrap() };
            func(ob.get_mut(), content.get(), data) != 0
        }

        fn underline(&mut self, ob: &mut Buffer, content: &Buffer) -> bool {
            let data = self.renderer.0 as *mut c_void;
            let func = unsafe { (*self.renderer.0).underline.unwrap() };
            func(ob.get_mut(), content.get(), data) != 0
        }

        fn highlight(&mut self, ob: &mut Buffer, content: &Buffer) -> bool {
            let data = self.renderer.0 as *mut c_void;
            let func = unsafe { (*self.renderer.0).highlight.unwrap() };
            func(ob.get_mut(), content.get(), data) != 0
        }

        fn quote_span(&mut self, ob: &mut Buffer, content: &Buffer) -> bool {
            let data = self.renderer.0 as *mut c_void;
            let func = unsafe { (*self.renderer.0).quote.unwrap() };
            func(ob.get_mut(), content.get(), data) != 0
        }

        fn image(&mut self, ob: &mut Buffer, link: &Buffer, title: &Buffer, alt: &Buffer) -> bool {
            let data = self.renderer.0 as *mut c_void;
            let func = unsafe { (*self.renderer.0).image.unwrap() };
            func(ob.get_mut(), link.get(), title.get(), alt.get(), data) != 0
        }

        fn line_break(&mut self, ob: &mut Buffer) -> bool {
            let data = self.renderer.0 as *mut c_void;
            let func = unsafe { (*self.renderer.0).linebreak.unwrap() };
            func(ob.get_mut(), data) != 0
        }

        fn link(&mut self, ob: &mut Buffer, content: &Buffer, link: &Buffer, title: &Buffer) -> bool {
            let data = self.renderer.0 as *mut c_void;
            let func = unsafe { (*self.renderer.0).link.unwrap() };
            func(ob.get_mut(), content.get(), link.get(), title.get(), data) != 0
        }

        fn triple_emphasis(&mut self, ob: &mut Buffer, content: &Buffer) -> bool {
            let data = self.renderer.0 as *mut c_void;
            let func = unsafe { (*self.renderer.0).triple_emphasis.unwrap() };
            func(ob.get_mut(), content.get(), data) != 0
        }

        fn strikethrough(&mut self, ob: &mut Buffer, content: &Buffer) -> bool {
            let data = self.renderer.0 as *mut c_void;
            let func = unsafe { (*self.renderer.0).strikethrough.unwrap() };
            func(ob.get_mut(), content.get(), data) != 0
        }

        fn superscript(&mut self, ob: &mut Buffer, content: &Buffer) -> bool {
            let data = self.renderer.0 as *mut c_void;
            let func = unsafe { (*self.renderer.0).superscript.unwrap() };
            func(ob.get_mut(), content.get(), data) != 0
        }

        fn footnote_reference(&mut self, ob: &mut Buffer, num: u32) -> bool {
            let data = self.renderer.0 as *mut c_void;
            let func = unsafe { (*self.renderer.0).footnote_ref.unwrap() };
            func(ob.get_mut(), num as u32, data) != 0
        }

        fn math(&mut self, ob: &mut Buffer, text: &Buffer, displaymode: i32) -> bool {
            let data = self.renderer.0 as *mut c_void;
            let func = unsafe { (*self.renderer.0).math.unwrap() };
            func(ob.get_mut(), text.get(), displaymode, data) != 0
        }

        fn html_span(&mut self, ob: &mut Buffer, text: &Buffer) -> bool {
            let data = self.renderer.0 as *mut c_void;
            let func = unsafe { (*self.renderer.0).raw_html.unwrap() };
            func(ob.get_mut(), text.get(), data) != 0
        }

        fn entity(&mut self, ob: &mut Buffer, text: &Buffer) {
            let data = self.renderer.0 as *mut c_void;
            let func = unsafe { (*self.renderer.0).entity.unwrap() };
            func(ob.get_mut(), text.get(), data);
        }

        fn normal_text(&mut self, ob: &mut Buffer, text: &Buffer) {
            let data = self.renderer.0 as *mut c_void;
            let func = unsafe { (*self.renderer.0).normal_text.unwrap() };
            func(ob.get_mut(), text.get(), data);
        }

        fn before_render(&mut self, ob: &mut Buffer, inline_render: bool) {
            let data = self.renderer.0 as *mut c_void;
            let func = unsafe { (*self.renderer.0).doc_header.unwrap() };
            func(ob.get_mut(), inline_render as i32, data);
        }

        fn after_render(&mut self, ob: &mut Buffer, inline_render: bool) {
            let data = self.renderer.0 as *mut c_void;
            let func = unsafe { (*self.renderer.0).doc_footer.unwrap() };
            func(ob.get_mut(), inline_render as i32, data);
        }
    }

    impl Drop for Html {
        fn drop(&mut self) {
            unsafe { hoedown_html_renderer_free(self.renderer.0); }
        }
    }
}

/// Contains the Closures renderer
#[allow(non_camel_case_types)]
pub mod closures {
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
    /// This renderer is meant for quick, one-off renderers.
    /// Handlers are passed to this type in the form of closures.
    ///
    ///``` rust
    ///# use hoedown::Markdown;
    ///# use hoedown::renderer::closures::Closures;
    ///# use hoedown::buffer::Buffer;
    ///let mut closures = Closures::new();
    ///
    ///closures.on_paragraph(|output: &mut Buffer, content: &Buffer| {
    ///    output.pipe(content);
    ///});
    ///
    ///closures.on_emphasis(|output: &mut Buffer, content: &Buffer| -> bool {
    ///    output.write_str("~~");
    ///    output.pipe(content);
    ///    output.write_str("~~");
    ///    true
    ///});
    ///
    ///let doc = Markdown::new("this _requires_ emphasis".as_bytes());
    ///let output = doc.render_to_buffer(closures);
    ///
    ///assert_eq!(output.as_str().unwrap(), "this ~~requires~~ emphasis");
    ///```
    pub struct Closures<'a> {
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

    impl <'a> Closures<'a> {
        pub fn new() -> Closures<'a> {
            Closures {
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

    impl<'a> Render for Closures<'a> {
        fn code_block(&mut self, output: &mut Buffer, text: &Buffer, lang: &Buffer) {
            if let Some(ref mut func) = self.code_block {
                func(output, text, lang);
            } else {
                output.write_str("MISSING CODE_BLOCK HANDLER\n").unwrap();
            }
        }

        fn quote_block(&mut self, output: &mut Buffer, content: &Buffer) {
            if let Some(ref mut func) = self.quote_block {
                func(output, content);
            } else {
                output.write_str("MISSING quote_block HANDLER\n").unwrap();
            }
        }

        fn header(&mut self, output: &mut Buffer, content: &Buffer, level: i32) {
            if let Some(ref mut func) = self.header {
                func(output, content, level);
            } else {
                output.write_str("MISSING HEADER HANDLER\n").unwrap();
            }
        }

        fn horizontal_rule(&mut self, output: &mut Buffer) {
            if let Some(ref mut func) = self.horizontal_rule {
                func(output);
            } else {
                output.write_str("MISSING HORIZONTAL_RULE HANDLER\n").unwrap();
            }
        }

        fn list(&mut self, output: &mut Buffer, content: &Buffer, flags: ::renderer::list::List) {
            if let Some(ref mut func) = self.list {
                func(output, content, flags);
            } else {
                output.write_str("MISSING LIST HANDLER\n").unwrap();
            }
        }

        fn list_item(&mut self, output: &mut Buffer, content: &Buffer, flags: ::renderer::list::List) {
            if let Some(ref mut func) = self.list_item {
                func(output, content, flags);
            } else {
                output.write_str("MISSING list_item HANDLER\n").unwrap();
            }
        }

        fn paragraph(&mut self, output: &mut Buffer, content: &Buffer) {
            if let Some(ref mut func) = self.paragraph {
                func(output, content);
            } else {
                output.write_str("MISSING PARAGRAPH HANDLER\n").unwrap();
            }
        }

        fn table(&mut self, output: &mut Buffer, content: &Buffer) {
            if let Some(ref mut func) = self.table {
                func(output, content);
            } else {
                output.write_str("MISSING TABLE HANDLER\n").unwrap();
            }
        }

        fn table_header(&mut self, output: &mut Buffer, content: &Buffer) {
            if let Some(ref mut func) = self.table_header {
                func(output, content);
            } else {
                output.write_str("MISSING TABLE_HEADER HANDLER\n").unwrap();
            }
        }

        fn table_body(&mut self, output: &mut Buffer, content: &Buffer) {
            if let Some(ref mut func) = self.table_body {
                func(output, content);
            } else {
                output.write_str("MISSING TABLE_BODY HANDLER\n").unwrap();
            }
        }

        fn table_row(&mut self, output: &mut Buffer, content: &Buffer) {
            if let Some(ref mut func) = self.table_row {
                func(output, content);
            } else {
                output.write_str("MISSING TABLE_ROW HANDLER\n").unwrap();
            }
        }

        fn table_cell(&mut self, output: &mut Buffer, content: &Buffer, flags: ::renderer::Table) {
            if let Some(ref mut func) = self.table_cell {
                func(output, content, flags);
            } else {
                output.write_str("MISSING TABLE_CELL HANDLER\n").unwrap();
            }
        }

        fn footnotes(&mut self, output: &mut Buffer, content: &Buffer) {
            if let Some(ref mut func) = self.footnotes {
                func(output, content);
            } else {
                output.write_str("MISSING FOOTNOTES HANDLER\n").unwrap();
            }
        }

        fn footnote_definition(&mut self, output: &mut Buffer, content: &Buffer, num: u32) {
            if let Some(ref mut func) = self.footnote_definition {
                func(output, content, num as u32);
            } else {
                output.write_str("MISSING FOOTNOTE_DEFINITION HANDLER\n").unwrap();
            }
        }

        fn html_block(&mut self, output: &mut Buffer, text: &Buffer) {
            if let Some(ref mut func) = self.html_block {
                func(output, text);
            } else {
                output.write_str("MISSING html_block HANDLER\n").unwrap();
            }
        }

        fn autolink(&mut self, output: &mut Buffer, link: &Buffer, ty: ::renderer::AutoLink) -> bool {
            if let Some(ref mut func) = self.autolink {
                func(output, link, ty)
            } else {
                false
            }
        }

        fn code_span(&mut self, output: &mut Buffer, text: &Buffer) -> bool {
            if let Some(ref mut func) = self.code_span {
                func(output, text)
            } else {
                false
            }
        }

        fn double_emphasis(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
            if let Some(ref mut func) = self.double_emphasis {
                func(output, content)
            } else {
                false
            }
        }

        fn emphasis(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
            if let Some(ref mut func) = self.emphasis {
                func(output, content)
            } else {
                false
            }
        }

        fn underline(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
            if let Some(ref mut func) = self.underline {
                func(output, content)
            } else {
                false
            }
        }

        fn highlight(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
            if let Some(ref mut func) = self.highlight {
                func(output, content)
            } else {
                false
            }
        }

        fn quote_span(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
            if let Some(ref mut func) = self.quote {
                func(output, content)
            } else {
                false
            }
        }

        fn image(&mut self, output: &mut Buffer, link: &Buffer, title: &Buffer, alt: &Buffer) -> bool {
            if let Some(ref mut func) = self.image {
                func(output, link, title, alt)
            } else {
                false
            }
        }

        fn line_break(&mut self, output: &mut Buffer) -> bool {
            if let Some(ref mut func) = self.line_break {
                func(output)
            } else {
                false
            }
        }

        fn link(&mut self, output: &mut Buffer, content: &Buffer, link: &Buffer, title: &Buffer) -> bool {
            if let Some(ref mut func) = self.link {
                func(output, content, link, title)
            } else {
                false
            }
        }

        fn triple_emphasis(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
            if let Some(ref mut func) = self.triple_emphasis {
                func(output, content)
            } else {
                false
            }
        }

        fn strikethrough(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
            if let Some(ref mut func) = self.strikethrough {
                func(output, content)
            } else {
                false
            }
        }

        fn superscript(&mut self, output: &mut Buffer, content: &Buffer) -> bool {
            if let Some(ref mut func) = self.superscript {
                func(output, content)
            } else {
                false
            }
        }

        fn footnote_reference(&mut self, output: &mut Buffer, num: u32) -> bool {
            if let Some(ref mut func) = self.footnote_reference {
                func(output, num as u32)
            } else {
                false
            }
        }

        fn math(&mut self, output: &mut Buffer, text: &Buffer, displaymode: i32) -> bool {
            if let Some(ref mut func) = self.math {
                func(output, text, displaymode)
            } else {
                false
            }
        }

        fn html_span(&mut self, output: &mut Buffer, text: &Buffer) -> bool {
            if let Some(ref mut func) = self.html_span {
                func(output, text)
            } else {
                false
            }
        }

        fn entity(&mut self, output: &mut Buffer, text: &Buffer) {
            if let Some(ref mut func) = self.entity {
                func(output, text);
            } else {
                output.pipe(text);
            }
        }

        fn normal_text(&mut self, output: &mut Buffer, text: &Buffer) {
            if let Some(ref mut func) = self.normal_text {
                func(output, text);
            } else {
                output.pipe(text);
            }
        }

        fn before_render(&mut self, output: &mut Buffer, inline_render: bool) {
            if let Some(ref mut func) = self.before_render {
                func(output, inline_render as i32);
            }
        }

        fn after_render(&mut self, output: &mut Buffer, inline_render: bool) {
            if let Some(ref mut func) = self.after_render {
                func(output, inline_render as i32);
            }
        }
    }

    impl<'a> Closures<'a> {
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
}

