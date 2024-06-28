#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BitSet(u16);

impl BitSet {
    pub fn new() -> BitSet {
        BitSet(0)
    }

    pub fn insert(&mut self, val: u32) {
        self.0 |= (1 << val) as u16;
    }

    pub fn len(&self) -> u32 {
        let mut len = 0;
        let mut bitset = self.0;

        while bitset != 0 {
            len += bitset & 1;
            bitset >>= 1;
        }

        len as u32
    }

    pub fn get_smallest(&self) -> u32 {
        let mut bitset = self.0;
        let mut shift_count = 0;

        while bitset & 1 == 0 {
            bitset >>= 1;
            shift_count += 1;
        }

        shift_count
    }

    pub fn contains(&self, val: u32) -> bool {
        (self.0 >> val) & 1 != 0
    }

    pub fn intersection(&self, other: &BitSet) -> BitSet {
        BitSet(self.0 & other.0)
    }

    pub fn union(&self, other: &BitSet) -> BitSet {
        BitSet(self.0 | other.0)
    }

    pub fn difference(&self, other: &BitSet) -> BitSet {
        BitSet(self.0 & !other.0)
    }

    pub fn iter(&self) -> BitSetIterator {
        BitSetIterator {
            bitset: self.0,
            pop_count: 0,
        }
    }
}

pub struct BitSetIterator {
    bitset: u16,
    pop_count: u32,
}

impl Iterator for BitSetIterator {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.bitset == 0 {
            return None;
        }

        while self.bitset & 1 == 0 {
            self.bitset >>= 1;
            self.pop_count += 1;
        }

        let res = self.pop_count;

        self.bitset >>= 1;
        self.pop_count += 1;

        Some(res)
    }
}
