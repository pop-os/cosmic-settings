// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

pub mod app_details;
pub mod applications_category;
pub mod home_category;
pub mod models;
pub mod other_category;
pub mod system_category;
mod utils;

use cosmic_settings_page::{self as page, Section, section};

use cosmic::app::context_drawer::ContextDrawer;
use cosmic::iced::widget::container::Style as ContainerStyle;
use cosmic::iced::{Alignment, Background, Border, Color, Length, Subscription};
use cosmic::widget::{button, column, container, icon, progress_bar, row, settings, text};
use cosmic::{Apply, Task};
use slab::Slab;
use slotmap::{Key, SlotMap};
use std::time::Duration;

use utils::{category_color, format_bytes, loading_spinner};

pub use models::{FlatpakApp, HomeCategory, StorageInfo, SystemCategory};

const COLOR_INDICATOR_SIZE: f32 = 12.0;
const STORAGE_BAR_HEIGHT: f32 = 24.0;
const SEGMENT_SPACING: u16 = 1;
const CORNER_RADIUS: f32 = 4.0;
const SMALL_CORNER_RADIUS: f32 = 2.0;

fn color_indicator<'a, Message: 'a>(color: Color) -> cosmic::Element<'a, Message> {
    container(cosmic::widget::Space::new(
        Length::Fixed(COLOR_INDICATOR_SIZE),
        Length::Fixed(COLOR_INDICATOR_SIZE),
    ))
    .style(move |_theme| ContainerStyle {
        background: Some(Background::Color(color)),
        border: Border {
            radius: SMALL_CORNER_RADIUS.into(),
            ..Default::default()
        },
        ..Default::default()
    })
    .into()
}

fn create_bar_segment<'a, Message: 'a>(
    portion: u16,
    color: Color,
    radius: [f32; 4],
) -> cosmic::Element<'a, Message> {
    container(cosmic::widget::Space::new(
        Length::Fill,
        Length::Fixed(STORAGE_BAR_HEIGHT),
    ))
    .width(Length::FillPortion(portion))
    .style(move |_theme| ContainerStyle {
        background: Some(Background::Color(color)),
        border: Border {
            radius: radius.into(),
            ..Default::default()
        },
        ..Default::default()
    })
    .into()
}

fn segment_radius(is_first: bool, is_last: bool) -> [f32; 4] {
    match (is_first, is_last) {
        (true, true) => [CORNER_RADIUS; 4],
        (true, false) => [CORNER_RADIUS, 0.0, 0.0, CORNER_RADIUS],
        (false, true) => [0.0, CORNER_RADIUS, CORNER_RADIUS, 0.0],
        (false, false) => [0.0; 4],
    }
}

fn segmented_storage_bar<'a, Message: 'a>(info: &StorageInfo) -> cosmic::Element<'a, Message> {
    if info.total == 0 {
        return container(cosmic::widget::Space::new(
            Length::Fill,
            Length::Fixed(STORAGE_BAR_HEIGHT),
        ))
        .into();
    }

    let segments_data = [
        (info.system, category_color(&CategoryType::System)),
        (info.home, category_color(&CategoryType::Home)),
        (
            info.applications,
            category_color(&CategoryType::Applications),
        ),
        (info.other, category_color(&CategoryType::Other)),
        (info.available, utils::COLOR_AVAILABLE),
    ];

    // Calculate scaling factor to fit all values in u16 range
    // We always need to scale since storage sizes in bytes exceed u16::MAX
    let scale = info.total as f64 / (u16::MAX as f64 * 0.9); // Use 90% of max to leave room

    // Scale and filter out zero-size segments
    let scaled_segments: Vec<(u16, Color)> = segments_data
        .iter()
        .map(|(size, color)| {
            let scaled = if *size > 0 {
                ((*size as f64 / scale) as u16).max(1) // Ensure non-zero sizes don't become 0
            } else {
                0
            };
            (scaled, *color)
        })
        .filter(|(portion, _)| *portion > 0)
        .collect();

    if scaled_segments.is_empty() {
        return container(cosmic::widget::Space::new(
            Length::Fill,
            Length::Fixed(STORAGE_BAR_HEIGHT),
        ))
        .into();
    }

    // Build row with appropriate corner rounding for each segment
    let last_index = scaled_segments.len() - 1;
    let segments = scaled_segments.into_iter().enumerate().fold(
        row::with_capacity(last_index + 1).spacing(SEGMENT_SPACING),
        |row, (index, (portion, color))| {
            let radius = segment_radius(index == 0, index == last_index);
            row.push(create_bar_segment(portion, color, radius))
        },
    );

    segments.width(Length::Fill).into()
}

