//! Contains the html renderer and utilities
use std::ptr::Unique;
use libc::c_void;

use buffer::Buffer;
use ffi::{
    hoedown_buffer,
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
    let content: &hoedown_buffer = content.as_ref();

    unsafe {
        hoedown_html_smartypants(
            output.as_mut(),
            content.data,
            content.size);
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
///
/// This can be used to render markdown documents to HTML. This
/// type can also be leveraged to create custom renderers that delegate
/// to the HTML renderer in certain cases, as shown in the `Render` trait
/// documentation example.
///
///``` rust
///# #![feature(io)]
///# use hoedown::renderer::html::{Html, Flags};
///# use hoedown::{Buffer, Render};
///let input = Buffer::from("EMPHASIZE");
///let mut output = Buffer::new(64usize);
///let mut html_renderer = Html::new(Flags::empty(), 0);
///
///html_renderer.emphasis(&mut output, &input);
///
///assert_eq!(output.to_str().unwrap(), "<em>EMPHASIZE</em>");
///```
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
            hoedown_html_renderer_new(flags.bits(), nesting_level)
        };

        Html {
            renderer: unsafe { Unique::new(renderer) },
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
           hoedown_html_toc_renderer_new(nesting_level)
        };

        Html {
            renderer: unsafe { Unique::new(renderer) },
        }
    }

    /// Get a reference to the underlying hoedown renderer
    pub fn get(&self) -> &hoedown_renderer {
        unsafe { self.renderer.get() }
    }

    /// Get a mutable reference to the underlying hoedown renderer
    pub fn as_mut(&mut self) -> &mut hoedown_renderer {
        unsafe { self.renderer.get_mut() }
    }
}

impl Render for Html {
    unsafe fn to_hoedown(&mut self) -> hoedown_renderer {
        *self.renderer.get()
    }

    fn code_block(&mut self, ob: &mut Buffer, text: &Buffer, lang: &Buffer) {
        let data = *self.renderer as *mut c_void;
        let func = unsafe { self.renderer.get().blockcode.unwrap() };
        func(ob.as_mut(), text.as_ref(), lang.as_ref(), data);
    }

    fn quote_block(&mut self, ob: &mut Buffer, content: &Buffer) {
        let data = *self.renderer as *mut c_void;
        let func = unsafe { self.renderer.get().blockquote.unwrap() };
        func(ob.as_mut(), content.as_ref(), data);
    }

    fn header(&mut self, ob: &mut Buffer, content: &Buffer, level: i32) {
        let data = *self.renderer as *mut c_void;
        let func = unsafe { self.renderer.get().header.unwrap() };
        func(ob.as_mut(), content.as_ref(), level, data);
    }

    fn horizontal_rule(&mut self, ob: &mut Buffer) {
        let data = *self.renderer as *mut c_void;
        let func = unsafe { self.renderer.get().hrule.unwrap() };
        func(ob.as_mut(), data);
    }

    fn list(&mut self, ob: &mut Buffer, content: &Buffer, flags: ::renderer::list::List) {
        let data = *self.renderer as *mut c_void;
        let func = unsafe { self.renderer.get().list.unwrap() };
        func(ob.as_mut(), content.as_ref(), flags, data);
    }

    fn list_item(&mut self, ob: &mut Buffer, content: &Buffer, flags: ::renderer::list::List) {
        let data = *self.renderer as *mut c_void;
        let func = unsafe { self.renderer.get().listitem.unwrap() };
        func(ob.as_mut(), content.as_ref(), flags, data);
    }

    fn paragraph(&mut self, ob: &mut Buffer, content: &Buffer) {
        let data = *self.renderer as *mut c_void;
        let func = unsafe { self.renderer.get().paragraph.unwrap() };
        func(ob.as_mut(), content.as_ref(), data);
    }

    fn table(&mut self, ob: &mut Buffer, content: &Buffer) {
        let data = *self.renderer as *mut c_void;
        let func = unsafe { self.renderer.get().table.unwrap() };
        func(ob.as_mut(), content.as_ref(), data);
    }

