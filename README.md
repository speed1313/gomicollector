# gomicollector
gomicollector is a simple mark-sweep garbage collector in Rust.

gomicollector collects garbage when the heap is full.

Since pointer operation is difficult in Rust, I implemented a gomicollector by thinking of the heap as a vector of object and the pointer as the index of the vector.


## How to use
1. Initialize a heap.
```Rust
let mut heap = Heap::<String>::new(4);
```

2. Allocate an object to the heap. when you call heap.allocate(), gc collects garabage if the heap is full and return allocated memory address.
```Rust
let obj1_id = heap.allocate("Obj1".to_string());
```

3. Set the root object to point to the obj1
```Rust
heap.root = obj1_id;
```

4. Set the head of obj1 to point to obj3
```Rust
let obj3_id = heap.allocate("Obj3".to_string());
heap.heap[obj1_id.unwrap()].set_head(obj3_id);
```

5. change the object data
```Rust
heap.heap[obj1_id.unwrap()].set_data("changed data".to_string());
```

###  Example
- Situation:
  - root->obj1->obj3 (reachable)
  - obj2 (unreachable or garbage, so it will be collected)

```
$ git clone https://github.com/speed1313/gomicollector.git
$ cd gomicollector
$ cargo run
mark and sweep
obj1 allocated Object { head: None, tail: None, marked: false, id: 3, data: Some("Obj1") }
tmp0 will be allocated
tmp1 will be allocated
tmp2 will be allocated
tmp3 will be allocated
mark and sweep
droped "tmp2"
droped "tmp1"
droped "tmp0"
obj2 allocated: Object { head: None, tail: None, marked: false, id: 1, data: Some("Obj2") }
tmp0 will be allocated
tmp1 will be allocated
mark and sweep
droped "tmp0"
droped "Obj2"
droped "tmp3"
tmp2 will be allocated
tmp3 will be allocated
mark and sweep
droped "tmp3"
droped "tmp2"
droped "tmp1"
obj3 allocated: Object { head: None, tail: None, marked: false, id: 2, data: Some("Obj3") }
tmp0 will be allocated
tmp1 will be allocated
tmp2 will be allocated
mark and sweep
droped "tmp1"
droped "tmp0"
tmp3 will be allocated
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
                "changed data",
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

## TODO
- [ ] allow allocated object to change its data, not via heap object.
- [ ] simulate stack machine with gomicollector


## Ref.
- https://speed1313.notion.site/Garbage-Collection-mark-sweep-b04f5cb763824b8b9cc3735c29fde545
- https://github.com/munificent/mark-sweep
- https://github.com/jorendorff/gc-in-50-lines
- ガベージコレクション 自動的メモリ管理を構成する理論と実装, Richard Jones et. al.(著), 前田 敦司 et. al. (訳)
- Crafting Interpreters, Robert Nystrom, https://craftinginterpreters.com/garbage-collection.html
