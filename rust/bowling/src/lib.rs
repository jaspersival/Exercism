#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Copy, Clone)]
pub enum FrameType {
    Open,
    Spare,
    Strike,
}

#[derive(Copy, Clone)]
pub struct Frame {
    frame_type: FrameType,
    pins_scored: u16,
}
impl Default for Frame {
    fn default() -> Self {
        Frame {
            frame_type: FrameType::Open,
            pins_scored: 0,
        }
    }
}
pub struct BowlingGame {
    frames: [Frame; 10],
    frame_index: usize,
    rolls: u8,
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
            frames: [Frame::default(); 10],
            frame_index: 0,
            rolls: 0,
            double_pins: 0,
        }
    }
    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            Err(Error::NotEnoughPinsLeft)
        } else if self.frame_index + 1 > self.frames.len() {
            Err(Error::GameComplete)
        } else {
            self.rolls += 1;
            if self.frame_index > 0 {
                match self.frames[self.frame_index - 1].frame_type {
                    FrameType::Open => {}
                    FrameType::Spare => {
                        if self.rolls == 1 {
                            self.frames[self.frame_index - 1].pins_scored += pins;
                        }
                    }
                    FrameType::Strike => {
                        self.frames[self.frame_index - 1].pins_scored += pins;
                    }
                }
            }
            self.frames[self.frame_index].pins_scored += pins;
            if pins == 10 && self.rolls == 1 {
                self.frames[self.frame_index].frame_type = FrameType::Strike;
                self.rolls = 0;
                self.frame_index += 1;
            }
            if self.rolls == 2 {
                if self.frames[self.frame_index].pins_scored == 10 {
                    self.frames[self.frame_index].frame_type = FrameType::Spare;
                }
                self.rolls = 0;
                self.frame_index += 1;
            }
            Ok(())
        }
    }

    pub fn score(&self) -> Option<u16> {
        if self.frame_index < 10 {
            None
        } else {
            Some(self.frames.iter().map(|elem| elem.pins_scored).sum())
        }
    }
}
