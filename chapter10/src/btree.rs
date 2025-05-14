use std::fmt::Display;

#[derive(PartialEq, Eq)]
pub enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}
impl<T: Display + PartialEq + Eq> Display for BinaryTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            BinaryTree::Empty => write!(f, "()"),
            BinaryTree::NonEmpty(ref node) => write!(f, "{}", node.element),
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_build_btree_left_and_right_5depth_element_string() {
        let tree = BinaryTree::NonEmpty(Box::new(TreeNode {
            element: "A",
            left: BinaryTree::NonEmpty(Box::new(TreeNode {
                element: "B",
                left: BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: "D",
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                })),
                right: BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: "E",
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                })),
            })),
            right: BinaryTree::NonEmpty(Box::new(TreeNode {
                element: "C",
                left: BinaryTree::Empty,
                right: BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: "F",
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                })),
            })),
        }));

        match tree {
            BinaryTree::Empty => assert!(BinaryTree::<&str>::Empty == BinaryTree::Empty),
            BinaryTree::NonEmpty(ref t) => {
                assert_eq!(t.element, "A")
            }
        }
    }
}
