#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    pins_scored: u16,
}

impl Default for BowlingGame {
    fn default() -> Self {
        BowlingGame::new()
    }
}

impl BowlingGame {
    pub fn new() -> Self {
        Self { pins_scored: 0 }
    }
    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins == 0 {
            Ok(())
        } else {
            Err(Error::NotEnoughPinsLeft)
        }
    }

    pub fn score(&self) -> Option<u16> {
        todo!("Return the score if the game is complete, or None if not.");
    }
}
