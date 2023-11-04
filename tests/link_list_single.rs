use rust_learn::link_list::single::*;

#[test]
fn test_single() {
    let mut head = LinkedList{
        head: Some(Box::new(Node{elem:1, next: None})),
    };
    head.head.unwrap().as_mut().next = Some(Box::new(
        Node{
            elem:2, 
            next: Some(Box::new(Node{elem:1, next: None}))
        }
    ));

    // let iter = head.iter_mut();
    
}