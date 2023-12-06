use std::{collections::VecDeque, iter::repeat};

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, Clone, PartialEq)]
enum GameState {
    GameInProgress(u32, Option<u16>),
    Finishing(u16, u16),
    GameDone,
}
use GameState::*;

#[derive(Debug, Clone)]
pub struct BowlingGame {
    score: u16,
    multipliers: VecDeque<u16>,
    state: GameState,
}

impl BowlingGame {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            score: 0,
            multipliers: repeat(1).take(2).collect(),
            state: GameInProgress(0, None),
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        match self.state {
            GameInProgress(frame_count, running_frame) => {
                let new_frame_count = frame_count + 1;

                if let Some(triped_pins) = running_frame {
                    let frame_triped_pins = pins + triped_pins;
                    if pins + triped_pins > 10 {
                        return Err(Error::NotEnoughPinsLeft);
                    }
                    let multiplier = self.multipliers.pop_front().expect("thre should always be two multipliers");
                    self.multipliers.push_back(1);

                    self.score += pins * multiplier;
                    if new_frame_count == 10  && frame_triped_pins == 10 {
                        self.state = Finishing(1, 10);
                    } else if new_frame_count == 10 {
                        self.state = GameDone;
                    } else {
                        if frame_triped_pins == 10 {
                            self.multipliers[0] += 1;
                        }
                        self.state = GameInProgress(new_frame_count, None);
                    }
                } else {
                    let multiplier = self.multipliers.pop_front().expect("thre should always be two multipliers");
                    self.multipliers.push_back(1);

                    self.score += pins * multiplier;
                    if pins == 10 {
                        if new_frame_count == 10 {
                            self.state = Finishing(2, 10);
                        } else {
                            self.multipliers[0] += 1;
                            self.multipliers[1] += 1;
                            self.state = GameInProgress(new_frame_count, None);
                        }
                    } else {
                        self.state = GameInProgress(frame_count, Some(pins));
                    }
                }
            },
            Finishing(rolls, finishing_pins) => {
                if pins > finishing_pins {
                    return Err(Error::NotEnoughPinsLeft);
                }

                let multiplier = self.multipliers.pop_front().expect("thre should always be two multipliers");
                self.multipliers.push_back(1);

                self.score += pins * multiplier;
                if rolls - 1 > 0 {
                    self.state = Finishing(rolls - 1, if pins == 10 { 10 } else {finishing_pins - pins})
                } else {
                    self.state = GameDone;
                }
            },
            GameDone => {return Err(Error::GameComplete);},
        }
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.state == GameDone {
            Some(self.score)
        } else {
            None
        }
    }
}
