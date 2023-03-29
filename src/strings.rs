/***************************
COPYRIGHT AYDAR.MEDIA (me@aydar.media),
2023

***************************/

#[rustfmt::skip]
#[allow(dead_code)]
pub mod commands {
	pub const REVEAL_CMD: &str = "reveal";
	pub const REVEAL_CMD_DESCRIPTION: &str = "Show saved bots and admin id";
	pub const RESET_CMD: &str = "reset";
	pub const RESET_CMD_DESCRIPTION: &str = "Remove saved config file (removes saved bots and admin id)";
	pub const ADDBOT_CMD: &str = "addbot";
	pub const ADDBOT_CMD_DESCRIPTION: &str = "Save new bot token";
	pub const RMBOT_CMD: &str = "rmbot";
	pub const RMBOT_CMD_DESCRIPTION: &str = "Remove saved bot token";
	pub const RUN_CMD: &str = "run";
	pub const RUN_CMD_DESCRIPTION: &str = "Send messages";
	pub const EDIT_CMD: &str = "edit";
	pub const EDIT_CMD_DESCRIPTION: &str = "Manually edit config file";
	pub const BOTNAME_ARG: &str = "BOT_NAME";
	pub const BOTNAME_ARG_DESCRIPTION: &str = "Bot name to use";
	pub const DATABASE_ARG: &str = "DATABASE";
	pub const DATABASE_ARG_DESCRIPTION: &str = "Path to .db file";
	pub const DATABASE_ARG_SHORT: &str = "d";
	pub const DATABASE_ARG_LONG: &str = "database";
	pub const TABLE_ARG: &str = "TABLE";
	pub const TABLE_ARG_DESCRIPTION: &str = "Title of sqlite table to use";
	pub const TABLE_ARG_SHORT: &str = "t";
	pub const TABLE_ARG_LONG: &str = "table";
	pub const COLUMN_ARG: &str = "COLUMN";
	pub const COLUMN_ARG_DESCRIPTION: &str = "Title of column containing user IDs. Defaults to 'id'";
	pub const COLUMN_ARG_SHORT: &str = "c";
	pub const COLUMN_ARG_LONG: &str = "column";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod env {
	pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");
	pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
	pub const PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod system {
	pub const VENDOR_NAME: &str = "aydar-media";
	pub const CONFIG_FILE_NAME: &str = "config.toml";
	pub const MANUAL_EDITOR: &str = "nano";
	pub const DEFAULT_COLUMN_TITLE: &str = "id";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod messages {
	pub const DELETION_FAIL: &str = "Deletion failed";
	pub const RESET_SUCCESS: &str = "Reset";
	pub const ADDBOT_SUCCESS: &str = "Added new bot";
	pub const RMBOT_SUCCESS: &str = "Removed saved bot";
	pub const CONFIGFILE_CREATING: &str = "Creating new config file...";
	pub const CONFIGFILE_REMOVING: &str = "Removing config file...";
	pub const CONFIGFILE_EDITING: &str = "Opening config file for editing...";
	pub const CONFIG_REVEALING: &str = "Revealing config data:";
	pub const BOT_ADDING: &str = "Adding new bot...";
	pub const BOT_REMOVING: &str = "Removing saved bot...";
	pub const CONFIGDIR_CREATING_FAIL: &str = "Couldn't create directory";
	pub const CONFIGDIR_ACCESS_FAIL: &str = "Couldn't access configuration directory";
	pub const CONFIGFILE_EDIT_FAIL: &str = "Shell failure";
	pub const INPUT_FAIL: &str = "Input error";
	pub const ADDBOT_FAIL: &str = "Bot with this name already exists";
	pub const RMBOT_FAIL: &str = "Bot with this name does not exist";
	pub const ADMINID_PROMPT: &str = "Enter admin user_id for initial testing: ";
	pub const TOKEN_PROMPT: &str = "Enter bot token: ";
	pub const DATABASE_PROMPT: &str = "Enter path to database: ";
	pub const TABLE_PROMPT: &str = "Enter table title: ";
	pub const ERROR_PREDICATE: &str = "error: ";
	pub const SUCCESS_PREDICATE: &str = "success: ";
	pub const WARNING_PREDICATE: &str = "warning: ";
	pub const ADMINID_PREDICATE: &str = "Admin user_id: ";
	pub const BOTS_PREDICATE: &str = "Bots: ";
}

#[rustfmt::skip]
#[allow(dead_code)]
#[cfg(any(target_os = "macos", target_os = "linux"))]
pub mod markers {
	pub const ERR: &str = "\x1b[31m";
	pub const SCS: &str = "\x1b[32m";
	pub const WRN: &str = "\x1b[33m";
	pub const BLD: &str = "\x1b[1m";
	pub const RES: &str = "\x1b[0m";
	pub const DIV: &str = "=========================";
	pub const IND: &str = "  ";
}

#[rustfmt::skip]
#[allow(dead_code)]
#[cfg(target_os = "windows")]
pub mod markers {
	pub const ERR: &str = "ERROR: ";
	pub const SCS: &str = "SUCCESS: ";
	pub const WRN: &str = "WARN: ";
	pub const DIV: &str = "=========================";
	pub const IND: &str = "  ";
}
