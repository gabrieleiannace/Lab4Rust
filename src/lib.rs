pub mod list1 {
    use std::{mem};

    pub enum ListLink<T> {
        Cons(T, Box<ListLink<T>>),
        Nil,
    }

    //In pratica sullo stack c'è solo la head, tutto il resto è una chain che si trova sullo heap!
    pub struct List<T> {
        head: ListLink<T>,
    }

    impl<T> List<T> {
        pub fn new() -> Self {
            Self {
                head: ListLink::Nil,
            }
        }

        // insert a new element at the beginning of the list
        // you may encouter a problem with the borrow checker while trying to move self.head to a new variable
        // why? look at mem::replace for solving it

        pub fn push(&mut self, elem: T) {
            //Per ora self.head è temporaneamente nullo, il suo valore è trasferito in head
            let head = mem::replace(&mut self.head, ListLink::Nil);
            //Creazione nuovo nodo
            let node = ListLink::Cons(elem, Box::new(head));
            //Il nuovo nodo è la head
            self.head = node;
        }

        //Supposto la pop sia sempre dalla head, LIFO
        fn pop(&mut self) -> Option<T> {
            match mem::replace(&mut self.head, ListLink::Nil) {
                ListLink::Cons(e, next) => {
                    self.head = *next;
                    Some(e)
                }
                ListLink::Nil => None,
            }
        }

        // return a referece to the first element of the list
        pub fn peek(&self) -> Option<&T> {
            match &self.head {
                ListLink::Cons(e, _) => Some(e),
                ListLink::Nil => None,
            }
        }

        // uncomment after having implemented the ListIter struct
        // return an interator over the list values
        fn iter(&self) -> ListIter<T> {
            ListIter::new(&self)
        }

        // take the first n elements of the list and return a new list with them
        pub fn take(&mut self, n: usize) -> List<T> {
            let mut res = List::new();
            for _ in 0..n {
                if let Some(e) = self.pop() {
                    res.push(e);
                } else {
                    panic!("Internal error while taking")
                }
            }
            res
        }
    }

    struct ListIter<'a, T> {
        // implement the iterator trait for ListIter
        next: &'a ListLink<T>,
    }

    impl<'a, T> ListIter<'a, T> {
        pub fn new(list: &'a List<T>) -> Self {
            Self { next: &list.head }
        }
    }

    impl<'a, T> Iterator for ListIter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            match self.next {
                ListLink::Cons(e, next) => {
                    self.next = next;
                    Some(e)
                },
                ListLink::Nil => None,
            }
        }
    }
}

pub mod list2 {

    pub struct Node<T> {
        elem: T,
        next: NodeLink<T>,
    }

    type NodeLink<T> = Option<Box<Node<T>>>;

    pub struct List<T> {
        head: NodeLink<T>,
    }

    // for this implementattion, since we are using option, take a look at the take method in Option<T>.
    // It allows to move the value of the option into another option and replace it with None
    // let mut a = Some(5);
    // let b = a.take(); // a is now None and b is Some(5)
    impl<T> List<T> {
        // same methods as List1
        pub fn new() -> Self {
            Self {
                head: None,
            }
        }


        pub fn push(&mut self, elem: T) {            
            let node = Box::new(Node{ elem, next: self.head.take()});
            self.head = Some(node);
        }

        pub fn pop(&mut self) -> Option<T>{
            match self.head.take(){
                Some(head) => {
                    self.head = head.next;
                    Some(head.elem)                           
                },
                None => None,
            }                        
        }

        pub fn peek(&self) -> Option<&T> {            
            self.head.as_ref().map(|n| &n.elem)            
        }

        pub fn take(&mut self, n: usize) -> List<T> {
            let mut new_list = List::new();
            for _ in 0..n {
                if let Some(e) = self.pop(){new_list.push(e)} else {panic!("internal error")}
            }
            new_list
        }        
    }

    pub struct ListIter<'a, T>{
        next: &'a NodeLink<T>
    }

    impl<'a, T> ListIter<'a, T>{
        pub fn new(list: &'a List<T>) -> Self {
            Self{ next: &list.head }
        }
    }

    impl<'a, T> Iterator for ListIter<'a, T>{
        type Item = &'a T;
    
        fn next(&mut self) -> Option<Self::Item> {
            match self.next{
                Some(next_node) => {
                    self.next = &next_node.next;
                    Some(&next_node.elem)
                },
                None => None,
            }            
        }        
    }
}

