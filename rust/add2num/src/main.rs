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

fn main() {
     // create 2 linked lists l1 and l2 from the above implementation of ListNode with the values [2,4,3] and [5,6,4] respectively
    // add the two numbers and return the sum as a linked list
    // the sum should be [7,0,8]
    // explanation: 342 + 465 = 807
    let l11 = Some(Box::new(ListNode::new(3)));
    let l12 = Some(Box::new(ListNode{
        val: 4,
        next: l11,
    }));
    let l13 = Some(Box::new(ListNode{
        val: 2,
        next: l12,
    }));

    let l21 = Some(Box::new(ListNode::new(4)));
    let l22 = Some(Box::new(ListNode{
        val: 6,
        next: l21,
    }));
    let l23 = Some(Box::new(ListNode{
        val: 5,
        next: l22,
    }));

    //do above for lists with [2,4,9] and [5,6,4,9]

    let l31 = Some(Box::new(ListNode::new(9)));
    let l32 = Some(Box::new(ListNode{
        val: 4,
        next: l31,
    }));
    let l33 = Some(Box::new(ListNode{
        val: 2,
        next: l32,
    }));

    let l41 = Some(Box::new(ListNode::new(9)));
    let l42 = Some(Box::new(ListNode{
        val: 4,
        next: l41,
    }));
    let l43 = Some(Box::new(ListNode{
        val: 6,
        next: l42,
    }));
    let l44 = Some(Box::new(ListNode{
        val: 5,
        next: l43,
    }));

    //
    println!("{:?}", l13);
    println!("{:?}", l23);
    

    let sum = add_two_numbers(l33, l44);
    //

     // l1 = [2,4,3], l2 = [5,6,4]
}
pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    println!("l1: {:?}", l1);
    println!("l2: {:?}", l2);
    let rev_l1 = reverse_list(l1);
    println!("rev_l1: {:?}", rev_l1);
    let rev_l2 = reverse_list(l2);
    println!("rev_l2: {:?}", rev_l2);
    let int1= combine_integers(rev_l1);
    let int2= combine_integers(rev_l2);
    println!("int1: {}", int1);
    println!("int2: {}", int2);

    //[2,4,9]
    //[5,6,4,9]
    //9,4,2
    //9,4,6,5
    //[7,0,4,0,1]
    //let sum = int1 + int2;
    let sum = sum_list_num(int1, int2);
    println!("sum: {}", sum);

    let combined = build_list_from_int(sum);
    
    //
    println!("Combined: {:?}", combined);
    combined



}

pub fn sum_list_num(n1:i32, n2:i32) -> i32{
    //sums two numbers
    //for instance, 569 and 5649 should return 70401
    let mut n1 = n1;
    let mut n2 = n2;
    let mut result = 0;
    let mut multiplier = 1;
    loop{
        let digit1 = n1 % 10;
        let digit2 = n2 % 10;
        let sum = digit1 + digit2;
        result += sum * multiplier;
        multiplier *= 10;
        n1 = n1 / 10;
        n2 = n2 / 10;
        if n1 == 0 && n2 == 0{
        break;
        }

    }
    result
    
}
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut curr = head;
    while let Some(mut boxed_node) = curr {
        let next = boxed_node.next.take();
        boxed_node.next = prev;
        prev = Some(boxed_node);
        curr = next;
    }
    prev
}
pub fn combine_integers(head: Option<Box<ListNode>>) -> i32 {
    let mut curr = head;
    let mut result = 0;
    let mut multiplier = 1;
    while let Some(mut boxed_node) = curr {
        result += boxed_node.val * multiplier;
        multiplier *= 10;
        curr = boxed_node.next;
    }
    result
}
pub fn build_list_from_int(num:i32)-> Option<Box<ListNode>>{
    let mut num = num;
    //let mut curr = result;
    let mut num_storage = Vec::new();
    loop{
        let digit = num % 10;
        num_storage.push(digit);
        num = num / 10;
        if num == 0{
            break;
        }
    }
    let mut result : Option<Box<ListNode>> = None;
    for digit in num_storage.iter(){
        let mut new_node = ListNode::new(*digit);
        if let Some(mut boxed_node) = result {
            new_node.next = Some(boxed_node);
            result = Some(Box::new(new_node));
        } else {
           result = Some(Box::new(new_node));
        }
    }

    result
    
}
//pub fn combine_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//    let mut curr1 = l1;
//    let mut curr2 = l2;
//    let mut result:Option<Box<ListNode>> = None;
//    let mut carry = 0;
//    while let Some(mut boxed_node1) = curr1 {
//        let mut curr_result = result;
//        let mut sum = boxed_node1.val;
//        if let Some(mut boxed_node2) = curr2 {
//            sum += boxed_node2.val;
//            curr2 = boxed_node2.next;
//        }
//        sum += carry;
//        carry = sum / 10;
//        let mut new_node = ListNode::new(sum % 10);
//        if let Some(mut boxed_node) = curr_result {
//            boxed_node.next = Some(Box::new(new_node));
//            curr_result = boxed_node.next;
//        } else {
//            result = Some(Box::new(new_node));
//            curr_result = result;
//        }
//        curr1 = boxed_node1.next;
//    }
//    result
//}
