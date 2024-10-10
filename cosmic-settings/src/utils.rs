use std::{future::Future, io, process};

use futures::{future::select, StreamExt};

/// Normalize the labeling of displays across settings pages.
pub fn display_name(name: &str, physical: (u32, u32)) -> String {
    let inches = ((physical.0.pow(2) + physical.1.pow(2)) as f32).sqrt() * 0.039_370_1;
    let inches_string = format!("{inches:.1}\"");

    match name {
        "eDP-1" | "LVDS1" => {
            fl!("display", "laptop", size = inches_string.as_str())
        }
        output => fl!(
            "display",
            "external",
            size = inches_string.as_str(),
            output = output
        ),
    }
}

/// Spawn a background tasks and forward its messages
pub fn forward_event_loop<M: 'static + Send, T: Future<Output = ()> + Send + 'static>(
    sender: tokio::sync::mpsc::Sender<crate::pages::Message>,
    message_map: fn(M) -> crate::pages::Message,
    event_loop: impl FnOnce(futures::channel::mpsc::Sender<M>) -> T + Send + 'static,
) -> tokio::sync::oneshot::Sender<()> {
    let (cancel_tx, cancel_rx) = tokio::sync::oneshot::channel::<()>();

    tokio::task::spawn(async move {
        let (tx, mut rx) = futures::channel::mpsc::channel(1);

        let cancel = std::pin::pin!(cancel_rx);

        let forwarder = std::pin::pin!(async move {
            while let Some(event) = rx.next().await {
                if sender.send(message_map(event)).await.is_err() {
                    break;
                }
            }
        });

        select(cancel, select(forwarder, std::pin::pin!(event_loop(tx)))).await;
    });

    cancel_tx
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
