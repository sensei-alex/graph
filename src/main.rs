use chrono::Datelike;
use log::{error, info};
use std::{collections::HashMap, ops::Range, panic, sync::Mutex};

use typst::{
    Library, World,
    diag::{FileError, FileResult, Warned},
    foundations::{Bytes, Datetime},
    layout::PagedDocument,
    syntax::{FileId, Source, VirtualPath},
    text::{Font, FontBook},
    utils::LazyHash,
};
use ui_helpers::{append_new_button, append_new_div};
use web_sys::window;

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Debug).expect("Can't get the console"); // TODO if it's a prod build, change to INFO

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

    let app = GraphWorld::new();
    root.set_inner_html(&app.render_to_svg().unwrap());
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

enum File {
    Typst(Source),
    // todo add other types
}

trait GraphSource {
    fn concat(&self, text: &str) -> Source;
}

impl GraphSource for Source {
    fn concat(&self, text: &str) -> Source {
        let mut note = self.clone();

        note.edit(Range { start: 0, end: 0 }, text);
        note
    }
}

struct Fonts {
    pub meta: LazyHash<FontBook>,
    pub data: Vec<Font>,
}

pub struct GraphWorld {
    files: Mutex<HashMap<FileId, File>>,
    /// The file currently on the screen
    current_buffer: GraphBuffer,
    /// The file that gets implicitly imported in all other files (intended for theming)
    global_context: FileId,
    fonts: Fonts,
    typst_std_lib: LazyHash<Library>,
}

#[derive(Debug)]
pub struct GraphBuffer {
    page: usize,
    note_id: FileId,
}

trait FromPath {
    fn from_path(path: &str) -> Self;
}

impl FromPath for FileId {
    fn from_path(path: &str) -> Self {
        Self::new(None, VirtualPath::new(path))
    }
}

impl GraphWorld {
    pub fn new() -> Self {
        let fonts = Self::get_default_fonts();
        let current_note = FileId::from_path("/README.typ");
        let files = Mutex::new(HashMap::from([(
            current_note,
            File::Typst(Source::new(current_note, "Hello world.".to_string())),
        )]));

        let current_buffer = GraphBuffer {
            page: 0,
            note_id: current_note,
        };

        Self {
            files,
            current_buffer, // TODO restore from localstorage
            global_context: FileId::from_path("/GLOBAL.typ"),
            fonts,
            typst_std_lib: LazyHash::new(Library::default()), // stdlib
        }
    }

    pub fn replace_file(&mut self, id: FileId, content: &str) {
        let mut files = self.files.lock().unwrap();

        match files.get_mut(&id).unwrap() {
            File::Typst(source) => source.replace(content),
        };
    }

    /// Renders a page to SVG
    pub fn render_to_svg(&self) -> Result<String, String> {
        info!("rendering");
        let Warned { output, warnings } = typst::compile::<PagedDocument>(&self);
        let document = match output {
            Ok(doc) => doc,
            Err(why) => return Err(format!("{:?}", why)),
        };

        match document.pages.get(self.current_buffer.page) {
            Some(page) => Ok(typst_svg::svg(page)),
            None => {
                error!(
                    "Invalid buffer {:?}: page ID out of range",
                    &self.current_buffer
                );
                Err("Invalid buffer: page ID out of range".to_string())
            }
        }
    }

    pub fn list_files(&self) -> Vec<FileId> {
        self.files
            .lock()
            .unwrap()
            .keys()
            .into_iter()
            .map(|id| id.clone())
            .collect()
    }

    pub fn open_file(&mut self, id: FileId) -> Result<(), FileError> {
        let files = self.files.lock().map_err(|_| FileError::AccessDenied)?;

        if files.contains_key(&id) {
            self.current_buffer = GraphBuffer {
                note_id: id,
                page: 0,
            };

            Ok(())
        } else {
            Err(FileError::AccessDenied)
        }
    }

    // from obsidian-typst
    fn get_default_fonts() -> Fonts {
        let mut meta = FontBook::new();
        let mut data = Vec::new();

        for bytes in typst_assets::fonts() {
            let buffer = Bytes::new(bytes);
            for font in Font::iter(buffer) {
                meta.push(font.info().clone());
                data.push(font);
            }
        }

        return Fonts {
            meta: meta.into(),
            data,
        };
    }
}

impl World for GraphWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.typst_std_lib
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.fonts.meta
    }

    fn main(&self) -> FileId {
        self.current_buffer.note_id
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        let files = self.files.lock().map_err(|_| FileError::AccessDenied)?;

        let global_context = match files.get(&self.global_context) {
            Some(file) => match file {
                File::Typst(source) => format!("{}\n", source.text()),
            },
            None => String::new(),
        };

        info!("Reading {:?}", &id);

        match files.get(&id).ok_or(FileError::AccessDenied)? {
            File::Typst(source) => Ok(source.concat(&global_context)),
        }
    }

    fn file(&self, _id: FileId) -> FileResult<Bytes> {
        todo!("Binary files are not implemented yet")
    }

    fn font(&self, index: usize) -> Option<typst::text::Font> {
        Some(self.fonts.data[index].clone())
    }

    fn today(&self, offset: Option<i64>) -> Option<typst::foundations::Datetime> {
        let now = chrono::Local::now();

        let naive = match offset {
            None => now.naive_local(),
            Some(o) => now.naive_utc() + chrono::Duration::hours(o),
        };

        Datetime::from_ymd(
            naive.year(),
            naive.month().try_into().ok()?,
            naive.day().try_into().ok()?,
        )
    }
}

mod ui_helpers {
    use crate::Action;
    use web_sys::{Document, Element};

    /// - Creates a new `div` element
    /// - Appends it to `parent`
    /// - Assigns it the `class`
    /// - Gives you the ownership
    ///
    /// ## Panics
    /// - if `document.create_element` fails
    /// - if `parent.append_child` fails
    pub fn append_new_div(document: &Document, parent: &Element, class: &str) -> Element {
        let div = document
            .create_element("div")
            .expect("Can't create a DOM element");
        div.set_class_name(class);
        parent
            .append_child(&div)
            .expect("Can't append a DOM element");
        div
    }

    /// - Creates a new `div` element
    /// - Appends it to `parent`
    /// - Assigns it the `class`
    /// - Gives you the ownership
    ///
    /// ## Panics
    /// - if `document.create_element` fails
    /// - if `parent.append_child` fails
    pub fn append_new_button(document: &Document, parent: &Element, action: Action) -> Element {
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
}
