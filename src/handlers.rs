use std::process::Command;

use log::{error, warn};
use obws::Client;
use winapi::um::{
    wingdi::{DEVMODEW, DM_PELSHEIGHT, DM_PELSWIDTH},
    winuser::{ChangeDisplaySettingsW, CDS_UPDATEREGISTRY, DISP_CHANGE_SUCCESSFUL},
};

use crate::{
    models::{
        BrowserPage, ContentType, Game, GameMode, KeyStatus, Power, Program, Resolution, Symbols,
    },
    Action, KEYBUFFER, KEYLOCK, SENDER,
};

const NORMAL_DIRECTORY: &'static str = r#"D:\Records"#;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Error {
    OBSNotConnected,
    Close,
    ResoluitionError,
    OBSError(String),
}

pub async fn handle_event(e: Action, client: Option<&Client>) -> Result<(), Error> {
    match e {
        Action::Content(content_type) => change_recording_folder(content_type, client).await?,
        Action::Close => {
            warn!("closing");
            return Err(Error::Close);
        }
        Action::Display(resolution) => match resolution {
            crate::models::Resolution::Normal => {
                warn!("set to desktop res");
                set_resolution(1920, 1080)?
            }
            crate::models::Resolution::Stretched => {
                warn!("set to game res");
                set_resolution(1440, 1080)?
            }
        },
        Action::Symbol(key) => {
            type_symbol(key);
        }
        Action::Key(k) => handle_keyboard_event(k),
        Action::Browser(page) => match page {
            BrowserPage::YouTube => {
                warn!("opening YouTube");
                let _ = Command::new("powershell")
                    .arg("start")
                    .arg("https://www.youtube.com")
                    .spawn()
                    .unwrap();
            }
            BrowserPage::Twitch => {
                warn!("opening Twitch");
                let _ = Command::new("powershell")
                    .arg("start")
                    .arg("https://www.twitch.tv")
                    .spawn();
            }
            BrowserPage::Gmail => {
                warn!("opening Gmail");
                let _ = Command::new("powershell")
                    .arg("start")
                    .arg("https://mail.google.com/mail/u/1")
                    .spawn();
            }
            BrowserPage::None => {}
        },
        Action::Program(p) => match p {
            Program::Calculator => {
                warn!("opening calculator");
                Command::new("powershell")
                    .arg("start")
                    .arg("calc")
                    .spawn()
                    .unwrap();
            }
            Program::Terminal => {
                warn!("opening terminal");
                Command::new("powershell")
                    .arg("start")
                    .arg("powershell")
                    .spawn()
                    .unwrap();
            }
            Program::Notepad => {
                warn!("opening notepad");
                Command::new("powershell")
                    .arg("start")
                    .arg("notepad")
                    .spawn()
                    .unwrap();
            }
            Program::None => {}
        },
        Action::Power(p) => match p {
            Power::Shutdown => {
                warn!("shutting down");
                Command::new("shutdown")
                    .args(["-s", "-t", "180"])
                    .spawn()
                    .unwrap();
            }
            Power::Reboot => {
                warn!("shutting down");
                Command::new("shutdown")
                    .args(["-r", "-t", "30"])
                    .spawn()
                    .unwrap();
            }
            Power::None => {}
        },
        Action::ClearBuffer => {
            warn!("clearing buffer");
            KEYBUFFER.lock().unwrap().clear()
        }
    }

    Ok(())
}

fn handle_keyboard_event(e: (KeyStatus, u32)) {
    match e.1 {
        36 => {
            let mut klock = KEYLOCK.lock().unwrap();
            if e.0 == KeyStatus::Release {
                if klock.clone() == false {
                    warn!("keylock");
                    *klock = true;
                } else {
                    *klock = false;
                }
            }
        }
        27 => {
            if e.0 == KeyStatus::Release && *KEYLOCK.lock().unwrap() {
                *KEYLOCK.lock().unwrap() = false;
                let _ = SENDER.lock().unwrap().send(Action::ClearBuffer);
            }
        }
        13 => {
            if e.0 == KeyStatus::Release && *KEYLOCK.lock().unwrap() {
                *KEYLOCK.lock().unwrap() = false;
                handle_buffer();
                let _ = SENDER.lock().unwrap().send(Action::ClearBuffer);
            }
        }
        _ => {
            if *KEYLOCK.lock().unwrap() && e.0 == KeyStatus::Release && e.1 != 36 {
                KEYBUFFER.lock().unwrap().push(e.1);
            }
        }
    }
}