// *****
// double linked list suggestion: use Rc, since we need more than one reference to the same node
// for mutating the list and changing the next and prev fields we also need to be able to mutate the node, therefore we can use RefCell

// how to access content of Rc<RefCell<T>>:
// es let a = Rc::new(RefCell::new(5));
// let mut x = (*a).borrow_mut();  // with (*a) we dereference the Rc, with (*a).borrow_mut() we get a mutable reference to the content of the RefCell
// *x = 6; // we can now change the content of the RefCell

// to take a value from a Rc (useful when popping a value from the list): usually it is not possible since it may be referenced elsewhere.
// if you can guarantee it's the only reference to the value  youu can use Rc::try_unwrap(a).unwrap().into_inner() to get the value
// it first takes out the value from the Rc, then it tries to unwrap the value from the Result, and finally it takes the inner value from the Result
// see here
// https://stackoverflow.com/questions/70404603/how-to-return-the-contents-of-an-rc

// other hint that may be useful: Option<T> has a default clone implementation which calls the clone of T. Therefore:
// Some(T).clone() ->  Some(T.clone())
// None.clone() -> None

//  type NodeLink = Option<Rc<RefCell<DNode>>>; // we define a type alias for better readibility
// Example
//  type NodeBackLink = ...
pub mod dlist{
    use core::panic;
    use std::{cell::{Ref, RefCell}, rc::{Rc, Weak}};

    pub struct DNode<T> {
        v: T,
        prev: NodeBackLink<T>, // here we can't put NodeLink to avoid a cycle reference, what do we use?
        next: NodeLink<T>
    }

    type NodeLink<T> = Option<Rc<RefCell<DNode<T>>>>;
    type NodeBackLink<T> = Option<Weak<RefCell<DNode<T>>>>;

    pub struct DList<T> {
        head: NodeLink<T>,
        tail: NodeLink<T>
    }    

    impl<T> DList<T>{
        pub fn new() -> Self{
            Self{ head: None, tail: None }
        }

        pub fn push_front(&mut self, elem: T){
            let old_head = self.head.take();
            let node = Rc::new(RefCell::new(DNode{ v: elem, prev: None, next: old_head.clone() }));
            match old_head{
                Some(head) => {
                    let mut head = (*head).borrow_mut();
                    head.prev = Some(Rc::downgrade(&node));
                },
                None => {                    
                    self.tail = Some(node.clone());
                },
            }
            self.head = Some(node);
        }

        pub fn pop_back(&mut self) -> Option<T>{
            let old_tail = self.tail.take();
            match old_tail{
                Some(tnode) => {
                    let prev = (*tnode).borrow().prev.clone();
                    match prev{
                        Some(pnode) => {
                            let pnode = pnode.upgrade().unwrap();                            
                            let mut mpnode = (*pnode).borrow_mut();
                            mpnode.next = None;
                            self.tail = Some(pnode.clone());
                        },
                        None => {self.head = None; self.tail = None},
                    }
                    let inner = Rc::try_unwrap(tnode).ok().unwrap().into_inner();
                    Some(inner.v)
                },
                None => None,
            }            
        }

        pub fn push_back(&mut self, elem: T){
            
            let old_tail = self.tail.take();            
            match old_tail{
                Some(tnode) => {
                    let node = Rc::new(RefCell::new(DNode{ v: elem, prev: Some(Rc::downgrade(&tnode.clone())), next: None }));
                    let mut tnode = (*tnode).borrow_mut();
                    tnode.next = Some(node.clone());
                    self.tail = Some(node.clone());
                },
                None => {
                    let node = Rc::new(RefCell::new(DNode{ v: elem, prev: None, next: None }));
                    self.head = Some(node.clone());
                    self.tail = Some(node.clone());
                },                
            }                        
        }

