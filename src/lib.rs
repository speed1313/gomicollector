/*
// gc.cpp - A simplistic mark-and-sweep garbage collector in 50 lines of C++

struct Object {
    Object* head;
    Object* tail;
    bool marked;
};*/
#[derive(Debug,Clone, PartialEq, Eq)]
struct Object {
    head: Option<Box<Object>>,
    tail : Option<Box<Object>>,
    marked: bool,
}


impl Object{
    pub fn new(heap: &mut Heap,head : Option<Box<Object>>, tail: Option<Box<Object>>) -> Object{
        let obj = heap.allocate();
        assert!(obj.is_some());
        Object{
            head: head,
            tail: tail,
            marked: false,
        }
    }
    pub fn set_head(&mut self, head: Option<Box<Object>>){
        self.head = head;
    }
    pub fn set_tail(&mut self, tail: Option<Box<Object>>){
        self.tail = tail;
    }
}

#[derive(Debug, Clone)]
struct Heap{
    heap: Vec<Object>, // heap is a vector of objects
    root: Option<Box<Object>>, // root of the heap
    size: usize, // size of the heap
    free_list: Vec<Object>, // free list contains the objects that are unreachable
}

impl Heap{
    pub fn new(heap_size: usize) -> Heap{
        Heap{
            heap: vec!(Object { head: None, tail: None, marked: false }; heap_size),
            root: None,
            size: heap_size,
            free_list: vec!(),
        }
    }
    fn add_to_free_list(&mut self, p: & Object){
        self.free_list.push(p.clone());
    }

    // mark all reachable objects recursively
    fn mark(&self, p: &mut Option<Box<Object>>){
        match p {
            Some(p) => {
                if p.marked {
                    return;
                }
                p.marked = true;
                self.mark(&mut p.head);
                self.mark(&mut p.tail);
            }
            None => {
                return;
            }
        }
    }

    fn allocate(&mut self) -> Option<Box<Object>>{
        if self.free_list.is_empty(){ // out of memory, do GC
            // 1. clear mark bits
            for i in 0..self.size{
                self.heap[i].marked = false;
            }
            // 2. mark phase
            self.mark(&mut self.root.clone());

             // 3. sweep phase
            self.free_list = vec!();
            for i in 0..self.size{
                if !self.heap[i].marked{
                    self.add_to_free_list(&self.heap[i].clone());
                }
            }
            if self.free_list.is_empty(){
                return None; // still out of memory
            }
        }
        let p = self.free_list.pop();
        match p {
            Some(mut p) => {
                p.head = None;
                p.tail = None;
                return Some(Box::new(p));
            }
            None => {
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_can_allocate_twice(){
        let heap_size = 10000;
        let mut heap = Heap::new(heap_size);
        let obj1 = heap.allocate();
        assert!(obj1.is_some());
        heap.root = obj1.clone();
        let obj2 = heap.allocate();
        assert!(obj2.is_some());
        assert!(std::ptr::eq(&obj1, &obj2) == false);
    }
    #[test]
    fn test_root_is_not_recycled() {
        let heap_size = 10000;
        let mut heap = Heap::new(heap_size);
        heap.root = heap.allocate();
        assert!(heap.root.clone().is_some());

        for _ in 0..(heap_size*2) {
            let tmp = heap.allocate();
            assert!(std::ptr::eq(&heap.root, &tmp) == false);
        }
    }

    #[test]
    fn test_full_heap() {
        let heap_size = 10;
        let mut heap = Heap::new(heap_size);
        for _ in 0..heap_size {
            let mut obj = heap.allocate();
            assert!(obj.is_some());
            obj.as_mut().expect("").set_head(None);
            obj.as_mut().expect("obj is none").set_tail(heap.root);
            heap.root = obj;
        }
        dbg!(heap.clone());
        for _ in 0..4{
            let obj = heap.allocate();
            dbg!(heap.clone());
            assert!(obj.is_none());
        }
    }



}
/*
const int HEAP_SIZE = 10000;
Object heap[HEAP_SIZE];
Object* root = nullptr;   // compile with -std=c++11 to get 'nullptr'
Object* free_list = nullptr;
void add_to_free_list(Object* p) {
    p->tail = free_list;
    free_list = p;
}
void init_heap() {
    for (int i = 0; i < HEAP_SIZE; i++)
        add_to_free_list(&heap[i]);
}
void mark(Object* p) {  // set the mark bit on p and all its descendants
    if (p == nullptr || p->marked)
        return;
    p->marked = true;
    mark(p->head);
    mark(p->tail);
}

Object* allocate() {
    if (free_list == nullptr) {      // out of memory, do GC
        for (int i = 0; i < HEAP_SIZE; i++)  // 1. clear mark bits
            heap[i].marked = false;
        mark(root);                          // 2. mark phase
        free_list = nullptr;                 // 3. sweep phase
        for (int i = 0; i < HEAP_SIZE; i++)
            if (!heap[i].marked)
                add_to_free_list(&heap[i]);
        if (free_list == nullptr)
            return nullptr;          // still out of memory :(
    }
    Object* p = free_list;
    free_list = free_list->tail;
    p->head = nullptr;
    p->tail = nullptr;
    return p;
}
 */