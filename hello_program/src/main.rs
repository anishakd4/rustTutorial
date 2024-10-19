const GLOBAL_CONST: u8 = 100;  //warning: constant `global_const` should have an upper case name. Also in case of constant
// we have to declare the type of data type. Type inference doesn't works in in case of constants.

fn main() {
    println!("Integer data types");
    integer_data_types();
    println!("");

    println!("immutable variable");
    immutable_data_type();
    println!("");

    println!("mutable variable");
    mutable_data_type();
    println!("");

    println!("str data type");
    str_data_type();
    println!("");

    println!("String data type");
    string_data_type();
    println!("");

    println!("tuple data type");
    tuple_data_type();
    println!("");

    println!("destructuring");
    destructuring();
    println!("");

    println!("functions");
    functions_overview();
    println!("");

    println!("Scoping overview");
    scoping_overview();
    print_global_constant();
    println!("");

    println!("Scoping overview");
    stack_operation();
    println!("");

    println!("ownership overview");
    ownership_overview();
    println!("");

    println!("process integer");
    process_integer_main();
    println!("");

    println!("ownership functions");
    ownership_functions();
    println!("");

    println!("ownership excercise");
    ownership_excercise();
    println!("");

    println!("avoiding_ownership using return");
    avoiding_ownership();
    println!("");

    println!("avoiding_ownership using clone");
    avoiding_ownership_using_clone();
    println!("");

    println!("avoiding_ownership using borrow");
    avoiding_ownership_using_borrow();
    println!("");

    println!("mutable_borrow 1");
    mutable_borrow();
    println!("");

    println!("mutable_borrow 2");
    mutable_borrow2();
    println!("");

    println!("reference_rules 1");
    reference_rules1();
    println!("");

    println!("reference_rules 2");
    reference_rules2();
    println!("");

    println!("reference_rules 3");
    reference_rules3();
    println!("");

    println!("reference_rules 4");
    reference_rules4();
    println!("");

    println!("reference_rules 5");
    reference_rules5();
    println!("");

    println!("reference_rules 6");
    reference_rules6();
    println!("");

    println!("referencing 1");
    referencing1();
    println!("");

    println!("auto dereferencing");
    auto_dereferencing();
    println!("");

    println!("auto dereferencing 2");
    auto_dereferencing2();
    println!("");

    println!("dereferencing");
    dereferencing();
    println!("");

    println!("dangling_reference");
    dangling_reference();
    println!("");

    println!("float data type");
    float_type();
    println!("");

    println!("bool data type");
    bool_type();
    println!("");

    println!("char data type");
    char_type();
    println!("");

    println!("array_type");
    array_type();
    println!("");

    println!("pass array to function directly");
    pass_array_to_function();
    println!("");

    println!("pass array to function via reference");
    pass_array_to_function2();
    println!("");

    println!("Vectors");
    println!("");
    vector_fn();
    println!("");

    println!("Vector passed to function");
    vector_fn2();
    println!("");

    println!("Vector passed as reference to function");
    vector_fn3_borrowing();
    println!("");

    println!("Vector passed as mutable reference to function");
    vector_fn3_borrowing_mutable();
    println!("");

    println!("Vector passed to function via cloning");
    vector_fn3_cloning();
    println!("");

    println!("Type inference");
    type_inference();
    println!("");

    println!("shadowing");
    shadowing();
    println!("");

    println!("If Else");
    if_else();
    println!("");

    println!("Loop");
    simple_loop();
    println!("");

    println!("While loop");
    while_loop();
    println!("");

    println!("For loop");
    for_loop();
    println!("");

    println!("match");
    match_number();
    println!("");

    println!("match2");
    match_number2();
    println!("");

    println!("io_example");
    io_example();
    println!("");
}

fn integer_data_types() {
    let num: u8 = 5;
    println!("This is stored in num: {}", num);
    let num2 = 6;
    println!("this is stored in num2: {}", num2);

    //let x: u8 = 256; //the literal `256` does not fit into the type `u8` whose range is `0..=255
    //let x:u8 = -5; //cannot apply unary operator `-` note: unsigned values cannot be negated
}

fn immutable_data_type(){
    let num3 = 7;
    //num3 = 8; //cannot assign twice to immutable variable
    println!("this is stored in num3: {}", num3);
}

fn mutable_data_type(){
    //warning: value assigned to `num3` is never read. Rust throws this warning as the value assigned to num3 is never used
    let mut num3 = 7;
    num3 = 8;
    println!("this is stored in num3: {}", num3);
}

