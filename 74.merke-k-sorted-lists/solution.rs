// 1. to implement a PRIORITY QUEUE, the BINARY HEAP data structure is chosen
use std::collections::BinaryHeap;

// 2. entire Box<ListNode> is stored in the BINARY HEAP, not just ListNode.val;
//    this requires implementation of Ord and PartialOrd for ListNode
use std::cmp::Ordering;

impl PartialOrd<ListNode> for ListNode
{
    fn partial_cmp(&self, other: &ListNode) -> Option<Ordering>
    {
        other.val.partial_cmp(&self.val)     // - for MIN HEAP
        //self.val.partial_cmp(&other.val)   // - for MAX HEAP
    }
}

impl Ord for ListNode
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        other.val.cmp(&self.val)             // - for MIN HEAP
        //self.val.cmp(&other.val)           // - for MAX HEAP
    }
}

impl Solution
{
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>>
    {
        // 3. maximum heap capacity is known in advance
        let mut heap: BinaryHeap<Box<ListNode>> = BinaryHeap::with_capacity(lists.len());

        // 4. initialize heap with first nodes from each ListNode
        for list in lists
        {
            match list
            {
                Some (node) => heap.push(node),
                None => {}
            }
        }

        // 5. initialize dummy node to reduce computations
        let mut dummy_node = Box::new(ListNode::new(0));

        // 6. to further build a list of nodes, we keep track of the current node
        let mut curr_node = &mut dummy_node;

        // 7. interaction with BINARY HEAP
        while let Some(node) = heap.pop()
        {
            // 8. get min value from the heap
            let mut new_node = Box::new(ListNode::new(node.val));
            curr_node.next = Some(new_node);
            curr_node = curr_node.next.as_mut().unwrap();

            // 9. push min.next value to heap
            if node.next.is_some()
            {
                heap.push(node.next.unwrap());
            }
        }

        return dummy_node.next;
    }
}
