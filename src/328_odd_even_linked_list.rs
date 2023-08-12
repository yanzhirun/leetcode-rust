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
pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy1 = Some(Box::new(ListNode::new(-1)));
    let mut dummy2: Option<Box<ListNode>> = Some(Box::new(ListNode::new(-1)));

    let mut p_dummy1 = &mut dummy1;
    let mut p_dummy2 = &mut dummy2;

    let mut cur = head;
    let mut is_odd = true;

    while let Some(mut node) = cur {
        cur = node.next.take();
        if is_odd {
            p_dummy1.as_mut().unwrap().next = Some(node);
            p_dummy1 = &mut p_dummy1.as_mut().unwrap().next;
        } else {
            p_dummy2.as_mut().unwrap().next = Some(node);
            p_dummy2 = &mut p_dummy2.as_mut().unwrap().next;
        }
        is_odd = !is_odd;
    }
    if let Some(node) = dummy2.as_mut().unwrap().next.take() {
        p_dummy1.as_mut().unwrap().next = Some(node);
    }

    dummy1.unwrap().next
}

#[cfg(test)]
#[test]
pub fn test_328() {
    let input_v: Vec<i32> = vec![1, 2, 3, 4, 5];

    let mut head = Some(Box::new(ListNode::new(input_v[0])));
    let mut p_head = &mut head;
    for idx in 1..input_v.len() {
        if let Some(node) = p_head {
            node.next = Some(Box::new(ListNode::new(input_v[idx])));
            p_head = &mut node.next;
        }
    }

    let mut v_t: Vec<i32> = Vec::new();

    let mut h = head.as_ref();
    while let Some(node) = h {
        h = node.next.as_ref();
        v_t.push(node.val);
    }

    // let mut n_v: Vec<i32> = Vec::new();
    // let mut p_head = head.clone();
    // while let Some(mut node) = p_head {
    //     p_head = node.next.take();
    //     n_v.push(node.val);
    // }

    // let mut n_v2 = Vec::new();
    // let mut ss = head;
    // while let Some(node) = ss {
    //     ss = node.next;
    //     n_v2.push(node.val);
    // }

    println!("get new from & vec {:?}", v_t);

    let mut res = odd_even_list(head);
    let mut output_v: Vec<i32> = Vec::new();

    while let Some(x) = res {
        res = x.next;
        output_v.push(x.val)
    }

    println!("output is {:?}", output_v);

    assert_eq!(vec![1, 3, 5, 2, 4], output_v);
}