fn category_button<'a>(
    label: &'a str,
    category_type: CategoryType,
    size: u64,
    loading: bool,
    animation_state: u8,
) -> cosmic::Element<'a, Message> {
    let color = category_color(&category_type);

    let size_element: cosmic::Element<Message> = if loading {
        loading_spinner(animation_state)
    } else {
        text::body(format_bytes(size)).into()
    };

    let row_content = row::with_capacity(4)
        .spacing(12)
        .align_y(Alignment::Center)
        .push(color_indicator(color))
        .push(text::body(label).width(Length::Fill))
        .push(size_element)
        .push(icon::from_name("go-next-symbolic").size(16));

    button::custom(row_content)
        .padding([12, 16])
        .on_press(Message::SelectCategory(Some(category_type)))
        .width(Length::Fill)
        .class(cosmic::theme::Button::MenuItem)
        .into()
}

#[derive(Clone, Debug, PartialEq)]
pub enum CategoryType {
    System,
    Home,
    Applications,
    Other,
}

#[derive(Clone, Debug)]
pub enum Message {
    StorageInfo(StorageInfo),
    FlatpakAppsWithSizes(Vec<FlatpakApp>),
    CategoryDetails {
        system: SystemCategory,
        home: HomeCategory,
    },
    // Incremental field updates
    SystemFieldUpdate(SystemFieldUpdate),
    HomeFieldUpdate(HomeFieldUpdate),
    HomeTotalAndVar {
        total_home: u64,
        var_dir: u64,
    },
    SelectCategory(Option<CategoryType>),
    AnimationTick,
}

#[derive(Clone, Debug)]
pub enum SystemFieldUpdate {
    SystemFiles(u64),
    PackageCache(u64),
    SystemLogs(u64),
    SystemCache(u64),
    BootFiles(u64),
    FlatpakRuntimes(u64),
}

#[derive(Clone, Debug)]
pub enum HomeFieldUpdate {
    Documents(u64),
    Downloads(u64),
    Pictures(u64),
    Videos(u64),
    Music(u64),
    Desktop(u64),
    Other(u64),
}

impl From<Message> for crate::app::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::Storage(message).into()
    }
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::Storage(message)
    }
}

#[derive(Clone, Debug)]
struct SubPages {
    system: page::Entity,
    home: page::Entity,
    applications: page::Entity,
    other: page::Entity,
}

