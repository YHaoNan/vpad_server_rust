use std::collections::VecDeque;

/// 基于VecDeque，实现迭代器trait，每次从队头取出一个元素放到队尾
/// 是一个无限的iterator
#[derive(Debug)]
pub struct CircleContainer<T>
    where T: Copy {
    underlying_deque: VecDeque<T>
}

impl<T> CircleContainer<T>
    where T: Copy {
    pub fn new(vec: Vec<T>) -> CircleContainer<T> {
        CircleContainer {
            underlying_deque: VecDeque::from(vec)
        }
    }
}

impl<T> Iterator for CircleContainer<T>
    where T: Copy {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.underlying_deque.len() == 0 {
            None
        } else {
            let cur_elem = self.underlying_deque.pop_front().unwrap();
            self.underlying_deque.push_back(cur_elem);
            Some(cur_elem)
        }
    }
}

#[cfg(test)]
mod circle_container_test{
    use crate::circle_container::CircleContainer;

    #[test]
    fn test_new() {
        let container = CircleContainer::new(vec![1, 2, 3]);
    }

    #[test]
    fn test_iterator_with_non_empty_container() {
        let mut container = CircleContainer::new(vec![1, 2, 3]);

        assert_eq!(container.next(), Some(1));
        assert_eq!(container.next(), Some(2));
        assert_eq!(container.next(), Some(3));
    }

    #[test]
    fn test_iterator_with_empty_container() {
        let mut container: CircleContainer<u32> = CircleContainer::new(vec![]);
        assert_eq!(container.next(), None);
    }
}