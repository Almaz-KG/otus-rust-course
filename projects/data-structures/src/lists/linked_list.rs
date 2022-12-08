use std::rc::Rc;
use std::cell::RefCell;

type NodeOption<T> = Option<Rc<RefCell<T>>>;

struct Node<T> {
    value: T,
    next: NodeOption<T>
}


pub struct LinkedList<T> {
    head: NodeOption<T>,
    tail: NodeOption<T>,
    size: usize
}