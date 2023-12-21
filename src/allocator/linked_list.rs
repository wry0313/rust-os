use core::mem;

use crate::allocator::align_up;

struct ListNode {
    size: usize,
    next: Option<&'static mut ListNode>, // &'static mut describes an owned object behind a pointer basically a box without a free at th end of the scope
}

impl ListNode {
    const fn new(size: usize) -> Self {
        ListNode { size, next: None }
    }
    fn start_addr(&self) -> usize {
        self as *const Self as usize
    }
    fn end_addr(&self) -> usize {
        self.start_addr() + self.size
    }
}

pub struct LinkedListAllocator {
    head: ListNode,
}

impl LinkedListAllocator {
    pub const fn new() -> Self {
        Self {
            head:ListNode::new(0),
        }
    }

    pub unsafe fn init(&mut self, heap_start:usize, heap_size: usize) {
        self.add_free_region(heap_start, heap_size);
    }
    /// adds the given memory to the front of the list
    unsafe fn add_free_region(&mut self, addr: usize, size: usize) {
        // ensure that the freed region is capable of holding a ListNode
        assert_eq!(align_up(addr, mem::align_of::<ListNode>()), addr);
        assert!(size >= mem::size_of::<ListNode>());

        let mut node = ListNode::new(size);
        node.next = self.head.next.take();
        let node_ptr = addr as *mut ListNode;
        node_ptr.write(node);
        self.head.next = Some(&mut *node_ptr);
    }

    fn find_region(&mut self, size: usize, align:usize) -> Option<(&'static mut ListNode, usize)> {

        let mut current = &mut self.head;

        while let Some(ref mut region) = current.next {
            if let Ok(alloc_start) = Self::alloc_from_region(&region, size, align) {
                // region suitable for allocation -> remove the node from list
                let next = region.next.take();
                let ret = Some((current.next.take().unwrap(), alloc_start));
                current.next = next;
                return ret;
            } else {
                current = current.next.as_mut().unwrap();
            }
        }

        None
    }
    
    fn alloc_from_region(region: &&mut &mut ListNode, size: usize, align: usize) -> Result<usize,()> {
        let alloc_start = align_up(region.start_addr(), align);
        let alloc_end = alloc_start.checked_add(size).ok_or(())?;

        if alloc_end > region.end_addr() {
            // region too small
            return Err(());
        }

        let excess_size = region.end_addr() - alloc_end;

        if excess_size > 0 && excess_size < mem::size_of::<ListNode>() {
            // rest of the region is too small to hold a ListNode
            return Err(());
        }

        Ok(alloc_start)
    }

    
}