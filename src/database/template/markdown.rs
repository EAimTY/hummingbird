use pulldown_cmark::{html, Options, Parser};

pub fn md_to_html(md: &str) -> String {
    let parser = Parser::new_ext(md, Options::all());

    let mut output = String::new();
    html::push_html(&mut output, parser);

    output
}
