use std::collections::BTreeMap;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
enum Rank {
    Two = 0,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Hand {
    HighRank(Rank, Rank, Rank, Rank, Rank),
    OnePair(Rank, Rank, Rank, Rank),
    TwoPairs(Rank, Rank, Rank),
    ThreeOfKind(Rank, Rank, Rank),
    Straight(Rank),
    Flush(Rank, Rank, Rank, Rank, Rank),
    FullHouse(Rank, Rank),
    FourOfKinds(Rank, Rank),
    StraightFlush(Rank),
}

fn to_suit(c: &char) -> Suit {
    use Suit::*;
    match c {
        'D' => Diamonds,
        'C' => Clubs,
        'H' => Hearts,
        'S' => Spades,
        _ => unreachable!(),
    }
}

fn to_rank(c: &[char]) -> Rank{
    use Rank::*;
    match c {
        ['2'] => Two,
        ['3'] => Three,
        ['4'] => Four,
        ['5'] => Five,
        ['6'] => Six,
        ['7'] => Seven,
        ['8'] => Eight,
        ['9'] => Nine,
        ['1', '0'] => Ten,
        ['J'] => Jack,
        ['Q'] => Queen,
        ['K'] => King,
        ['A'] => Ace,
        _ => unreachable!(),
    }
}

fn get_straight(hand_vec: &Vec<(Rank, Suit)>) -> Option<Rank> {
    use Rank::*;
    match hand_vec.as_slice() {
        [(Ace, _), (Five, _), (Four, _), (Three, _), (Two, _)] => Some(Five),
        _ => {
            for (i, (r, _)) in hand_vec[1..].iter().enumerate() {
                if (hand_vec[i].0 as usize) != (*r as usize) + 1 {
                    return None
                }
            }
            Some(hand_vec[0].0)
        },
    }
}

fn to_hand(hand_str: &str) -> Hand {
    let mut hand_vec: Vec<(Rank, Suit)> = Vec::with_capacity(5);
    let mut rank_hash: BTreeMap<Rank, u32> = BTreeMap::new();
    for str in  hand_str.split(" ") {
        let v = str.chars().collect::<Vec<char>>();
        let rank = to_rank(if v.len() == 3 {&v[0..2]} else {&v[0..1]});
        let suit = to_suit(v.last().unwrap());
        hand_vec.push((rank, suit));
        *rank_hash.entry(rank).or_insert(0) += 1;
    }

    hand_vec.sort_by(|v1, v2| v2.0.cmp(&v1.0));

    let mut rank_counts: Vec<(Rank, u32)> = rank_hash.iter().map(|(&r, &c)| (r, c)).collect();
    rank_counts.sort_by(|(r1, c1), (r2, c2)| c2.cmp(c1).then(r2.cmp(r1)));

    use Hand::*;

    match rank_counts.as_slice() {
        [(r1, 4), (r2, 1)] => FourOfKinds(*r1, *r2),
        [(r1, 3), (r2, 2)] => FullHouse(*r1, *r2),
        [(r1, 3), (r2, 1), (r3, 1)] => ThreeOfKind(*r1, *r2, *r3),
        [(r1, 2), (r2, 2), (r3, 1)] => TwoPairs(*r1, *r2, *r3),
        [(r1, 2), (r2, 1), (r3, 1), (r4, 1)] => OnePair(*r1, *r2, *r3, *r4),
        [(r1, 1), (r2, 1), (r3, 1), (r4, 1), (r5, 1)] => {
            let is_flush = {
                let mut iter = hand_vec.iter();
                let tmp_suit = iter.next().unwrap().1;
                iter.all(|(_, s)| tmp_suit == *s)
            };
            let is_straight = get_straight(&hand_vec);
            match (is_flush, is_straight) {
                (true, Some(r)) => StraightFlush(r),
                (false, Some(r)) => Straight(r),
                (true, None) => Flush(*r1, *r2, *r3, *r4, *r5),
                (_, _) => HighRank(*r1, *r2, *r3, *r4, *r5),
            }
        },
        _ => unreachable!(),
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut hands_vec: Vec<(Hand, &str)> = Vec::with_capacity(hands.len());
    for h in hands {
        let hand = to_hand(h);
        hands_vec.push((hand, &h));
    }
    hands_vec.sort_by(|(h1, _), (h2, _)| h2.cmp(h1));

    let mut ret: Vec<&'a str> = Vec::new();
    for v in hands_vec.iter() {
        if v.0 != hands_vec[0].0 {
            break;
        }
        ret.push(v.1);
    }

    ret
}
