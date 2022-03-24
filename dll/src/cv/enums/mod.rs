#[macro_use]
mod yes_no_enum;

mod game_mode;
mod game_options_types;
mod game_type;
mod multiplayer_option_types;
mod slot_status;

pub use game_mode::GameMode;
pub use game_options_types::GameOptionTypes;
pub use game_type::GameType;
pub use multiplayer_option_types::MultiplayerOptionTypes;
pub use slot_status::SlotStatus;

yes_no_enum!(EraTypes, GameSpeed, PlayerTypes, TeamTypes);
