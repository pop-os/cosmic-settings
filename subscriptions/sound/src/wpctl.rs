use numtoa::BaseN;

pub async fn set_default(id: u32) {
    let id = numtoa::BaseN::<10>::u32(id);
    _ = tokio::process::Command::new("wpctl")
        .args(["set-default", id.as_str()])
        .status()
        .await;
}

pub async fn set_profile(id: u32, index: u32) {
    set("set-profile", id, index).await
}

pub async fn set(command: &'static str, id: u32, index: u32) {
    let id = BaseN::<10>::u32(id);
    let index = BaseN::<10>::u32(index);

    _ = tokio::process::Command::new("wpctl")
        .args([command, id.as_str(), index.as_str()])
        .status()
        .await;
}

pub async fn set_mute(id: u32, mute: bool) {
    let default = numtoa::BaseN::<10>::u32(id);
    _ = tokio::process::Command::new("wpctl")
        .args(["set-mute", default.as_str(), if mute { "1" } else { "0" }])
        .status()
        .await;
}

pub async fn set_volume(id: u32, volume: u32) {
    let id = numtoa::BaseN::<10>::u32(id);
    let volume = format!("{}.{:02}", volume / 100, volume % 100);
    _ = tokio::process::Command::new("wpctl")
        .args(["set-volume", id.as_str(), volume.as_str()])
        .status()
        .await;
}
