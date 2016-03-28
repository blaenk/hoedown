bitflags! {
    /// Constants for the various Hoedown extensions
    pub flags Extension: u32 {
        // block-level

        /// Process table syntax
        const TABLES                = 1 << 0,

        /// Process fenced code
        const FENCED_CODE           = 1 << 1,

        /// Process footnotes
        const FOOTNOTES             = 1 << 2,

        // span-level

        /// Automatically link URLs and emails
        const AUTOLINK              = 1 << 3,

        /// Enable strikethrough syntax, e.g. `~~strike one~~`
        const STRIKETHROUGH         = 1 << 4,

        /// Perform an underline instead of emphasis
        const UNDERLINE             = 1 << 5,

        /// Process highlight syntax, e.g. `==highlight me==`
        const HIGHLIGHT             = 1 << 6,

        /// Render quotes differently, e.g. the html renderer may use the `<q>` tag
        const QUOTE                 = 1 << 7,

        /// Process superscript syntax, e.g. `2^3 = 8`
        const SUPERSCRIPT           = 1 << 8,

        /// Process math syntax, e.g. `$$x + y = z$$`
        const MATH                  = 1 << 9,

        // other flags

        /// Don't parse emphasis inside of words, e.g. `foo_bar_baz` won't emphasize the 'bar'
        const NO_INTRA_EMPHASIS     = 1 << 11,

        /// Process ATX header syntax, e.g. `# Topic`
        const SPACE_HEADERS         = 1 << 12,

        /// Process the single dollar math syntax, e.g. `$x + y = 3$`
        const MATH_EXPLICIT         = 1 << 13,

        // negative flags

        /// Ignore indented code blocks
        const DISABLE_INDENTED_CODE = 1 << 14,
    }
}
