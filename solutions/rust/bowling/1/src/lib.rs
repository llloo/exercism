#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    frames: Vec<Vec<u16>>,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            frames: Vec::new(),
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.frames.len() < 10 {
            if self.frames.is_empty()
                || self.frames.last().unwrap().len() == 2
                || self.frames.last().unwrap()[0] == 10
            {
                self.frames.push(vec![pins]);
            } else {
                let last = self.frames.last_mut().unwrap();
                if last[0] + pins > 10 {
                    return Err(Error::NotEnoughPinsLeft);
                }
                last.push(pins);
            }
        } else {
            let last = self.frames.last_mut().unwrap();
            if last.len() == 3 {
                return Err(Error::GameComplete);
            }
            if last.len() == 2 && last[0] + last[1] < 10 {
                return Err(Error::GameComplete);
            }

            // Validate according to 10th-frame bonus rules:
            // - If adding the 2nd roll (last.len() == 1) and the first roll wasn't a strike,
            //   the two rolls must sum to at most 10.
            // - If adding the 3rd roll (last.len() == 2):
            //     * If the first roll was a strike and the first bonus was NOT a strike,
            //       then the second bonus cannot be a strike and the two bonus rolls' sum <= 10.
            //     * Otherwise (first bonus was a strike, or it was a spare), standard 0..=10 applies.
            if last.len() == 1 {
                if last[0] != 10 {
                    if last[0] + pins > 10 {
                        return Err(Error::NotEnoughPinsLeft);
                    }
                }
                // if last[0] == 10 -> first of 10th was strike, second roll allowed (validated below if needed)
            } else if last.len() == 2 {
                // We're about to add the third roll.
                if last[0] == 10 {
                    // First roll of 10th was a strike.
                    if last[1] != 10 {
                        // First bonus was NOT a strike:
                        // - second bonus cannot be a strike
                        // - bonus rolls cannot sum to more than 10
                        if pins == 10 {
                            return Err(Error::NotEnoughPinsLeft);
                        }
                        if last[1] + pins > 10 {
                            return Err(Error::NotEnoughPinsLeft);
                        }
                    } else {
                        // last[1] == 10 -> two consecutive strikes: third roll can be 0..=10
                    }
                } else {
                    // First roll of 10th was not a strike, but since we're adding a third roll,
                    // it must be a spare (last[0] + last[1] == 10). The bonus roll can be 0..=10.
                }
            }

            last.push(pins);
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.frames.len() != 10 {
            return None;
        }
        let last_frame = &self.frames[9];
        if last_frame.len() == 2 && last_frame[0] + last_frame[1] >= 10 && last_frame.len() != 3 {
            return None;
        }
        if last_frame.len() < 2 {
            return None;
        }

        let mut total = 0;
        for i in 0..10 {
            let frame = &self.frames[i];
            if frame[0] == 10 {
                // Strike
                total += 10;
                if i < 9 {
                    total += self.frames[i + 1][0];
                    if self.frames[i + 1].len() > 1 {
                        total += self.frames[i + 1][1];
                    } else {
                        total += self.frames[i + 2][0];
                    }
                } else {
                    total += frame[1] + frame[2];
                }
            } else if frame[0] + frame[1] == 10 {
                // Spare
                total += 10;
                if i < 9 {
                    total += self.frames[i + 1][0];
                } else {
                    total += frame[2];
                }
            } else {
                total += frame[0] + frame[1];
            }
        }
        Some(total)
    }
}
