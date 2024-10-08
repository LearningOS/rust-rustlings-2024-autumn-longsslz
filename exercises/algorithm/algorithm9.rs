/*
	heap
	This question requires you to implement ac function
*/


use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: Vec::new(),
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.items.push(value);
        self.count += 1;
        self.heapify_up(self.count - 1);
    }

    fn heapify_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent = self.parent_idx(index);
            if (self.comparator)(&self.items[index], &self.items[parent]) {
                self.items.swap(index, parent);
                index = parent;
            } else {
                break;
            }
        }
    }

    fn heapify_down(&mut self, mut index: usize) {
        while self.children_present(index) {
            let left_child = self.left_child_idx(index);
            let right_child = self.right_child_idx(index);
            let mut extreme = index;

            if left_child < self.count && (self.comparator)(&self.items[left_child], &self.items[extreme]) {
                extreme = left_child;
            }
            if right_child < self.count && (self.comparator)(&self.items[right_child], &self.items[extreme]) {
                extreme = right_child;
            }
            if extreme == index {
                break;
            }
            self.items.swap(index, extreme);
            index = extreme;
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        (idx - 1) / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) < self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2 + 1
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        idx * 2 + 2
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let min_value = self.items[0].clone();
        let last_value = self.items.pop().unwrap();
        self.count -= 1;
        if !self.is_empty() {
            self.items[0] = last_value;
            self.heapify_down(0);
        }
        Some(min_value)
    }
}

pub struct MinHeap;

impl MinHeap {
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(11);
        heap.add(9);

        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(11));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);


        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(2));
    }
}