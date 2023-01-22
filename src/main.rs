use gomicollector::Heap;

fn main() {
    let heap_size = 4;
    let mut heap = Heap::<String>::new(heap_size);
    // allocate an object
    let obj1_id = heap.allocate("Obj1".to_string());
    let obj1 = heap.get(obj1_id.unwrap());
    println!("obj1 allocated {:?}", obj1);
    // root -> obj1
    heap.root = obj1_id;


    // allocate a lot of objects to check gc works well
    for i in 0..(heap_size) {
        println!("tmp{:?} will be allocated", i);
        let tmp = heap.allocate(format!("tmp{}", i));
        assert_ne!(obj1_id, tmp);
    }

    // allocate obj2 which is unreachable from the root set
    let obj2_id = heap.allocate("Obj2".to_string());
    let obj2 = heap.get(obj2_id.unwrap());
    println!("obj2 allocated: {:?}", obj2);

    // allocate a lot of objects and check gc collects obj2 memory
    for i in 0..(heap_size) {
        println!("tmp{:?} will be allocated", i);
        let _ = heap.allocate(format!("tmp{}", i));
    }

    // root -> obj1 -> obj3
    let obj3_id = heap.allocate("Obj3".to_string());
    let obj3 = heap.get(obj3_id.unwrap());
    println!("obj3 allocated: {:?}", obj3);
    heap.heap[obj1_id.unwrap()].set_head(obj3_id);
    // allocate a lot of objects and check gc does not collect obj3 memory
    for i in 0..(heap_size) {
        println!("tmp{:?} will be allocated", i);
        let _ = heap.allocate(format!("tmp{}", i));
    }
    heap.heap[obj1_id.unwrap()].set_data("changed data".to_string());

    // ojb1 and obj3 is still in the heap because the root points to it.
    println!("heap: {:#?}", heap);
}
