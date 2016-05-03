use libc::{c_void, c_int, c_uint};
use buffer::Buffer;
use ffi::{hoedown_buffer, hoedown_renderer};
use renderer::Render;

#[inline]
fn get_renderer<'a, R>(data: &'a *mut c_void) -> &'a mut R {
    unsafe {
        let renderer = *data as *mut hoedown_renderer;

        if renderer.is_null() {
            panic!("callback data is null");
        }

        let renderer = (*renderer).opaque as *mut R;

        if renderer.is_null() {
            panic!("callback data opaque is null");
        }

        &mut *renderer
    }
}

pub extern "C" fn blockcode<R>(ob: *mut hoedown_buffer,
                               text: *const hoedown_buffer,
                               lang: *const hoedown_buffer,
                               data: *mut c_void)
where R: Render {
    let renderer = get_renderer::<R>(&data);
    let mut out = Buffer::from(ob);
    let text = Buffer::from(text);
    let lang = Buffer::from(lang);
    renderer.code_block(&mut out, &text, &lang);
}

pub extern "C" fn blockquote<R>(ob: *mut hoedown_buffer,
                                content: *const hoedown_buffer,
                                data: *mut c_void)
where R: Render {
    let renderer = get_renderer::<R>(&data);
    let mut out = Buffer::from(ob);
    let content = Buffer::from(content);
    renderer.quote_block(&mut out, &content);
}

pub extern "C" fn header<R>(ob: *mut hoedown_buffer,
                            content: *const hoedown_buffer,
                            level: c_int,
                            data: *mut c_void)
where R: Render {
    let renderer = get_renderer::<R>(&data);
    let mut out = Buffer::from(ob);
    let content = Buffer::from(content);
    renderer.header(&mut out, &content, level as i32);
}

pub extern "C" fn hrule<R>(ob: *mut hoedown_buffer,
                           data: *mut c_void)
where R: Render {
    let renderer = get_renderer::<R>(&data);
    let mut out = Buffer::from(ob);
    renderer.horizontal_rule(&mut out);
}

pub extern "C" fn list<R>(ob: *mut hoedown_buffer,
                          content: *const hoedown_buffer,
                          flags: u32,
                          data: *mut c_void)
where R: Render {
    let renderer = get_renderer::<R>(&data);
    let mut out = Buffer::from(ob);
    let content = Buffer::from(content);
    renderer.list(&mut out, &content, ::renderer::list::List::from_arbitrary_bits(flags));
}

pub extern "C" fn listitem<R>(ob: *mut hoedown_buffer,
                              content: *const hoedown_buffer,
                              flags: u32,
                              data: *mut c_void)
where R: Render {
    let renderer = get_renderer::<R>(&data);
    let mut out = Buffer::from(ob);
    let content = Buffer::from(content);
    renderer.list_item(&mut out, &content, ::renderer::list::List::from_arbitrary_bits(flags));
}

pub extern "C" fn paragraph<R>(ob: *mut hoedown_buffer,
                               content: *const hoedown_buffer,
                               data: *mut c_void)
where R: Render {
    let renderer = get_renderer::<R>(&data);
    let mut out = Buffer::from(ob);
    let content = Buffer::from(content);
    renderer.paragraph(&mut out, &content);
}

pub extern "C" fn table<R>(ob: *mut hoedown_buffer,
                           content: *const hoedown_buffer,
                           data: *mut c_void)
where R: Render {
    let renderer = get_renderer::<R>(&data);
    let mut out = Buffer::from(ob);
    let content = Buffer::from(content);
    renderer.table(&mut out, &content);
}

pub extern "C" fn table_header<R>(ob: *mut hoedown_buffer,
                                  content: *const hoedown_buffer,
                                  data: *mut c_void)
where R: Render {
    let renderer = get_renderer::<R>(&data);
    let mut out = Buffer::from(ob);
    let content = Buffer::from(content);
    renderer.table_header(&mut out, &content);
}

pub extern "C" fn table_body<R>(ob: *mut hoedown_buffer,
                                content: *const hoedown_buffer,
                                data: *mut c_void)
where R: Render {
    let renderer = get_renderer::<R>(&data);
    let mut out = Buffer::from(ob);
    let content = Buffer::from(content);
    renderer.table_body(&mut out, &content);
}

pub extern "C" fn table_row<R>(ob: *mut hoedown_buffer,
                               content: *const hoedown_buffer,
                               data: *mut c_void)
where R: Render {
    let renderer = get_renderer::<R>(&data);
    let mut out = Buffer::from(ob);
    let content = Buffer::from(content);
    renderer.table_row(&mut out, &content);
}

