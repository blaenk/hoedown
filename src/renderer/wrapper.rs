use buffer::Buffer;
use super::{Render, AutoLink, Table, list};

pub trait Wrapper {
    type Base: Render;

    fn base(&mut self) -> &mut Self::Base;

    #[inline(always)]
    fn code_block(&mut self, output: &mut Buffer, code: Option<&Buffer>, lang: Option<&Buffer>) {
        self.base().code_block(output, code, lang)
    }

    #[inline(always)]
    fn quote_block(&mut self, ob: &mut Buffer, content: Option<&Buffer>) {
        self.base().quote_block(ob, content);
    }

    #[inline(always)]
    fn header(&mut self, ob: &mut Buffer, content: Option<&Buffer>, level: i32) {
        self.base().header(ob, content, level);
    }

    #[inline(always)]
    fn horizontal_rule(&mut self, ob: &mut Buffer) {
        self.base().horizontal_rule(ob);
    }

    #[inline(always)]
    fn list(&mut self, ob: &mut Buffer, content: Option<&Buffer>, flags: list::List) {
        self.base().list(ob, content, flags);
    }

    #[inline(always)]
    fn list_item(&mut self, ob: &mut Buffer, content: Option<&Buffer>, flags: list::List) {
        self.base().list_item(ob, content, flags);
    }

    #[inline(always)]
    fn paragraph(&mut self, ob: &mut Buffer, content: Option<&Buffer>) {
        self.base().paragraph(ob, content);
    }

    #[inline(always)]
    fn table(&mut self, ob: &mut Buffer, content: Option<&Buffer>) {
        self.base().table(ob, content);
    }

    #[inline(always)]
    fn table_header(&mut self, ob: &mut Buffer, content: Option<&Buffer>) {
        self.base().table_header(ob, content);
    }

    #[inline(always)]
    fn table_body(&mut self, ob: &mut Buffer, content: Option<&Buffer>) {
        self.base().table_body(ob, content);
    }

    #[inline(always)]
    fn table_row(&mut self, ob: &mut Buffer, content: Option<&Buffer>) {
        self.base().table_row(ob, content);
    }

    #[inline(always)]
    fn table_cell(&mut self, ob: &mut Buffer, content: Option<&Buffer>, flags: Table) {
        self.base().table_cell(ob, content, flags);
    }

    #[inline(always)]
    fn footnotes(&mut self, ob: &mut Buffer, content: Option<&Buffer>) {
        self.base().footnotes(ob, content);
    }

    #[inline(always)]
    fn footnote_definition(&mut self, ob: &mut Buffer, content: Option<&Buffer>, num: u32) {
        self.base().footnote_definition(ob, content, num);
    }

    #[inline(always)]
    fn html_block(&mut self, ob: &mut Buffer, text: Option<&Buffer>) {
        self.base().html_block(ob, text);
    }

    #[inline(always)]
    fn autolink(&mut self, ob: &mut Buffer, link: Option<&Buffer>, ty: AutoLink) -> bool {
        self.base().autolink(ob, link, ty)
    }

    #[inline(always)]
    fn code_span(&mut self, ob: &mut Buffer, text: Option<&Buffer>) -> bool {
        self.base().code_span(ob, text)
    }

    #[inline(always)]
    fn double_emphasis(&mut self, ob: &mut Buffer, content: Option<&Buffer>) -> bool {
        self.base().double_emphasis(ob, content)
    }

    #[inline(always)]
    fn emphasis(&mut self, ob: &mut Buffer, content: Option<&Buffer>) -> bool {
        self.base().emphasis(ob, content)
    }

    #[inline(always)]
    fn underline(&mut self, ob: &mut Buffer, content: Option<&Buffer>) -> bool {
        self.base().underline(ob, content)
    }

    #[inline(always)]
    fn highlight(&mut self, ob: &mut Buffer, content: Option<&Buffer>) -> bool {
        self.base().highlight(ob, content)
    }

    #[inline(always)]
    fn quote_span(&mut self, ob: &mut Buffer, content: Option<&Buffer>) -> bool {
        self.base().quote_span(ob, content)
    }

    #[inline(always)]
    fn image(&mut self, ob: &mut Buffer, link: Option<&Buffer>, title: Option<&Buffer>, alt: Option<&Buffer>) -> bool {
        self.base().image(ob, link, title, alt)
    }

    #[inline(always)]
    fn line_break(&mut self, ob: &mut Buffer) -> bool {
        self.base().line_break(ob)
    }

    #[inline(always)]
    fn link(&mut self, ob: &mut Buffer, content: Option<&Buffer>, link: Option<&Buffer>, title: Option<&Buffer>) -> bool {
        self.base().link(ob, content, link, title)
    }

    #[inline(always)]
    fn triple_emphasis(&mut self, ob: &mut Buffer, content: Option<&Buffer>) -> bool {
        self.base().triple_emphasis(ob, content)
    }

    #[inline(always)]
    fn strikethrough(&mut self, ob: &mut Buffer, content: Option<&Buffer>) -> bool {
        self.base().strikethrough(ob, content)
    }

