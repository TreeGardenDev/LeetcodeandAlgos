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
pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

    let mut result=Some(Box::new(ListNode::new(0)));
    let mut p1=list1;
    let mut p2=list2;
    let mut p3=&mut result;
    while p1.is_some() && p2.is_some(){
        let p1_val=p1.as_ref().unwrap().val;
        let p2_val=p2.as_ref().unwrap().val;
        if p1_val<p2_val{
            p3.as_mut().unwrap().next=Some(Box::new(ListNode::new(p1_val)));
            p1=p1.unwrap().next;
        }else{
            p3.as_mut().unwrap().next=Some(Box::new(ListNode::new(p2_val)));
            p2=p2.unwrap().next;
        }
        p3=&mut p3.as_mut().unwrap().next;
    }
    if p1.is_some(){
        p3.as_mut().unwrap().next=p1;
    }
    if p2.is_some(){
        p3.as_mut().unwrap().next=p2;
    }
    result.unwrap().next
    
}



fn main() {
    //list1 = [1,2,4], list2 = [1,3,4]
    //
    let list11 = ListNode::new(4);
    let list12=ListNode{val:2,next:Some(Box::new(list11))};
    let list13=ListNode{val:1,next:Some(Box::new(list12))};

    let list21 = ListNode::new(4);
    let list22=ListNode{val:3,next:Some(Box::new(list21))};
    let list23=ListNode{val:1,next:Some(Box::new(list22))};
    let result=merge_two_lists(Some(Box::new(list13)),Some(Box::new(list23)));
    println!("{:?}",result);
        



}