pub extern "C" fn table_cell<R>(ob: *mut hoedown_buffer,
                                content: *const hoedown_buffer,
                                flags: ::renderer::Table,
                                data: *mut c_void)
where R: Render {
    let renderer = get_renderer::<R>(&data);
    let mut out = Buffer::from(ob);
    let content = Buffer::from(content);
    renderer.table_cell(&mut out, &content, flags);
}

pub extern "C" fn footnotes<R>(ob: *mut hoedown_buffer,
                               content: *const hoedown_buffer,
                               data: *mut c_void)
where R: Render {
    let renderer = get_renderer::<R>(&data);
    let mut out = Buffer::from(ob);
    let content = Buffer::from(content);
    renderer.footnotes(&mut out, &content);
}

pub extern "C" fn footnote_def<R>(ob: *mut hoedown_buffer,
                                  content: *const hoedown_buffer,
                                  num: c_uint,
                                  data: *mut c_void)
where R: Render {
    let renderer = get_renderer::<R>(&data);
    let mut out = Buffer::from(ob);
    let content = Buffer::from(content);
    renderer.footnote_definition(&mut out, &content, num);
}

pub extern "C" fn blockhtml<R>(ob: *mut hoedown_buffer,
                               content: *const hoedown_buffer,
                               data: *mut c_void)
where R: Render {
    let renderer = get_renderer::<R>(&data);
    let mut out = Buffer::from(ob);
    let content = Buffer::from(content);
    renderer.html_block(&mut out, &content);
}

// span
pub extern "C" fn autolink<R>(ob: *mut hoedown_buffer,
                              link: *const hoedown_buffer,
                              link_type: ::renderer::AutoLink,
                              data: *mut c_void) -> i32
where R: Render {
    let renderer = get_renderer::<R>(&data);
    let mut out = Buffer::from(ob);
    let link = Buffer::from(link);
    renderer.autolink(&mut out, &link, link_type) as i32
}

pub extern "C" fn codespan<R>(ob: *mut hoedown_buffer,
                              text: *const hoedown_buffer,
                              data: *mut c_void) -> i32
where R: Render {
    let renderer = get_renderer::<R>(&data);
    let mut out = Buffer::from(ob);
    let text = Buffer::from(text);
    renderer.code_span(&mut out, &text) as i32
}

pub extern "C" fn double_emphasis<R>(ob: *mut hoedown_buffer,
                                     content: *const hoedown_buffer,
                                     data: *mut c_void) -> i32
where R: Render {
    let renderer = get_renderer::<R>(&data);
    let mut out = Buffer::from(ob);
    let content = Buffer::from(content);
    renderer.double_emphasis(&mut out, &content) as i32
}

pub extern "C" fn emphasis<R>(ob: *mut hoedown_buffer,
                              content: *const hoedown_buffer,
                              data: *mut c_void) -> i32
where R: Render {
    let renderer = get_renderer::<R>(&data);
    let mut out = Buffer::from(ob);
    let content = Buffer::from(content);
    renderer.emphasis(&mut out, &content) as i32
}

pub extern "C" fn underline<R>(ob: *mut hoedown_buffer,
                               content: *const hoedown_buffer,
                               data: *mut c_void) -> i32
where R: Render {
    let renderer = get_renderer::<R>(&data);
    let mut out = Buffer::from(ob);
    let content = Buffer::from(content);
    renderer.underline(&mut out, &content) as i32
}

pub extern "C" fn highlight<R>(ob: *mut hoedown_buffer,
                               content: *const hoedown_buffer,
                               data: *mut c_void) -> i32
where R: Render {
    let renderer = get_renderer::<R>(&data);
    let mut out = Buffer::from(ob);
    let content = Buffer::from(content);
    renderer.highlight(&mut out, &content) as i32
}

pub extern "C" fn quote<R>(ob: *mut hoedown_buffer,
                           content: *const hoedown_buffer,
                           data: *mut c_void) -> i32
where R: Render {
    let renderer = get_renderer::<R>(&data);
    let mut out = Buffer::from(ob);
    let content = Buffer::from(content);
    renderer.quote_span(&mut out, &content) as i32
}

pub extern "C" fn image<R>(ob: *mut hoedown_buffer,
                           link: *const hoedown_buffer,
                           title: *const hoedown_buffer,
                           alt: *const hoedown_buffer,
                           data: *mut c_void) -> i32
