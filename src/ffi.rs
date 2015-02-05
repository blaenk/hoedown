use libc::{c_void, c_int, c_uint, size_t};

#[allow(non_camel_case_types)]
mod callbacks {
    use libc::{c_void, c_int, c_uint};
    use super::hoedown_buffer;

    pub type blockcode = extern "C" fn(*mut hoedown_buffer, *const hoedown_buffer, *const hoedown_buffer, *mut c_void) -> ();
    pub type blockquote = extern "C" fn(*mut hoedown_buffer, *const hoedown_buffer, *mut c_void) -> ();
    pub type header = extern "C" fn(*mut hoedown_buffer, *const hoedown_buffer, c_int, *mut c_void) -> ();
    pub type hrule = extern "C" fn(*mut hoedown_buffer, *mut c_void) -> ();
    pub type list = extern "C" fn(*mut hoedown_buffer, *const hoedown_buffer, ::renderer::list::List, *mut c_void) -> ();
    pub type listitem = extern "C" fn(*mut hoedown_buffer, *const hoedown_buffer, ::renderer::list::List, *mut c_void) -> ();
    pub type paragraph = extern "C" fn(*mut hoedown_buffer, *const hoedown_buffer, *mut c_void) -> ();
    pub type table = extern "C" fn(*mut hoedown_buffer, *const hoedown_buffer, *mut c_void) -> ();
    pub type table_header = extern "C" fn(*mut hoedown_buffer, *const hoedown_buffer, *mut c_void) -> ();
    pub type table_body = extern "C" fn(*mut hoedown_buffer, *const hoedown_buffer, *mut c_void) -> ();
    pub type table_row = extern "C" fn(*mut hoedown_buffer, *const hoedown_buffer, *mut c_void) -> ();
    pub type table_cell = extern "C" fn(*mut hoedown_buffer, *const hoedown_buffer, ::renderer::Table, *mut c_void) -> ();
    pub type footnotes = extern "C" fn(*mut hoedown_buffer, *const hoedown_buffer, *mut c_void) -> ();
    pub type footnote_def = extern "C" fn(*mut hoedown_buffer, *const hoedown_buffer, c_uint, *mut c_void) -> ();
    pub type blockhtml = extern "C" fn(*mut hoedown_buffer, *const hoedown_buffer, *mut c_void) -> ();
    pub type autolink = extern "C" fn(*mut hoedown_buffer, *const hoedown_buffer, ::renderer::AutoLink, *mut c_void) -> i32;
    pub type codespan = extern "C" fn(*mut hoedown_buffer, *const hoedown_buffer, *mut c_void) -> i32;
    pub type double_emphasis = extern "C" fn(*mut hoedown_buffer, *const hoedown_buffer, *mut c_void) -> i32;
    pub type emphasis = extern "C" fn(*mut hoedown_buffer, *const hoedown_buffer, *mut c_void) -> i32;
    pub type underline = extern "C" fn(*mut hoedown_buffer, *const hoedown_buffer, *mut c_void) -> i32;
    pub type highlight = extern "C" fn(*mut hoedown_buffer, *const hoedown_buffer, *mut c_void) -> i32;
    pub type quote = extern "C" fn(*mut hoedown_buffer, *const hoedown_buffer, *mut c_void) -> i32;
    pub type image = extern "C" fn(*mut hoedown_buffer, *const hoedown_buffer, *const hoedown_buffer, *const hoedown_buffer, *mut c_void) -> i32;
    pub type linebreak = extern "C" fn(*mut hoedown_buffer, *mut c_void) -> i32;
    pub type link = extern "C" fn(*mut hoedown_buffer, *const hoedown_buffer, *const hoedown_buffer, *const hoedown_buffer, *mut c_void) -> i32;
    pub type triple_emphasis = extern "C" fn(*mut hoedown_buffer, *const hoedown_buffer, *mut c_void) -> i32;
    pub type strikethrough = extern "C" fn(*mut hoedown_buffer, *const hoedown_buffer, *mut c_void) -> i32;
    pub type superscript = extern "C" fn(*mut hoedown_buffer, *const hoedown_buffer, *mut c_void) -> i32;
    pub type footnote_ref = extern "C" fn(*mut hoedown_buffer, c_uint, *mut c_void) -> i32;
    pub type math = extern "C" fn(*mut hoedown_buffer, *const hoedown_buffer, c_int, *mut c_void) -> i32;
    pub type raw_html = extern "C" fn(*mut hoedown_buffer, *const hoedown_buffer, *mut c_void) -> i32;
    pub type entity = extern "C" fn(*mut hoedown_buffer, *const hoedown_buffer, *mut c_void) -> ();
    pub type normal_text = extern "C" fn(*mut hoedown_buffer, *const hoedown_buffer, *mut c_void) -> ();
    pub type doc_header = extern "C" fn(*mut hoedown_buffer, c_int, *mut c_void) -> ();
    pub type doc_footer = extern "C" fn(*mut hoedown_buffer, c_int, *mut c_void) -> ();

