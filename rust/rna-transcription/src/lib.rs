#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        if let Some(pos) = dna
            .chars()
            .position(|c| c != 'G' && c != 'C' && c != 'T' && c != 'A')
        {
            Err(pos)
        } else {
            Ok(Self(dna.to_string()))
        }
    }

    pub fn into_rna(self) -> Rna {
        Rna(self
            .0
            .chars()
            .map(|c| match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => unreachable!(),
            })
            .collect())
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        if let Some(pos) = rna
            .chars()
            .position(|c| c != 'C' && c != 'G' && c != 'A' && c != 'U')
        {
            Err(pos)
        } else {
            Ok(Self(rna.to_string()))
        }
    }
}
