//! Given a binary search tree, find the Nth largest element.

pub struct BinarySearchNode<T> {
    pub item: T,
    pub left: Option<Box<BinarySearchNode<T>>>,
    pub right: Option<Box<BinarySearchNode<T>>>,
}

impl<T> BinarySearchNode<T> {
    pub fn new(item: T) -> BinarySearchNode<T> {
        BinarySearchNode {
            item,
            left: None,
            right: None,
        }
    }

    pub fn new_with_left(item: T, left: BinarySearchNode<T>) -> BinarySearchNode<T> {
        BinarySearchNode {
            item,
            left: Some(Box::new(left)),
            right: None,
        }
    }

    pub fn new_with_right(item: T, right: BinarySearchNode<T>) -> BinarySearchNode<T> {
        BinarySearchNode {
            item,
            left: None,
            right: Some(Box::new(right)),
        }
    }

    pub fn new_with_left_and_right(
        item: T,
        left: BinarySearchNode<T>,
        right: BinarySearchNode<T>,
    ) -> BinarySearchNode<T> {
        BinarySearchNode {
            item,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
}

impl<T: Clone> BinarySearchNode<T> {
    /// Returns the Nth largest element in a BinarySearchTree
    pub fn find_nth_largest(&self, n: u32) -> Option<T> {
        self.find_nth_largest_recurse(n)
    }

    fn find_nth_largest_recurse(&self, n: u32) -> Option<T> {
        if n == 0 {
            return None;
        }

        let mut counter = n - 1;
        self.find_nth_largest_recurse_helper(&mut counter)
    }

    fn find_nth_largest_recurse_helper(&self, counter: &mut u32) -> Option<T> {
        if let Some(right) = &self.right {
            if let Some(result) = right.find_nth_largest_recurse_helper(counter) {
                return Some(result);
            }

            // Since we didn't find it in the right subtree, we should
            // subtract all the elements in the right subtree from the count
            // (note that the subtree already subtracted all other elements from the counter)
            *counter -= 1;
        }

        if *counter == 0 {
            return Some(self.item.clone());
        }

        if let Some(left) = &self.left {
            // Increment counter for left, does not get
            // checked until fully down the right subtree
            *counter -= 1;

            if let Some(result) = left.find_nth_largest_recurse_helper(counter) {
                return Some(result);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_as_expected() {
        // Tree looks like:
        //          5
        //    3         15
        // 1    4           30
        let head = {
            let e3 = {
                let e1 = BinarySearchNode::new(1);
                let e4 = BinarySearchNode::new(4);

                BinarySearchNode::new_with_left_and_right(3, e1, e4)
            };

            let e15 = {
                let e30 = BinarySearchNode::new(30);
                BinarySearchNode::new_with_right(15, e30)
            };

            BinarySearchNode::new_with_left_and_right(5, e3, e15)
        };

        assert_eq!(head.find_nth_largest(0), None);
        assert_eq!(head.find_nth_largest(1), Some(30));
        assert_eq!(head.find_nth_largest(2), Some(15));
        assert_eq!(head.find_nth_largest(3), Some(5));
        assert_eq!(head.find_nth_largest(4), Some(4));
        assert_eq!(head.find_nth_largest(5), Some(3));
        assert_eq!(head.find_nth_largest(6), Some(1));
        assert_eq!(head.find_nth_largest(7), None);
    }
}