impl Default for SubPages {
    fn default() -> Self {
        Self {
            system: page::Entity::null(),
            home: page::Entity::null(),
            applications: page::Entity::null(),
            other: page::Entity::null(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Page {
    entity: page::Entity,
    storage_info: StorageInfo,
    system_category: SystemCategory,
    home_category: HomeCategory,
    loading: bool,
    pending_tasks: u8,
    on_enter_handle: Option<cosmic::iced::task::Handle>,
    sub_pages: SubPages,
    animation_state: u8,
    home_total_and_var: Option<(u64, u64)>,
    home_dirs_loaded_count: u8,
}

impl page::AutoBind<crate::pages::Message> for Page {
    fn sub_pages(
        mut page: page::Insert<crate::pages::Message>,
    ) -> page::Insert<crate::pages::Message> {
        let system = page.sub_page_with_id::<system_category::Page>();
        let home = page.sub_page_with_id::<home_category::Page>();
        let applications = page.sub_page_with_id::<applications_category::Page>();
        let other = page.sub_page_with_id::<other_category::Page>();

        let model = page.model.page_mut::<Page>().unwrap();
        model.sub_pages.system = system;
        model.sub_pages.home = home;
        model.sub_pages.applications = applications;
        model.sub_pages.other = other;

        page
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }

    fn content(
        &self,
        sections: &mut SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
            sections.insert(storage_overview()),
            sections.insert(storage_categories()),
        ])
    }

    fn info(&self) -> page::Info {
        page::Info::new("storage", "drive-harddisk-symbolic")
            .title(fl!("storage"))
            .description(fl!("storage", "desc"))
    }

    fn on_enter(&mut self) -> Task<crate::pages::Message> {
        if self.loading || self.storage_info.total > 0 {
            return Task::none();
        }

        self.loading = true;

        let (task, handle) = Task::future(async move {
            crate::pages::Message::Storage(Message::StorageInfo(StorageInfo::load()))
        })
        .abortable();

        self.on_enter_handle = Some(handle);
        task
    }

    fn on_leave(&mut self) -> Task<crate::pages::Message> {
        if let Some(handle) = self.on_enter_handle.take() {
            handle.abort();
        }
        Task::none()
    }

    fn context_drawer(&self) -> Option<ContextDrawer<'_, crate::pages::Message>> {
        None
    }

    fn subscription(&self, _core: &cosmic::Core) -> Subscription<crate::pages::Message> {
        if self.loading || self.pending_tasks > 0 {
            // Animate loading indicator while loading
            cosmic::iced::time::every(Duration::from_millis(500))
                .map(|_| crate::pages::Message::Storage(Message::AnimationTick))
        } else {
            Subscription::none()
        }
    }
}

impl Page {
    pub fn update(&mut self, message: Message) -> cosmic::app::Task<crate::Message> {
        match message {
            Message::StorageInfo(info) => {
                let old_system = self.storage_info.system;
                let old_home = self.storage_info.home;
                let old_applications = self.storage_info.applications;
                let old_other = self.storage_info.other;
                let old_apps = self.storage_info.flatpak_apps.clone();

                let apps_to_load = if old_apps.is_empty() {
                    info.flatpak_apps.clone()
                } else {
                    old_apps.clone()
                };

                self.storage_info = info;

                self.storage_info.system = old_system.max(self.storage_info.system);
                self.storage_info.home = old_home.max(self.storage_info.home);
                self.storage_info.applications =
                    old_applications.max(self.storage_info.applications);
                self.storage_info.other = old_other.max(self.storage_info.other);
                if !old_apps.is_empty() {
                    self.storage_info.flatpak_apps = old_apps;
                }

                // Keep loading state true until all background tasks complete
                // self.loading will be set to false when pending_tasks reaches 0
                // Track background tasks: 1 flatpak apps + 6 system fields + 6 home dirs = 13
                // (home "other" will be calculated from total_and_var + sum of 6 dirs, no extra scan)
                self.pending_tasks = 13;
                self.home_total_and_var = None;
                self.home_dirs_loaded_count = 0;

                // Spawn background tasks without blocking navigation
                // Use spawn_blocking to prevent heavy I/O from blocking the async executor
                let mut tasks = vec![
                    cosmic::Task::future(async move {
                        let apps = tokio::task::spawn_blocking(move || {
                            StorageInfo::load_flatpak_apps_with_sizes(apps_to_load)
                        })
                        .await
                        .unwrap_or_default();
                        Message::FlatpakAppsWithSizes(apps)
                    })
                    .map(crate::app::Message::from)
                    .map(Into::into),
                ];

                tasks.push(
                    cosmic::Task::future(async {
                        let size = tokio::task::spawn_blocking(|| {
                            models::SystemCategory::load_system_files()
                        })
                        .await
                        .unwrap_or(0);
                        Message::SystemFieldUpdate(SystemFieldUpdate::SystemFiles(size))
                    })
                    .map(crate::app::Message::from)
                    .map(Into::into),
                );

                tasks.push(
                    cosmic::Task::future(async {
                        let size = tokio::task::spawn_blocking(|| {
                            models::SystemCategory::load_boot_files()
                        })
                        .await
                        .unwrap_or(0);
                        Message::SystemFieldUpdate(SystemFieldUpdate::BootFiles(size))
                    })
                    .map(crate::app::Message::from)
                    .map(Into::into),
                );

                tasks.push(
                    cosmic::Task::future(async {
                        let size = tokio::task::spawn_blocking(|| {
                            models::SystemCategory::load_system_logs()
                        })
                        .await
                        .unwrap_or(0);
                        Message::SystemFieldUpdate(SystemFieldUpdate::SystemLogs(size))
                    })
                    .map(crate::app::Message::from)
                    .map(Into::into),
                );

                tasks.push(
                    cosmic::Task::future(async {
                        let size = tokio::task::spawn_blocking(|| {
                            models::SystemCategory::load_package_cache()
                        })
                        .await
                        .unwrap_or(0);
                        Message::SystemFieldUpdate(SystemFieldUpdate::PackageCache(size))
                    })
                    .map(crate::app::Message::from)
                    .map(Into::into),
                );

                tasks.push(
                    cosmic::Task::future(async {
                        let size = tokio::task::spawn_blocking(|| {
                            models::SystemCategory::load_flatpak_runtimes()
                        })
                        .await
                        .unwrap_or(0);
                        Message::SystemFieldUpdate(SystemFieldUpdate::FlatpakRuntimes(size))
                    })
                    .map(crate::app::Message::from)
                    .map(Into::into),
                );

                tasks.push(
                    cosmic::Task::future(async {
                        let (total_cache, package_cache) = tokio::task::spawn_blocking(|| {
                            models::SystemCategory::load_system_cache()
                        })
                        .await
                        .unwrap_or((0, 0));
                        Message::SystemFieldUpdate(SystemFieldUpdate::SystemCache(
                            total_cache.saturating_sub(package_cache),
                        ))
                    })
                    .map(crate::app::Message::from)
                    .map(Into::into),
                );

                tasks.push(
                    cosmic::Task::future(async {
                        let size =
                            tokio::task::spawn_blocking(|| models::HomeCategory::load_documents())
                                .await
                                .unwrap_or(0);
                        Message::HomeFieldUpdate(HomeFieldUpdate::Documents(size))
                    })
                    .map(crate::app::Message::from)
                    .map(Into::into),
                );

                tasks.push(
                    cosmic::Task::future(async {
                        let size =
                            tokio::task::spawn_blocking(|| models::HomeCategory::load_downloads())
                                .await
                                .unwrap_or(0);
                        Message::HomeFieldUpdate(HomeFieldUpdate::Downloads(size))
                    })
                    .map(crate::app::Message::from)
                    .map(Into::into),
                );

                tasks.push(
                    cosmic::Task::future(async {
                        let size =
                            tokio::task::spawn_blocking(|| models::HomeCategory::load_pictures())
                                .await
                                .unwrap_or(0);
                        Message::HomeFieldUpdate(HomeFieldUpdate::Pictures(size))
                    })
                    .map(crate::app::Message::from)
                    .map(Into::into),
                );

                tasks.push(
                    cosmic::Task::future(async {
                        let size =
                            tokio::task::spawn_blocking(|| models::HomeCategory::load_videos())
                                .await
                                .unwrap_or(0);
                        Message::HomeFieldUpdate(HomeFieldUpdate::Videos(size))
                    })
                    .map(crate::app::Message::from)
                    .map(Into::into),
                );

                tasks.push(
                    cosmic::Task::future(async {
                        let size =
                            tokio::task::spawn_blocking(|| models::HomeCategory::load_music())
                                .await
                                .unwrap_or(0);
                        Message::HomeFieldUpdate(HomeFieldUpdate::Music(size))
                    })
                    .map(crate::app::Message::from)
                    .map(Into::into),
                );

                tasks.push(
                    cosmic::Task::future(async {
                        let size =
                            tokio::task::spawn_blocking(|| models::HomeCategory::load_desktop())
                                .await
                                .unwrap_or(0);
                        Message::HomeFieldUpdate(HomeFieldUpdate::Desktop(size))
                    })
                    .map(crate::app::Message::from)
                    .map(Into::into),
                );

                tasks.push(
                    cosmic::Task::future(async {
                        let (total_home, var_dir) = tokio::task::spawn_blocking(|| {
                            models::HomeCategory::load_total_and_var()
                        })
                        .await
                        .unwrap_or((0, 0));
                        Message::HomeTotalAndVar {
                            total_home,
                            var_dir,
                        }
                    })
                    .map(crate::app::Message::from)
                    .map(Into::into),
                );

                return cosmic::Task::batch(tasks);
            }
            Message::CategoryDetails { system, home } => {
                self.system_category = system.clone();
                self.home_category = home.clone();

                self.storage_info.system = system.total_size();
                self.storage_info.home = home.total_size();

                self.pending_tasks = self.pending_tasks.saturating_sub(1);

                let loading = self.pending_tasks > 0;

                if self.pending_tasks == 0 {
                    self.loading = false;
                    self.storage_info.other = self.storage_info.used.saturating_sub(
                        self.storage_info.system
                            + self.storage_info.home
                            + self.storage_info.applications,
                    );
                }

                // Sync data to sub-pages so they update if currently viewing
                return cosmic::Task::batch(vec![
                    cosmic::task::message(crate::app::Message::from(
                        crate::pages::Message::StorageSystemCategory(
                            system_category::Message::SetData {
                                data: system,
                                loading,
                            },
                        ),
                    )),
                    cosmic::task::message(crate::app::Message::from(
                        crate::pages::Message::StorageHomeCategory(
                            home_category::Message::SetData {
                                data: home,
                                loading,
                            },
                        ),
                    )),
                    cosmic::task::message(crate::app::Message::from(
                        crate::pages::Message::StorageOtherCategory(
                            other_category::Message::SetData {
                                size: self.storage_info.other,
                                loading,
                            },
                        ),
                    )),
                ]);
            }
            Message::FlatpakAppsWithSizes(apps_with_sizes) => {
                self.storage_info.flatpak_apps = apps_with_sizes.clone();

                let flatpak_total: u64 = self
                    .storage_info
                    .flatpak_apps
                    .iter()
                    .map(|app| app.total_size())
                    .sum();
                self.storage_info.applications = flatpak_total;

                self.pending_tasks = self.pending_tasks.saturating_sub(1);

                let loading = self.pending_tasks > 0;

                // Only recalculate "other" when all background tasks are complete
                // to avoid showing incorrect values based on partial data
                if self.pending_tasks == 0 {
                    self.loading = false;
                    self.storage_info.other = self.storage_info.used.saturating_sub(
                        self.storage_info.system
                            + self.storage_info.home
                            + self.storage_info.applications,
                    );
                }

                // Sync data to sub-pages so they update if currently viewing
                return cosmic::Task::batch(vec![
                    cosmic::task::message(crate::app::Message::from(
                        crate::pages::Message::StorageApplicationsCategory(
                            applications_category::Message::SetApps(apps_with_sizes),
                        ),
                    )),
                    cosmic::task::message(crate::app::Message::from(
                        crate::pages::Message::StorageOtherCategory(
                            other_category::Message::SetData {
                                size: self.storage_info.other,
                                loading,
                            },
                        ),
                    )),
                ]);
            }
            Message::SystemFieldUpdate(field_update) => {
                // Update individual system category field and notify sub-page
                match field_update.clone() {
                    SystemFieldUpdate::SystemFiles(size) => {
                        self.system_category.system_files = size
                    }
                    SystemFieldUpdate::PackageCache(size) => {
                        self.system_category.package_cache = size
                    }
                    SystemFieldUpdate::SystemLogs(size) => self.system_category.system_logs = size,
                    SystemFieldUpdate::SystemCache(size) => {
                        self.system_category.system_cache = size
                    }
                    SystemFieldUpdate::BootFiles(size) => self.system_category.boot_files = size,
                    SystemFieldUpdate::FlatpakRuntimes(size) => {
                        self.system_category.flatpak_runtimes = size
                    }
                }

                self.storage_info.system = self.system_category.total_size();

                self.pending_tasks = self.pending_tasks.saturating_sub(1);

                if self.pending_tasks == 0 {
                    self.loading = false;
                    self.storage_info.other = self.storage_info.used.saturating_sub(
                        self.storage_info.system
                            + self.storage_info.home
                            + self.storage_info.applications,
                    );
                }

                // Forward update to system sub-page
                return cosmic::task::message(crate::app::Message::from(
                    crate::pages::Message::StorageSystemCategory(
                        system_category::Message::FieldUpdate(field_update),
                    ),
                ));
            }
            Message::HomeFieldUpdate(field_update) => {
                let is_dir_field = match field_update.clone() {
                    HomeFieldUpdate::Documents(size) => {
                        self.home_category.documents = size;
                        true
                    }
                    HomeFieldUpdate::Downloads(size) => {
                        self.home_category.downloads = size;
                        true
                    }
                    HomeFieldUpdate::Pictures(size) => {
                        self.home_category.pictures = size;
                        true
                    }
                    HomeFieldUpdate::Videos(size) => {
                        self.home_category.videos = size;
                        true
                    }
                    HomeFieldUpdate::Music(size) => {
                        self.home_category.music = size;
                        true
                    }
                    HomeFieldUpdate::Desktop(size) => {
                        self.home_category.desktop = size;
                        true
                    }
                    HomeFieldUpdate::Other(size) => {
                        self.home_category.other = size;
                        false
                    }
                };

                if is_dir_field {
                    self.home_dirs_loaded_count += 1;

                    if self.home_dirs_loaded_count == 6 {
                        if let Some((total_home, var_dir)) = self.home_total_and_var {
                            let dirs_sum = self.home_category.documents
                                + self.home_category.downloads
                                + self.home_category.pictures
                                + self.home_category.videos
                                + self.home_category.music
                                + self.home_category.desktop;

                            let other = total_home.saturating_sub(dirs_sum + var_dir);
                            self.home_category.other = other;

                            let other_update_task =
                                cosmic::task::message(crate::app::Message::from(
                                    crate::pages::Message::StorageHomeCategory(
                                        home_category::Message::FieldUpdate(
                                            HomeFieldUpdate::Other(other),
                                        ),
                                    ),
                                ));

                            self.storage_info.home = self.home_category.total_size();

                            self.pending_tasks = self.pending_tasks.saturating_sub(1);

                            if self.pending_tasks == 0 {
                                self.loading = false;
                                self.storage_info.other = self.storage_info.used.saturating_sub(
                                    self.storage_info.system
                                        + self.storage_info.home
                                        + self.storage_info.applications,
                                );
                            }

                            return cosmic::Task::batch(vec![
                                cosmic::task::message(crate::app::Message::from(
                                    crate::pages::Message::StorageHomeCategory(
                                        home_category::Message::FieldUpdate(field_update),
                                    ),
                                )),
                                other_update_task,
                            ]);
                        }
                    }

                    self.pending_tasks = self.pending_tasks.saturating_sub(1);
                }

                self.storage_info.home = self.home_category.total_size();

                if self.pending_tasks == 0 {
                    self.loading = false;
                    self.storage_info.other = self.storage_info.used.saturating_sub(
                        self.storage_info.system
                            + self.storage_info.home
                            + self.storage_info.applications,
                    );
                }
                return cosmic::task::message(crate::app::Message::from(
                    crate::pages::Message::StorageHomeCategory(
                        home_category::Message::FieldUpdate(field_update),
                    ),
                ));
            }
            Message::HomeTotalAndVar {
                total_home,
                var_dir,
            } => {
                self.home_total_and_var = Some((total_home, var_dir));

                if self.home_dirs_loaded_count == 6 {
                    let dirs_sum = self.home_category.documents
                        + self.home_category.downloads
                        + self.home_category.pictures
                        + self.home_category.videos
                        + self.home_category.music
                        + self.home_category.desktop;

                    let other = total_home.saturating_sub(dirs_sum + var_dir);
                    self.home_category.other = other;

                    self.storage_info.home = self.home_category.total_size();

                    return cosmic::task::message(crate::app::Message::from(
                        crate::pages::Message::StorageHomeCategory(
                            home_category::Message::FieldUpdate(HomeFieldUpdate::Other(other)),
                        ),
                    ));
                }
            }
            Message::AnimationTick => {
                // Cycle through animation states: 0 -> 1 -> 2 -> 0
                self.animation_state = (self.animation_state + 1) % 3;
            }
            Message::SelectCategory(category) => {
                if let Some(cat) = category {
                    return match cat {
                        CategoryType::System => {
                            let loading = self.pending_tasks > 0;
                            let set_data_task = cosmic::task::message(crate::app::Message::from(
                                crate::pages::Message::StorageSystemCategory(
                                    system_category::Message::SetData {
                                        data: self.system_category.clone(),
                                        loading,
                                    },
                                ),
                            ));
                            let navigate_task = cosmic::task::message(crate::app::Message::Page(
                                self.sub_pages.system,
                            ));
                            cosmic::Task::batch(vec![set_data_task, navigate_task])
                        }
                        CategoryType::Home => {
                            let loading = self.pending_tasks > 0;
                            let set_data_task = cosmic::task::message(crate::app::Message::from(
                                crate::pages::Message::StorageHomeCategory(
                                    home_category::Message::SetData {
                                        data: self.home_category.clone(),
                                        loading,
                                    },
                                ),
                            ));
                            let navigate_task = cosmic::task::message(crate::app::Message::Page(
                                self.sub_pages.home,
                            ));
                            cosmic::Task::batch(vec![set_data_task, navigate_task])
                        }
                        CategoryType::Applications => {
                            let set_data_task = cosmic::task::message(crate::app::Message::from(
                                crate::pages::Message::StorageApplicationsCategory(
                                    applications_category::Message::SetApps(
                                        self.storage_info.flatpak_apps.clone(),
                                    ),
                                ),
                            ));
                            let navigate_task = cosmic::task::message(crate::app::Message::Page(
                                self.sub_pages.applications,
                            ));
                            cosmic::Task::batch(vec![set_data_task, navigate_task])
                        }
                        CategoryType::Other => {
                            let loading = self.pending_tasks > 0;
                            let set_data_task = cosmic::task::message(crate::app::Message::from(
                                crate::pages::Message::StorageOtherCategory(
                                    other_category::Message::SetData {
                                        size: self.storage_info.other,
                                        loading,
                                    },
                                ),
                            ));
                            let navigate_task = cosmic::task::message(crate::app::Message::Page(
                                self.sub_pages.other,
                            ));
                            cosmic::Task::batch(vec![set_data_task, navigate_task])
                        }
                    };
                }
            }
        }

        Task::none()
    }
}

fn storage_overview() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let overview_title = descriptions.insert(fl!("storage-overview"));

