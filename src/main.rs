use std::sync::Mutex;

use handlers::handle_event;
use models::{Action, ContentType};
use obws::Client;
use simplelog::{CombinedLogger, Config, LevelFilter, TermLogger};
use tokio::{
    signal,
    sync::mpsc::{self, UnboundedSender},
};

use log::error;
use windows::keyboard_listener::start_keyboard_hook;

mod handlers;
mod models;
mod windows;

lazy_static::lazy_static! {
    pub static ref SENDER: Mutex<UnboundedSender<Action>> = Mutex::new(mpsc::unbounded_channel::<Action>().0);
}

lazy_static::lazy_static! {
    pub static ref KEYLOCK: Mutex<bool> = Mutex::new(false);
}

lazy_static::lazy_static! {
    pub static ref KEYBUFFER: Mutex<Vec<u32>> = Mutex::new(Vec::new());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            simplelog::TerminalMode::Mixed,
            simplelog::ColorChoice::Auto,
        ),
        TermLogger::new(
            LevelFilter::Error,
            Config::default(),
            simplelog::TerminalMode::Mixed,
            simplelog::ColorChoice::Auto,
        ),
    ])
    .unwrap();

    let (tx, mut rx) = mpsc::unbounded_channel::<Action>();

    *SENDER.lock().unwrap() = tx.clone();

    let client = match Client::connect("192.168.0.108", 4455, Some("ABOCDPCyssl3CKFN")).await {
        Ok(c) => Some(c),
        Err(e) => {
            error!("OBS might not be connected: {:?}", e);
            None
        }
    };

    let _ = tokio::spawn(async move {
        signal::ctrl_c().await.expect("failed to listen for event");
        let _ = SENDER
            .lock()
            .unwrap()
            .send(Action::Content(ContentType::default()));
        SENDER.lock().unwrap().send(Action::Close).unwrap();
    });

    start_keyboard_hook();

    loop {
        match rx.recv().await {
            Some(e) => {
                if let Err(err) = handle_event(e, client.as_ref()).await {
                    match err {
                        handlers::Error::OBSNotConnected => {
                            error!("OBS not connected. Skipping this event.");
                        }
                        handlers::Error::OBSError(e) => {
                            error!("OBS error: {}", e);
                        }
                        handlers::Error::ResoluitionError => {
                            error!("Failed to change display settings.");
                        }
                        handlers::Error::Close => {
                            break;
                        }
                    }
                }
            }
            None => {}
        }
    }

    Ok(())
}
