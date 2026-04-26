type Link<T> = Option<Box<Node<T>>>;

struct List<T> {
    size: usize,
    head: Link<T>,
}

struct Node<T> {
    elem: T,
    next: Link<T>
}

impl<T> List<T> {
    fn new() -> Self {
        Self {
            size: 0,
            head: None
        }
    }

    fn len(&self) -> usize { self.size }

    fn is_empty(&self) -> bool { 0 == self.size }

    fn clear(&mut self) {
        self.size = 0;
        self.head = None;
    }

    // add a new node ahead of the head node
    fn push(&mut self, elem: T) {
        let node = Box::new(Node {
            elem: elem,
            next: self.head.take()
        });
        self.head = Some(node);
        self.size += 1;
    }

    // take will move data out from node and left a vaccum
    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.elem
        })
    }

    // peek get a unmutable reference
    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    // peek_mut gets a mutable reference
    fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_deref() }
    }

    fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { next: self.head.as_deref_mut() }
    }
}

// Implementation of three iterations
struct IntoIter<T>(List<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // (List<T>) tuple's 0th item
        self.0.pop()
    }
}

struct Iter<'a, T: 'a> { next: Option<&'a Node<T>> }
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

struct IterMut<'a, T: 'a> { next: Option<&'a mut Node<T>> }
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

// custom implementation of Drop for the linked list
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(mut node) = link {
            link = node.next.take();
        }
    }
}

// fn main() {
//     basic_test();
//     into_iter_test();
//     iter_test();
//     iter_mut_test();

//     fn basic_test() {
//         let mut list = List::new();
//         list.push(1); list.push(2); list.push(3);

//         assert_eq!(list.len(), 3);
//         assert_eq!(list.is_empty(), false);
//         assert_eq!(list.pop(), Some(3));
//         assert_eq!(list.peek(), Some(&2));
//         assert_eq!(list.peek_mut(), Some(&mut 2));

//         list.peek_mut().map(|val| {
//             *val = 4;
//         });

//         assert_eq!(list.peek(), Some(&4));
//         list.clear();
//         println!("basics test Ok!");
//     }

//     fn into_iter_test() {
//         let mut list = List::new();
//         list.push(1); list.push(2); list.push(3);

//         let mut iter = list.into_iter();
//         assert_eq!(iter.next(), Some(3));
//         assert_eq!(iter.next(), Some(2));
//         assert_eq!(iter.next(), Some(1));
//         assert_eq!(iter.next(), None);

//         println!("into_iter test Ok!");
//     }

//     fn iter_test() {
//         let mut list = List::new();
//         list.push(1); list.push(2); list.push(3);

//         let mut iter = list.iter();
//         assert_eq!(iter.next(), Some(&3));
//         assert_eq!(iter.next(), Some(&2));
//         assert_eq!(iter.next(), Some(&1));
//         assert_eq!(iter.next(), None);
//         println!("iter test Ok!");
//     }

//     fn iter_mut_test() {
//         let mut list = List::new();
//         list.push(1); list.push(2); list.push(3);

//         let mut iter = list.iter_mut();
//         assert_eq!(iter.next(), Some(&mut 3));
//         assert_eq!(iter.next(), Some(&mut 2));
//         assert_eq!(iter.next(), Some(&mut 1));
//         assert_eq!(iter.next(), None);
//         println!("iter_mut test Ok!");
//     }
// }