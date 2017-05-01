// My first rustlang project

// Rustlang primitive types
// boolean -- bool -- true, false
// numbers -- ix, -- x = 8, 16, 32, 64 (integers) [ux for unsigned]
// floating numbers -- fx -- x = 32, 64
// arrays -- [T;nbr] -- nbr is a compile time constant
// slices -- &[T] -- &a[..] --> a slice containing all the elements of the array
//					&a[1..5] --> from index 1 to index 4, 5 is exclusive
// slices need to work on borrowed references of the arrays
// tuples : (x, y, z)
// functions :   fn name(params) -> return type {}

// fn foo(x: i32) -> i32 { x }
// let x: fn(i32) -> i32 = foo;
// x is a ‘function pointer’ to a function that takes an i32 and returns an i32





// Testing out enums and structs and tuples
fn main() {

    // a normal struct
    // added the debug trait for allowing println!
    #[derive(Debug)]
    struct AnyStruct {
        elem1: i32,
        elem2: String,
        elem3: Vec<String>,
    };

    // a struct containing mutable references to some values
    // needs a mandatory lifetime declaration, so that the
    // structure doesn't outlive it's borrowed elements
    struct MutStruct<'a> {
        mut_ref1: &'a mut Vec<i32>,
        ref2: &'a i32,
    };


    // a tuple struct that has anonymous field names
    // adding the #[derive(Debug)] trait just for printing
    #[derive(Debug)]
    struct TupleStruct(i32, String, i32, String);

    // empty struct
    //    struct EmptyDefStruct {};

    // emptry struct
    //    struct EmptyStruct;

    // enum
    #[derive(Debug)]
    enum MyFirstEnum {
        // just one case without any parameters
        Option1,
        // case looking like a tuplestruct
        Option2(i32, String, u64),
        // a case representing something like a struct def
        // formatted by rustfmt
        Option3 { x: i32, y: i64, z: String, k: f32 },
        // looks like a function
        Option4(String),
    };

    // field accesses for structs, tuples and enums
    // tuple
    let tuple1: (i32, i32) = (5, 6);

    // tuples have 0 based indexes and are accessed like
    // members rather than subscripts
    let tuple_index1: i32 = tuple1.0;
    let tuple_index2: i32 = tuple1.1;

    println!("tuple: 0: {index0}, 1: {index1}",
             index1 = tuple_index2,
             index0 = tuple_index1);

    // field accesses for structs
    // Note: The binding of the structs can be mut, not the individual
    // fields of the struct
    // Note: `String` != `str`
    // String is a struct built using the `str` - builtin string type
    // so "String" is of `str` type, while "String".to_string() is of `String` type
    let struct_instance: AnyStruct = AnyStruct {
        elem1: 1,
        elem2: "String type".to_string(),
        elem3: vec!["String vec1".to_string()],
    };

    println!("AnyStruct = {:?}", struct_instance);

    // declaring a mutable vector for passing the reference to mut_struct_instance
    let mut vec_instance: Vec<i32> = vec![5, 6, 7];
    let ref2: i32 = 5;
    let mut_struct_instance: MutStruct = MutStruct {
        mut_ref1: &mut vec_instance,
        ref2: &ref2,
    };

    println!("mut_struct_instance : mut_ref1 = {:?}, ref2 = {}",
             mut_struct_instance.mut_ref1,
             mut_struct_instance.ref2);

    // tuple struct instance
    // initialization is different from a normal struct
    // they are initialized like calling a function
    let tuple_struct_inst: TupleStruct = TupleStruct(1, "2".to_string(), 4, "5".to_string());

    // needed to add #[derive(Debug)] on the TupleStruct definition for gettign
    // print formatting ability
    // else would have to define the method for it
    println!("TupleStruct : {:?}", tuple_struct_inst);

    // Enum access
    // enum access using `::` operator or a match statement
    let enum_option1: MyFirstEnum = MyFirstEnum::Option1;
    let enum_option2: MyFirstEnum = MyFirstEnum::Option2(1, "1".to_string(), 2);
    let enum_option3: MyFirstEnum = MyFirstEnum::Option3 {
        x: 1,
        y: 34,
        z: "Abc".to_string(),
        k: 2.34,
    };
    let enum_option4: MyFirstEnum = MyFirstEnum::Option4("String type".to_string());

    // A match is basically like a switch-case but more powerful, since it matches
    // patterns
    let enum_option: MyFirstEnum = match enum_option3 {
        MyFirstEnum::Option1 => {
            println!("Option1");
            enum_option1
        }
        MyFirstEnum::Option2(_, _, _) => {
            println!("Option2");
            enum_option2
        }
        // used a variable struct content
        // A struct can include .. to indicate that you want to use
        // a copy of some other struct for some of the values.
        MyFirstEnum::Option3 { .. } => {
            println!("Option3");
            enum_option3
        }
        MyFirstEnum::Option4(_) => {
            println!("Option4");
            enum_option4
        }
    };

    println!("value of enum_option = {:?}", enum_option);

    // The below line is now invalid since the ownership of enum_option3 has transferred
    // to the match body and the match body has ended
    // VERY POWERFUL CONCEPT
    //    println!("enum_option3 = {:?}", enum_option3);

    // "stting".to_string() is the same as String::from("strign")
    let string_from: String = String::from("Hellp String");

    println!("String = {:?}", string_from);

    // str is an unsized type, so when we declare a string literal
    // it has a type of &'static str
    // it is a borrowed slice with static lifetime (lifetime of the entire program)
    // so it doesn't get destroyed even after it goes out of scope
    // although it is not accessible though
    // str is basically a string slice
    {
        let test_string: &'static str = "String";
        println!("test_string={}", test_string);
    }

    // need turbofish `::<T>` operator for this type of
    // bindings
    let vector = Vec::<bool>::new();
    // else go with
    let vector: Vec<bool> = Vec::new();
}