fn str_data_type(){
    //String -- dynamic length strings -- heap allocated -- String length can be changed
    //$str -- fixed length strings -- Special read only memory -- String length can not be changed
    let string_literal = "hello from anish";
    println!("this is stored in string_literal: {}", string_literal);
}

fn string_data_type(){
    let string_literal2 = String::from("hello from anish");
    println!("this is stored in string_literal2: {}", string_literal2);
    //string_literal2.push_str(" dsfdsfsdfdsfd"); //cannot borrow as mutable
    println!("this is stored in string_literal2: {}", string_literal2);

    let mut string_literal3 = String::from("hello from anish");
    println!("this is stored in string_literal3: {}", string_literal3);
    string_literal3.push_str(" dsfdsfsdfdsfd");
    println!("this is stored in string_literal3: {}", string_literal3);
}

fn tuple_data_type(){
    let emp_info: (&str, u8) = ("Ramesh", 9);
    let emp_name = emp_info.0;
    let emp_age = emp_info.1;
    println!("employee name: {}, employee ageL {} ", emp_name, emp_age);
}

fn destructuring(){
    let emp_info: (&str, u8) = ("Ramesh", 9);
    let (emp_name2, emp_age2) = emp_info;
    println!("employee name: {}, employee ageL {} ", emp_name2, emp_age2);
}

fn functions_overview(){
    print_value(5);
    let m1: u8 = 5;
    let m2: u8 = 6;
    let res = add(m1, m2);
    println!("res: {}", res);
}

fn print_value(item: u8) {
    println!("My name is anish");
    println!("item: {}", item);
}

fn add(item1: u8, item2: u8) -> u8 {
    return item1 + item2;
}

fn scoping_overview(){
    let outside_variable = 5;

    {
        let inside_variable = 10;
        println!("inside_variable: {}", inside_variable);
        println!("outside_variable: {}", outside_variable);
    }

    //println!("inside_variable: {}", inside_variable); cannot find value `inside_variable` in this scope
    println!("outside_variable: {}", outside_variable);
}

fn print_global_constant() {
    println!("GLOBAL_CONST: {}", GLOBAL_CONST);
}

fn stack_operation() {
    let a = 5;
    let b = a;
    println!("a: {}", a);
    println!("b: {}", b);
    // everything is fixed at compile time. Stack is getting used here.
}

fn ownership_overview() {
    let str1 = String::from("Hello"); //str1 is the owner of hello value
    // str1.push_str(" world");
    let str2 = str1; // transfer of ownership because as per rules there can be only 1 owner. str2 is the new owner
    //println!("str1 : {}", str1); //borrow of moved value: `str1`
    println!("str2 : {}", str2);
}

fn process_integer_main() {
    let x: u8 = 10;
    process_integer(x);
    println!("process_integer_main : x: {}", x);
}

fn process_integer(x: u8) { //here x is a new memory
    println!("x: {}", x);
}

fn ownership_functions() {
    let x: String = String::from("Hello");
    process_string(x); //transfer of ownership
    //println!("process_string_main : x: {}", x); //x in nothing here as ownership is already transferred.
}

fn process_string(item: String) { //hello new owner is item
    println!("item: {}", item);
}

fn ownership_excercise() {
    let s1: String = get_string(); //s1 is the owner of  "hello"
    println!("this is s1: {}", s1);

    let s2: String = String::from("World"); //s2 is owner of "world"
    let s3: String = send_get_string(s2); //transfer of ownership from s2 to received_string

    println!("s3: {}", s3);
}

fn get_string() -> String {
    let new_string = String::from("Hello"); //new_string is owner here.
    return new_string; //transferring ownership
}

fn send_get_string(received_string: String) -> String {
    return received_string; //transfer of ownership from received_string to s3
}

fn avoiding_ownership() {
    let s1: String = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("s2: {}", s2);
    println!("len: {}", len);
}

fn calculate_length(s1: String) -> (String, usize) {
    let length: usize = s1.len();
    return (s1, length);
}

fn avoiding_ownership_using_clone() {
    let s1: String = String::from("hello");
    let len = calculate_length2(s1.clone());
    println!("s1: {}", s1);
    println!("len: {}", len);
}

fn calculate_length2(s1: String) -> usize {
    let length: usize = s1.len();
    return length;
}

fn avoiding_ownership_using_borrow() {
    let s1: String = String::from("hello");
    let len = calculate_length3(&s1); //borrow operation
    println!("s1: {}", s1);
    println!("len: {}", len);
}

