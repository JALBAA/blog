
#[derive(Debug)]
struct ListNode<T> {
    next: Option<Box<ListNode<T>>>,
    data: Option<T>,
}

impl<T> ListNode<T> {
    fn new (d: T) -> ListNode<T> {
        ListNode {
            next: None,
            data: Some(d),
        }
    }
}

fn main () {
    let mut l1 = ListNode::new(32);
    let l2 = ListNode::new(33);

    l1.next = Some(Box::new(l2));
    
    if let Some(l3) = &mut l1.next {
        l3.data = Some(55);
        println!("{:?}", l3);
    }

    println!("{:?}", l1);
}