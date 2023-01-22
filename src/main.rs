use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

use gomicollector::Heap;

fn main() {
    let heap_size = 4;
    let mut heap = RefCell::new(Heap::<String>::new(heap_size));
    // allocate an object
    let obj1_id = heap.borrow_mut().allocate("Obj1".to_string());

    //let obj1 = heap.borrow_mut().get(obj1_id.unwrap());
    //println!("obj1 allocated {:?}", obj1);
    // root -> obj1
    heap.borrow_mut().root = obj1_id;

    // allocate a lot of objects to check gc works well
    for i in 0..(heap_size) {
        println!("tmp{:?} will be allocated", i);
        let tmp = heap.borrow_mut().allocate(format!("tmp{}", i));
        assert_ne!(obj1_id, tmp);
    }

    // allocate obj2 which is unreachable from the root set
    let obj2_id = heap.borrow_mut().allocate("Obj2".to_string());
    //let obj2 = heap.borrow_mut().heap[obj2_id.unwrap()].borrow_mut();
    //println!("obj2 allocated: {:?}", obj2);

    // allocate a lot of objects and check gc collects obj2 memory
    for i in 0..(heap_size) {
        println!("tmp{:?} will be allocated", i);
        let _ = heap.borrow_mut().allocate(format!("tmp{}", i));
    }

    // root -> obj1 -> obj3
    let obj3_id = heap.borrow_mut().allocate("Obj3".to_string());
    //let obj3 = heap.get(obj3_id.unwrap());
    //println!("obj3 allocated: {:?}", obj3);
    heap.borrow().heap[obj1_id.unwrap()]
        .borrow_mut()
        .set_head(obj3_id);
    // allocate a lot of objects and check gc does not collect obj3 memory
    for i in 0..(heap_size) {
        println!("tmp{:?} will be allocated", i);
        let _ = heap.borrow_mut().allocate(format!("tmp{}", i));
    }
    heap.borrow().heap[obj1_id.unwrap()]
        .borrow_mut()
        .set_data("changed data".to_string());

    // ojb1 and obj3 is still in the heap because the root points to it.
    println!("heap: {:#?}", heap);
}