    fn table_header(&mut self, ob: &mut Buffer, content: &Buffer) {
        let data = *self.renderer as *mut c_void;
        let func = unsafe { self.renderer.get().table_header.unwrap() };
        func(ob.as_mut(), content.as_ref(), data);
    }

    fn table_body(&mut self, ob: &mut Buffer, content: &Buffer) {
        let data = *self.renderer as *mut c_void;
        let func = unsafe { self.renderer.get().table_body.unwrap() };
        func(ob.as_mut(), content.as_ref(), data);
    }

    fn table_row(&mut self, ob: &mut Buffer, content: &Buffer) {
        let data = *self.renderer as *mut c_void;
        let func = unsafe { self.renderer.get().table_row.unwrap() };
        func(ob.as_mut(), content.as_ref(), data);
    }

    fn table_cell(&mut self, ob: &mut Buffer, content: &Buffer, flags: ::renderer::Table) {
        let data = *self.renderer as *mut c_void;
        let func = unsafe { self.renderer.get().table_cell.unwrap() };
        func(ob.as_mut(), content.as_ref(), flags, data);
    }

    fn footnotes(&mut self, ob: &mut Buffer, content: &Buffer) {
        let data = *self.renderer as *mut c_void;
        let func = unsafe { self.renderer.get().footnotes.unwrap() };
        func(ob.as_mut(), content.as_ref(), data);
    }

    fn footnote_definition(&mut self, ob: &mut Buffer, content: &Buffer, num: u32) {
        let data = *self.renderer as *mut c_void;
        let func = unsafe { self.renderer.get().footnote_def.unwrap() };
        func(ob.as_mut(), content.as_ref(), num, data);
    }

    fn html_block(&mut self, ob: &mut Buffer, text: &Buffer) {
        let data = *self.renderer as *mut c_void;
        let func = unsafe { self.renderer.get().blockhtml.unwrap() };
        func(ob.as_mut(), text.as_ref(), data);
    }

    fn autolink(&mut self, ob: &mut Buffer, link: &Buffer, ty: ::renderer::AutoLink) -> bool {
        let data = *self.renderer as *mut c_void;
        let func = unsafe { self.renderer.get().autolink.unwrap() };
        func(ob.as_mut(), link.as_ref(), ty, data) != 0
    }

    fn code_span(&mut self, ob: &mut Buffer, text: &Buffer) -> bool {
        let data = *self.renderer as *mut c_void;
        let func = unsafe { self.renderer.get().codespan.unwrap() };
        func(ob.as_mut(), text.as_ref(), data) != 0
    }

    fn double_emphasis(&mut self, ob: &mut Buffer, content: &Buffer) -> bool {
        let data = *self.renderer as *mut c_void;
        let func = unsafe { self.renderer.get().double_emphasis.unwrap() };
        func(ob.as_mut(), content.as_ref(), data) != 0
    }

    fn emphasis(&mut self, ob: &mut Buffer, content: &Buffer) -> bool {
        let data = *self.renderer as *mut c_void;
        let func = unsafe { self.renderer.get().emphasis.unwrap() };
        func(ob.as_mut(), content.as_ref(), data) != 0
    }

    fn underline(&mut self, ob: &mut Buffer, content: &Buffer) -> bool {
        let data = *self.renderer as *mut c_void;
        let func = unsafe { self.renderer.get().underline.unwrap() };
        func(ob.as_mut(), content.as_ref(), data) != 0
    }

    fn highlight(&mut self, ob: &mut Buffer, content: &Buffer) -> bool {
        let data = *self.renderer as *mut c_void;
        let func = unsafe { self.renderer.get().highlight.unwrap() };
        func(ob.as_mut(), content.as_ref(), data) != 0
    }

    fn quote_span(&mut self, ob: &mut Buffer, content: &Buffer) -> bool {
        let data = *self.renderer as *mut c_void;
        let func = unsafe { self.renderer.get().quote.unwrap() };
        func(ob.as_mut(), content.as_ref(), data) != 0
    }

