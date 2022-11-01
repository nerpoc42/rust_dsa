/// Link is an optional connection between two nodes
type Link<T> = Option<Box<Node<T>>>;

/// Node holds data and optional next node
struct Node<T> {
    lower_node: Link<T>,
    data: T,
}

impl<T> Node<T> {
    fn make_link(data: T, lower_node: Link<T>) -> Link<T> {
        Some(Box::new(Self { lower_node, data }))
    }
}

/// Stack with basic methods
pub struct Stack<T> {
    top_node: Link<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { top_node: None }
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Stack main methods
impl<T> Stack<T> {
    pub fn peek(&self) -> Option<&T> {
        self.top_node.as_ref().map(|node| &node.data)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.top_node.as_mut().map(|n| &mut n.data)
    }

    pub fn nth(&self, idx: usize) -> Option<&T> {
        self.iter().nth(idx)
    }

    pub fn nth_mut(&mut self, idx: usize) -> Option<&mut T> {
        self.iter_mut().nth(idx)
    }

    pub fn push(&mut self, data: T) {
        self.top_node = Node::make_link(data, self.top_node.take());
    }

    pub fn pop(&mut self) -> Option<T> {
        self.iter_mut().remove_next()
    }

    pub fn remove(&mut self, idx: usize) -> Option<T> {
        self.iter_mut().remove_nth(idx)
    }
}

impl<T: PartialEq<T>> Stack<T> {
    pub fn contains(&self, data: &T) -> bool {
        self.iter().find(|d| d == &data).is_some()
    }
}

/// Stack iterator methods
impl<T> Stack<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter::from(&self.top_node)
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut::from(&mut self.top_node)
    }
}

impl<T> IntoIterator for Stack<T> {
    type IntoIter = IntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::from(self.top_node)
    }
}

/// Node immutable iterator
pub struct Iter<'a, T> {
    node: Option<&'a Node<T>>,
}

impl<'a, T> From<&'a Link<T>> for Iter<'a, T> {
    fn from(link: &'a Link<T>) -> Self {
        Self { node: link.as_deref() }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // Gets current node
        let node = self.node?;

        // Observes next node
        self.node = node.lower_node.as_deref();
        
        // Returns old data
        Some(&node.data)
    }
}

/// Node mutable iterator
pub struct IterMut<'a, T> {
    link: Option<&'a mut Link<T>>,
}

impl<'a, T> From<&'a mut Link<T>> for IterMut<'a, T> {
    fn from(link: &'a mut Link<T>) -> Self {
        Self { link: Some(link) }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        // Gets current node
        let node = self.link.take()?.as_mut()?;
        
        // Observes next link
        self.link.replace(&mut node.lower_node);

        // Returns old data
        Some(&mut node.data)
    }
}

/// Extended stack specific methods for mutable iterator
impl<'a, T> IterMut<'a, T> {
    pub fn remove_next(&mut self) -> Option<T> {
        // Gets current link
        let link = self.link.take()?;

        // Takes out current node
        let node = link.take()?;

        // Replaces link's node
        *link = node.lower_node;

        // Returns old data
        Some(node.data)
    }

    pub fn remove_nth(&mut self, idx: usize) -> Option<T> {
        // Iterates till it gets to the index
        for _ in 0..idx {
            self.next()?;
        }

        self.remove_next()
    }
}

/// Node into iterator
pub struct IntoIter<T> {
    link: Link<T>
}

impl<T> From<Link<T>> for IntoIter<T> {
    fn from(link: Link<T>) -> Self {
        Self { link }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // Gets current node
        let node = self.link.take()?;

        // Observes next link
        self.link = node.lower_node;

        // Returns old data
        Some(node.data)
    }
}

