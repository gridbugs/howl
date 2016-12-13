pub struct BidirectionalList<T: Default> {
    negative: Vec<T>,
    non_negative: Vec<T>,
    default: T,
}

impl<T: Default> BidirectionalList<T> {
    pub fn new() -> Self {
        BidirectionalList {
            negative: Vec::new(),
            non_negative: Vec::new(),
            default: Default::default(),
        }
    }

    pub fn clear(&mut self) {
        self.negative.clear();
        self.non_negative.clear();
    }

    pub fn len(&self) -> usize {
        self.negative.len() + self.non_negative.len()
    }

    pub fn is_empty(&self) -> bool {
        self.negative.is_empty() && self.non_negative.is_empty()
    }

    pub fn get(&self, index: isize) -> Option<&T> {
        if index >= 0 {
            self.non_negative.get(index as usize)
        } else {
            self.negative.get(!(index as usize))
        }
    }

    pub fn get_mut(&mut self, index: isize) -> Option<&mut T> {
        if index >= 0 {
            self.non_negative.get_mut(index as usize)
        } else {
            self.negative.get_mut(!(index as usize))
        }
    }

    pub fn get_checked(&self, index: isize) -> &T {
        if index >= 0 {
            &self.non_negative[index as usize]
        } else {
            &self.negative[!(index as usize)]
        }
    }

    pub fn get_checked_mut(&mut self, index: isize) -> &mut T {
        if index >= 0 {
            &mut self.non_negative[index as usize]
        } else {
            &mut self.negative[!(index as usize)]
        }
    }

    pub unsafe fn get_unchecked(&self, index: isize) -> &T {
        if index >= 0 {
            self.non_negative.get_unchecked(index as usize)
        } else {
            self.negative.get_unchecked(!(index as usize))
        }
    }

    pub unsafe fn get_unchecked_mut(&mut self, index: isize) -> &mut T {
        if index >= 0 {
            self.non_negative.get_unchecked_mut(index as usize)
        } else {
            self.negative.get_unchecked_mut(!(index as usize))
        }
    }

    pub fn get_with_default(&self, index: isize) -> &T {
        self.get(index).unwrap_or(&self.default)
    }

    pub fn get_mut_with_default(&mut self, index: isize) -> &mut T {
        if index >= 0 {
            let index = index as usize;
            for _ in self.non_negative.len()..(index + 1) {
                self.non_negative.push(Default::default());
            }
            unsafe {
                self.non_negative.get_unchecked_mut(index)
            }
        } else {
            if self.non_negative.is_empty() {
                self.non_negative.push(Default::default());
            }
            let index = !(index as usize);
            for _ in self.negative.len()..(index + 1) {
                self.negative.push(Default::default());
            }
            unsafe {
                self.negative.get_unchecked_mut(index)
            }
        }
    }
}

impl<T: Default> Default for BidirectionalList<T> {
    fn default() -> Self {
        Self::new()
    }
}
