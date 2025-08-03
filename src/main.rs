use std::sync::mpsc::{Receiver, Sender, channel};
use web_sys::{Document, Element, window};

fn main() {
    println!("todo setup panic hook!");

    let document = window()
        .and_then(|win| win.document())
        .expect("Can't get document");
    let body = document.body().expect("Can't get body");

    let root = append_new_div(&document, &body, "root");
    let graph_container = append_new_div(&document, &root, "graph");
    let page_toolbar = append_new_div(&document, &root, "toolbar toolbar--page");
    let doc_toolbar = append_new_div(&document, &root, "toolbar toolbar--doc");
    let page_container = append_new_div(&document, &root, "page");

    let button_help = append_new_button(&document, &doc_toolbar, Action::Help);
    let _ = append_new_div(&document, &doc_toolbar, "toolbar__spacer");
    let button_prev = append_new_button(&document, &doc_toolbar, Action::PrevPage);
    let button_next = append_new_button(&document, &doc_toolbar, Action::NextPage);
}

/// - Creates a new `div` element
/// - Appends it to `parent`
/// - Assigns it the `class`
/// - Gives you the ownership
///
/// ## Panics
/// - if `document.create_element` fails
/// - if `parent.append_child` fails
fn append_new_div(document: &Document, parent: &Element, class: &str) -> Element {
    let div = document
        .create_element("div")
        .expect("Can't create a DOM element");
    div.set_class_name(class);
    parent
        .append_child(&div)
        .expect("Can't append a DOM element");
    div
}

#[derive(Clone, Copy)]
enum Icon {
    QuestionMark,
    Next,
    Prev,
}

#[derive(Clone, Copy)]
enum Action {
    NextPage,
    PrevPage,
    Help,
}

impl Icon {
    fn to_svg(self) -> &'static str {
        match self {
            Self::QuestionMark => include_str!("../icons/circle-question-mark.svg"),
            Self::Next => include_str!("../icons/chevron-right.svg"),
            Self::Prev => include_str!("../icons/chevron-left.svg"),
        }
    }
}

impl Action {
    fn to_icon(self) -> Icon {
        match self {
            Self::NextPage => Icon::Next,
            Self::PrevPage => Icon::Prev,
            Self::Help => Icon::QuestionMark,
        }
    }

    fn to_hotkey(self) -> char {
        match self {
            Self::NextPage => 'n',
            Self::PrevPage => 'p',
            Self::Help => 'h',
        }
    }
}

fn append_new_button(document: &Document, parent: &Element, action: Action) -> Element {
    let button = document
        .create_element("button")
        .expect("Can't create a button in the DOM");
    button.set_class_name("button");
    button.set_inner_html(action.to_icon().to_svg());

    let hotkey = action.to_hotkey().to_string();
    let hotkey_element = document
        .create_element("span")
        .expect("Can't create a span in the DOM");
    hotkey_element.set_class_name("button__hotkey");
    hotkey_element.set_text_content(Some(&hotkey));
    button
        .append_child(&hotkey_element)
        .expect("Can't append a DOM element");

    parent
        .append_child(&button)
        .expect("Can't append a DOM element");
    button
}
