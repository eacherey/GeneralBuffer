mod api;
mod cli;
mod clipboard;
mod config;
mod error;

use clap::Parser;
use global_hotkey::{hotkey::{Code, HotKey, Modifiers}, GlobalHotKeyManager, GlobalHotKeyEvent, HotKeyState};

use crate::{
    cli::{Cli, Command},
    clipboard::ClipBoardManager,
    config::Config,
    error::ClientError,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load()?;
    let mut clipboard = ClipBoardManager::new()?;

    let cli = Cli::parse();
    match &cli.command {
        Command::Push => {
            let text = clipboard
                .get_clipboard()
                .ok_or(ClientError::EmptyClipboard)?;

            api::push_clip(&config, text.clone())?;
            println!("Push {}", text);
        }
        Command::Pull => {
            let clip = api::pull_latest_clip(&config)?;

            clipboard.set_clipboard(&clip.content)?;
            println!("[latest] {}", clip.content);

        }
        Command::Get { id } => {
            let clip = api::get_clip_by_id(&config, *id)?;

            clipboard.set_clipboard(&clip.content)?;
            println!("[{}] {}", clip.id, clip.content);
        }
        Command::Delete { id } => {
            let clip = api::delete_clip_by_id(&config, *id)?;

            println!("Deleted {}", clip.content);
        }
        Command::List => {
            let clips = api::get_list_clips(&config)?;

            for clip in &clips {
                println!("[{}] {}", clip.id, clip.content);
            }
        }
        Command::Hotkey => {
            run_daemon(config)?;
        }
    }

    Ok(())
}

fn run_daemon(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut clipboard = ClipBoardManager::new()?;

    let hotkey_push = HotKey {
        mods: Modifiers::CONTROL | Modifiers::ALT,
        key: Code::KeyC,
        id: 0,
    };
    let hotkey_pull =  HotKey {
        mods: Modifiers::CONTROL | Modifiers::ALT,
        key: Code::KeyV,
        id: 1,
    };

    std::thread::spawn(move || {
        let hotkey = GlobalHotKeyManager::new().expect("Failed to create global hotkey manager");
        hotkey.register(hotkey_push).expect("Failed to register hotkey");
        hotkey.register(hotkey_pull).expect("Failed to register hotkey");

        #[cfg(target_os = "windows")]
        unsafe {
            use windows_sys::Win32::UI::WindowsAndMessaging::{
                DispatchMessageW, GetMessageW, TranslateMessage, MSG,
            };
            let mut msg: MSG = std::mem::zeroed();
            while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    });

    println!("Daemon started. Press Ctrl+Alt+C to Push, Ctrl+Alt+V to Pull. Ctrl+C to exit.");

    let receiver = GlobalHotKeyEvent::receiver();

    while let Ok(event) = receiver.recv() {
        if event.state != HotKeyState::Pressed {
            continue;
        }

        if event.id == hotkey_push.id {
            match clipboard.get_clipboard() {
                Some(text) => match api::push_clip(&config, text.clone()) {
                    Ok(_) => println!("[HOTKEY] Pushed: {}", text),
                    Err(e) => eprintln!("[HOTKEY] Push failed: {}", e),
                },
                None => eprintln!("[HOTKEY] Clipboard is empty"),
            }
        } else if event.id == hotkey_pull.id {
            match api::pull_latest_clip(&config) {
                Ok(clip) => {
                    let _ = clipboard.set_clipboard(&clip.content);
                    println!("[HOTKEY] Pulled: {}", clip.content);
                }
                Err(e) => eprintln!("[HOTKEY] Pull failed: {}", e),
            }
        }
    }

    Ok(())
}