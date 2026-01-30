use std::{future::Future, io, process};

use futures::future::select;

/// Normalize the labeling of displays across settings pages.
pub fn display_name(name: &str, physical: (u32, u32)) -> String {
    let inches = ((physical.0.pow(2) + physical.1.pow(2)) as f32).sqrt() * 0.039_370_1;
    let inches_string = format!("{inches:.1}\"");

    if name.starts_with("eDP-") || name.starts_with("LVDS-") || name.starts_with("DSI-") {
        fl!("display", "laptop", size = inches_string.as_str())
    } else {
        fl!(
            "display",
            "external",
            size = inches_string.as_str(),
            output = name
        )
    }
}

/// Spawn a background tasks and forward its messages
pub fn forward_event_loop<M: 'static + Send, T: Future<Output = ()> + Send + 'static>(
    event_loop: impl FnOnce(futures::channel::mpsc::Sender<M>) -> T + Send + 'static,
) -> (tokio::sync::oneshot::Sender<()>, cosmic::Task<M>) {
    let (cancel_tx, cancel_rx) = tokio::sync::oneshot::channel::<()>();

    let task = cosmic::Task::stream(cosmic::iced_futures::stream::channel(
        1,
        |emitter| async move {
            select(
                std::pin::pin!(cancel_rx),
                std::pin::pin!(event_loop(emitter)),
            )
            .await;
        },
    ));

    (cancel_tx, task)
}

/// On process failure, return stderr as `String`.
pub fn map_stderr_output(result: io::Result<process::Output>) -> Result<(), String> {
    result.map_err(|why| why.to_string()).and_then(|output| {
        if !output.status.success() {
            Err(String::from_utf8(output.stderr).unwrap_or_default())
        } else {
            Ok(())
        }
    })
}

/// Creates a slab with predefined items
#[macro_export]
macro_rules! slab {
    ( $descriptions:ident { $( $txt_id:ident = $txt_expr:expr; )+ } ) => {
        let mut $descriptions = slab::Slab::new();

        $(
            let $txt_id = $descriptions.insert($txt_expr);
        )+
    }
}
