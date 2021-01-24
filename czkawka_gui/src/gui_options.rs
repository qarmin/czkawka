#[derive(Clone)]
pub struct GUIOptions {}

impl GUIOptions {
    pub fn create_from_builder(_builder: &gtk::Builder) -> Self {
        // let notebook_main: gtk::Notebook = builder.get_object("notebook_main").unwrap();

        Self {}
    }
}
