use std::ops::Index;

use gc::Heap;

fn main() {
    let heap_size = 4;
    let mut heap = Heap::<String>::new(heap_size);
    println!("free list: {:?}", &heap.free_list);
    let obj1_id = heap.allocate("Obj1".to_string());
    println!("free list after obj1 allocated: {:?}", &heap.free_list);
    heap.root = obj1_id;
    let obj1 = heap.get(obj1_id.unwrap());
    println!("obj1: {:?}", obj1);

    for i in 0..(heap_size) {
        let tmp = heap.allocate(format!("tmp{}",i));
        println!("free list after tmp{:?} allocated: {:?}",i, &heap.free_list);
        assert_ne!(obj1_id, tmp);
    }

    let obj2_id = heap.allocate("Obj2".to_string());
    println!("free list after obj2 allocated: {:?}", &heap.free_list);
    let obj2 = heap.get(obj2_id.unwrap());
    println!("obj2: {:?}", obj2);

    for i in 0..(heap_size) {
        let _ = heap.allocate(format!("tmp{}",i));
        println!("free list after tmp{:?} allocated : {:?}",i, &heap.free_list);
    }

    let obj3_id = heap.allocate("Obj3".to_string());
    println!("free list after obj3 allocated: {:?}", &heap.free_list);
    let obj3 = heap.get(obj3_id.unwrap());
    println!("obj3: {:?}", obj3);
    heap.heap[obj1_id.unwrap()].set_head(obj3_id);

    for i in 0..(heap_size) {
        let _ = heap.allocate(format!("tmp{}",i));
        println!("free list after tmp{:?} allocated : {:?}",i, &heap.free_list);
    }
    // ojb1 and obj3 is still in the heap because the root points to it.
    println!("heap: {:#?}", heap);
}
