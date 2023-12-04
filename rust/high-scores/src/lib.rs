#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a[u32],
}

impl <'a> HighScores<'a> {
    pub fn new<'b>(scores: &'b[u32]) ->Self
    where 'b : 'a
    {
        Self { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut s = self.scores.to_vec();
        s.sort_unstable_by(|a, b| b.cmp(a));
        s.truncate(3);
        s
    }
}
