use std::iter::once;

use itertools::Itertools;

pub struct PascalsTriangle {
    values: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut values: Vec<Vec<u32>> = vec![];
        for _ in 0..row_count {
            if let Some(row) = values.last() {
                if row.len() == 1 {
                    values.push(vec![1, 1]);
                } else {
                    values.push(
                        once(&0)
                            .chain(row.iter())
                            .chain(once(&0))
                            .tuple_windows()
                            .map(|(a, b)| a + b)
                            .collect(),
                    );
                }
            } else {
                values.push(vec![1]);
            }
        }

        Self { values }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.values.clone()
    }
}