    // // renderer state
    pub type link_attributes =
        extern "C" fn(*mut hoedown_buffer, *const hoedown_buffer, *mut c_void);
}

#[allow(raw_pointer_derive)]
#[derive(Copy)]
#[repr(C)]
pub struct hoedown_renderer {
    pub opaque: *mut c_void,
    pub blockcode: Option<callbacks::blockcode>,
    pub blockquote: Option<callbacks::blockquote>,
    pub header: Option<callbacks::header>,
    pub hrule: Option<callbacks::hrule>,
    pub list: Option<callbacks::list>,
    pub listitem: Option<callbacks::listitem>,
    pub paragraph: Option<callbacks::paragraph>,
    pub table: Option<callbacks::table>,
    pub table_header: Option<callbacks::table_header>,
    pub table_body: Option<callbacks::table_body>,
    pub table_row: Option<callbacks::table_row>,
    pub table_cell: Option<callbacks::table_cell>,
    pub footnotes: Option<callbacks::footnotes>,
    pub footnote_def: Option<callbacks::footnote_def>,
    pub blockhtml: Option<callbacks::blockhtml>,
    pub autolink: Option<callbacks::autolink>,
    pub codespan: Option<callbacks::codespan>,
    pub double_emphasis: Option<callbacks::double_emphasis>,
    pub emphasis: Option<callbacks::emphasis>,
    pub underline: Option<callbacks::underline>,
    pub highlight: Option<callbacks::highlight>,
    pub quote: Option<callbacks::quote>,
    pub image: Option<callbacks::image>,
    pub linebreak: Option<callbacks::linebreak>,
    pub link: Option<callbacks::link>,
    pub triple_emphasis: Option<callbacks::triple_emphasis>,
    pub strikethrough: Option<callbacks::strikethrough>,
    pub superscript: Option<callbacks::superscript>,
    pub footnote_ref: Option<callbacks::footnote_ref>,
    pub math: Option<callbacks::math>,
    pub raw_html: Option<callbacks::raw_html>,
    pub entity: Option<callbacks::entity>,
    pub normal_text: Option<callbacks::normal_text>,
    pub doc_header: Option<callbacks::doc_header>,
    pub doc_footer: Option<callbacks::doc_footer>,
}

#[allow(unused)]
#[repr(C)]
pub struct hoedown_html_renderer_state {
    opaque: *mut c_void,
    toc_data: html_toc_data,
    flags: c_uint,
    link_attributes: Option<callbacks::link_attributes>,
}

#[allow(unused)]
#[repr(C)]
struct html_toc_data {
    header_count: c_int,
    current_level: c_int,
    level_offset: c_int,
    nesting_level: c_int,
}

#[allow(raw_pointer_derive)]
#[derive(Copy)]
#[repr(C)]
pub struct hoedown_buffer {
    pub data: *mut u8,
    pub size: size_t,
    asize: size_t,
    unit: size_t,
}

#[allow(non_camel_case_types)]
pub type hoedown_document = c_void;

extern "C" {
    // renderer
    pub fn hoedown_html_renderer_new(
        render_flags: c_uint,
        nesting_level: c_int,
    ) -> *mut hoedown_renderer;

    pub fn hoedown_html_toc_renderer_new(
        nesting_level: c_int,
    ) -> *mut hoedown_renderer;

    pub fn hoedown_html_smartypants(
        output: *mut hoedown_buffer,
        data: *const u8,
        size: size_t,
    );

    pub fn hoedown_html_renderer_free(renderer: *mut hoedown_renderer);

    // document
    pub fn hoedown_document_new(
        renderer: *const hoedown_renderer,
        extensions: c_uint,
        max_nesting: size_t,
    ) -> *mut hoedown_document;

    pub fn hoedown_document_render(
        document: *mut hoedown_document,
        output_buffer: *mut hoedown_buffer,
        input: *const u8,
        input_size: size_t,
    );

    pub fn hoedown_document_render_inline(
        document: *mut hoedown_document,
        output_buffer: *mut hoedown_buffer,
        input: *const u8,
        input_size: size_t,
    );

    pub fn hoedown_document_free(markdown: *mut hoedown_document);

    // buffer
    pub fn hoedown_buffer_new(unit: size_t) -> *mut hoedown_buffer;
    pub fn hoedown_buffer_put(buffer: *mut hoedown_buffer, data: *const u8, size: size_t);
    pub fn hoedown_buffer_free(buffer: *mut hoedown_buffer);
}
