#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    pins_scored: u16,
    frames: u8,
    rolls: u8,
}

impl Default for BowlingGame {
    fn default() -> Self {
        BowlingGame::new()
    }
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            pins_scored: 0,
            frames: 10,
            rolls: 0,
        }
    }
    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            Err(Error::NotEnoughPinsLeft)
        } else if self.frames == 0 {
            Err(Error::GameComplete)
        } else {
            self.rolls += 1;
            if self.rolls == 2 {
                self.rolls = 0;
                self.frames -= 1;
            }
            Ok(())
        }
    }

    pub fn score(&self) -> Option<u16> {
        if self.frames > 0 {
            None
        } else {
            Some(self.pins_scored)
        }
    }
}
