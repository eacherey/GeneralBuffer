use crate::error::ClientResult;
use arboard::Clipboard;

pub struct ClipBoardManager {
    pub clipboard: Clipboard,
}

impl ClipBoardManager {
    pub fn new() -> ClientResult<Self> {
        let clipboard = Clipboard::new()?;

        Ok(Self { clipboard })
    }

    pub fn get_clipboard(&mut self) -> Option<String> {
        self.clipboard.get_text().ok()
    }

    pub fn set_clipboard(&mut self, text: &str) -> ClientResult<()> {
        self.clipboard.set_text(text)?;

        Ok(())
    }
}
