use cosmic_settings_page as page;

pub mod keyboard;
mod mouse;

#[derive(Clone, Debug)]
pub enum Message {
    SetAcceleration(bool),
    SetNaturalScroll(bool),
    SetScrollSpeed(u32),
    SetDoubleClickSpeed(u32),
    SetMouseSpeed(u32),
    PrimaryButtonSelected(cosmic::widget::segmented_button::Entity),
    // seperate close message, to make sure another isn't closed?
    ExpandInputSourcePopover(Option<String>),
    OpenKeyboardShortcuts,
}

#[derive(derivative::Derivative)]
#[derivative(Default)]
pub struct Page {
    // Mouse
    #[derivative(Default(value = "mouse::default_primary_button()"))]
    primary_button: cosmic::widget::segmented_button::SingleSelectModel,
    acceleration: bool,
    natural_scroll: bool,
    double_click_speed: u32,
    scroll_speed: u32,
    mouse_speed: u32,

    // Keyboard
    expanded_source_popover: Option<String>,
    #[derivative(Default(value = "keyboard::default_input_sources()"))]
    sources: Vec<keyboard::InputSource>,
}

impl Page {
    // TODO
    pub fn update(&mut self, message: Message) {
        match message {
            Message::SetAcceleration(value) => {
                self.acceleration = value;
            }
            Message::SetNaturalScroll(value) => {
                self.natural_scroll = value;
            }
            Message::SetScrollSpeed(value) => {
                self.scroll_speed = value;
            }
            Message::SetDoubleClickSpeed(value) => {
                self.double_click_speed = value;
            }
            Message::SetMouseSpeed(value) => {
                self.mouse_speed = value;
            }
            Message::PrimaryButtonSelected(entity) => {
                self.primary_button.activate(entity);
            }
            Message::ExpandInputSourcePopover(value) => {
                self.expanded_source_popover = value;
            }
            // TODO Specially handled in app.rs
            Message::OpenKeyboardShortcuts => {}
        }
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn info(&self) -> page::Info {
        // XXX icon?
        page::Info::new("input", "input-keyboard-symbolic")
            .title(fl!("input"))
            .description(fl!("input", "desc"))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {
    fn sub_pages(page: page::Insert<crate::pages::Message>) -> page::Insert<crate::pages::Message> {
        page.sub_page::<keyboard::Page>().sub_page::<mouse::Page>()
    }
}
