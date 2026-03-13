/*
Whenever you create a Rust program, there are two sort of memories that you have:
1. Stack 
2. Heap
*/

/*
| Heap                                          | Stack                                                |
|-----------------------------------------------|------------------------------------------------------|
| 1. Dynamically allocated at runtime           | 1. Static, allocated at compile time                 |
| 2. Much larger in size                        | 2. Smaller in size                                   |
| 3. Slow due to dynamic allocation/deallocation| 3. Faster                                            |
| 4. Used for dynamic and large data structures | 4. Used for small, fixed-size variables and function |
| (e.g., Vector, HashMap, Box)                  |    call information                                  |
*/

/*
Stored on the Stack:
1. Numbers - i32, u64, f32
2. Booleans - true, false
3. Fixed Sized Arrays = [i32;4]
4. Structs - { x: i32, y: i32 }
5. References

Stored on the Heap:
1. Strings
2. Vectors
3. HashMap
4. Large Arrays/Structs that can't fit in the Stack
*/

/*
Why are Strings stored on the heap?
1. They are large
2. Their size can change at runtime, and the size of a stack frame needs to be fixed
*/

/////////////////////////////////////////////////////////////////////////////////////////

/*
Three ways of doing Memory Management:
1. Garbage Collector: 
    i) Written by smart people
    ii) Usually no dangling pointers/memory issues
    iii) You cannot do manual memory management
    iv) Examples: Java, JavaScript

2. Manual:
    i) You allocate and deallocate memory yourself
    ii) Can lead to dangling pointers/memory issues
    iii) Learning curve is high since you have to do the manual memory management
    iv) Examples: C

3. The Rust Way:
    i) Rust has its own ownership model for memory management.
    ii) Makes it extremely safe to memory errors
*/