use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if nucleotide != 'A' && nucleotide != 'C' && nucleotide != 'T' && nucleotide != 'G' {
        return Err(nucleotide);
    }
    let mut acc = 0;
    for c in dna.chars() {
        if c != 'A' && c != 'C' && c != 'T' && c != 'G' {
            return Err(c);
        }
        if c == nucleotide {
            acc += 1;
        }
    }
    Ok(acc)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts: HashMap<char, usize> = HashMap::new();
    counts.insert('A', 0);
    counts.insert('C', 0);
    counts.insert('T', 0);
    counts.insert('G', 0);
    for c in dna.chars() {
        if c != 'A' && c != 'C' && c != 'T' && c != 'G' {
            return Err(c);
        }
        *counts.entry(c).or_default() += 1;
    }
    Ok(counts)
}
