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

    fn add_free_region(&self, heap_start: usize, heap_size: usize) {
        todo!()
    }
}