use std::ops::Index;

#[derive(Debug)]
pub struct BitArray(Vec<usize>);

impl Index<usize> for BitArray {
    type Output = bool;
    fn index(&self, index: usize) -> &Self::Output {
        self.get_unchecked(index)
    }
}

impl BitArray {
    pub fn get_unchecked(&self, index: usize) -> &bool {
        match self.0[index / usize::BITS as usize] >> (index % usize::BITS as usize) & 1 {
            0 => &false,
            1 => &true,
            _ => unreachable!(),
        }
    }
    pub fn set_unchecked(&mut self, index: usize) {
        self.0[index / usize::BITS as usize] |= 1 << (index % usize::BITS as usize)
    }

    pub fn reset_unchecked(&mut self, index: usize) {
        self.0[index / usize::BITS as usize] &= !(1 << (index % usize::BITS as usize))
    }

    pub fn set_value_unchecked(&mut self, index: usize, value: bool) {
        match value {
            true => self.set_unchecked(index),
            false => self.reset_unchecked(index),
        }
    }

    pub fn get(&self, index: usize) -> Option<&bool> {
        if index / usize::BITS as usize <= self.0.len() {
            Some(self.get_unchecked(index))
        } else {
            None
        }
    }

    pub fn set(&mut self, index: usize) -> bool {
        if index / usize::BITS as usize <= self.0.len() {
            self.set_unchecked(index);
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self, index: usize) -> bool {
        if index / usize::BITS as usize <= self.0.len() {
            self.reset_unchecked(index);
            true
        } else {
            false
        }
    }

    pub fn set_value(&mut self, index: usize, value: bool) -> bool {
        if index / usize::BITS as usize <= self.0.len() {
            self.set_value_unchecked(index, value);
            true
        } else {
            false
        }
    }

    pub fn new_zeros(n: usize) -> Self {
        Self(vec![0; n.div_ceil(usize::BITS as usize)])
    }

    pub fn new_ones(n: usize) -> Self {
        Self(vec![usize::max_value(); n.div_ceil(usize::BITS as usize)])
    }

    pub fn iter(&self) -> BitArrayIterator {
        BitArrayIterator {
            bit_array: self,
            index: 0,
        }
    }
}

pub struct BitArrayIterator<'a> {
    bit_array: &'a BitArray,
    index: usize,
}

impl<'a> Iterator for BitArrayIterator<'a> {
    type Item = &'a bool;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.bit_array.get(self.index - 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::BitArray;

    #[test]
    fn index() {
        let bit_array = BitArray::new_zeros(1000);
        assert_eq!(bit_array[5], false);
    }

    #[test]
    fn set() {
        let mut bit_array = BitArray::new_zeros(1000);
        assert_eq!(bit_array.set(2), true);
        assert_eq!(bit_array[2], true);

        bit_array.set(65);
        assert_eq!(bit_array[65], true);
    }

    #[test]
    fn reset() {
        let mut bit_array = BitArray::new_zeros(1000);
        bit_array.reset(2);
        assert_eq!(bit_array[2], false);
    }

    #[test]
    fn set_value() {
        let mut bit_array = BitArray::new_zeros(1000);
        bit_array.set_value(2, true);
        assert_eq!(bit_array[2], true);
    }
}
