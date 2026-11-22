use std::time::{Duration, Instant};

use crate::sprites::{Sprite, SpriteSet};

const SLEEP_AFTER: Duration = Duration::from_secs(15);
const SLEEP_FRAME_MS: Duration = Duration::from_millis(600);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PetState {
    #[default]
    Idle,
    Typing,
    MouseLeft,
    MouseRight,
    Petted,
    Sleep,
}

pub struct StateManager {
    pub current: PetState,
    action_until: Option<Instant>,
    last_activity: Instant,
    type_toggle: usize,
    sleep_frame: usize,
    next_sleep_tick: Option<Instant>,
}

impl Default for StateManager {
    fn default() -> Self {
        Self {
            current: PetState::Idle,
            action_until: None,
            last_activity: Instant::now(),
            type_toggle: 0,
            sleep_frame: 0,
            next_sleep_tick: None,
        }
    }
}

impl StateManager {
    pub fn trigger(&mut self, state: PetState, duration_ms: u64) {
        let now = Instant::now();
        if state == PetState::Typing {
            self.type_toggle = 1 - self.type_toggle;
        }
        self.current = state;
        self.last_activity = now;
        self.action_until = Some(now + Duration::from_millis(duration_ms));
        self.next_sleep_tick = None;
    }

    pub fn update(&mut self, now: Instant) -> bool {
        if let Some(deadline) = self.action_until {
            if now >= deadline {
                self.current = PetState::Idle;
                self.action_until = None;
                return true;
            }
        }

        match self.current {
            PetState::Idle if now >= self.last_activity + SLEEP_AFTER => {
                self.current = PetState::Sleep;
                self.sleep_frame = 0;
                self.next_sleep_tick = Some(now + SLEEP_FRAME_MS);
                true
            }
            PetState::Sleep => {
                if let Some(tick) = self.next_sleep_tick {
                    if now >= tick {
                        self.sleep_frame = (self.sleep_frame + 1) % 3;
                        self.next_sleep_tick = Some(now + SLEEP_FRAME_MS);
                        return true;
                    }
                }
                false
            }
            _ => false,
        }
    }

    pub fn next_deadline(&self) -> Option<Instant> {
        if let Some(until) = self.action_until {
            return Some(until);
        }
        match self.current {
            PetState::Idle => Some(self.last_activity + SLEEP_AFTER),
            PetState::Sleep => self.next_sleep_tick,
            _ => None,
        }
    }

    pub fn current_sprite<'a>(&self, sprites: &'a SpriteSet) -> &'a Sprite {
        match self.current {
            PetState::Idle => &sprites.idle,
            PetState::Typing => &sprites.type_frames[self.type_toggle],
            PetState::MouseLeft => &sprites.type_frames[1], // type2 左側下壓
            PetState::MouseRight => &sprites.type_frames[0], // type1 右側下壓
            PetState::Petted => &sprites.pet,
            PetState::Sleep => &sprites.sleep_frames[self.sleep_frame],
        }
    }
}
