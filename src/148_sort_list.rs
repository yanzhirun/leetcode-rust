// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut v = Vec::new();
    let mut cur = head;

    while let Some(node) = cur {
        v.push(node.val);
        cur = node.next;
    }
    v.sort();
    let mut dummy = ListNode {
        val: -1,
        next: None,
    };
    let mut head = &mut dummy;
    for x in v {
        head.next = Some(Box::new(ListNode::new(x)));
        head = head.next.as_mut().unwrap();
    }
    dummy.next

    // let mut v = Vec::new();
    // let mut cur = head;

    // while let Some(node) = cur {
    //     v.push(node.val);
    //     cur = node.next;
    // }

    // if v.len() == 0 {
    //     return None;
    // }

    // v.sort();

    // let mut phead = Box::new(ListNode::new(v[0]));
    // let mut p = phead.as_mut();
    // for idx in 1..v.len() {
    //     p.next = Some(Box::new(ListNode::new(v[idx])));
    //     p = p.next.as_mut().unwrap();
    // }
    // return Some(phead);
}

#[cfg(test)]
#[test]
pub fn test_148() {
    let input = vec![4, 2, 1, 3];
    let mut dummy = ListNode {
        next: None,
        val: -1,
    };
    let mut cur = &mut dummy;

    for x in input {
        cur.next = Some(Box::new(ListNode::new(x)));
        cur = cur.next.as_mut().unwrap();
    }

    let res = sort_list(dummy.next);
    let mut resv = Vec::new();
    let mut cur = res;
    while let Some(node) = cur {
        resv.push(node.val);
        cur = node.next;
    }

    assert_eq!(resv, vec![1, 2, 3, 4]);
}
