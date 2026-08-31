//! Adapts the shared printer-details screen to a Settings page.

use cosmic::Element;
use cosmic::iced::{Subscription, event, keyboard};
use cosmic_printers_ui::strings;
use cosmic_settings_page::{self as page, Section, section};
use slotmap::SlotMap;

pub use cosmic_printers_ui::details::Message;

#[derive(Default)]
pub struct Page {
    entity: page::Entity,
    pub(crate) ui: cosmic_printers_ui::details::State,
}

impl Page {
    pub fn update(&mut self, message: Message) -> cosmic::app::Task<crate::Message> {
        self.ui.update(message)
    }
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::PrinterDetails(message)
    }
}

impl From<Message> for crate::app::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::PrinterDetails(message).into()
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

impl page::Page<crate::pages::Message> for Page {
    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }

    fn info(&self) -> page::Info {
        page::Info::new("printer-details", "printer-symbolic")
            .title(strings::printer_details())
            .description(strings::printer_details_description())
    }

    fn subscription(&self, _core: &cosmic::Core) -> Subscription<crate::pages::Message> {
        event::listen_with(|event, _, _| match event {
            cosmic::iced::Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) => {
                Some(crate::pages::Message::PrinterQueue(
                    super::queue::Message::ModifiersChanged(modifiers),
                ))
            }
            _ => None,
        })
    }

    fn header(&self) -> Option<Element<'_, crate::pages::Message>> {
        cosmic_printers_ui::details::header_view(&self.ui)
            .map(|element| element.map(crate::pages::Message::PrinterDetails))
    }

    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
            sections.insert(default_and_queue()),
            sections.insert(printer_information()),
            sections.insert(printer_preferences()),
            sections.insert(supplies()),
            sections.insert(remove_printer()),
            sections.insert(nothing_selected()),
        ])
    }

    fn dialog(&self) -> Option<Element<'_, crate::pages::Message>> {
        cosmic_printers_ui::details::dialog_view(&self.ui)
            .map(|element| element.map(crate::pages::Message::PrinterDetails))
    }
}

fn nothing_selected() -> Section<crate::pages::Message> {
    Section::default()
        // Not a setting, so it has no business in search results.
        .search_ignore()
        .show_while::<Page>(|page| !page.ui.has_printer())
        .view::<Page>(|_binder, _page, _section| {
            cosmic_printers_ui::details::nothing_selected_view()
                .map(crate::pages::Message::PrinterDetails)
        })
}

fn default_and_queue() -> Section<crate::pages::Message> {
    crate::slab!(descriptions {
        _set_default = strings::set_as_default_printer();
        _queue = strings::printer_queue();
    });

    Section::default()
        .title(strings::printer_details())
        .descriptions(descriptions)
        .show_while::<Page>(|page| page.ui.has_printer())
        .view::<Page>(move |_binder, page, section| {
            cosmic_printers_ui::details::default_and_queue_view(&page.ui, &section.title)
                .map(crate::pages::Message::PrinterDetails)
        })
}

fn printer_information() -> Section<crate::pages::Message> {
    let [location, model, device_name, driver_version] = strings::printer_information_rows();
    crate::slab!(descriptions {
        _location_row = location;
        _model_row = model;
        _device_name_row = device_name;
        _driver_version_row = driver_version;
    });

    Section::default()
        .title(strings::printer_information())
        .descriptions(descriptions)
        .show_while::<Page>(|page| page.ui.has_printer())
        .view::<Page>(move |_binder, page, section| {
            cosmic_printers_ui::details::printer_information_view(&page.ui, &section.title)
                .map(crate::pages::Message::PrinterDetails)
        })
}

fn printer_preferences() -> Section<crate::pages::Message> {
    let [paper_size, print_sides] = strings::printing_preferences_rows();
    crate::slab!(descriptions {
        _paper_size_row = paper_size;
        _print_sides_row = print_sides;
    });

    Section::default()
        .title(strings::printing_preferences())
        .descriptions(descriptions)
        .show_while::<Page>(|page| page.ui.has_printer())
        .view::<Page>(move |_binder, page, section| {
            cosmic_printers_ui::details::printer_preferences_view(
                &page.ui,
                &section.title,
                crate::Message::from,
            )
            .map(crate::pages::Message::PrinterDetails)
        })
}

fn supplies() -> Section<crate::pages::Message> {
    Section::default()
        .title(strings::supplies())
        .show_while::<Page>(|page| page.ui.has_supplies())
        .view::<Page>(|_binder, page, section| {
            cosmic_printers_ui::details::supplies_view(&page.ui, &section.title)
                .map(crate::pages::Message::PrinterDetails)
        })
}

fn remove_printer() -> Section<crate::pages::Message> {
    Section::default()
        .title(strings::remove_printer())
        .show_while::<Page>(|page| page.ui.can_remove_printer())
        .view::<Page>(|_binder, page, _section| {
            cosmic_printers_ui::details::remove_printer_view(&page.ui)
                .map(crate::pages::Message::PrinterDetails)
        })
}
