mod editor;

fn main() -> std::io::Result<()> {
    editor::Editor::default().run()?;

    Ok(())
}
