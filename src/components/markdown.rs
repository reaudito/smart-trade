use kuchikiki::traits::*;
use pulldown_cmark::{html::push_html, Options, Parser};

pub fn parse_text_to_html(value: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    let parser = Parser::new_ext(&value, options);
    let mut parsed_text = String::new();
    push_html(&mut parsed_text, parser);
    // Parse the generated HTML
    let document = kuchikiki::parse_html().one(parsed_text.as_str());

    // Traverse the document and add a class to <a> tags
    for css_match in document.select("a").unwrap() {
        let mut attributes = css_match.attributes.borrow_mut();
        attributes.insert("class", "text-blue-500  hover:bg-gray-700".to_string());
    }

    parsed_text = document.to_string();

    parsed_text
}
