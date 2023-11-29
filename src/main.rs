mod data;
mod string_functions;

use data::Game;
use core::panic;
use winreg::RegKey;
use std::{fs::OpenOptions, path::Path, io::Write};


const ROOT_SHORTCUT_PATH: &str = "C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs\\Games\\";

fn main() {
    let games = get_steam_games();
    write_game_shortcut(games);
}

fn get_steam_games() -> Vec<Game> {
    let hive = winreg::enums::HKEY_LOCAL_MACHINE;
    let key_path = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\";

    let main_key = match RegKey::predef(hive).open_subkey(key_path) {
        Ok(value) => value,
        Err(_e) => panic!("Unable to find: SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\"),
    };


    let mut game_list = Vec::<Game>::new();
    for i in main_key.enum_keys().map(|x| x.unwrap()).filter(|x| x.starts_with("Steam App")) {
        let key = RegKey::predef(hive).open_subkey(format!("{}\\{}", key_path, i)).unwrap();
        
        let id = string_functions::substring_at_last(i, ' ');

        let display_name: String = key.get_value("DisplayName").expect("Unable to get display name");
        let display_icon: String = key.get_value("DisplayIcon").expect("Unable to get display icon");
        game_list.push(Game::new(id, display_name, display_icon))
    }

    return game_list;
}

fn write_game_shortcut(games: Vec<Game>) {
    for game in games {
        let shortcut_path = format!("{}{}.url", ROOT_SHORTCUT_PATH, game.display_name);

        if !(Path::new(&shortcut_path).exists()) {
            let mut file = match OpenOptions::new()
                .create_new(true)
                .write(true)
                .append(true)
                .open(shortcut_path) {
                    Ok(value) => value,
                    Err(_e) => { println!("Unable to write file, this is most likely a permission issue"); return; },
                    
                };
            
            writeln!(file, "[InternetShortcut]").expect("Unable to write to file");
            writeln!(file, "URL=steam://rungameid/{}", game.id).expect("Unable to write to file");
    
            // Optionally, specify the icon path
            if !game.display_icon.is_empty() {
                writeln!(file, "IconFile={}", game.display_icon).expect("Unable to write to file");
                writeln!(file, "IconIndex=0").expect("Unable to write to file");
            }
        }
    }
}