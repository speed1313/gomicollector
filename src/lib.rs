use std::{collections::HashSet, fmt::Debug};

/// GomiCollector
/// A simple mark and sweep garbage collector
///
///

/// Object in the heap
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Object<T: Debug + Clone> {
    head: Option<usize>,
    tail: Option<usize>,
    marked: bool,
    id: usize,
    data: Option<T>,
}

impl<T: Debug + Clone> Object<T> {
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
pub struct Heap<T: Debug + Clone> {
    pub heap: Vec<Object<T>>,     // heap is a vector of objects
    pub root_set: HashSet<usize>, // root of the heap
    size: usize,                  // size of the heap
    pub free_list: Vec<usize>,    // free list contains the objects that are unreachable
}

impl<T: Debug + Clone> Heap<T> {
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
            root_set: HashSet::new(),
            size: heap_size,
            free_list: vec![],
        }
    }

    /// get the object at the given id
    pub fn get(&self, id: usize) -> &Object<T> {
        &self.heap[id]
    }
    pub fn get_data(&self, id: usize) -> &T {
        self.heap[id].data.as_ref().unwrap()
    }
    /// add unreachable object to the free list
    fn add_to_free_list(&mut self, id: usize) {
        self.free_list.push(id);
    }

    /// mark all reachable objects recursively from p
    fn mark(&mut self, p: &usize) {
        if self.heap[*p].marked {
            return;
        }
        self.heap[*p].marked = true;
        if let Some(head) = self.heap[*p].head {
            self.mark(&head);
        }
        if let Some(tail) = self.heap[*p].tail {
            self.mark(&tail);
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
            for id in self.root_set.clone().iter() {
                self.mark(id);
            }
            // 3. sweep phase
            self.free_list = vec![];
            for i in 0..self.size {
                if !self.heap[i].marked {
                    if self.heap[i].data.is_some() {
                        println!("droped {:#?}", self.heap[i].data.clone().unwrap());
                    }
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

    /// force heap to be full using only root object while heap is not full, allocating new object which points to root object.
    /// tmp_data is just a tmp data to force gc.
    fn force_gc(&mut self, tmp_data: T) {
        let origin_root = *self.root_set.iter().next().unwrap();
        let mut tmp_root = origin_root;
        // force heap to be full using only root object
        while let Some(obj) = self.allocate(tmp_data.clone()) {
            self.heap[obj].set_head(Some(tmp_root));
            // swap tmp root
            self.root_set.remove(&tmp_root);
            self.root_set.insert(obj);
            tmp_root = obj;
        }
        // restore the original root
        self.root_set.remove(&tmp_root);
        self.root_set.insert(origin_root);

        self.allocate(tmp_data);
    }

    /// collect all reachable objects from the root set
    pub fn reachable_set(&mut self) -> HashSet<usize> {
        let mut reachable_set = HashSet::new();
        for i in 0..self.size {
            self.heap[i].marked = false;
        }
        for id in self.root_set.clone().iter() {
            reachable_set.insert(*id);
            reachable_set = reachable_set
                .union(&self.collect_reachable_set(*id))
                .map(|x| *x)
                .collect();
        }
        reachable_set
    }
    // collect all reachable objects from the given object
    fn collect_reachable_set(&mut self, id: usize) -> HashSet<usize> {
        let mut reachable_set = HashSet::new();
        if self.heap[id].marked {
            return reachable_set;
        }
        self.heap[id].marked = true;
        if let Some(head) = self.heap[id].head {
            reachable_set.insert(head);
            reachable_set = reachable_set
                .union(&self.collect_reachable_set(head))
                .map(|x| *x)
                .collect();
        }
        if let Some(tail) = self.heap[id].tail {
            reachable_set.insert(tail);
            reachable_set = reachable_set
                .union(&self.collect_reachable_set(tail))
                .map(|x| *x)
                .collect();
        }
        reachable_set
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
        heap.root_set.insert(obj1_id);
        let obj2 = heap.allocate("obj2".to_string());
        assert!(obj2.is_some());
        assert_ne!(obj1_id, obj2.unwrap());
        dbg!(heap);
    }
    #[test]
    fn test_root_is_not_recycled() {
        let heap_size = 10;
        let mut heap = Heap::<String>::new(heap_size);
        let root_id = heap.allocate("root".to_string());
        assert!(root_id.is_some());
        heap.root_set.insert(root_id.unwrap());
        for _ in 0..(heap_size * 2) {
            let tmp = heap.allocate("tmp".to_string());
            assert_ne!(root_id, tmp);
        }
        dbg!(heap);
    }

    #[test]
    fn test_full_heap() {
        let heap_size = 10;
        let mut heap = Heap::<String>::new(heap_size);
        let root_id = None;
        for _ in 0..heap_size {
            let mut root_id = root_id;
            let obj = heap.allocate("tmp".to_string());
            assert!(obj.is_some());
            match obj {
                Some(obj) => {
                    heap.heap[obj].set_head(None);
                    heap.heap[obj].set_tail(root_id);
                    // swap root
                    if let Some(root_id) = root_id {
                        heap.root_set.remove(&root_id);
                    }
                    root_id = Some(obj);
                    heap.root_set.insert(root_id.unwrap());
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
        let mut root_id = None;
        for _ in 0..(heap_size - 1) {
            let obj = heap.allocate("tmp".to_string());
            assert!(obj.is_some());
            match obj {
                Some(obj) => {
                    heap.heap[obj].set_head(None);
                    heap.heap[obj].set_tail(root_id);
                    // swap root
                    if let Some(root_id) = root_id {
                        heap.root_set.remove(&root_id);
                    }
                    root_id = Some(obj);
                    heap.root_set.insert(root_id.unwrap());
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

    #[test]
    fn test_reachable_objects_not_collected() {
        let heap_size = 10;
        let mut heap = Heap::<String>::new(heap_size);
        let obj1 = heap.allocate("obj1".to_string());
        assert!(obj1.is_some());
        heap.root_set.insert(obj1.unwrap());
        let obj2 = heap.allocate("obj2".to_string());
        assert!(obj2.is_some());
        heap.heap[obj1.unwrap()].set_head(Some(obj2.unwrap()));
        let obj3 = heap.allocate("obj3".to_string());
        assert!(obj3.is_some());
        heap.heap[obj1.unwrap()].set_tail(Some(obj3.unwrap()));
        let obj4 = heap.allocate("obj4".to_string());
        let root_head = heap.heap[obj1.unwrap()].head;
        heap.heap[root_head.unwrap()].set_head(Some(obj4.unwrap()));
        let obj5 = heap.allocate("obj5".to_string());
        heap.heap[root_head.unwrap()].set_tail(Some(obj5.unwrap()));
        dbg!(&heap);
        heap.force_gc("tmp".to_string());
        let root_id = *heap.root_set.iter().next().unwrap();
        assert_eq!(root_id, obj1.unwrap());
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

    #[test]
    /// root_set = {obj1, obj2}, obj1->obj3, obj2->obj4, obj5
    fn test_root_set() {
        let heap_size = 10;
        let mut heap = Heap::<String>::new(heap_size);
        let obj1 = heap.allocate("obj1".to_string());
        assert!(obj1.is_some());
        heap.root_set.insert(obj1.unwrap());
        let obj2 = heap.allocate("obj2".to_string());
        assert!(obj2.is_some());
        heap.root_set.insert(obj2.unwrap());
        let obj3 = heap.allocate("obj3".to_string());
        assert!(obj3.is_some());
        heap.heap[obj1.unwrap()].set_head(Some(obj3.unwrap()));
        let obj4 = heap.allocate("obj4".to_string());
        heap.heap[obj2.unwrap()].set_tail(Some(obj4.unwrap()));
        let obj5 = heap.allocate("obj5".to_string());
        dbg!(&heap);
        heap.force_gc("tmp".to_string());
        let root_set: Vec<usize> = heap.root_set.iter().cloned().collect();
        dbg!(&heap);
        assert!(root_set.contains(&obj1.unwrap()));
        assert!(root_set.contains(&obj2.unwrap()));

        assert_eq!(
            heap.heap[obj3.unwrap()].data.clone().unwrap(),
            "obj3".to_string()
        );
        assert_eq!(
            heap.heap[obj4.unwrap()].data.clone().unwrap(),
            "obj4".to_string()
        );
        assert_eq!(
            heap.heap[obj5.unwrap()].data.clone().unwrap(),
            "tmp".to_string()
        );
        dbg!(heap);
    }
}
