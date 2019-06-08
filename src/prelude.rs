#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
#[macro_export]
macro_rules! linked_list {
    () => {
        None
    };
    ($($e:expr), *) => {
        {
            let mut head = Box::new(ListNode::new(0));
            let mut ref_head = &mut head;

            $(
            ref_head.next = Some(Box::new(ListNode::new($e)));
            ref_head = ref_head.next.as_mut().unwrap();
            )*

            let _ = ref_head;
            head.next
        }
    };
}
#[macro_export]
macro_rules! string_vec {
    ($($x:expr),*) => (
        vec![$($x.to_owned()),*]
    );
}

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[macro_export]
macro_rules! btree {
    () => {
        None
    };
    ($($x:expr), *) => {
        {
            use std::rc::Rc;
            use std::cell::RefCell;

            let items = vec![$(stringify!($x)), *];
            let items = items.iter().map(|n| n.parse::<i32>().ok()).collect::<Vec<_>>();
            let root = Some(Rc::new(RefCell::new(TreeNode::new(items[0].unwrap()))));
            let mut nodes = std::collections::VecDeque::new();
            nodes.push_back(root.as_ref().unwrap().clone());
            for pair in items[1..].chunks(2) {
                let node = nodes.pop_front().unwrap();
                if let Some(&Some(val)) = pair.get(0) {
                    node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                    nodes.push_back(node.borrow().left.as_ref().unwrap().clone());
                }
                if let Some(&Some(val)) = pair.get(1) {
                    node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                    nodes.push_back(node.borrow().right.as_ref().unwrap().clone());
                }
            }
            root
        }
    };
}