        pub fn pop_front(&mut self) -> Option<T>{
            let old_head = self.head.take();
            match old_head{
                Some(hnode) => {
                    let next = (*hnode).borrow().next.clone();
                    
                    match next{
                        Some(nnode) => {
                            let mut mut_nnode = (*nnode).borrow_mut();
                            mut_nnode.prev = None;                                                        
                            self.head = Some(nnode.clone());
                        },
                        None => {
                            self.head = None;
                            self.tail = None;                            
                        },
                    }
                    let inner = Rc::try_unwrap(hnode).ok().unwrap().into_inner();
                    return Some(inner.v);                                   
                },
                None => None,
            }            
        }

        pub fn take(&mut self, n: usize) -> DList<T> {
            let mut rlist = DList::new();
            for _ in 0..n{
                if let Some(e) = self.pop_front(){rlist.push_front(e)}
                else {panic!("Internal error!")}
            }
            rlist
        }

        // return a referece to the first element of the list
        pub fn peek(&self) -> Option<Ref<T>>{
            match &self.head{
                Some(head) => {
                    let head = (*head).borrow();
                    Some(Ref::map(head, |n| {
                        &n.v
                    }))                                      
                },
                None => None,
            }            
        }

        pub fn popn(&mut self, n: usize) -> Option<T>{
            if n == 1 { return self.pop_front()}
            
            let mut cnode = self.head.clone();
            for _ in 0..n {
                match cnode {
                    Some(node) => {
                        let next = (*node).borrow().next.clone();
                        cnode = next;
                    },
                    None => return None,
                }
            }

            // da cambiare con .and_then()
            if let Some(cnode) = cnode {
                let prev = (*cnode).borrow().prev.clone().unwrap().upgrade();
                let next = (*cnode).borrow().next.clone();

                match (prev, next){
                    (None, None) => {
                        self.head = None;
                        self.tail = None;                        
                    },

                    (None, Some(n)) => {
                        (*n).borrow_mut().prev = None;                    
                    },
                    
                    (Some(p), None) => {
                        (*p).borrow_mut().next = None;                                                
                    },

                    (Some(p), Some(n)) => {
                        (*p).borrow_mut().next = Some(n.clone());
                        (*n).borrow_mut().prev = Some(Rc::downgrade(&p));                        
                    },
                }                
                let p = Rc::try_unwrap(cnode).ok().unwrap().into_inner();
                Some(p.v)                
            }
            else{return self.pop_back()}                        
        }
    }

    pub struct DListIter<T>{
        next: NodeLink<T>
    }

    impl<T> DListIter<T>{
        pub fn new(list: &DList<T>) -> Self{
            Self { next:  list.head.clone() }
        }
    }

    impl<T> Iterator for DListIter<T>{
        type Item = Rc<RefCell<DNode<T>>>;
    
        fn next(&mut self) -> Option<Self::Item> {
            match self.next.clone(){
                Some(node) => {
                    self.next = (*node).borrow().next.clone();
                    Some(node)
                },
                None => None,
            }
        }
    }


    
    #[test]
    fn test_push_front() {
        let mut list: DList<i32> = DList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_push_back() {
        let mut list: DList<i32> = DList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn test_pop_front() {
        let mut list: DList<i32> = DList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_pop_back() {
        let mut list: DList<i32> = DList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    // #[test]
    // fn test_peek() {
    //     let mut list: DList<i32> = DList::new();
    //     list.push_back(1);
    //     list.push_back(2);
    //     list.push_back(3);

    //     assert_eq!(list.peek(), Some(&1));
    //     assert_eq!(list.pop_front(), Some(1));
    //     assert_eq!(list.peek(), Some(&2));
    //     assert_eq!(list.pop_front(), Some(2));
    //     assert_eq!(list.peek(), Some(&3));
    //     assert_eq!(list.pop_front(), Some(3));
    //     assert_eq!(list.peek(), None);
    // }

    #[test]
    fn test_take() {
        let mut list: DList<i32> = DList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let mut taken = list.take(2);
        assert_eq!(taken.pop_front(), Some(2));
        assert_eq!(taken.pop_front(), Some(2));
        assert_eq!(taken.pop_front(), None);

        // assert_eq!(list.pop_front(), Some(3));
        // assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_popn() {
        let mut list: DList<i32> = DList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        list.push_back(5);

        assert_eq!(list.popn(3), Some(3));
        assert_eq!(list.popn(2), Some(5));
        assert_eq!(list.popn(1), Some(4));
        assert_eq!(list.popn(1), None);
    }

}



    