    Section::default()
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let desc = &section.descriptions;
            let info = &page.storage_info;
            let animation_state = page.animation_state;

            let content = if page.loading {
                // Show UI structure with spinner while loading
                column()
                    .push(text::heading(&*desc[overview_title]))
                    .push(cosmic::widget::vertical_space().height(Length::Fixed(8.0)))
                    .push(
                        row()
                            .spacing(8)
                            .push(loading_spinner(animation_state))
                            .push(text::body("Loading...")),
                    )
                    .push(cosmic::widget::vertical_space().height(Length::Fixed(8.0)))
                    .push(
                        progress_bar(0.0..=100.0, 0.0)
                            .width(Length::Fill)
                            .height(Length::Fixed(24.0)),
                    )
                    .push(cosmic::widget::vertical_space().height(Length::Fixed(4.0)))
                    .push(
                        row()
                            .spacing(8)
                            .push(loading_spinner(animation_state))
                            .push(text::caption("Calculating...")),
                    )
                    .padding(16)
            } else {
                // Show actual data with color-coded segmented bar
                let storage_bar = segmented_storage_bar(info);

                column()
                    .push(text::heading(&*desc[overview_title]))
                    .push(cosmic::widget::vertical_space().height(Length::Fixed(8.0)))
                    .push(text::body(format!(
                        "{} of {} used",
                        format_bytes(info.used),
                        format_bytes(info.total)
                    )))
                    .push(cosmic::widget::vertical_space().height(Length::Fixed(8.0)))
                    .push(storage_bar)
                    .push(cosmic::widget::vertical_space().height(Length::Fixed(4.0)))
                    .push(text::caption(format!(
                        "{} available",
                        format_bytes(info.available)
                    )))
                    .padding(16)
            };

