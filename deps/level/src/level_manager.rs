use std::ops::{Deref, DerefMut};

use refs::{MainLock, Own};

use crate::Level;

static SELF: MainLock<LevelManager> = MainLock::const_new();

#[derive(Default)]
pub struct LevelManager {
    level: Option<Own<dyn Level>>,
}

impl LevelManager {
    pub fn update() {
        if Self::no_level() {
            return;
        }
        Self::level_mut().base_mut().update_physics(1. / 60.);
        Self::level_mut().update();
    }
}

impl LevelManager {
    pub fn set_level(level: impl Level + 'static) {
        let mut level = Own::new(level);
        level.setup();
        SELF.get_mut().level = Some(level);
    }

    pub fn level() -> &'static dyn Level {
        SELF.level.as_ref().expect("No Level").deref()
    }

    pub fn level_mut() -> &'static mut dyn Level {
        SELF.get_mut().level.as_mut().expect("No Level").deref_mut()
    }

    pub fn no_level() -> bool {
        SELF.level.is_none()
    }
}