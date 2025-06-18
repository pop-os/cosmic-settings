// TODO: Do not use subscriptions for pages.
mod desktop_files;
pub use desktop_files::*;
#[cfg(feature = "ashpd")]
mod daytime;
#[cfg(feature = "ashpd")]
pub use daytime::*;
mod wallpapers;
pub use wallpapers::*;
