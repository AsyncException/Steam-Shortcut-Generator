mod data;
mod string_functions;

use data::Game;
use core::panic;
use winreg::RegKey;
use std::{fs::OpenOptions, path::Path, io::{Write, self, ErrorKind}};


const ROOT_SHORTCUT_PATH: &str = "C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs\\Games\\";

fn main() {
    let games = get_steam_games();
    write_game_shortcuts(games);
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

fn write_game_shortcuts(games: Vec<Game>) {
    for game in games {
        let shortcut_path = format!("{}{}.url", ROOT_SHORTCUT_PATH, game.display_name);

        if !Path::new(&shortcut_path).exists() {
            write_shortcut(game, shortcut_path);
        }
    }
}

fn write_shortcut(game: Game, shortcut_path: String) {
    let file_result = OpenOptions::new()
    .create_new(true)
    .write(true)
    .open(shortcut_path);

    if let Err(ref e) = file_result {
        let err_kind = e.kind();

        if err_kind == io::ErrorKind::PermissionDenied {
            println!("Permission denied error while creating shortuct for {}", game.display_name);
            return;
        } else {
            println!("Error while making shortcut for {}\n{}", game.display_name, err_kind);
            return;
        }
    };

    let mut file = file_result.unwrap();
        
    match writeln!(file, "[InternetShortcut]") {
        Ok(_v) => (),
        Err(e) => println!("Unable to write type to shortcut {}\n{}", game.display_name, e.kind()),
    };

    match writeln!(file, "URL=steam://rungameid/{}", game.id) {
        Ok(_v) => (),
        Err(e) => println!("Unable to write URL to shortcut {}\n{}", game.display_name, e.kind()),
    };

    // Optionally, specify the icon path
    if !game.display_icon.is_empty() && Path::new(&game.display_icon).exists() {
        match writeln!(file, "IconFile={}", game.display_icon) {
            Ok(_v) => (),
            Err(e) => println!("Unable to write icon file to shortcut {}\n{}", game.display_name, e.kind()),
        };
        
        match writeln!(file, "IconIndex=0") {
            Ok(_v) => (),
            Err(e) => println!("Unable to write icon index to shortcut {}\n{}", game.display_name, e.kind()),
        };
    }
}