// The Guessing game!

// the way to include external libraries as dependencies
// here rand is the external library
// extern crate rand;

// use is the analogy for import, to add to the namespace
// std is the main module, io is a submodule, and stdin is a method defined inside io and so on.

// The prelude is the list of things that Rust automatically imports into every Rust program.
// prelude is kept at min

//use std::io;
//use std::cmp::Ordering;
//use rand::Rng;

//**
//A structure called Point
//it has the following members x and y each of i32 type
//*/
//struct Point {
//    x: i32,
//    y: i32,
//}
//
//
//
//fn main() {
//
//    //! The main()
//
//    // the entire structure is to be mutable, not the individual members
//    let mut b = Point { x: 5, y: 6 };
//
//    // when passing to println macro, the objects are passed as lifetime by
//    // default, it is the same as passing then as function<'a, 'b>(...)
//    // format.
//    println!("Point: x{}, y{}", b.x, b.y);
//
//    // b still in main scope, hence is still owned by main scope
//    // and since b is a mutable instance, we can modify the value
//    b.x = 4;
//    b.y = 8;
//
//    // this will print Point: x4, y8
//    println!("Point: x{}, y{}", b.x, b.y);
//
//    // passing a mutable reference to the function mod_point
//    // this way, I'm not handing over the ownership of the point b
//    mod_point(&mut b);
//
//    println!("Point: x{}, y{}", b.x, b.y);
//
//    let k: i32;
//    let kk: &i32;
//    {
//        k = 5;
//        kk = pikapika(&k);
//
//        println!("inside kk={}", kk);
//    }
//
//    println!("outside : k={} and kk={}", k, kk);
//}

/*
// Every reference(borrowed) in rustlang has a lifetime associated with it.
// In the function pikapika I declare a lifetime named `'a` since that is the syntax,
// now, each function has a input lifetime and output lifetime.

// Input lifetime:
//	k: &'a i32
// Output lifetime:
//	&'a i32
	
An input lifetime is a lifetime associated with a parameter of a function, 
and an output lifetime is a lifetime associated with the return value of a function.

Lifetimes are only used for borrowed values/references, not for ownership transfers.

Lifetime 3 rules for elision/ automatic lifetime inference by rustlang compiler

Here are the three rules:

* Each elided lifetime in a function’s arguments becomes a distinct lifetime parameter.

* If there is exactly one input lifetime, elided or not, that lifetime is assigned to all 
	elided lifetimes in the return values of that function.

* If there are multiple input lifetimes, but one of them is &self or &mut self, 
	the lifetime of self is assigned to all elided output lifetimes.

Otherwise, it is an error to elide an output lifetime.
*/
//fn pikapika<'a>(k: &'a i32) -> &'a i32 {
//
//    /// This function <'a> means that, the return value will have the same lifetime as the input value
//    /// lifetime being the same scope (lifetime is just the alternate name for named scopes)
//    return k;
//}



// mod_point<'b>(b: &'b mut Point) -> () {...}
// This function takes a mutable lifetime reference of point.
// This way, b maintains lifetime even after the program's control returns from the function mod_point
//fn mod_point<'b>(b: &'b mut Point) {
//
//    b.x = 90;
//    b.y = 90;
//
//    println!("Point: x{}, y{}", &b.x, &b.y);
//}



//fn main() {
//
//    let mut x: String = String::new();
//
//    std::io::stdin().read_line(&mut x).expect("Hey there this is wrong");
//
//    println!("String readIn = {}", x);
//
//}



//fn main() {
//
//    // generate a random number between 0 -100 (0 <= secret < 100)
//    let secret: u32 = rand::thread_rng().gen_range(0, 100);
//
//    println!("Hey there, enter your name:");
//
//    // By default, all the instances in Rust are immutable
//    // need to explicitly say `mut` to make the value mutable
//    // need to make the String mutable in order to update it with the value
//    // from the stdin.
//    let mut name = String::new();
//
//    io::stdin()
//        .read_line(&mut name)
//        .expect("PLease enter a valid name lol!");
//
//    if name.contains("Si") {
//
//        println!("So your name is {}?", &name);
//    } else {
//
//        println!("Nice name you got there!");
//    }
//
//    println!("{}, enter a number [0, 100):", name);
//
//    loop {
//
//        let mut number: String = String::new();
//
//        io::stdin()
//            .read_line(&mut number)
//            .expect("Please enter a valid whole number");
//
//        let number: u32 = match number.trim().parse() {
//
//            Ok(number) => number,
//            Err(_) => continue,
//        };
//
//        match number.cmp(&secret) {
//            Ordering::Equal => {
//                println!("Well done, {}. You won!", &name);
//                break;
//            }
//            Ordering::Less => println!("Guess higher, {}", &name),
//            Ordering::Greater => println!("Guess lower, {}", &name),
//        };
//    }
//
//}