    fn image(&mut self, ob: &mut Buffer, link: &Buffer, title: &Buffer, alt: &Buffer) -> bool {
        let data = *self.renderer as *mut c_void;
        let func = unsafe { self.renderer.get().image.unwrap() };
        func(ob.as_mut(), link.as_ref(), title.as_ref(), alt.as_ref(), data) != 0
    }

    fn line_break(&mut self, ob: &mut Buffer) -> bool {
        let data = *self.renderer as *mut c_void;
        let func = unsafe { self.renderer.get().linebreak.unwrap() };
        func(ob.as_mut(), data) != 0
    }

    fn link(&mut self, ob: &mut Buffer, content: &Buffer, link: &Buffer, title: &Buffer) -> bool {
        let data = *self.renderer as *mut c_void;
        let func = unsafe { self.renderer.get().link.unwrap() };
        func(ob.as_mut(), content.as_ref(), link.as_ref(), title.as_ref(), data) != 0
    }

    fn triple_emphasis(&mut self, ob: &mut Buffer, content: &Buffer) -> bool {
        let data = *self.renderer as *mut c_void;
        let func = unsafe { self.renderer.get().triple_emphasis.unwrap() };
        func(ob.as_mut(), content.as_ref(), data) != 0
    }

    fn strikethrough(&mut self, ob: &mut Buffer, content: &Buffer) -> bool {
        let data = *self.renderer as *mut c_void;
        let func = unsafe { self.renderer.get().strikethrough.unwrap() };
        func(ob.as_mut(), content.as_ref(), data) != 0
    }

    fn superscript(&mut self, ob: &mut Buffer, content: &Buffer) -> bool {
        let data = *self.renderer as *mut c_void;
        let func = unsafe { self.renderer.get().superscript.unwrap() };
        func(ob.as_mut(), content.as_ref(), data) != 0
    }

    fn footnote_reference(&mut self, ob: &mut Buffer, num: u32) -> bool {
        let data = *self.renderer as *mut c_void;
        let func = unsafe { self.renderer.get().footnote_ref.unwrap() };
        func(ob.as_mut(), num, data) != 0
    }

    fn math(&mut self, ob: &mut Buffer, text: &Buffer, displaymode: i32) -> bool {
        let data = *self.renderer as *mut c_void;
        let func = unsafe { self.renderer.get().math.unwrap() };
        func(ob.as_mut(), text.as_ref(), displaymode, data) != 0
    }

    fn html_span(&mut self, ob: &mut Buffer, text: &Buffer) -> bool {
        let data = *self.renderer as *mut c_void;
        let func = unsafe { self.renderer.get().raw_html.unwrap() };
        func(ob.as_mut(), text.as_ref(), data) != 0
    }

    fn entity(&mut self, ob: &mut Buffer, text: &Buffer) {
        let data = *self.renderer as *mut c_void;
        let func = unsafe { self.renderer.get().entity.unwrap() };
        func(ob.as_mut(), text.as_ref(), data);
    }

    fn normal_text(&mut self, ob: &mut Buffer, text: &Buffer) {
        let data = *self.renderer as *mut c_void;
        let func = unsafe { self.renderer.get().normal_text.unwrap() };
        func(ob.as_mut(), text.as_ref(), data);
    }

    fn before_render(&mut self, ob: &mut Buffer, inline_render: bool) {
        let data = *self.renderer as *mut c_void;
        let func = unsafe { self.renderer.get().doc_header.unwrap() };
        func(ob.as_mut(), inline_render as i32, data);
    }

    fn after_render(&mut self, ob: &mut Buffer, inline_render: bool) {
        let data = *self.renderer as *mut c_void;
        let func = unsafe { self.renderer.get().doc_footer.unwrap() };
        func(ob.as_mut(), inline_render as i32, data);
    }
}

impl Drop for Html {
    fn drop(&mut self) {
        unsafe { hoedown_html_renderer_free(*self.renderer); }
    }
}