    #[inline(always)]
    fn superscript(&mut self, ob: &mut Buffer, content: Option<&Buffer>) -> bool {
        self.base().superscript(ob, content)
    }

    #[inline(always)]
    fn footnote_reference(&mut self, ob: &mut Buffer, num: u32) -> bool {
        self.base().footnote_reference(ob, num)
    }

    #[inline(always)]
    fn math(&mut self, ob: &mut Buffer, text: Option<&Buffer>, displaymode: i32) -> bool {
        self.base().math(ob, text, displaymode)
    }

    #[inline(always)]
    fn html_span(&mut self, ob: &mut Buffer, text: Option<&Buffer>) -> bool {
        self.base().html_span(ob, text)
    }

    #[inline(always)]
    fn entity(&mut self, ob: &mut Buffer, text: Option<&Buffer>) {
        self.base().entity(ob, text)
    }

    #[inline(always)]
    fn normal_text(&mut self, ob: &mut Buffer, text: Option<&Buffer>) {
        self.base().normal_text(ob, text)
    }

    #[inline(always)]
    fn before_render(&mut self, output: &mut Buffer, inline_render: bool) {
        self.base().before_render(output, inline_render)
    }

    #[inline(always)]
    fn after_render(&mut self, output: &mut Buffer, inline_render: bool) {
        self.base().after_render(output, inline_render)
    }
}

