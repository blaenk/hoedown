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

macro_rules! buffers {
    ($out:ident, $($inp:ident),*) => ({
        let out = unsafe { Buffer::from($out) };

        $(
            let $inp = unsafe { Buffer::from($inp as *mut _) };
        )*

        (out, $($inp),*)
    })
}

macro_rules! wrapper {
    ($func:ident) => (
        wrapper!($func => $func: () AND ());
    );

    ($rust:ident => $ffi:ident) => (
        wrapper!($rust => $ffi: () AND ());
    );

    ($func:ident: ($($name:ident),*)) => (
        wrapper!($func => $func: ($($name),*) AND ());
    );

    ($func:ident: ($($name:ident),*) AND ($($other_name:ident: $other_typ:ty),*)) => (
        wrapper!($func => $func: ($($name),*) AND ($($other_name: $other_typ),*));
    );

    ($rust:ident => $ffi:ident: ($($name:ident),*)) => (
        wrapper!($rust => $ffi: ($($name),*) AND ());
    );

    ($rust:ident => $ffi:ident:
     ($($name:ident),*)
     AND ($($other_name:ident: $other_typ:ty),*)) => (
        pub extern "C" fn $ffi<R>(
            out: *mut hoedown_buffer,
            $($name: *const hoedown_buffer,)*
            $($other_name: $other_typ,)*
            data: *mut c_void
        ) where R: Render {
            let renderer = get_renderer::<R>(&data);
            let (mut out, $($name),*) = buffers!(out, $($name),*);
            renderer.$rust(&mut out, $(&$name,)* $($other_name),*);
        }
    );
}

macro_rules! span {
    ($func:ident) => (
        span!($func => $func: () AND ());
    );

    ($rust:ident => $ffi:ident) => (
        span!($rust => $ffi: () AND ());
    );

    ($func:ident: ($($name:ident),*)) => (
        span!($func => $func: ($($name),*) AND ());
    );

    ($func:ident: ($($name:ident),*) AND ($($other_name:ident: $other_typ:ty),*)) => (
        span!($func => $func: ($($name),*) AND ($($other_name: $other_typ),*));
    );

    ($rust:ident => $ffi:ident: ($($name:ident),*)) => (
        span!($rust => $ffi: ($($name),*) AND ());
    );

    ($rust:ident => $ffi:ident:
     ($($name:ident),*)
     AND ($($other_name:ident: $other_typ:ty),*)) => (
        pub extern "C" fn $ffi<R>(
            out: *mut hoedown_buffer,
            $($name: *const hoedown_buffer,)*
            $($other_name: $other_typ,)*
            data: *mut c_void
        ) -> i32
        where R: Render {
            let renderer = get_renderer::<R>(&data);
            let (mut out, $($name),*) = buffers!(out, $($name),*);
            renderer.$rust(&mut out, $(&$name,)* $($other_name),*) as i32
        }
    );
}

wrapper!(code_block => blockcode: (text, lang));
wrapper!(quote_block => blockquote: (content));
wrapper!(header: (content) AND (level: c_int));
wrapper!(horizontal_rule => hrule);
wrapper!(list: (content) AND (flags: ::renderer::list::List));
wrapper!(list_item => listitem: (content) AND (flags: ::renderer::list::List));
wrapper!(paragraph: (content));
wrapper!(table: (content));
wrapper!(table_header: (content));
wrapper!(table_body: (content));
wrapper!(table_row: (content));
wrapper!(table_cell: (content) AND (flags: ::renderer::Table));
wrapper!(footnotes: (content));
wrapper!(footnote_definition => footnote_def: (content) AND (num: c_uint));
wrapper!(html_block => blockhtml: (content));

// span
span!(autolink: (link) AND (ty: ::renderer::AutoLink));
span!(code_span => codespan: (text));
span!(double_emphasis: (content));
span!(emphasis: (content));
span!(underline: (content));
span!(highlight: (content));
span!(quote_span => quote: (content));
span!(image: (link, title, alt));
span!(line_break => linebreak);
span!(link: (content, link, title));
span!(triple_emphasis: (content));
span!(strikethrough: (content));
span!(superscript: (content));
span!(footnote_reference => footnote_ref: () AND (num: c_uint));
span!(math: (text) AND (displaymode: c_int));
span!(html_span => raw_html: (text));

// low-level
wrapper!(entity: (text));
wrapper!(normal_text: (text));

// misc
pub extern "C" fn doc_header<R>(ob: *mut hoedown_buffer, inline_render: c_int, data: *mut c_void)
where R: Render {
    let renderer = get_renderer::<R>(&data);
    let mut out = unsafe { Buffer::from(ob) };
    renderer.before_render(&mut out, inline_render != 0);
}

pub extern "C" fn doc_footer<R>(ob: *mut hoedown_buffer, inline_render: c_int, data: *mut c_void)
where R: Render {
    let renderer = get_renderer::<R>(&data);
    let mut out = unsafe { Buffer::from(ob) };
    renderer.after_render(&mut out, inline_render != 0);
}

