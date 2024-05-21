use cosmic::iced::widget;
use cosmic::{
    widget::settings,
    Apply,
};
use cosmic_settings_page::Section;
use cosmic_settings_page::{self as page, section};
use slotmap::SlotMap;
use std::error::Error;
use std::process::Command;

static POWER_PROFILES_DAEMON : &str = "powerprofilesctl";

#[derive(Default)]
pub struct Page;

impl Page {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::PowerProfileChange(s) => update_selected_profile(s),
        }
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(profiles())])
    }

    fn info(&self) -> page::Info {
        page::Info::new("power-profiles", "battery-symbolic")
            .title(fl!("power-profiles"))
            .description(fl!("power-profiles", "desc"))
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    PowerProfileChange(String),
}

fn profiles() -> Section<crate::pages::Message> {
    Section::default()
        .title(fl!("power-profiles"))
        .descriptions(vec![fl!("power", "desc").into()])
        .view::<Page>(|_binder, page, section| {
            // Safe to do we are always returning ok
            let profiles = load_profiles().unwrap();

            let mut section = settings::view_section(&section.title);

            let mut widgets = Vec::new();
            
            for v in profiles.into_iter() {
                let selected = if get_selected_profile() == v { 
                    Some(true)
                } else {
                    None
                };

                let widget = widget::Radio::new("", true, selected, |_| {
                    Message::PowerProfileChange(v.clone())
                });
                let item = settings::item::builder(v)
                    .control(widget);
                widgets.push(item);
            }

            for item in widgets {
                section = section.add(item);
            }

            section
                .apply(cosmic::Element::from)
                .map(crate::pages::Message::PowerProfile)
        })
}

impl page::AutoBind<crate::pages::Message> for Page {}

fn load_profiles() -> Result<Vec<String>, Box<dyn Error>> {
    let output = match Command::new(POWER_PROFILES_DAEMON).arg("list").output() {
        Ok(output) => output,
        Err(err) => {
            println!("Power profiles deamon is not installed.");
            return Ok(Vec::new());
        }
    };

    let content = String::from_utf8_lossy(&output.stdout);

    let content = content.replace("*", "").replace(" ", "");

    let mut profiles = Vec::new();

    for l in content.lines() {
        let profile = prepare_for_display(&l.to_string());
        if !profile.is_empty() {
            profiles.push(profile);
        }
    }

    Ok(profiles)
}

fn update_selected_profile(mut profile_name: String) {
    profile_name = prepare_for_command(&profile_name);

    match Command::new(POWER_PROFILES_DAEMON).arg("set").arg(profile_name).output() {
        Ok(output) => output,
        Err(_) => return,
    };

    println!("Power profile updated.");
}

fn get_selected_profile() -> String {
    let output = match Command::new(POWER_PROFILES_DAEMON).arg("get").output() {
        Ok(output) => output,
        Err(err) => {
            println!("Power profiles deamon is not installed.");
            return "default".to_string();
        }
    };

    let profile_name = String::from_utf8_lossy(&output.stdout).to_string();

    prepare_for_display(&profile_name)
}

fn prepare_for_display(str: &String) -> String {
    let mut profile = str.replace(":", "").replace("-", " ");
    if let Some(first_char) = profile.chars().next() {
        if first_char.is_ascii_uppercase() {
            return String::new();
        }
    }
    if !profile.is_empty() {
        profile = profile[..1].to_uppercase() + &profile[1..];
    }
    profile.trim().to_string()
}

fn prepare_for_command(str: &String) -> String {
    str.replace(" ", "-").to_lowercase()
}