fn calculate_length3(s1: &String) -> usize {
    let length: usize = s1.len();
    return length;
}

fn mutable_borrow() {
    let s1: String = String::from("hello");
    append_string(&s1); //borrow operation
    println!("s1: {}", s1);
}
fn append_string(s1: &String) {
    //s1.push_str("World"); //s1` is a `&` reference, so the data it refers to cannot be borrowed as mutable
}

fn mutable_borrow2() {
    let mut s1: String = String::from("hello");
    append_string2(&mut s1); //borrow operation
    println!("s1: {}", s1);
}
fn append_string2(s1: &mut String) {
    s1.push_str("World")
}

fn reference_rules1() {
    let s1: String = String::from("Hello");
    let r1 = &s1;
    let r2 = &s1;

    println!("r1:{} r2:{}", r1, r2);
}

fn reference_rules2() {
    let s1: String = String::from("Hello");

    let r1 = &s1;
    println!("r1:{}", r1);

    let r2 = &s1;
    println!("r2:{}", r2);
}

fn reference_rules3() {
    let mut s1: String = String::from("Hello");

    let w1 = &mut s1;
    w1.push_str(" World");
    println!("w1:{}", w1);

    let w2 = &mut s1;
    w2.push_str(" Code");
    println!("w2:{}", w2);
}

fn reference_rules4() {
    let mut s1: String = String::from("Hello");

    let w1 = &mut s1; //first mutable borrow occurs here
    w1.push_str(" World");

    //second mutable borrow occurs here
    let w2 = &mut s1;
    w2.push_str(" Code");

    //println!("w2:{} w1:{}", w2, w1); //first borrow later used here
}


fn reference_rules5() {
    let mut s1: String = String::from("Hello");

    let w1 = &mut s1;
    w1.push_str(" World");
    println!("w1:{} ", w1);

    let r1 = &s1;
    println!("r1:{}", r1);
}

fn reference_rules6() {
    let mut s1: String = String::from("Hello");

    let w1 = &mut s1; //mutable borrow occurs here
    w1.push_str(" World");

    let r1 = &s1; //immutable borrow occurs here
    println!("r1:{}", r1);

    // println!("w1:{} ", w1); //mutable borrow later used here
}

fn referencing1() {
    let x = 5;
    println!("address: {:p}", &x);
    let y = &x; // y is the reference to the value of x
    println!("address: {:p}", y);
}

fn auto_dereferencing() {
    let x = 5;
    let y = &x;
    println!("y: {}", y); //auto dereferencing
    println!("y: {}", *y); //dereferencing
}

fn auto_dereferencing2() {
    let s1: String = String::from("hello");
    let len = calculate_length4(&s1); //borrow operation
    println!("s1: {}", s1);
    println!("len: {}", len);
}

fn calculate_length4(s1: &String) -> usize {
    let length: usize = s1.len(); //auto dereferencing . same as (*s).len()
    return length;
}

fn dereferencing() {
    let mut x = 5;
    x = x + 1;
    let y = &mut x; // y is reference to the value of x
    *y = *y + 1;
    //y = y + 1; // cannot add `{integer}` to `&mut {integer}`
    println!("x: {}", x);
}

fn dangling_reference() {
    //let ref_to_nothing = create_string_ref();
}

// fn create_string_ref() -> &String{
//     let s: String = String::from("Hello");
//     return &s; //returns a reference to data owned by the current function
// }

fn float_type() {
    let float_32_num: f32 = 34.5; //f32 floating point number
    let float_64_num = 34.4545; //default type is f64. type inference

    println!("float_32_num : {}", float_32_num);
    println!("float_64_num : {}", float_64_num);
}

fn bool_type() {
    let is_raining: bool = true;
    let is_sunny = false;

    let need_umbrella = is_raining && !is_sunny;
    let need_glasses = is_raining || is_sunny;

    println!("need_umbrella : {} need_glasses: {}", need_umbrella, need_glasses);
}

fn char_type() {
    let letter_a = 'a';
    let emoji = '😀';
    let kanji = '漢';

    println!("letter a: {}", letter_a);
    println!("emoji a: {}", emoji);
    println!("kanji a: {}", kanji);
}

fn array_type() {

    //let mut arr1:[u8;5]; //array declarations

    let mut arr1;
    arr1 = [1, 2, 3, 4, 5];

    println!("arr1[0] = {}", arr1[0]);
    println!("arr1 = {:?}", arr1);

    arr1[2] = 30;
    println!("arr1 = {:?}", arr1);

    println!("length of arr1: {}", arr1.len());
}

