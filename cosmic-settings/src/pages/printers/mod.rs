//! Adapts the shared printer screens to COSMIC Settings pages.

pub mod details;
pub mod queue;

use cosmic::Element;
use cosmic::iced::{Subscription, event, keyboard};
use cosmic_printers_ui::strings;
use cosmic_settings_page::{self as page, Section, section};
use slotmap::SlotMap;

pub use cosmic_printers_ui::list::Message;

#[derive(Default)]
pub struct Page {
    entity: page::Entity,
    pub(crate) ui: cosmic_printers_ui::list::State,
}

impl Page {
    pub fn update(&mut self, message: Message) -> cosmic::app::Task<crate::Message> {
        self.ui.update(message)
    }
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::Printers(message)
    }
}

impl From<Message> for crate::Message {
    fn from(message: Message) -> Self {
        crate::Message::PageMessage(message.into())
    }
}

/// Required because task mapping does not chain `From` conversions.
impl From<cosmic_printers_ui::add_printer::Message> for crate::Message {
    fn from(message: cosmic_printers_ui::add_printer::Message) -> Self {
        crate::Message::from(Message::AddPrinter(message))
    }
}

impl From<cosmic_printers_ui::Request> for crate::pages::Message {
    fn from(request: cosmic_printers_ui::Request) -> Self {
        crate::pages::Message::PrinterRequest(request)
    }
}

impl From<cosmic_printers_ui::Request> for crate::Message {
    fn from(request: cosmic_printers_ui::Request) -> Self {
        crate::Message::PageMessage(request.into())
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }

    fn info(&self) -> page::Info {
        page::Info::new("printers", "printer-symbolic").title(strings::printers())
    }

    fn header(&self) -> Option<Element<'_, crate::pages::Message>> {
        Some(cosmic_printers_ui::list::page_header().map(crate::pages::Message::Printers))
    }

    fn dialog(&self) -> Option<Element<'_, crate::pages::Message>> {
        self.ui.add_printer_dialog().map(|dialog| {
            cosmic_printers_ui::add_printer::dialog(dialog)
                .map(Message::AddPrinter)
                .map(crate::pages::Message::Printers)
        })
    }

    fn on_enter(&mut self) -> cosmic::Task<crate::pages::Message> {
        cosmic::task::message(crate::pages::Message::Printers(Message::Refresh))
    }

    fn subscription(&self, _core: &cosmic::Core) -> Subscription<crate::pages::Message> {
        Subscription::batch([
            Subscription::run(printer_events).map(crate::pages::Message::Printers),
            // The queue is a drawer, so the visible printer page forwards modifiers to it.
            event::listen_with(|event, _, _| match event {
                cosmic::iced::Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) => {
                    Some(crate::pages::Message::PrinterQueue(
                        queue::Message::ModifiersChanged(modifiers),
                    ))
                }
                _ => None,
            }),
        ])
    }

    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
            sections.insert(default_printer_section()),
            sections.insert(printers_section()),
        ])
    }
}

impl page::AutoBind<crate::pages::Message> for Page {
    fn sub_pages(page: page::Insert<crate::pages::Message>) -> page::Insert<crate::pages::Message> {
        page.sub_page::<details::Page>().sub_page::<queue::Page>()
    }
}

fn printer_events() -> impl cosmic::iced::futures::Stream<Item = Message> {
    cosmic_printers_ui::list::printer_events_subscription(cosmic_printers_ui::Backend::default())
}

fn default_printer_section() -> Section<crate::pages::Message> {
    crate::slab!(descriptions {
        _default_printer = strings::default_printer();
    });

    Section::default()
        .title(strings::default_printer())
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, _section| {
            cosmic_printers_ui::list::default_printer_view(&page.ui, crate::Message::from)
                .map(crate::pages::Message::Printers)
        })
}

fn printers_section() -> Section<crate::pages::Message> {
    Section::default()
        .title(strings::printers())
        .view::<Page>(|_binder, page, _section| {
            cosmic_printers_ui::list::printers_view(&page.ui).map(crate::pages::Message::Printers)
        })
}
