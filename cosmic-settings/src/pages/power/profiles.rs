//Localization TO-DO

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
        page::Info::new("power-profiles", "preferences-desktop-locale-symbolic")
            .title("Power Profiles")
            .description("Manage power profiles")
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    PowerProfileChange(String),
}

fn profiles() -> Section<crate::pages::Message> {
    Section::default()
        .title("Power Settings")
        .descriptions(vec!["power settings into".into()])
        .view::<Page>(|_binder, page, section| {
            // Safe to do we are always returning ok
            let profiles = load_profiles().unwrap();

            let mut section = settings::view_section(&section.title); // Note the mutable binding

            let mut widgets = Vec::new();
            
            for v in profiles.into_iter() {
                println!("{}", get_selected_profile());
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

            // Now add all widgets to the section
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
    let output = match Command::new("powerprofilesctl").arg("list").output() {
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
        let mut profile = l.replace(":", "").replace("-", " ");

        if let Some(first_char) = profile.chars().next() {
            if first_char.is_ascii_uppercase() {
                continue;
            }
        }
    
        if !profile.is_empty() {
            profile = profile[..1].to_uppercase() + &profile[1..];
            profiles.push(profile);
        }
    }
    

   
    Ok(profiles)
}


fn update_selected_profile(mut profile_name: String) {
    profile_name = profile_name.replace(" ", "-");

    profile_name = profile_name.to_lowercase();

    match Command::new("powerprofilesctl").arg("set").arg(profile_name).output() {
        Ok(output) => output,
        Err(_) => return
    };

    println!("Power profile updated.");

}

fn get_selected_profile() -> String {
    let output = match Command::new("powerprofilesctl").arg("get").output() {
        Ok(output) => output,
        Err(err) => {
            println!("Power profiles deamon is not installed.");
            return "default".to_string();
        }
    };

    let profile_name = String::from_utf8_lossy(&output.stdout).to_string();

    profile_name[0..1].to_uppercase() + &profile_name[1..profile_name.len()].trim_end_matches("\n")

}

// TO-DO move char logic here
fn prepare_for_display(str: &String){

}


// TO-DO move char logic here
fn prepare_for_command(str: &String){

}
