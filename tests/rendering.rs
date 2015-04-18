extern crate hoedown;

use hoedown::Markdown;
use hoedown::renderer::html;

#[test]
fn test_render_inline() {
    let doc = Markdown::new("some _emphasis_ required");
    let html = html::Html::new(html::Flags::empty(), 0);

    let res = doc.render_inline_to_buffer(html);

    assert_eq!(res.to_str().unwrap(), "some <em>emphasis</em> required");
}

#[test]
fn test_render_toc() {
    let toc = html::Html::toc(16);
    let doc = Markdown::new(
"# first

this is some paragraph

## sub section

note the following

## another sub section

heh

# conclusion

this");

    let res = doc.render_to_buffer(toc);

    assert_eq!(res.to_str().unwrap(),
"<ul>
<li>
<a href=\"#toc_0\">first</a>
<ul>
<li>
<a href=\"#toc_1\">sub section</a>
</li>
<li>
<a href=\"#toc_2\">another sub section</a>
</li>
</ul>
</li>
<li>
<a href=\"#toc_3\">conclusion</a>
</li>
</ul>\n");
}

