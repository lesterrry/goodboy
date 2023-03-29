/***************************
COPYRIGHT AYDAR.MEDIA (me@aydar.media),
2023

***************************/

use crate::strings::{commands::*, env::*, markers::*, messages::*, system::*};
use clap::{App, Arg, SubCommand};
use directories::ProjectDirs;
use rpassword::read_password;
use rustyline::Editor;
use serde_derive::{Deserialize, Serialize};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::panic::set_hook;

mod strings;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
	admin_id: String,
	bots: HashMap<String, String>,
}

#[allow(dead_code)]
enum Format {
	Error,
	Success,
	Warning,
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
fn report(with_msg: &str, format: Format) {
	let marker: String;
	match format {
		Format::Error => marker = format!("{}{}{}{}", ERR, BLD, ERROR_PREDICATE, RES),
		Format::Success => marker = format!("{}{}{}{}", SCS, BLD, SUCCESS_PREDICATE, RES),
		Format::Warning => marker = format!("{}{}{}{}", WRN, BLD, WARNING_PREDICATE, RES),
	}
	println!("{}{}", marker, with_msg);
}
#[cfg(target_os = "windows")]
fn report(with_msg: &str, format: Format) {
	let marker: String;
	match format {
		Format::Error => marker = format!("{}", ERR),
		Format::Success => marker = format!("{}", SCS),
		Format::Warning => marker = format!("{}", WRN),
	}
	println!("{}{}", marker, with_msg);
}
fn save_config(config: &Config, file: &PathBuf) {
	let ser = toml::to_vec(&config).unwrap();
	fs::write(file, ser).unwrap();
}

fn main() {
	set_hook(Box::new(|info| {
        if let Some(s) = info.payload().downcast_ref::<String>() {
            report(s, Format::Error)
        }
    }));
	let matches = App::new(PKG_NAME)
		.version(PKG_VERSION)
		.author(PKG_AUTHORS)
		.subcommand(SubCommand::with_name(RESET_CMD).about(RESET_CMD_DESCRIPTION))
		.subcommand(SubCommand::with_name(REVEAL_CMD).about(REVEAL_CMD_DESCRIPTION))
		.subcommand(SubCommand::with_name(EDIT_CMD).about(EDIT_CMD_DESCRIPTION))
		.subcommand(
			SubCommand::with_name(ADDBOT_CMD)
				.about(ADDBOT_CMD_DESCRIPTION)
				.arg(
					Arg::with_name(BOTNAME_ARG)
						.help(BOTNAME_ARG_DESCRIPTION)
						.required(true)
						.index(1),
				),
		)
		.subcommand(
			SubCommand::with_name(RMBOT_CMD)
				.about(RMBOT_CMD_DESCRIPTION)
				.arg(
					Arg::with_name(BOTNAME_ARG)
						.help(BOTNAME_ARG_DESCRIPTION)
						.required(true)
						.index(1),
				),
		)
		.subcommand(
			SubCommand::with_name(RUN_CMD)
				.about(RUN_CMD_DESCRIPTION)
				.arg(
					Arg::with_name(BOTNAME_ARG)
						.help(BOTNAME_ARG_DESCRIPTION)
						.required(true)
						.index(1),
				)
				.arg(
					Arg::with_name(DATABASE_ARG)
						.short(DATABASE_ARG_SHORT)
						.long(DATABASE_ARG_LONG)
						.help(DATABASE_ARG_DESCRIPTION)
						.takes_value(true),
				)
				.arg(
					Arg::with_name(TABLE_ARG)
						.short(TABLE_ARG_SHORT)
						.long(TABLE_ARG_LONG)
						.help(TABLE_ARG_DESCRIPTION)
						.takes_value(true),
				)
				.arg(
					Arg::with_name(COLUMN_ARG)
						.short(COLUMN_ARG_SHORT)
						.long(COLUMN_ARG_LONG)
						.help(COLUMN_ARG_DESCRIPTION)
						.takes_value(true),
				),
		)
		.get_matches();

	let mut config;
	let mut config_file;
	let mut readline_editor = Editor::<()>::new();

	println!("Welcome to {} v{}\n{}", PKG_NAME, PKG_VERSION, DIV);

	if let Some(proj_dirs) = ProjectDirs::from("com", VENDOR_NAME, PKG_NAME) {
		let config_dir: PathBuf = proj_dirs.config_dir().to_owned();
		config_file = config_dir.clone();
		config_file.push(CONFIG_FILE_NAME);
		if config_file.exists() {
			if matches.subcommand_matches(RESET_CMD).is_some() {
				println!("{}", CONFIGFILE_REMOVING);
				fs::remove_file(config_file).expect(DELETION_FAIL);
				report(RESET_SUCCESS, Format::Success);
				std::process::exit(0)
			}
			let content = fs::read(&config_file).unwrap();
			config = toml::from_slice(&content).unwrap();
		} else {
			println!("{}", CONFIGFILE_CREATING);
			fs::create_dir_all(config_dir).expect(CONFIGDIR_CREATING_FAIL);
			let admin_id = readline_editor.readline(ADMINID_PROMPT).expect(INPUT_FAIL);
			config = Config {
				admin_id: admin_id,
				bots: HashMap::new(),
			};
			save_config(&config, &config_file);
		}
	} else {
		panic!("{}", CONFIGDIR_ACCESS_FAIL);
	}

	if matches.subcommand_matches(REVEAL_CMD).is_some() {
		println!("{}", CONFIG_REVEALING);
		println!("{}{}", ADMINID_PREDICATE, config.admin_id);
		println!("{}", BOTS_PREDICATE);
		for (k, _) in config.bots.iter() {
			println!("{}{}", IND, k)
		}
	} else if matches.subcommand_matches(EDIT_CMD).is_some() {
		println!("{}", CONFIGFILE_EDITING);
		Command::new(MANUAL_EDITOR)
			.arg(&config_file.into_os_string())
			.status()
			.expect(CONFIGFILE_EDIT_FAIL);
	} else if let Some(matches) = matches.subcommand_matches(ADDBOT_CMD) {
		println!("{}", BOT_ADDING);
		let name = matches.value_of(BOTNAME_ARG).unwrap();
		if let Entry::Occupied(_) = config.bots.entry(name.to_string()) {
			panic!("{}", ADDBOT_FAIL)
		}
		println!("{}", TOKEN_PROMPT);
		let token = read_password().expect(INPUT_FAIL);
		config.bots.insert(name.to_string(), token);
		save_config(&config, &config_file);
		report(ADDBOT_SUCCESS, Format::Success)
	} else if let Some(matches) = matches.subcommand_matches(RMBOT_CMD) {
		println!("{}", BOT_REMOVING);
		let name = matches.value_of(BOTNAME_ARG).unwrap();
		if let Entry::Vacant(_) = config.bots.entry(name.to_string()) {
			panic!("{}", RMBOT_FAIL)
		}
		config.bots.remove(name);
		save_config(&config, &config_file);
		report(RMBOT_SUCCESS, Format::Success)
	} else if let Some(matches) = matches.subcommand_matches(RUN_CMD) {
		println!("{}", "Preparing to send messages...");
		let name = matches.value_of(BOTNAME_ARG).unwrap();
		let token: &str;
		match config.bots.entry(name.to_string()) {
			Entry::Vacant(_) => panic!("{}", RMBOT_FAIL),
			Entry::Occupied(e) => token = e.get(),
		}
		let db = if let Some(a) = matches.value_of(DATABASE_ARG) { a.to_string() } else { readline_editor.readline(DATABASE_PROMPT).expect(INPUT_FAIL).trim().to_string() };
		let table = if let Some(a) = matches.value_of(TABLE_ARG) { a.to_string() } else { readline_editor.readline(TABLE_PROMPT).expect(INPUT_FAIL).trim().to_string() };
		let column = if let Some(a) = matches.value_of(COLUMN_ARG) { a.to_string() } else { DEFAULT_COLUMN_TITLE.to_string() };
		let connection = sqlite::Connection::open_with_flags(db, sqlite::OpenFlags::new().set_read_only()).expect("Couldn't access database file");
		let query = format!("SELECT {} FROM {}", column, table);
		connection.iterate(query, |pairs| {
			for &(key, value) in pairs.iter() {
				println!("{} = {}", key, value.unwrap());
			}
			true
		})
		.expect("Database error");
	}
}
