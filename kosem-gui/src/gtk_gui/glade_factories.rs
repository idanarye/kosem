use crate::gtk_gui::glade_templating::GladeFactory;

pub struct GladeFactories {
    pub join_menu: JoinMenuFactories,
}

impl GladeFactories {
    pub fn new() -> Self {
        Self {
            join_menu: JoinMenuFactories::new(),
        }
    }
}

pub struct JoinMenuFactories {
    pub request_row: GladeFactory<gtk::ListBoxRow>,
    pub window: GladeFactory<gtk::ApplicationWindow>,
}

impl JoinMenuFactories {
    pub fn new() -> Self {
        let mut xml_extractor = crate::gtk_gui::Asset::xml_extractor("main_menu.glade");

        let request_row = xml_extractor.extract("request_row");
        let window = xml_extractor.extract("window");

        Self {
            request_row,
            window,
        }
    }
}