//pass directly
fn pass_array_to_function() {
    let arr: [&str; 3] = ["Hello", "world", "coders"];
    write_arr(arr); // here array is fixed array there is no heap involved here. Array is passes directly to function
    println!("arr : {:?}", arr);
}

//This method of directly passing array is expensive as we are making a copy of the whole array.
fn write_arr(mut arr1: [&str; 3]) { //arr1 is a new copy of arr in the memory
    arr1[0] = "Fellow"; //changes here is only for arr1 not for arr
    println!("arr1: {:?}", arr1);
}

//pass by reference
fn pass_array_to_function2() {
    let mut arr: [&str; 3] = ["Hello", "world", "coders"];
    write_arr2(&mut arr);
    println!("arr : {:?}", arr);
}

fn write_arr2(arr2: &mut [&str; 3]) {
    arr2[0] = "Fellow";
    println!("arr2: {:?}", arr2);
}

fn vector_fn() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    println!("v: {:?}", v);

    let mut w = Vec::<i32>::new();
    w.push(4);
    w.push(5);
    w.push(6);
    println!("w: {:?}", w);

    let mut x = vec![7, 8, 9, 10, 11];
    x.push(20);
    x.push(21);
    println!("x: {:?}", x);
}

fn vector_fn2() {
    let vrr: Vec<&str> = vec!["Hello", "World!", "coders"];
    write_vec3(vrr); //ownership transferred
    //println!("vrr: {:?}", vrr); //so this won't compile
}

fn write_vec3(vrr3: Vec<&str>) { //vrr3 is the current owner
    println!("vrr3: {:?}", vrr3);
}

fn vector_fn3_borrowing() {
    let vrr: Vec<&str> = vec!["Hello", "World!", "coders"];
    write_vec4(&vrr);
    println!("vrr: {:?}", vrr);
}

fn write_vec4(vrr3: &Vec<&str>) {
    println!("vrr3: {:?}", vrr3);
}

fn vector_fn3_borrowing_mutable() {
    let mut vrr: Vec<&str> = vec!["Hello", "World!", "coders"];
    write_vec5(&mut vrr);
    println!("vrr: {:?}", vrr);
}

fn write_vec5(vrr3: &mut Vec<&str>) {
    vrr3.push("anish");
    println!("vrr3: {:?}", vrr3);
}

fn vector_fn3_cloning() {
    let vrr: Vec<&str> = vec!["Hello", "World!", "coders"];
    write_vec6(vrr.clone());
    println!("vrr: {:?}", vrr);
}

fn write_vec6(vrr3: Vec<&str>) {
    println!("vrr3: {:?}", vrr3);
}

fn type_inference() {
    let x = 5;
    let y = 6.5;
    let z = "Hello";

    print_variable_type(&x);
    print_variable_type(&y);
    print_variable_type(&z);
}

fn print_variable_type<K>(_: &K) {
    println!("{}", std::any::type_name::<K>())
}

fn shadowing() {
    let x = 5;
    println!("x: {}", x);
    let x = "Anish";
    println!("x: {}", x);
    let x = x.len();
    println!("x: {}", x);
}

fn if_else() {
    let num = 12;

    if num % 3 == 0 && num % 4 == 0 {
        println!("Case 1");
    } else if num % 3 == 0 {
        println!("Case 2");
    } else if num % 4 == 0 {
        println!("Case 3");
    } else {
        println!("case 4");
    }
}

fn simple_loop() {
    // loop {
    //     println!("Hello"); //This will continue to print hello until a break statement is used
    // }
}

fn while_loop() {
    let mut count = 0;

    while count < 5 {
        println!("While loop");
        count += 1;
    }
}

fn for_loop() {
    let arr = [1, 2, 3];

    for element in &arr {
        println!("{}", element);
    }
}

fn match_number() {
    let num = 5;

    match num {
        1 => println!("One"),
        2 | 3 => println!("Two"),
        4 => println!("Three"),
        _ => println!("Unknown"),
    }
}

fn match_number2() {
    fn is_even(x: i32) -> bool {
        if x % 2 == 0 {
            return true;
        }
        return false;
    }

    let num = 5;

    match num {
        x if is_even(x) => println!("x is even"),
        x if !is_even(x) => println!("x is not even"),
        _ => println!("Unknown"),
    }
}

use std::io;
fn io_example() {
    let mut input = String::new();
    println!("please input your name:");
    io::stdin().read_line(&mut input).expect("Input failed");
    println!("input: {}", input);
}