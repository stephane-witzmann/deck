use rand::{Rng, thread_rng};
use rand::seq::SliceRandom;

pub struct Deck<T> {
    draw_pile: Vec<T>,
    discard_pile: Vec<T>,
    removed_pile: Vec<T>
}

impl<T> Deck<T> {
    pub fn new() -> Self {
        Self {
            draw_pile: Vec::<T>::new(),
            discard_pile: Vec::<T>::new(),
            removed_pile: Vec::<T>::new(),
        }
    }

    pub fn can_draw(&self) -> bool {
        self.draw_pile.len() > 0
    }

    pub fn draw_top(&mut self) -> Option<T> {
        self.draw_pile.pop()
    }

    pub fn draw_bottom(&mut self) -> Option<T> {
        if self.draw_pile.is_empty() {
            return None;
        }

        Some(self.draw_pile.remove(0))
    }

    pub fn put_top(&mut self, x: T) {
        self.draw_pile.push(x);
    }

    pub fn put_bottom(&mut self, x: T) { self.draw_pile.insert(0, x); }

    pub fn put_sparse(&mut self, elements: Vec<T>) {
        if elements.is_empty() {
            return;
        }

        let n_buckets = elements.len();
        let bucket_standard_size = self.draw_pile.len() / n_buckets;
        let mut carry = self.draw_pile.len() % n_buckets;

        let mut start = 0_usize;
        for x in elements {
            let size = bucket_standard_size + if carry > 0 { carry -= 1; 1 } else { 0 };
            let index = thread_rng().gen_range(0..=size);
            self.draw_pile.insert(start + index, x);

            start += size + 1;
        }
    }

    pub fn discard(&mut self, x: T) {
        self.discard_pile.push(x);
    }

    pub fn remove(&mut self, x: T) {
        self.removed_pile.push(x);
    }

    pub fn remaining(&self) -> usize {
        self.draw_pile.len()
    }

    pub fn see_draw(&mut self) -> &[T] { self.draw_pile.as_slice() }

    pub fn see_discarded(&self) -> &[T] {
        self.discard_pile.as_slice()
    }

    pub fn see_removed(&self) -> &[T] {
        self.removed_pile.as_slice()
    }

    pub fn shuffle_draw(&mut self) { self.draw_pile.as_mut_slice().shuffle(&mut thread_rng()); }

    pub fn shuffle_discard(&mut self) { self.discard_pile.as_mut_slice().shuffle(&mut thread_rng()); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw() {
        let mut deck = Deck::<u8>::new();

        assert!(!deck.can_draw());
        assert_eq!(deck.draw_pile.len(), 0);
        assert_eq!(deck.remaining(), 0);
        assert_eq!(deck.draw_top(), None);
        assert_eq!(deck.draw_bottom(), None);

        deck.put_top(11);
        assert_eq!(deck.draw_pile.len(), 1);
        assert_eq!(deck.remaining(), 1);
        assert!(deck.can_draw());

        deck.put_top(7);
        assert_eq!(deck.draw_pile.len(), 2);
        assert_eq!(deck.remaining(), 2);
        assert!(deck.can_draw());

        deck.put_bottom(5);
        assert_eq!(deck.draw_pile.len(), 3);
        assert_eq!(deck.remaining(), 3);
        assert!(deck.can_draw());

        assert_eq!(deck.draw_top(), Some(7));
        assert!(deck.can_draw());
        assert_eq!(deck.draw_bottom(), Some(5));
        assert!(deck.can_draw());
        assert_eq!(deck.draw_bottom(), Some(11));

        assert!(!deck.can_draw());
        assert_eq!(deck.draw_top(), None);
        assert_eq!(deck.draw_bottom(), None);
    }

    #[test]
    fn test_shuffle_draw() {
        let mut deck = Deck::<u8>::new();
        deck.put_top(1);
        deck.put_bottom(2);
        for _ in 0..2 {
            deck.put_top(0);
            deck.put_bottom(0);
        }

        for _ in 0..10000 { // just try long enough
            deck.shuffle_draw();
            if deck.draw_pile.last() == Some(&1) {
                break;
            }
        }
        assert_eq!(deck.draw_top(), Some(1));

        for _ in 0..10000 { // again
            deck.shuffle_draw();
            if deck.draw_pile.last() == Some(&2) {
                break;
            }
        }
        assert_eq!(deck.draw_top(), Some(2));
    }

    #[test]
    fn test_shuffle_discard() {
        let mut deck = Deck::<u8>::new();
        deck.discard(0);
        deck.discard(1);
        assert_eq!(deck.see_discarded(), [0, 1]);

        for _ in 0..1000 { // just try long enough
            deck.shuffle_discard();
            if deck.discard_pile.last() == Some(&0) {
                break;
            }
        }

        assert_eq!(deck.see_discarded(), [1, 0]);
    }

    #[test]
    fn test_remove() {
        let mut deck = Deck::<u8>::new();
        assert_eq!(deck.removed_pile.len(), 0);
        deck.remove(3);
        assert_eq!(deck.removed_pile.len(), 1);
        deck.remove(8);
        assert_eq!(deck.removed_pile.len(), 2);
        assert_eq!(deck.see_removed(), [3, 8]);
    }

    #[test]
    fn test_discard() {
        let mut deck = Deck::<u8>::new();
        assert_eq!(deck.discard_pile.len(), 0);
        deck.discard(5);
        assert_eq!(deck.discard_pile.len(), 1);
        deck.discard(7);
        assert_eq!(deck.discard_pile.len(), 2);
        assert_eq!(deck.see_discarded(), [5, 7]);
    }

    #[test]
    fn test_put_sparse() {
        sub_test_sparse_zero(50);

        for deck_size in 0..60 {
            for n_insert in 1..(deck_size + 1) {
                sub_test_sparse(deck_size, n_insert);
            }
        }
    }

    fn sub_test_sparse_zero(initial_deck_size: usize) {
        let mut deck = Deck::<usize>::new();
        (0..initial_deck_size).for_each(|i| deck.put_top(i));

        deck.put_sparse(Vec::new());
        assert_eq!(deck.draw_pile.len(), initial_deck_size);
        for i in 0..deck.draw_pile.len() {
            assert_eq!(deck.draw_pile[i], i);
        }
    }

    fn sub_test_sparse(initial_deck_size: usize, n_insert: usize) {
        let mut deck = Deck::<usize>::new();
        (0..initial_deck_size).for_each(|i| deck.put_top(i));

        deck.put_sparse((initial_deck_size..initial_deck_size + n_insert).collect());
        assert_eq!(deck.draw_pile.len(), initial_deck_size + n_insert);

        let bucket_standard_size = initial_deck_size / n_insert + 1;
        let mut carry = initial_deck_size % n_insert;
        let mut start_counter: usize = 0;
        let mut remaining = deck.draw_pile.as_slice();
        let mut expected = initial_deck_size;

        while !remaining.is_empty() {
            let size = bucket_standard_size + if carry > 0 { carry -= 1; 1 } else { 0 };

            let (bucket, tail) = remaining.split_at(size);

            // check bucket
            assert_eq!(bucket.len(), size);
            assert!(bucket.contains(&expected));

            for (i, e) in bucket.iter().filter(|x| **x != expected).enumerate() {
                assert_eq!(*e, start_counter + i);
            }

            // loop
            remaining = tail;
            start_counter += size - 1;
            expected += 1;
        }
    }
}