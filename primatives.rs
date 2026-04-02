
// Scalar Types// https://doc.rust-lang.org/stable/rust-by-example/primitives.html 

// Primitives
// Rust provides access to a wide variety of primitives. A sample includes:

// Scalar Types
// Signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
// Unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
// Floating point: f32, f64
// char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
// bool either true or false


// The unit type (), whose only possible value is an empty tuple: ()
// Despite the value of a unit type being a tuple, it is not considered a compound type because it does not contain multiple values.

// Compound Types
// Arrays like [1, 2, 3]
// Tuples like (1, true)

// Variables can always be type annotated. Numbers may additionally be annotated via a suffix or by default. Integers default to i32 and floats to f64. Note that Rust can also infer types from context.

fn main() {

    //variables can be type annotated
    let logical: bool = true; 

    let a_float: f64 = 1.0; //regular annotation 
    let an_integer = 5i32; // Suffix annotation

    //or a default notation will be used.
    let default_float = 3.0; //`f64` 
    let default_integer = 7; //`i32` 

    //A type can also be inferred from context. 
    let mut inferred_type = 12; //Type i64 is inferred from another line 
    inferred_type = 4294967296i64;

    // A mutable variable's value can be changed. 
    let mut mutable = 12; //Mutable `i32`
    mutable = 21; 

    //Error! the type of a variable can't be changed. 
    //mutable = true; 

    //variables can be overwritten with shadowing 
    let mutable = true;  

    /* Compound types - Array and tuple */ 

    // array signature consists of Type T and length as [T; length].
    let my_array: [i32; 5] = [1,2,3,4,5];

    //Tuple is a collection of values of different types 
    // and is constructured using parenthesis (). 
    let my_tuple = (5u32, 1u8, true, -504f32);



}

//OUTPUT 

/*

warning: unused variable: `logical`
  --> primatives.rs:27:9
   |
27 |     let logical: bool = true; 
   |         ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_logical`
   |
   = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `a_float`
  --> primatives.rs:29:9
   |
29 |     let a_float: f64 = 1.0; //regular annotation 
   |         ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_a_float`

warning: unused variable: `an_integer`
  --> primatives.rs:30:9
   |
30 |     let an_integer = 5i32; // Suffix annotation
   |         ^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_an_integer`

warning: unused variable: `default_float`
  --> primatives.rs:33:9
   |
33 |     let default_float = 3.0; //`f64` 
   |         ^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_default_float`

warning: unused variable: `default_integer`
  --> primatives.rs:34:9
   |
34 |     let default_integer = 7; //`i32` 
   |         ^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_default_integer`

warning: variable `inferred_type` is assigned to, but never used
  --> primatives.rs:37:9
   |
37 |     let mut inferred_type = 12; //Type i64 is inferred from anothe...
   |         ^^^^^^^^^^^^^^^^^
   |
   = note: consider using `_inferred_type` instead

warning: variable `mutable` is assigned to, but never used
  --> primatives.rs:41:9
   |
41 |     let mut mutable = 12; //Mutable `i32`
   |         ^^^^^^^^^^^
   |
   = note: consider using `_mutable` instead

warning: unused variable: `mutable`
  --> primatives.rs:48:9
   |
48 |     let mutable = true;  
   |         ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_mutable`

warning: unused variable: `my_array`
  --> primatives.rs:53:9
   |
53 |     let my_array: [i32; 5] = [1,2,3,4,5];
   |         ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_my_array`

warning: unused variable: `my_tuple`
  --> primatives.rs:57:9
   |
57 |     let my_tuple = (5u32, 1u8, true, -504f32);
   |         ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_my_tuple`

warning: value assigned to `inferred_type` is never read
  --> primatives.rs:38:5
   |
38 |     inferred_type = 4294967296i64;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: maybe it is overwritten before being read?
   = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default

warning: value assigned to `mutable` is never read
  --> primatives.rs:42:5
   |
42 |     mutable = 21; 
   |     ^^^^^^^^^^^^
   |
   = help: maybe it is overwritten before being read?

warning: 12 warnings emitted


*/ 