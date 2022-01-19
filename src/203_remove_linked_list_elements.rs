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

pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut dummy_h = Box::new(ListNode::new(0));
    dummy_h.next = head;
    let mut plist = &mut dummy_h;

    while let Some(node) = plist.next.take() {
        if node.val == val {
            plist.next = node.next;
        } else {
            plist.next = Some(node);
            plist = plist.next.as_mut().unwrap();
        }
    }
    dummy_h.next
}

#[cfg(test)]
#[test]
pub fn test_203() {
    let test_case_val1: i32 = 6;
    let mut test_case_list1: ListNode = ListNode::new(1);
    test_case_list1.append(2);
    test_case_list1.append(6);
    test_case_list1.append(3);
    test_case_list1.append(5);
    test_case_list1.append(6);

    let test_case_head1 = Some(Box::new(test_case_list1));
    let res = remove_elements(test_case_head1, test_case_val1);
    let dummylist = Some(Box::new(ListNode{val:0, next:res}));
    let mut pres = dummylist.as_ref();
    let mut valvec: Vec<i32> = Vec::new();

    while pres.unwrap().next.is_some() {
        let _node = pres.unwrap().next.as_ref();
        valvec.push(_node.unwrap().val);
        pres = pres.unwrap().next.as_ref();
    }
    let show = dummylist.unwrap().next;
    show.unwrap().show_list();

    assert_eq!(vec![1,2,3,5], valvec);
}


#[test]
pub fn test_203_2() {
    let test_case_val1: i32 = 6;

    let test_case_head1 = None;
    let res = remove_elements(test_case_head1, test_case_val1);
    let dummylist = Some(Box::new(ListNode{val:0, next:res}));
    let mut pres = dummylist.as_ref();
    let mut valvec: Vec<i32> = Vec::new();

    while pres.unwrap().next.is_some() {
        let _node = pres.unwrap().next.as_ref();
        valvec.push(_node.unwrap().val);
        pres = pres.unwrap().next.as_ref();
    }

    let show = dummylist.unwrap().next;
    match show {
        Some(show) => show.show_list(),
        None => println!("empty list"),
    }

    assert_eq!(0, valvec.len());
}
