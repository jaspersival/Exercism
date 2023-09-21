#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    pins_scored: u16,
    frames: u8,
    rolls: u8,
    pins_scored_in_frame: u16,
    double_pins: u8,
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
            pins_scored_in_frame: 0,
            double_pins: 0,
        }
    }
    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        let mut multiplier: u16 = 1;
        if pins > 10 {
            Err(Error::NotEnoughPinsLeft)
        } else if self.frames == 0 {
            Err(Error::GameComplete)
        } else {
            self.rolls += 1;
            if self.double_pins > 0 {
                multiplier = 2;
                self.double_pins -= 1;
            }
            self.pins_scored += multiplier * pins;
            self.pins_scored_in_frame += pins;
            if self.rolls == 2 {
                if self.pins_scored_in_frame == 10 {
                    self.double_pins = 1;
                } else if pins == 10 {
                    self.double_pins = 2;
                }
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
