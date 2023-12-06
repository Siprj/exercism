use std::default;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, Clone, PartialEq)]
enum GameState {
    Starting,
    GameInProgress,
    FinishingFrame(u16),
    FinalRolls(u8, u16),
    GameDone,
}
use GameState::*;

#[derive(Debug, Clone)]
pub struct BowlingGame {
    rolls: Vec<u16>,
    state: GameState,
    frames: usize,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            rolls: vec![],
            state: Starting,
            frames: 0,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        match self.state {
            Starting => {
                self.state = GameInProgress;
                self.rolls.push(pins);
                if pins == 10 {
                    self.frames += 1;
                } else {
                    self.state = FinishingFrame(pins);
                }
                Ok(())
            },
            FinishingFrame(previous) => {
                if previous + pins > 10 {
                    return Err(Error::NotEnoughPinsLeft);
                }
                self.rolls.push(pins);
                self.frames += 1;
                if self.frames == 10 {
                    if previous + pins == 10 {
                        self.state = FinalRolls(1, 10);
                    }else {
                        self.state = GameDone;
                    }
                } else {
                        self.state = GameInProgress;
                }
                Ok(())
            },
            GameDone => Err(Error::GameComplete),
            GameInProgress => {
                self.rolls.push(pins);
                if pins == 10 {
                    self.frames += 1;
                    if self.frames == 10 {
                        self.state = FinalRolls(2, 10);
                    }
                } else {
                    self.state = FinishingFrame(pins);
                }
                Ok(())
            },
            FinalRolls(count, pins_left) => {
                self.rolls.push(pins);
                if pins > pins_left {
                    return Err(Error::NotEnoughPinsLeft);
                }
                if count == 1 {
                    self.state = GameDone;
                } else {
                    let nex_pin_count = if pins == 10 { 10 } else {10 - pins};
                    self.state = FinalRolls(1, nex_pin_count);
                }
                Ok(())
            },
        }
    }

    pub fn score(&self) -> Option<u16> {
        if self.state != GameDone {
            return None;
        }
        let mut index = 0;
        let mut score: u16 = 0;
        for _ in 0..10 {
            if self.rolls[index] == 10 {
                score += 10 + self.rolls[index + 1] + self.rolls[index + 2];
                index += 1;

            } else {
                let frame_sum = self.rolls[index] + self.rolls[index + 1];
                score += frame_sum;
                if frame_sum == 10 {
                    score += self.rolls[index + 2];
                }
                index += 2;
            }
        }
        Some(score)
    }
}