where R: Render {
    let renderer = get_renderer::<R>(&data);
    let mut out = Buffer::from(ob);
    let link = Buffer::from(link);
    let title = Buffer::from(title);
    let alt = Buffer::from(alt);
    renderer.image(&mut out, &link, &title, &alt) as i32
}

pub extern "C" fn linebreak<R>(ob: *mut hoedown_buffer,
                               data: *mut c_void) -> i32
where R: Render {
    let renderer = get_renderer::<R>(&data);
    let mut out = Buffer::from(ob);
    renderer.line_break(&mut out) as i32
}

pub extern "C" fn link<R>(ob: *mut hoedown_buffer,
                          content: *const hoedown_buffer,
                          link: *const hoedown_buffer,
                          title: *const hoedown_buffer,
                          data: *mut c_void) -> i32
where R: Render {
    let renderer = get_renderer::<R>(&data);
    let mut out = Buffer::from(ob);
    let content = Buffer::from(content);
    let link = Buffer::from(link);
    let title = Buffer::from(title);
    renderer.link(&mut out, &content, &link, &title) as i32
}

pub extern "C" fn triple_emphasis<R>(ob: *mut hoedown_buffer,
                                     content: *const hoedown_buffer,
                                     data: *mut c_void) -> i32
where R: Render {
    let renderer = get_renderer::<R>(&data);
    let mut out = Buffer::from(ob);
    let content = Buffer::from(content);
    renderer.triple_emphasis(&mut out, &content) as i32
}

pub extern "C" fn strikethrough<R>(ob: *mut hoedown_buffer,
                                   content: *const hoedown_buffer,
                                   data: *mut c_void) -> i32
where R: Render {
    let renderer = get_renderer::<R>(&data);
    let mut out = Buffer::from(ob);
    let content = Buffer::from(content);
    renderer.strikethrough(&mut out, &content) as i32
}

pub extern "C" fn superscript<R>(ob: *mut hoedown_buffer,
                                 content: *const hoedown_buffer,
                                 data: *mut c_void) -> i32
where R: Render {
    let renderer = get_renderer::<R>(&data);
    let mut out = Buffer::from(ob);
    let content = Buffer::from(content);
    renderer.superscript(&mut out, &content) as i32
}

pub extern "C" fn footnote_ref<R>(ob: *mut hoedown_buffer,
                                  num: c_uint,
                                  data: *mut c_void) -> i32
where R: Render {
    let renderer = get_renderer::<R>(&data);
    let mut out = Buffer::from(ob);
    renderer.footnote_reference(&mut out, num) as i32
}

pub extern "C" fn math<R>(ob: *mut hoedown_buffer,
                          text: *const hoedown_buffer,
                          displaymode: c_int,
                          data: *mut c_void) -> i32
where R: Render {
    let renderer = get_renderer::<R>(&data);
    let mut out = Buffer::from(ob);
    let text = Buffer::from(text);
    renderer.math(&mut out, &text, displaymode) as i32
}

pub extern "C" fn raw_html<R>(ob: *mut hoedown_buffer,
                              text: *const hoedown_buffer,
                              data: *mut c_void) -> i32
where R: Render {
    let renderer = get_renderer::<R>(&data);
    let mut out = Buffer::from(ob);
    let text = Buffer::from(text);
    renderer.html_span(&mut out, &text) as i32
}

// low-level
pub extern "C" fn entity<R>(ob: *mut hoedown_buffer,
                            text: *const hoedown_buffer,
                            data: *mut c_void)
where R: Render {
    let renderer = get_renderer::<R>(&data);
    let mut out = Buffer::from(ob);
    let text = Buffer::from(text);
    renderer.entity(&mut out, &text)
}

pub extern "C" fn normal_text<R>(ob: *mut hoedown_buffer,
                                 text: *const hoedown_buffer,
                                 data: *mut c_void)
where R: Render {
    let renderer = get_renderer::<R>(&data);
    let mut out = Buffer::from(ob);
    let text = Buffer::from(text);
    renderer.normal_text(&mut out, &text)
}

// misc
pub extern "C" fn doc_header<R>(ob: *mut hoedown_buffer, inline_render: c_int, data: *mut c_void)
where R: Render {
    let renderer = get_renderer::<R>(&data);
    let mut out = Buffer::from(ob);
    renderer.before_render(&mut out, inline_render != 0);
}

pub extern "C" fn doc_footer<R>(ob: *mut hoedown_buffer, inline_render: c_int, data: *mut c_void)
where R: Render {
    let renderer = get_renderer::<R>(&data);
    let mut out = Buffer::from(ob);
    renderer.after_render(&mut out, inline_render != 0);
}
