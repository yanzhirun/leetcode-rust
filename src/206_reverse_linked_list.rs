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

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut cur = head;
    let mut pre = None;

    while let Some(mut node) = cur {
        cur = node.next;
        node.next = pre;
        pre = Some(node);
    }

    pre
}

#[cfg(test)]
#[test]
pub fn test_206() {
    let mut list = ListNode::new(0);
    for x in 1 .. 10 {
        list.append(x);
    }

    list.show_list();

    let head = Some(Box::new(list));
    let result = reverse_list(head);

    println!("show the result:");
    result.unwrap().show_list();
}
