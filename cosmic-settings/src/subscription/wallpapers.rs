use std::path::PathBuf;

use cosmic::iced::{
    Subscription,
    futures::{SinkExt, StreamExt, channel::mpsc::Sender, future},
    stream,
};
use image::{ImageBuffer, Rgba};

#[derive(Clone, Debug)]
/// Event emitted by the wallpaper subscription
pub enum WallpaperEvent {
    /// Started loading wallpapers
    Loading,
    /// Loaded wallpaper
    Load {
        path: PathBuf,
        display: ImageBuffer<Rgba<u8>, Vec<u8>>,
        selection: ImageBuffer<Rgba<u8>, Vec<u8>>,
    },
    /// Wallpaper source finished loading
    Loaded,
    /// An error ocurred loading wallpapers
    Error(String),
}

pub fn wallpapers(current_dir: PathBuf) -> cosmic::iced::Subscription<WallpaperEvent> {
    Subscription::run_with_id(
        current_dir.clone(),
        stream::channel(2, |tx| async {
            if let Err(err) = inner(tx, current_dir).await {
                tracing::error!("Wallpapers subscription error: {:?}", err);
            }
            future::pending().await
        }),
    )
}

async fn inner(tx: Sender<WallpaperEvent>, current_dir: PathBuf) -> anyhow::Result<()> {
    tx.clone().send(WallpaperEvent::Loading).await?;

    let mut stream = cosmic_settings_wallpaper::load_each_from_path(current_dir).await;

    while let Some((path, display, selection)) = stream.next().await {
        if let Err(e) = tx
            .clone()
            .send(WallpaperEvent::Load {
                path,
                display,
                selection,
            })
            .await
        {
            tracing::error!("Failed to send WallpaperEvent: {:?}", e);
        }
    }

    tx.clone().send(WallpaperEvent::Loaded).await?;

    Ok(())
}
