#[path = "core/editor.rs"]
mod editor;

fn main() {
    editor::Editor::default().run();
}
