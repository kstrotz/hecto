/*
 * Hecto: a simple text editor in Rust.
 * https://www.philippflenker.com/hecto/
 *
 * 19 May 2020
 * 20 May 2020: completed up to Ch. 5, Performace Improvements
 * 22 May 2020: completed up to Ch. 7, Colorful Strings
 */

mod document;
mod editor;
mod filetype;
mod highlighting;
mod row;
mod terminal;

pub use document::Document;
use editor::Editor;
pub use editor::Position;
pub use editor::SearchDirection;
pub use filetype::FileType;
pub use filetype::HighlightingOptions;
pub use row::Row;
pub use terminal::Terminal;

fn main() -> () {
	Editor::default().run();
}
