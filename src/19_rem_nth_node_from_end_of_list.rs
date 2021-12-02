// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}


impl ListNode {
    pub fn show_list(&self) {
        let mut node_num = 0;
        let mut cur = self.next.as_ref();
        println!("this ({})th node is {:?}", node_num, self.val);

        while cur.is_some() {
            node_num += 1;
            println!("this ({})th node is {:}",node_num, cur.unwrap().val);
            cur = cur.unwrap().next.as_ref();

        }
    }

    pub fn get_last (&mut self) -> &mut Self {
        if let Some(ref mut node) = self.next {
            node.get_last()
        } else {
            return self
        }
    }

    pub fn append(&mut self, value: i32) {
        let _node = ListNode::new(value);
        self.get_last().next = Some(Box::new(_node));
    }
}

pub fn remove_nth_from_end2(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy_head = Some(Box::new(ListNode {
        val: 0, next: head,
    }));
    let mut len = 0;
    {
        let mut p = dummy_head.as_ref();
        while p.unwrap().next.is_some() {
            len += 1;
            p = p.unwrap().next.as_ref();
        }
    }
    let idx = len - n;
    {
        let mut p = dummy_head.as_mut();
        for _ in 0..(idx) {
            p = p.unwrap().next.as_mut();
        }
        p.as_mut().unwrap().next = p.as_mut().unwrap().next.as_mut().unwrap().next.take();
        // let next = p.as_mut().unwrap().next.as_mut().unwrap().next.take();
        // p.as_mut().unwrap().next = next;
    }
    dummy_head.unwrap().next
}

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    unsafe {
        let mut head = head;
        let mut front: *mut Option<Box<ListNode>> = &mut head;
        let mut tail: *mut Option<Box<ListNode>> = &mut head;
        for _ in 0..n {
            front = &mut (*front).as_mut().unwrap().next;
        }
        if (*front).is_none() {
            return head.take().unwrap().next;
        }
        loop {
            front = &mut (*front).as_mut().unwrap().next;
            if (*front).is_none() {
                break;
            }
            tail = &mut (*tail).as_mut().unwrap().next;
        }
        (*tail).as_mut().unwrap().next = (*tail).as_mut().unwrap().next.as_mut().unwrap().next.take();
        head
    }
}


#[cfg(test)]
#[test]
pub fn test_19() {
    let mut list = ListNode::new(0);
    // list.append(1);
    for x in 1 .. 8 {
        list.append(x);
    }

    list.show_list();
    let head_node = Some(Box::new(list));
    let result = remove_nth_from_end(head_node, 3);
    //let result = remove_nth_from_end2(head_node, 3);
    println!("show the result:");
    result.unwrap().show_list();
}
