use std::slice::Iter;

fn main() {
    println!("Hello, world!");
    let mut bin = BinaryHeap::new();

    bin.push(0);
    bin.push(1);
    bin.push(10);
    dbg!(bin.pop());
    dbg!(bin.pop());
    dbg!(bin.pop());
    bin.push(45);
    bin.push(4534);
    bin.push(4);
    while !bin.is_empty() {
        dbg!(bin.pop());
    }
}

/// Min heap
struct BinaryHeap<T: PartialOrd> {
    vec: Vec<T>,
}
impl<T: PartialOrd> BinaryHeap<T> {
    pub fn new() -> Self {
        BinaryHeap { vec: Vec::new() }
    }
    pub fn len(&self) -> usize {
        self.vec.len()
    }
    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    /// iterate in arbitrary order
    pub fn iter(&self) -> Iter<'_, T> {
        self.vec.iter()
    }

    /// drops all values in heap
    pub fn clear(&mut self) {
        self.vec.clear();
    }

    fn parent_idx(you: usize) -> usize {
        debug_assert!(you != 0, "try to get parent of root");
        (you - 1) >> 1
    }
    fn child_idxs(you: usize) -> (usize, usize) {
        ((you << 1) + 1, (you << 1) + 2)
    }
    fn higher_in_tree(&self, you: usize, they: usize) -> bool {
        self.vec[you] < self.vec[they]
    }
    fn flow_up(&mut self, mut idx: usize) {
        loop {
            if idx == 0 {
                return;
            }
            let parent_idx = Self::parent_idx(idx);
            if self.higher_in_tree(idx, parent_idx) {
                self.vec.swap(parent_idx, idx);
                idx = parent_idx;
            } else {
                return;
            }
        }
    }
    // must always pick min child, as that one can only be parent to the other.
    /// Can be called on node that doesn't exist, as it will not have any children
    fn flow_down(&mut self, mut idx: usize) {
        loop {
            let (child1_idx, child2_idx) = Self::child_idxs(idx);
            let highest_child_idx = if child1_idx >= self.vec.len() {
                return; // don't have any children
            } else if child2_idx >= self.vec.len() || self.higher_in_tree(child1_idx, child2_idx) {
                child1_idx // only have left child || OR, left is higher than right
            } else {
                child2_idx // child2 was higher, has to be parent
            };
            if self.higher_in_tree(idx, highest_child_idx) {
                // we are higher than both. Stay here
                return;
            } else {
                // we are lower than the highest one, we can ONLY switch places with highest child.
                self.vec.swap(idx, highest_child_idx);
                idx = highest_child_idx;
            }
        }
    }
    fn flow_down_rec(&mut self, idx: usize, vec_len: usize) {
        let child_idx = 'blk: {
            let left_child_idx = (idx << 1) + 1;
            if left_child_idx >= vec_len {
                return;
            }
            let right_child_idx = left_child_idx + 1;
            if right_child_idx >= vec_len {
                break 'blk left_child_idx;
            }
            if self.vec[left_child_idx] < self.vec[right_child_idx] {
                left_child_idx
            } else {
                right_child_idx
            }
        };
        if self.vec[idx] > self.vec[child_idx] {
            return; // if both are lower, don't do anything
        }
        self.vec.swap(idx, child_idx);
        self.flow_down_rec(child_idx, vec_len);
    }
    pub fn push(&mut self, val: T) {
        let idx = self.vec.len();
        self.vec.push(val);
        self.flow_up(idx);
    }

    /// get minimum element, panics on empty
    pub fn pop(&mut self) -> T {
        // get last element first, the min at end
        assert!(
            !self.vec.is_empty(),
            "Can't pop with no elements in binary heap!"
        );
        let end = self.vec.len() - 1;
        self.vec.swap(0, end);
        let return_val = self.vec.pop().unwrap();
        self.flow_down(0);
        return_val
    }
    /// returns reference to min element, panics on empty
    pub fn peek(&mut self) -> &T {
        assert!(
            !self.vec.is_empty(),
            "Can't peek with no elements in binary heap!"
        );
        &self.vec[0]
    }
}


mod test {
    use std::{fs::File, io::Read};

    #[test]
    fn fuzzy_test_binary_heap_against_std_binary_heap() {
        use crate::BinaryHeap as MyHeap;
        use std::collections::BinaryHeap as StdHeap;
        let mut a = MyHeap::new();
        let mut b = StdHeap::new();
        let mut watermark_len = 0;

        let mut file = File::open("/dev/random").unwrap();
        for _ in 0..1000000 {
            assert_eq!(a.len(), b.len());
            watermark_len = watermark_len.max(a.len());
            let mut buf = [0;16];
            file.read_exact(&mut buf).unwrap();
            let num: [u8; 8] = buf[8..].try_into().unwrap();
            let num = u64::from_le_bytes(num);
            match buf[0] % 3 {
                0 => {
                    
                    a.push(num);
                    b.push(std::cmp::Reverse(num));
                }
                1 => {
                    if !a.is_empty() {
                        assert_eq!(a.pop(), b.pop().unwrap().0);
                    }
                }
                2 => {
                    if !a.is_empty() {
                        assert_eq!(*a.peek(), b.peek().unwrap().0);
                    }
                }
                _ => panic!()
            }
            
        }
        println!("watermark len: {}", watermark_len);
    }
}