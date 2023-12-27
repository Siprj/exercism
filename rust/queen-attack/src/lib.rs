#[derive(Debug)]
pub struct ChessPosition {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if (0..8).contains(&rank) && (0..8).contains(&file) {
            Some(Self { x: file, y: rank })
        } else {
            None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.0.x == other.0.x
            || self.0.y == other.0.y
            || (self.0.y.abs_diff(other.0.y) == self.0.x.abs_diff(other.0.x))
    }
}