            container(content)
                .apply(cosmic::Element::from)
                .map(crate::pages::Message::Storage)
        })
}

fn storage_categories() -> Section<crate::pages::Message> {
    let mut descriptions = Slab::new();

    let system_label = descriptions.insert(fl!("storage-category-system"));
    let home_label = descriptions.insert(fl!("storage-category-home"));
    let apps_label = descriptions.insert(fl!("storage-category-apps"));
    let other_label = descriptions.insert(fl!("storage-category-other"));

    Section::default()
        .title(fl!("storage-categories"))
        .descriptions(descriptions)
        .view::<Page>(move |_binder, page, section| {
            let desc = &section.descriptions;
            let info = &page.storage_info;
            let loading = page.loading;
            let animation_state = page.animation_state;

            settings::section()
                .title(&section.title)
                .add(category_button(
                    &desc[system_label],
                    CategoryType::System,
                    info.system,
                    loading,
                    animation_state,
                ))
                .add(category_button(
                    &desc[home_label],
                    CategoryType::Home,
                    info.home,
                    loading,
                    animation_state,
                ))
                .add(category_button(
                    &desc[apps_label],
                    CategoryType::Applications,
                    info.applications,
                    loading,
                    animation_state,
                ))
                .add(category_button(
                    &desc[other_label],
                    CategoryType::Other,
                    info.other,
                    loading,
                    animation_state,
                ))
                .apply(cosmic::Element::from)
                .map(crate::pages::Message::Storage)
        })
}
