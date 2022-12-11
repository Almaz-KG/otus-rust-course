use std::rc::Rc;
use std::cell::RefCell;

type NodeOption<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
struct Node<T>
    where T: Clone {
    value: T,
    next: NodeOption<T>,
}

impl<T: Clone> Node<T> {
    pub fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { value, next: None }))
    }
}

#[derive(Debug, Default)]
pub struct LinkedList<T: Clone> {
    head: NodeOption<T>,
    tail: NodeOption<T>,
    size: usize,
}

impl<T: Clone> LinkedList<T> {
    pub fn append(&mut self, value: T) {
        let new = Node::new(value);
        match self.head.as_ref() {
            None => {
                self.head = Some(new.clone());
            },
            Some(_) => {
                match self.tail.take() {
                    Some(old) => old.borrow_mut().next = Some(new.clone()),
                    None => self.head = Some(new.clone())
                }
            }
        }
        self.tail = Some(new);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self
            .head
            .take()
            .map(|head| {
                if let Some(next) = head.borrow_mut().next.take() {
                    self.head = Some(next);
                } else {
                    self.tail.take();
                }

                self.size -= 1;
                Rc::try_unwrap(head)
                    .ok()
                    .expect("Ooooops")
                    .into_inner()
                    .value
            })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_an_empty_list() {
        let an_empty_list: LinkedList<i32> = LinkedList::default();

        assert!(an_empty_list.head.is_none());
        assert!(an_empty_list.tail.is_none());
        assert_eq!(an_empty_list.size, 0)
    }

    #[test]
    fn add_one_element_to_empty_list() {
        let mut a_list: LinkedList<i32> = LinkedList::default();
        assert_eq!(a_list.size, 0);

        a_list.append(1);

        assert_eq!(a_list.size, 1);
        assert!(a_list.head.is_some());
        assert!(a_list.tail.is_some());
        assert_eq!(a_list.head, a_list.tail);
    }

    #[test]
    fn add_two_elements_to_empty_list() {
        let mut a_list: LinkedList<i32> = LinkedList::default();
        assert_eq!(a_list.size, 0);

        a_list.append(1);

        assert_eq!(a_list.size, 1);
        assert!(a_list.head.is_some());

        a_list.append(2);

        assert_eq!(a_list.size, 2);
        assert!(a_list.head.is_some());
        assert!(a_list.tail.is_some());
        assert_eq!(a_list.head.unwrap().borrow().next, a_list.tail);
    }

    #[test]
    fn pop_element_from_empty_list() {
        let mut an_empty_list: LinkedList<i32> = LinkedList::default();
        let pop_result = an_empty_list.pop();
        assert!(pop_result.is_none());
    }

    #[test]
    fn pop_element_from_one_element_list(){
        let mut list: LinkedList<i32> = LinkedList::default();
        list.append(1);

        let pop_result = list.pop();
        assert!(pop_result.is_some());
        assert!(list.head.is_none());
        assert!(list.tail.is_none());

        assert_eq!(pop_result.unwrap(), 1);
        assert_eq!(list.size, 0);
    }

    #[test]
    fn pop_element_from_two_element_list(){
        let mut list: LinkedList<i32> = LinkedList::default();
        list.append(2);
        list.append(1);

        let pop_result = list.pop();
        assert!(pop_result.is_some());
        assert_eq!(pop_result.unwrap(), 2);
        assert_eq!(list.size, 1);

        assert!(list.head.is_some());
        assert!(list.tail.is_some());
        assert_eq!(list.head, list.tail);

        let pop_result = list.pop();
        assert!(pop_result.is_some());
        assert_eq!(pop_result.unwrap(), 1);
        assert_eq!(list.size, 0);

        assert!(list.head.is_none());
        assert!(list.tail.is_none());
        assert_eq!(list.head, list.tail);
    }
}