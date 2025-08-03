use std::sync::mpsc::{Receiver, Sender, channel};
use web_sys::{Document, Element, window};

fn main() {
    println!("todo setup panic hook!");

    let document = window()
        .and_then(|win| win.document())
        .expect("Can't get document");
    let body = document.body().expect("Can't get body");

    let text = document.create_element("div").unwrap();
    text.set_text_content(Some("Hello, world."));
    body.append_child(&text).unwrap();

    let buttons = vec![Button::new(
        Icon::Next,
        Shortcut {
            mode: vec![Mode::Normal],
            ctrl: false,
            key: Key::Down,
        },
    )];
}

struct Button {
    pub icon: Icon,
    pub shortcut: Shortcut,
    pub rx: Receiver<()>,
    tx: Sender<()>,
}

struct Shortcut {
    mode: Vec<Mode>,
    ctrl: bool,
    key: Key,
}

enum Key {
    Char(char),
    Esc,
    Left,
    Down,
    Up,
    Right,
    Return,
}

enum Mode {
    Readonly,
    Normal,
    Visual,
    VisualLine,
    VisualBlock,
    Insert,
    Object,
}

enum Icon {
    Next,
    Prev,
    QuestionMark,
}

impl Button {
    fn new(icon: Icon, shortcut: Shortcut) -> Self {
        let (tx, rx) = channel();

        Self {
            icon,
            shortcut,
            rx,
            tx,
        }
    }
}

fn build_toolbar(document: &Document, buttons: &Vec<Button>) -> Option<Element> {
    let container = document.create_element("div").ok()?;

    None
}
