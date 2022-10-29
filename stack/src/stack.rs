type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    lower_node: Link<T>,
    data: T,
}

impl<T> Node<T> {
    pub fn new_link(data: T, lower_node: Link<T>) -> Link<T> {
        Some(Box::new(Self { lower_node, data }))
    }

    pub fn remove_next(&mut self) -> Option<T> {
        self.lower_node.take().map(|target| {
            self.lower_node = target.lower_node;
            target.data
        })
    }

    pub fn nth_node(&mut self, idx: usize) -> Option<&mut Self> {
        let mut iter = Some(self);

        for _ in 0..idx {
            iter = match iter {
                None => break,
                Some(node) => node.lower_node.as_deref_mut()
            }
        }

        iter
    }
}

pub struct Stack<T> {
    top_node: Link<T>,
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { top_node: None }
    }
}

impl<T> IntoIterator for Stack<T> {
    type IntoIter = IntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::from(self.top_node)
    }
}

impl<T> Stack<T> {
    pub fn peek(&self) -> Option<&T> {
        self.top_node.as_ref().map(|node| &node.data)
    }

    pub fn peek_mut(&mut self) -> Option<&T> {
        self.top_node.as_mut().map(|n| &n.data)
    }

    pub fn push(&mut self, data: T) {
        self.top_node = Node::new_link(data, self.top_node.take());
    }

    pub fn pop(&mut self) -> Option<T> {
        self.top_node.take().map(|node| {
            self.top_node = node.lower_node;
            node.data
        })
    }

    pub fn iter(&self) -> Iter<T> {
        Iter::from(&self.top_node)
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut::from(&mut self.top_node)
    }

    pub fn nth(&self, idx: usize) -> Option<&T> {
        self.iter().nth(idx)
    }

    pub fn nth_mut(&mut self, idx: usize) -> Option<&mut T> {
        self.iter_mut().nth(idx)
    }

    pub fn remove(&mut self, idx: usize) -> Option<T> {
        if idx == 0 {
            return self.pop();
        }

        self.top_node.as_mut()?.nth_node(idx-1)?.remove_next()
    }
}

impl<T: PartialEq<T>> Stack<T> {
    pub fn contains(&self, data: &T) -> bool {
        self.iter().find(|d| d == &data).is_some()
    }
}

// Node mutable iterator
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
        self.link.take()?.as_mut().map(|node| {
            self.link.replace(&mut node.lower_node);
            &mut node.data
        })
    }
}

// Node reference iterator
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
        self.node.map(|node| {
            self.node = node.lower_node.as_deref();
            &node.data
        })
    }
}

// Node into iterator
pub struct IntoIter<T> {
    link: Option<Link<T>>
}

impl<T> From<Link<T>> for IntoIter<T> {
    fn from(link: Link<T>) -> Self {
        Self { link: Some(link) }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.link.take()?.map(|node| {
            self.link = Some(node.lower_node);
            node.data
        })
    }
}
