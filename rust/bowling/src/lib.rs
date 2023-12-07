use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, Clone)]
pub struct BowlingGame {
    score: u16,
    multipliers: VecDeque<u16>,
    frame: u16,
    pending_pins: Option<u16>,
    ongoing: bool,
}

impl BowlingGame {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            score: 0,
            multipliers: VecDeque::new(),
            frame: 0,
            pending_pins: None,
            ongoing: true,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if !self.ongoing {
            return Err(Error::GameComplete);
        }
        if pins + self.pending_pins.unwrap_or(0) > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.frame < 10 {
            self.score += pins;
        }

        if let Some(m) = self.multipliers.pop_front() {
            self.score += pins * m;
        }

        if let Some(pending) = self.pending_pins.take() {
            if pending + pins == 10 {
                // Spare
                if self.frame < 10 {
                    if let Some(m) = self.multipliers.front_mut() {
                        *m += 1;
                    } else {
                        self.multipliers.push_back(1);
                    }
                }
            }
        } else if pins == 10 {
            // Strike
            if self.frame < 10 {
                if let Some(m) = self.multipliers.front_mut() {
                    *m += 1;
                } else {
                    self.multipliers.push_back(1);
                }
                self.multipliers.push_back(1);
            }
        } else {
            self.pending_pins = Some(pins);
        }

        if self.pending_pins.is_none() {
            self.frame += 1;
        }

        dbg!(&self.frame);
        dbg!(&self.multipliers);

        if self.frame >= 10 && self.multipliers.is_empty() {
            self.ongoing = false;
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.ongoing {
            Some(self.score)
        } else {
            None
        }
    }
}