fn handle_buffer() {
    let b: Vec<u32> = KEYBUFFER.lock().unwrap().clone();

    warn!("got buffer: {:?}", b);

    match b[0] {
        82 => {
            let mut game: Game = Game::None;
            let mut game_mode: GameMode = GameMode::Normal;
            if b.len() > 1 {
                match b[1] {
                    86 => {
                        game = Game::Valorant;
                        match b[2] {
                            49 => {
                                game_mode = GameMode::Competitive;
                            }
                            50 => {
                                game_mode = GameMode::Deathmatch;
                            }
                            _ => {}
                        }
                    }
                    48 => {}
                    _ => {}
                }
            }
            SENDER
                .lock()
                .unwrap()
                .send(Action::Content(ContentType::new(game, game_mode)))
                .unwrap();
        }
        83 => {
            let mut symbol: Symbols = Symbols::None;
            if b.len() > 1 {
                match b[1] {
                    83 => {
                        symbol = Symbols::HardSign;
                    }
                    222 => {
                        symbol = Symbols::Eh;
                    }
                    191 => {
                        symbol = Symbols::BigEh;
                    }
                    72 => {
                        symbol = Symbols::Heart;
                    }
                    65 => {
                        symbol = Symbols::LeftArrow;
                    }
                    68 => {
                        symbol = Symbols::RightArrow;
                    }
                    _ => {}
                }
            }
            SENDER.lock().unwrap().send(Action::Symbol(symbol)).unwrap();
        }
        68 => {
            let mut res: Resolution = Resolution::Normal;
            if b.len() > 1 {
                match b[1] {
                    49 => {
                        res = Resolution::Normal;
                    }
                    50 => {
                        res = Resolution::Stretched;
                    }
                    _ => {}
                }
            }
            SENDER.lock().unwrap().send(Action::Display(res)).unwrap();
        }
        66 => {
            let mut page: BrowserPage = BrowserPage::None;
            if b.len() > 1 {
                match b[1] {
                    49 => {
                        page = BrowserPage::YouTube;
                    }
                    50 => {
                        page = BrowserPage::Twitch;
                    }
                    51 => {
                        page = BrowserPage::Gmail;
                    }
                    _ => {}
                }
            }
            SENDER.lock().unwrap().send(Action::Browser(page)).unwrap();
        }
        65 => {
            let mut prog: Program = Program::None;
            if b.len() > 1 {
                match b[1] {
                    67 => {
                        prog = Program::Calculator;
                    }
                    84 => {
                        prog = Program::Terminal;
                    }
                    78 => {
                        prog = Program::Notepad;
                    }
                    _ => {}
                }
            }
            SENDER.lock().unwrap().send(Action::Program(prog)).unwrap();
        }
        80 => {
            let mut power: Power = Power::None;
            if b.len() > 1 {
                match b[1] {
                    83 => {
                        power = Power::Shutdown;
                    }
                    82 => {
                        power = Power::Reboot;
                    }
                    _ => {}
                }
            }
            SENDER.lock().unwrap().send(Action::Power(power)).unwrap();
        }
        _ => {}
    }
}

fn set_resolution(x: u32, y: u32) -> Result<(), Error> {
    unsafe {
        let mut dev_action: DEVMODEW = std::mem::zeroed();
        dev_action.dmSize = std::mem::size_of::<DEVMODEW>() as u16;
        dev_action.dmFields = DM_PELSWIDTH | DM_PELSHEIGHT;
        dev_action.dmPelsWidth = x;
        dev_action.dmPelsHeight = y;

        let result = ChangeDisplaySettingsW(&mut dev_action, CDS_UPDATEREGISTRY);
        if result == DISP_CHANGE_SUCCESSFUL {
            warn!("Changed display settings");
            Ok(())
        } else {
            error!("Failed to change display settings");
            Err(Error::ResoluitionError)
        }
    }
}

pub async fn change_recording_folder(
    content: ContentType,
    client: Option<&Client>,
) -> Result<(), Error> {
    let mut path = String::new();

    match content.game {
        Game::None => {
            warn!("set to norm");
            path.push_str(NORMAL_DIRECTORY);
        }
        Game::Valorant => {
            warn!("set to comp");
            path = format!("{}\\Valorant", NORMAL_DIRECTORY);
        }
        _ => {}
    }

    match content.mode {
        GameMode::Competitive => {
            path.push_str("\\Competitive");
        }
        GameMode::Deathmatch => {
            path.push_str("\\Deathmatch");
        }
        _ => {}
    }

    return if let Some(c) = client {
        if let Err(e) = c.config().set_record_directory(&path).await {
            Err(Error::OBSError(e.to_string()))
        } else {
            Ok(())
        }
    } else {
        return Err(Error::OBSNotConnected);
    };
}

pub fn type_symbol(key: Symbols) {
    let symbol: u32 = match key {
        Symbols::HardSign => 'ы'.into(),
        Symbols::Eh => 'э'.into(),
        Symbols::BigEh => 'Э'.into(),
        Symbols::Heart => '♥'.into(),
        Symbols::LeftArrow => '←'.into(),
        Symbols::RightArrow => '→'.into(),
        Symbols::None => return,
    };

    let symbol: u16 = symbol as u16;

    crate::windows::type_symbol(symbol);
}
