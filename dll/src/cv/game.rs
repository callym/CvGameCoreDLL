use super::{
  enums::{GameMode, GameOptionTypes, MultiplayerOptionTypes, SlotStatus},
  globals::Globals,
  init_core::InitCore,
  random::{CvRandom, Random},
};
use alloc::vec::Vec;
use core::ptr::NonNull;
use num_enum::TryFromPrimitive;

/// cbindgen:ignore
extern "thiscall" {
  pub type CvGame;

  #[link_name = "?getSorenRand@CvGame@@QAEAAVCvRandom@@XZ"]
  fn CvGame_getSorenRand(cvGame: NonNull<CvGame>) -> NonNull<CvRandom>;
}

pub struct Game {
  cpp: NonNull<CvGame>,
}

impl Game {
  pub fn new(cpp: NonNull<CvGame>) -> Self {
    Self { cpp }
  }

  pub fn get_soren_rand(&self) -> Random {
    unsafe { Random::new(CvGame_getSorenRand(self.cpp)) }
  }

  pub fn set_mp_options(&self, init: &InitCore) {
    // Turn off all MP options if it's a single player game
    if init.get_type().is_single_player() {
      for i in 0..(MultiplayerOptionTypes::NumTypes as i32) {
        let option = MultiplayerOptionTypes::try_from_primitive(i).unwrap();

        init.set_mp_option(option, false);
      }
    }

    // If this is a hot seat game, simultaneous turns is always off
    if init.get_type().is_hot_seat() || init.get_type().is_pbem() {
      init.set_mp_option(MultiplayerOptionTypes::SimultaneousTurns, false);
    }

    // If we didn't set a time in the Pitboss, turn timer off
    if init.get_mode() == GameMode::Pitboss && init.get_pitboss_turn_time() == 0 {
      init.set_mp_option(MultiplayerOptionTypes::TurnTimer, false);
    }
  }

  pub fn shuffle_teams(&self, init: &InitCore) {
    if init.get_mp_option(MultiplayerOptionTypes::ShuffleTeams) == false {
      return;
    }

    let mut players: Vec<_> = (0..Globals::new().max_players())
      .into_iter()
      .filter(|i| init.get_slot_status((*i).into()) == SlotStatus::Taken)
      .map(|i| init.get_team(i.into()))
      .collect();

    for i in 0..players.len() {
      let j = self.get_soren_rand().get(players.len() as u16 - i as u16) as usize + i;

      if i != j {
        let temp = players[i];
        players[i] = players[j];
        players[j] = temp;
      }
    }

    for i in 0..players.len() as i32 {
      init.set_team(i.into(), players[i as usize]);
    }
  }

  pub fn lock_mods(&self, init: &InitCore) {
    if init.get_option(GameOptionTypes::LockMods) == false {
      return;
    }

    if init.get_type().is_multiplayer() {
      init.set_option(GameOptionTypes::LockMods, false);
      return;
    }

    const PASSWORD_SIZE: usize = 8;

    let mut password = (0..PASSWORD_SIZE)
      .map(|_| self.get_soren_rand().get(128))
      .collect::<Vec<_>>();

    password[PASSWORD_SIZE - 1] = 0;

    init.set_admin_password(password);
  }
}

/// This is a Rust port of the logic from `void CvGame::init(HandicapTypes eHandicap)`
#[no_mangle]
pub extern "C" fn rust__game__init(game: NonNull<CvGame>) {
  let game = Game::new(game);

  let globals = Globals::new();
  let init_core = globals.init_core();

  game.set_mp_options(&init_core);
  game.shuffle_teams(&init_core);
  game.lock_mods(&init_core);
}
