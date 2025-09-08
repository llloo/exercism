#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        Self {
            scores: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        if self.scores.is_empty() {
            return None;
        }
        Some(self.scores[self.scores.len() - 1])
    }

    pub fn personal_best(&self) -> Option<u32> {
        if self.scores.is_empty() {
            return None;
        }
        let mut n = 0u32;
        for score in &self.scores {
            if *score > n {
                n = *score;
            }
        }
        Some(n)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        if self.scores.is_empty() {
            return Vec::new();
        }

        let mut scores = self.scores.clone();
        scores.sort_unstable();
        scores.reverse();

        if scores.len() <= 3 {
            scores
        } else {
            scores[0..3].to_vec()
        }
    }
}
