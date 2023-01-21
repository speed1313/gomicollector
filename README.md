# gomicollector
gomicollector is a toy garbage collector.
gomicollector's gc algorithm is mark and sweep algorithm.

Since pointer operation is difficult in Rust, I implemented a gomicollector by thinking of the heap as a vector of object and the pointer as the index of the vector.


## How to use
1. initialize a heap.
```
let mut heap = Heap::<String>::new(4);
```
2. allocate an object to the heap. when you call heap.allocate(), gc collects usable memory with mark and sweep algorithm.
```
let obj1_id = heap.allocate("Obj1".to_string());
```
3 set the root object to point to the obj1
```
heap.root = obj1_id;
```
- set the head of obj1 to point to obj3
```
let obj3_id = heap.allocate("Obj3".to_string());
heap.heap[obj1_id.unwrap()].set_head(obj3_id);
```

###  example
root->obj1->obj3
obj2

```
$ git clone https://github.com/speed1313/gomicollector.git
$ cd gomicollector
$ cargo run
free list: []
mark and sweep
free list after obj1 allocated: [0, 1, 2]
obj1: Object { head: None, tail: None, marked: false, id: 3, data: Some("Obj1") }
free list after tmp0 allocated: [0, 1]
free list after tmp1 allocated: [0]
free list after tmp2 allocated: []
mark and sweep
free list after tmp3 allocated: [0, 1]
free list after obj2 allocated: [0]
obj2: Object { head: None, tail: None, marked: false, id: 1, data: Some("Obj2") }
free list after tmp0 allocated : []
mark and sweep
free list after tmp1 allocated : [0, 1]
free list after tmp2 allocated : [0]
free list after tmp3 allocated : []
mark and sweep
free list after obj3 allocated: [0, 1]
obj3: Object { head: None, tail: None, marked: false, id: 2, data: Some("Obj3") }
free list after tmp0 allocated : [0]
free list after tmp1 allocated : []
mark and sweep
free list after tmp2 allocated : [0]
free list after tmp3 allocated : []
heap: Heap {
    heap: [
        Object {
            head: None,
            tail: None,
            marked: false,
            id: 0,
            data: Some(
                "tmp3",
            ),
        },
        Object {
            head: None,
            tail: None,
            marked: false,
            id: 1,
            data: Some(
                "tmp2",
            ),
        },
        Object {
            head: None,
            tail: None,
            marked: true,
            id: 2,
            data: Some(
                "Obj3",
            ),
        },
        Object {
            head: Some(
                2,
            ),
            tail: None,
            marked: true,
            id: 3,
            data: Some(
                "Obj1",
            ),
        },
    ],
    root: Some(
        3,
    ),
    size: 4,
    free_list: [],
}
```

## Ref.

- https://github.com/munificent/mark-sweep
- https://github.com/jorendorff/gc-in-50-lines
- ガベージコレクション 自動的メモリ管理を構成する理論と実装, Richard Jones et. al.(著), 前田 敦司 et. al. (訳)
- Crafting Interpreters, Robert Nystrom, https://craftinginterpreters.com/garbage-collection.html