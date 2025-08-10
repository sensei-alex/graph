use chrono::Datelike;
use log::{error, info, warn};
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
use ui_helpers::append_new_div;
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

    let app = App::new();
    page_container.set_inner_html(&app.open_file(app.readme).unwrap().render());
    doc_toolbar.set_inner_html(&app.open_file(app.doc_toolbar).unwrap().render());
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

#[derive(Debug)]
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
        info!("{:#?}", &note.text());
        note
    }
}

#[derive(Debug)]
struct Fonts {
    pub meta: LazyHash<FontBook>,
    pub data: Vec<Font>,
}

#[derive(Debug)]
pub struct App {
    files: Mutex<HashMap<FileId, File>>,
    /// The file that gets implicitly imported in all other files (intended for theming)
    pub readme: FileId,
    pub global_context: FileId,
    pub doc_toolbar: FileId,
    fonts: Fonts,
    typst_std_lib: LazyHash<Library>,
}

#[derive(Debug)]
pub struct Buffer<'a> {
    note: FileId,
    page: usize,
    store: &'a App,
}

trait FromPath {
    fn from_path(path: &str) -> Self;
}

impl FromPath for FileId {
    fn from_path(path: &str) -> Self {
        Self::new(None, VirtualPath::new(path))
    }
}

impl App {
    pub fn new() -> Self {
        let fonts = Self::get_default_fonts();
        let readme = FileId::from_path("/readme.typ");
        let global_context = FileId::from_path("/global ctx.typ");
        let toolbar_utils = FileId::from_path("/toolbar.typ");
        let doc_toolbar = FileId::from_path("/doc toolbar.typ");

        let files = Mutex::new(HashMap::from([
            Self::file_with_default_content(readme, "Hello, world!"),
            Self::file_with_default_content(global_context, include_str!("./ui/global-ctx.typ")),
            Self::file_with_default_content(toolbar_utils, include_str!("./ui/toolbar.typ")),
            Self::file_with_default_content(doc_toolbar, include_str!("./ui/doc-toolbar.typ")),
        ]));

        Self {
            files,
            readme,
            global_context,
            doc_toolbar,
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

    pub fn list_files(&self) -> Vec<FileId> {
        self.files
            .lock()
            .unwrap()
            .keys()
            .into_iter()
            .map(|id| id.clone())
            .collect()
    }

    pub fn open_file(&self, id: FileId) -> Result<Buffer, FileError> {
        let files = self.files.lock().map_err(|_| FileError::AccessDenied)?;

        if files.contains_key(&id) {
            Ok(Buffer {
                note: id,
                page: 0,
                store: self,
            })
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

    fn file_with_default_content(id: FileId, text: &str) -> (FileId, File) {
        (id, File::Typst(Source::new(id, text.to_string())))
    }
}

impl Buffer<'_> {
    /// Renders a page to SVG
    pub fn render(&self) -> String {
        info!("rendering the buffer");
        let Warned { output, warnings } = typst::compile::<PagedDocument>(&self);

        warn!("{:#?}", warnings);

        let document = match output {
            Ok(doc) => doc,
            Err(why) => return format!("<div class='error'>{:?}</div>", why),
        };

        match document.pages.get(self.page) {
            Some(page) => typst_svg::svg(page),
            None => {
                format!("Invalid buffer {:?}: page ID out of range", &self)
            }
        }
    }
}

impl World for Buffer<'_> {
    fn library(&self) -> &LazyHash<Library> {
        &self.store.typst_std_lib
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.store.fonts.meta
    }

    fn main(&self) -> FileId {
        self.note
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        let files = self
            .store
            .files
            .lock()
            .map_err(|_| FileError::AccessDenied)?;

        let global_context = match files.get(&self.store.global_context) {
            Some(file) => match file {
                File::Typst(source) => format!("{}\n", source.text()),
            },
            None => String::from("Where'd it go?"),
        };

        info!("Reading {:?}", &id);
        info!("Context {:?}", &global_context);

        match files.get(&id).ok_or(FileError::AccessDenied)? {
            File::Typst(source) => Ok(source.concat(&global_context)),
        }
    }

    fn file(&self, _id: FileId) -> FileResult<Bytes> {
        todo!("Binary files are not implemented yet")
    }

    fn font(&self, index: usize) -> Option<typst::text::Font> {
        Some(self.store.fonts.data[index].clone())
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
}
