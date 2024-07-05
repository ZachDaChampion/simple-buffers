use super::{SyntaxTree, TaggedSyntaxTree};
use std::collections::VecDeque;

/// An iterator over the direct children of a SyntaxTree.
struct Children<'a> {
    /// The tree whose children are being iterated over.
    tree: &'a TaggedSyntaxTree<'a>,

    /// The index of the next child to return when iterating forward.
    index: usize,

    /// The index of the next child to return when iterating backward.
    index_back: usize,
}

impl<'a> Iterator for Children<'a> {
    type Item = &'a TaggedSyntaxTree<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // If the forward index is greater than or equal to the backward index, there are no more
        // children.
        if self.index >= self.index_back {
            return None;
        }

        // Figure out which child is next depending on the type of the tree.
        let res = match &self.tree.data {
            // If the tree is a file, sequence, enum, or oneof, the children are in a vector.
            SyntaxTree::File(vec)
            | SyntaxTree::Sequence(_, vec)
            | SyntaxTree::Enum(_, vec)
            | SyntaxTree::OneOf(vec) => vec.get(self.index),

            // If the tree is a field or array, the child is a single tree.
            SyntaxTree::Field(_, child) | SyntaxTree::Array(child) => {
                if self.index == 0 {
                    Some(child.as_ref())
                } else {
                    None
                }
            }

            // If the tree is a primitive, there are no children.
            SyntaxTree::EnumEntry(_, _) | SyntaxTree::Type(_) => None,
        };

        self.index += 1;
        res
    }
}

impl<'a> DoubleEndedIterator for Children<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        // If the backward index is less than or equal to the forward index, there are no more
        // children.
        if self.index_back <= self.index {
            return None;
        }

        // If the backward index is 0, there are no more children.
        if self.index_back == 0 {
            return None;
        }
        self.index_back -= 1;

        // Figure out which child is next depending on the type of the tree.
        let res = match &self.tree.data {
            // If the tree is a file, sequence, enum, or oneof, the children are in a vector.
            SyntaxTree::File(vec)
            | SyntaxTree::Sequence(_, vec)
            | SyntaxTree::Enum(_, vec)
            | SyntaxTree::OneOf(vec) => vec.get(self.index_back),

            // If the tree is a field or array, the child is a single tree.
            SyntaxTree::Field(_, child) | SyntaxTree::Array(child) => {
                if self.index_back == 0 {
                    Some(child.as_ref())
                } else {
                    None
                }
            }

            // If the tree is a primitive, there are no children.
            SyntaxTree::EnumEntry(_, _) | SyntaxTree::Type(_) => None,
        };

        res
    }
}

/// Trait for types that have children. This adds a method to get an iterator over the children.
trait ChildrenIterator<'a> {
    fn children(&'a self) -> Children<'a>;
}

impl<'a> ChildrenIterator<'a> for TaggedSyntaxTree<'a> {
    fn children(&'a self) -> Children<'a> {
        Children {
            tree: self,
            index: 0,
            index_back: match &self.data {
                SyntaxTree::File(vec)
                | SyntaxTree::Sequence(_, vec)
                | SyntaxTree::Enum(_, vec)
                | SyntaxTree::OneOf(vec) => vec.len(),

                SyntaxTree::Field(_, _) | SyntaxTree::Array(_) => 1,

                SyntaxTree::EnumEntry(_, _) | SyntaxTree::Type(_) => 0,
            },
        }
    }
}

/// A trait that implements iterators for traversing a tree.
pub trait TreeTraversal {
    /// The type of the iterator that iterates over the tree.
    type BreadthFirstIterator;
    type DepthFirstIterator;

    /// Iterate over the tree in breadth-first order.
    #[allow(dead_code)]
    fn iter_breadth_first(&self) -> Self::BreadthFirstIterator;

    /// Iterate over the tree in depth-first order.
    fn iter_depth_first(&self) -> Self::DepthFirstIterator;
}

impl<'a> TreeTraversal for &'a TaggedSyntaxTree<'a> {
    type BreadthFirstIterator = BreadthFirstSyntaxTreeInterator<'a>;
    type DepthFirstIterator = DepthFirstSyntaxTreeInterator<'a>;

    fn iter_breadth_first(&self) -> Self::BreadthFirstIterator {
        BreadthFirstSyntaxTreeInterator::new(self)
    }

    fn iter_depth_first(&self) -> Self::DepthFirstIterator {
        DepthFirstSyntaxTreeInterator::new(self)
    }
}

/// A breadth-first iterator over a SyntaxTree.
pub struct BreadthFirstSyntaxTreeInterator<'a> {
    /// Queue of nodes to visit.
    queue: VecDeque<&'a TaggedSyntaxTree<'a>>,
}

impl<'a> BreadthFirstSyntaxTreeInterator<'a> {
    /// Create a new breadth-first syntax tree iterator.
    pub fn new(tree: &'a TaggedSyntaxTree) -> Self {
        let mut queue = VecDeque::new();
        queue.push_back(tree);
        Self { queue }
    }
}

impl<'a> Iterator for BreadthFirstSyntaxTreeInterator<'a> {
    type Item = &'a TaggedSyntaxTree<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(tree) = self.queue.pop_front() {
            for child in tree.children() {
                self.queue.push_back(child);
            }
            Some(tree)
        } else {
            None
        }
    }
}

/// A depth-first iterator over a SyntaxTree.
pub struct DepthFirstSyntaxTreeInterator<'a> {
    /// Stack of nodes to visit.
    stack: Vec<&'a TaggedSyntaxTree<'a>>,
}

impl<'a> DepthFirstSyntaxTreeInterator<'a> {
    /// Create a new depth-first syntax tree iterator.
    pub fn new(tree: &'a TaggedSyntaxTree) -> Self {
        let stack = vec![tree];
        Self { stack }
    }
}

impl<'a> Iterator for DepthFirstSyntaxTreeInterator<'a> {
    type Item = &'a TaggedSyntaxTree<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(tree) = self.stack.pop() {
            for child in tree.children().rev() {
                self.stack.push(child);
            }
            Some(tree)
        } else {
            None
        }
    }
}
