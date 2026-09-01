//! Adapts the shared print queue to a Settings context drawer.

use cosmic::app::context_drawer::ContextDrawer;
use cosmic_settings_page::{self as page, Section, section};
use slotmap::SlotMap;

pub use cosmic_printers_ui::queue::Message;

#[derive(Default)]
pub struct Page {
    entity: page::Entity,
    pub(crate) ui: cosmic_printers_ui::queue::State,
}

impl Page {
    pub fn update(&mut self, message: Message) -> cosmic::app::Task<crate::Message> {
        self.ui.update(message)
    }
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::PrinterQueue(message)
    }
}

impl From<Message> for crate::app::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::PrinterQueue(message).into()
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

impl page::Page<crate::pages::Message> for Page {
    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }

    fn info(&self) -> page::Info {
        page::Info::new("printer-queue", "printer-symbolic")
            .title(cosmic_printers_ui::strings::printer_queue())
            .description(cosmic_printers_ui::strings::printer_queue_description())
    }

    fn content(
        &self,
        _sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        None
    }

    fn context_drawer(&self) -> Option<ContextDrawer<'_, crate::pages::Message>> {
        if !self.ui.has_printer() {
            return None;
        }

        Some(
            cosmic::app::context_drawer(
                cosmic_printers_ui::queue::queue_view(&self.ui)
                    .map(crate::pages::Message::PrinterQueue),
                crate::pages::Message::CloseContextDrawer,
            )
            .title(cosmic_printers_ui::strings::printer_queue()),
        )
    }

    fn on_context_drawer_close(&mut self) -> cosmic::Task<crate::pages::Message> {
        self.ui.clear_selection();
        cosmic::Task::none()
    }
}
