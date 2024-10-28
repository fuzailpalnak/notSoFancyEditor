#[path = "core/scribe.rs"]
mod scribe;

fn main() {
    scribe::NotSoFancy::default().run();
}
