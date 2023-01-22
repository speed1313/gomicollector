/// GomiCollector
/// A simple mark and sweep garbage collector
///
///

/// Object in the heap
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Object<T> {
    head: Option<usize>,
    tail: Option<usize>,
    marked: bool,
    id: usize,
    data: Option<T>,
}

impl<T> Object<T> {
    /// set head to point to another object
    pub fn set_head(&mut self, head: Option<usize>) {
        self.head = head;
    }
    /// set tail to point to another object
    pub fn set_tail(&mut self, tail: Option<usize>) {
        self.tail = tail;
    }
    /// get the id, or the address of the heap
    pub fn get_id(&self) -> usize {
        self.id
    }
    pub fn set_data(&mut self, data: T) {
        self.data = Some(data);
    }
}

/// Heap has a vector of objects. the elements is either None or Some(Object)
#[derive(Debug, Clone)]
pub struct Heap<T> {
    pub heap: Vec<Object<T>>,  // heap is a vector of objects
    pub root: Option<usize>,   // root of the heap
    size: usize,               // size of the heap
    pub free_list: Vec<usize>, // free list contains the objects that are unreachable
}

impl<T> Heap<T> {
    pub fn new(heap_size: usize) -> Heap<T> {
        let heap = (0..heap_size)
            .map(|i| Object {
                head: None,
                tail: None,
                marked: false,
                id: i,
                data: None,
            })
            .collect::<Vec<_>>();
        Heap {
            heap: heap,
            root: None,
            size: heap_size,
            free_list: vec![],
        }
    }

    /// get the object at the given id
    pub fn get(&self, id: usize) -> &Object<T> {
        &self.heap[id]
    }
    /// add unreachable object to the free list
    fn add_to_free_list(&mut self, id: usize) {
        self.free_list.push(id);
    }

    /// mark all reachable objects recursively
    fn mark(&mut self, p: &mut Option<usize>) {
        match p {
            Some(p) => {
                if self.heap[*p].marked {
                    return;
                }
                self.heap[*p].marked = true;
                match self.heap[*p].head {
                    Some(head) => {
                        self.mark(&mut Some(head));
                    }
                    None => {}
                }
                match self.heap[*p].tail {
                    Some(tail) => {
                        self.mark(&mut Some(tail));
                    }
                    None => {}
                }
            }
            None => {
                return;
            }
        }
    }

    /// allocate a new object to the heap if there is space
    pub fn allocate(&mut self, data: T) -> Option<usize> {
        if self.free_list.is_empty() {
            // out of memory, do GC
            println!("mark and sweep");
            // 1. clear mark bits
            for i in 0..self.size {
                self.heap[i].marked = false;
            }
            // 2. mark phase
            self.mark(&mut self.root.clone());
            // 3. sweep phase
            self.free_list = vec![];
            for i in 0..self.size {
                if !self.heap[i].marked {
                    self.add_to_free_list(i);
                }
            }
            if self.free_list.is_empty() {
                return None; // still out of memory
            }
        }
        let p = self.free_list.pop();
        match p {
            Some(p) => {
                self.heap[p].data = Some(data);
                Some(p)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_can_allocate_twice() {
        let heap_size = 10;
        let mut heap = Heap::<String>::new(heap_size);
        let obj1 = heap.allocate("obj1".to_string());
        assert!(obj1.is_some());
        let obj1_id = obj1.unwrap();
        heap.root = Some(obj1_id);
        let obj2 = heap.allocate("obj2".to_string());
        assert!(obj2.is_some());
        assert_ne!(obj1_id, obj2.unwrap());
        dbg!(heap);
    }
    #[test]
    fn test_root_is_not_recycled() {
        let heap_size = 10;
        let mut heap = Heap::<String>::new(heap_size);
        heap.root = heap.allocate("root".to_string());
        assert!(heap.root.clone().is_some());

        for _ in 0..(heap_size * 2) {
            let tmp = heap.allocate("tmp".to_string());
            assert_ne!(heap.root, tmp);
        }
        dbg!(heap);
    }

    #[test]
    fn test_full_heap() {
        let heap_size = 10;
        let mut heap = Heap::<String>::new(heap_size);
        for _ in 0..heap_size {
            let root_id = heap.root.clone();
            let obj = heap.allocate("tmp".to_string());
            assert!(obj.is_some());
            match obj {
                Some(obj) => {
                    heap.heap[obj].set_head(None);
                    heap.heap[obj].set_tail(root_id);
                    heap.root = Some(obj);
                }
                None => {
                    assert!(false);
                }
            }
        }
        dbg!(heap.clone());
        for _ in 0..4 {
            let obj = heap.allocate("tmp".to_string());
            assert!(dbg!(obj).is_none());
        }
        dbg!(heap);
    }

    #[test]
    fn test_nearly_full_heap() {
        let heap_size = 10;
        let mut heap = Heap::<String>::new(heap_size);
        for _ in 0..(heap_size - 1) {
            let root_id = heap.root.clone();
            let obj = heap.allocate("tmp".to_string());
            assert!(obj.is_some());
            match obj {
                Some(obj) => {
                    heap.heap[obj].set_head(None);
                    heap.heap[obj].set_tail(root_id);
                    heap.root = Some(obj);
                }
                None => {
                    assert!(false);
                }
            }
        }
        let last = heap.allocate("last".to_string());
        assert!(last.is_some());
        for _ in 0..10 {
            let tmp = heap.allocate("tmp".to_string());
            assert_eq!(tmp, last);
        }
    }

    // force heap to be full using only root object
    // while heap is not full, allocating new object which points to root object
    fn force_gc(heap: &mut Heap<String>) {
        let origin_root = heap.root;
        // force heap to be full using only root object
        while let Some(obj) = heap.allocate("tmp".to_string()) {
            heap.heap[obj].set_head(heap.root);
            heap.root = Some(obj);
        }
        heap.root = origin_root;
        heap.allocate("tmp".to_string());
    }

    #[test]
    fn test_reachable_objects_not_collected() {
        let heap_size = 10;
        let mut heap = Heap::<String>::new(heap_size);
        let obj1 = heap.allocate("obj1".to_string());
        assert!(obj1.is_some());
        heap.root = obj1;
        let obj2 = heap.allocate("obj2".to_string());
        assert!(obj2.is_some());
        heap.heap[heap.root.unwrap()].set_head(Some(obj2.unwrap()));
        let obj3 = heap.allocate("obj3".to_string());
        assert!(obj3.is_some());
        heap.heap[heap.root.unwrap()].set_tail(Some(obj3.unwrap()));
        let obj4 = heap.allocate("obj4".to_string());
        let root_head = heap.heap[heap.root.unwrap()].head;
        heap.heap[root_head.unwrap()].set_head(Some(obj4.unwrap()));
        let obj5 = heap.allocate("obj5".to_string());
        heap.heap[root_head.unwrap()].set_tail(Some(obj5.unwrap()));
        dbg!(&heap);
        force_gc(&mut heap);
        assert_eq!(heap.root.unwrap(), obj1.unwrap());
        let root_id = heap.root.unwrap();
        assert_eq!(heap.heap[root_id].head.unwrap(), obj2.unwrap());
        assert_eq!(heap.heap[root_id].tail.unwrap(), obj3.unwrap());
        assert_eq!(
            heap.heap[heap.heap[root_id].head.unwrap()].head.unwrap(),
            obj4.unwrap()
        );
        assert_eq!(
            heap.heap[heap.heap[root_id].head.unwrap()].tail.unwrap(),
            obj5.unwrap()
        );
        dbg!(heap);
    }
}
