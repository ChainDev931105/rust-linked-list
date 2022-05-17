use std::mem;

pub struct LinkedList<T> {
    head: Node<T>,
    len: i32
}

pub struct Node<T> {
    next: Option<Box<Node<T>>>,
    data: T
}

pub struct NodeIterator<'r, T:'r> {
    slice: Option<&'r Node<T>>,
    index: i32
}


impl<T> LinkedList<T> {
    pub fn new(item: T) -> LinkedList<T> {
        LinkedList {
            head: Node::new(item),
            len: 0
        }
    }

    pub fn push(&mut self, item: T) {
        self.head.push(item);
        self.len += 1;
    }

    pub fn pop(&mut self) {
        self.head.pop();
        self.len -= 1;
    }

    pub fn insert(&mut self, item: T, index: i32) {
        let index_: i32 = if index > self.len {
                        panic!("List index out of range");
                    } else if index < 0 {
                        self.len + index - 1
                    } else {
                        index
                    };
        self.head.insert(item, index_);
        self.len += 1;
    }

    pub fn remove(&mut self, index: i32) {
        let index_: i32 = if index > self.len {
                        panic!("List index out of range");
                    } else if index < 0 {
                        self.len + index - 1
                    } else {
                        index
                    };
        self.head.remove(index_);
        self.len -= 1;
    }

    pub fn iter(&mut self) -> NodeIterator<T> {
        self.head.iter()
    }

    pub fn to_array(&mut self) -> Vec<&T> {
        let mut arr: Vec<&T> = Vec::with_capacity(self.len as usize);
        let mut cursor: &Node<T> = &self.head;
        let mut next_exist: bool = true;
        while next_exist {
            arr.push(&cursor.data);
            next_exist = match cursor.next {
                Some(ref next) => {
                    cursor = &next;
                    true
                }
                None => { false }
            }
        }
        arr
    }
}

impl<T> Node<T> {
    fn new(item: T) -> Node<T> {
        Node {
            next: None,
            data: item
        }
    }

    fn push(&mut self, item: T) {
        match self.next {
            Some(ref mut next) => return next.push(item),
            None => {}
        };
            self.next = Some(Box::new(Node::new(item)))
    }

    fn pop(&mut self) {
        match self.next {
            Some(ref mut first_next) => {
                match first_next.next {
                    Some(_) => { return first_next.pop() },
                    None => {}
                }
            },
            None => {}
        }
        self.next = None;
    }

    fn insert(&mut self, item: T, index: i32) {
        match self.next {
            Some(ref mut next) => {
                if index > 0 {
                    return next.insert(item, index - 1)
                }
            },
            None => {}
        };

        let chain_node: Option<Box<Node<T>>> = mem::replace(&mut self.next, None);
        let mut new_node: Box<Node<T>> = Box::new(Node::new(item));
        new_node.next = chain_node;
        self.next = Some(new_node);
    }

    fn remove(&mut self, index: i32) {
        match self.next {
            Some(ref mut next) => {
                if index > 0 {
                    return next.remove(index - 1);
                }
            },
            None => {}
        };

        self.next = match self.next {
            Some(ref mut target) => { mem::replace(&mut target.next, None) },
            None => { panic!("List index out of range.") }
        };
    }

    fn iter(&self) -> NodeIterator<T> {
        NodeIterator { slice: Some(self), index: 0 }
    }
}
