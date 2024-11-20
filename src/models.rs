#[derive(Debug, Clone)]
pub enum BrowserPage {
    YouTube,
    Twitch,
    Gmail,
    None
}

#[derive(Debug, Clone)]
pub enum Game {
    Valorant,
    Fortnite,
    Roblox,
    Minecraft,
    Majestic,
    None,
}

#[derive(Debug, Clone)]
pub enum GameMode {
    Competitive,
    Deathmatch,
    Normal,
}

#[derive(Debug, Clone)]
pub struct ContentType {
    pub game: Game,
    pub mode: GameMode,
}

impl ContentType {
    pub fn new(game: Game, mode: GameMode) -> Self {
        ContentType { game, mode }
    }
}

impl Default for ContentType {
    fn default() -> Self {
        ContentType {
            game: Game::None,
            mode: GameMode::Normal,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Resolution {
    Normal,
    Stretched,
}

#[derive(Debug, Clone)]
pub enum Symbols {
    HardSign,
    Eh,
    BigEh,
    Heart,
    LeftArrow,
    RightArrow,
    Tire,
    None
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeyStatus {
    Press,
    Release
}

#[derive(Debug, Clone)]
pub enum Program {
    Calculator,
    Terminal,
    Notepad,
    Telegram,
    None
}

#[derive(Debug, Clone)]
pub enum Power {
    Shutdown,
    Reboot,
    None
}

#[derive(Debug, Clone)]
pub enum Action {
    Content(ContentType),
    Close,
    Display(Resolution),
    Key((KeyStatus, u32)),
    Symbol(Symbols),
    Browser(BrowserPage),
    Program(Program),
    Power(Power),
    ClearBuffer
}