#[macro_export]
macro_rules! wrap {
    ($name:ty) => {
        impl $crate::renderer::Render for $name {
            // block-level: not registered = skip the block
            #[inline(always)]
            fn code_block(&mut self, output: &mut $crate::Buffer, text: Option<&$crate::Buffer>, lang: Option<&$crate::Buffer>) {
                $crate::renderer::wrapper::Wrapper::code_block(self, output, text, lang)
            }
            #[inline(always)]
            fn quote_block(&mut self, output: &mut $crate::Buffer, content: Option<&$crate::Buffer>) {
                $crate::renderer::wrapper::Wrapper::quote_block(self, output, content)
            }
            #[inline(always)]
            fn header(&mut self, output: &mut $crate::Buffer, content: Option<&$crate::Buffer>, level: i32) {
                $crate::renderer::wrapper::Wrapper::header(self, output, content, level)
            }
            #[inline(always)]
            fn horizontal_rule(&mut self, output: &mut $crate::Buffer) {
                $crate::renderer::wrapper::Wrapper::horizontal_rule(self, output)
            }
            #[inline(always)]
            fn list(&mut self, output: &mut $crate::Buffer, content: Option<&$crate::Buffer>, flags: $crate::renderer::list::List) {
                $crate::renderer::wrapper::Wrapper::list(self, output, content, flags)
            }
            #[inline(always)]
            fn list_item(&mut self, output: &mut $crate::Buffer, content: Option<&$crate::Buffer>, flags: $crate::renderer::list::List) {
                $crate::renderer::wrapper::Wrapper::list_item(self, output, content, flags)
            }
            #[inline(always)]
            fn paragraph(&mut self, output: &mut $crate::Buffer, content: Option<&$crate::Buffer>) {
                $crate::renderer::wrapper::Wrapper::paragraph(self, output, content)
            }
            #[inline(always)]
            fn table(&mut self, output: &mut $crate::Buffer, content: Option<&$crate::Buffer>) {
                $crate::renderer::wrapper::Wrapper::table(self, output, content)
            }
            #[inline(always)]
            fn table_header(&mut self, output: &mut $crate::Buffer, content: Option<&$crate::Buffer>) {
                $crate::renderer::wrapper::Wrapper::table_header(self, output, content)
            }
            #[inline(always)]
            fn table_body(&mut self, output: &mut $crate::Buffer, content: Option<&$crate::Buffer>) {
                $crate::renderer::wrapper::Wrapper::table_body(self, output, content)
            }
            #[inline(always)]
            fn table_row(&mut self, output: &mut $crate::Buffer, content: Option<&$crate::Buffer>) {
                $crate::renderer::wrapper::Wrapper::table_row(self, output, content)
            }
            #[inline(always)]
            fn table_cell(&mut self, output: &mut $crate::Buffer, content: Option<&$crate::Buffer>, flags: $crate::renderer::Table) {
                $crate::renderer::wrapper::Wrapper::table_cell(self, output, content, flags)
            }
            #[inline(always)]
            fn footnotes(&mut self, output: &mut $crate::Buffer, content: Option<&$crate::Buffer>) {
                $crate::renderer::wrapper::Wrapper::footnotes(self, output, content)
            }
            #[inline(always)]
            fn footnote_definition(&mut self, output: &mut $crate::Buffer, content: Option<&$crate::Buffer>, num: u32) {
                $crate::renderer::wrapper::Wrapper::footnote_definition(self, output, content, num)
            }
            #[inline(always)]
            fn html_block(&mut self, output: &mut $crate::Buffer, text: Option<&$crate::Buffer>) {
                $crate::renderer::wrapper::Wrapper::html_block(self, output, text)
            }

            // span-level: not registered = pass-through
            #[inline(always)]
            fn autolink(&mut self, output: &mut $crate::Buffer, link: Option<&$crate::Buffer>, link_type: $crate::renderer::AutoLink) -> bool {
                $crate::renderer::wrapper::Wrapper::autolink(self, output, link, link_type)
            }
            #[inline(always)]
            fn code_span(&mut self, output: &mut $crate::Buffer, text: Option<&$crate::Buffer>) -> bool {
                $crate::renderer::wrapper::Wrapper::code_span(self, output, text)
            }
            #[inline(always)]
            fn double_emphasis(&mut self, output: &mut $crate::Buffer, content: Option<&$crate::Buffer>) -> bool {
                $crate::renderer::wrapper::Wrapper::double_emphasis(self, output, content)
            }
            #[inline(always)]
            fn emphasis(&mut self, output: &mut $crate::Buffer, content: Option<&$crate::Buffer>) -> bool {
                $crate::renderer::wrapper::Wrapper::emphasis(self, output, content)
            }
            #[inline(always)]
            fn underline(&mut self, output: &mut $crate::Buffer, content: Option<&$crate::Buffer>) -> bool {
                $crate::renderer::wrapper::Wrapper::underline(self, output, content)
            }
            #[inline(always)]
            fn highlight(&mut self, output: &mut $crate::Buffer, content: Option<&$crate::Buffer>) -> bool {
                $crate::renderer::wrapper::Wrapper::highlight(self, output, content)
            }
            #[inline(always)]
            fn quote_span(&mut self, output: &mut $crate::Buffer, content: Option<&$crate::Buffer>) -> bool {
                $crate::renderer::wrapper::Wrapper::quote_span(self, output, content)
            }
            #[inline(always)]
            fn image(&mut self, output: &mut $crate::Buffer, link: Option<&$crate::Buffer>, title: Option<&$crate::Buffer>, alt: Option<&$crate::Buffer>) -> bool {
                $crate::renderer::wrapper::Wrapper::image(self, output, link, title, alt)
            }
            #[inline(always)]
            fn line_break(&mut self, output: &mut $crate::Buffer) -> bool {
                $crate::renderer::wrapper::Wrapper::line_break(self, output)
            }
            #[inline(always)]
            fn link(&mut self, output: &mut $crate::Buffer, content: Option<&$crate::Buffer>, link: Option<&$crate::Buffer>, title: Option<&$crate::Buffer>) -> bool {
                $crate::renderer::wrapper::Wrapper::link(self, output, content, link, title)
            }
            #[inline(always)]
            fn triple_emphasis(&mut self, output: &mut $crate::Buffer, content: Option<&$crate::Buffer>) -> bool {
                $crate::renderer::wrapper::Wrapper::triple_emphasis(self, output, content)
            }
            #[inline(always)]
            fn strikethrough(&mut self, output: &mut $crate::Buffer, content: Option<&$crate::Buffer>) -> bool {
                $crate::renderer::wrapper::Wrapper::strikethrough(self, output, content)
            }
            #[inline(always)]
            fn superscript(&mut self, output: &mut $crate::Buffer, content: Option<&$crate::Buffer>) -> bool {
                $crate::renderer::wrapper::Wrapper::superscript(self, output, content)
            }
            #[inline(always)]
            fn footnote_reference(&mut self, output: &mut $crate::Buffer, num: u32) -> bool {
                $crate::renderer::wrapper::Wrapper::footnote_reference(self, output, num)
            }
            #[inline(always)]
            fn math(&mut self, output: &mut $crate::Buffer, text: Option<&$crate::Buffer>, displaymode: i32) -> bool {
                $crate::renderer::wrapper::Wrapper::math(self, output, text, displaymode)
            }
            #[inline(always)]
            fn html_span(&mut self, output: &mut $crate::Buffer, text: Option<&$crate::Buffer>) -> bool {
                $crate::renderer::wrapper::Wrapper::html_span(self, output, text)
            }

            // low-level: not registered = pass-through
            #[inline(always)]
            fn entity(&mut self, output: &mut $crate::Buffer, text: Option<&$crate::Buffer>) {
                $crate::renderer::wrapper::Wrapper::entity(self, output, text)
            }
            #[inline(always)]
            fn normal_text(&mut self, output: &mut $crate::Buffer, text: Option<&$crate::Buffer>) {
                $crate::renderer::wrapper::Wrapper::normal_text(self, output, text)
            }

            // misc callbacks
            #[inline(always)]
            fn before_render(&mut self, output: &mut $crate::Buffer, inline_render: bool) {
                $crate::renderer::wrapper::Wrapper::before_render(self, output, inline_render)
            }
            #[inline(always)]
            fn after_render(&mut self, output: &mut $crate::Buffer, inline_render: bool) {
                $crate::renderer::wrapper::Wrapper::after_render(self, output, inline_render)
            }
        }
    };
}
