const GLOBAL_CONST:u8 = 100;  //warning: constant `global_const` should have an upper case name

fn main() {
    println!("Hello, Anish!");
    let num:u8 = 5;
    println!("This is stored in num: {}", num);

    let num2 = 6;
    println!("this is stored in num2: {}", num2);

    //warning: value assigned to `num3` is never read. Rust throws this warning as the value assigned to num3 is never used
    let mut num3 = 7;
    num3 = 8;
    println!("this is stored in num3: {}", num3);

    //String -- dynamic length strings -- heap allocated -- String length can be changed
    //$str -- fixed length strings -- Special read only memory -- String length can not be changed
    let string_literal = "hello from anish";
    println!("this is stored in string_literal: {}", string_literal);

    let mut string_literal2 = String::from("hello from anish");
    println!("this is stored in string_literal2: {}", string_literal2);
    string_literal2.push_str(" dsfdsfsdfdsfd");
    println!("this is stored in string_literal2: {}", string_literal2);

    let emp_info: (&str, u8) = ("Ramesh", 9);
    let emp_name = emp_info.0;
    let emp_age = emp_info.1;
    println!("employee name: {}, employee ageL {} ", emp_name, emp_age);

    //destructuring
    let (emp_name2, emp_age2) = emp_info;
    println!("employee name: {}, employee ageL {} ", emp_name2, emp_age2);
    print_value(5);

    let m1: u8 = 5;
    let m2: u8 = 6;
    let res = add(m1, m2);
    println!("res: {}", res);

    let outside_variable = 5;

    {
        let inside_variable = 10;
        println!("inside_variable: {}", inside_variable);
        println!("outside_variable: {}", outside_variable);
    }

    //println!("inside_variable: {}", inside_variable); cannot find value `inside_variable` in this scope
    println!("outside_variable: {}", outside_variable);

    print_value2();

    ab1();
    str1str2();

    process_integer_main();
    process_string_main();

    s2_s3_s4();

    avoiding_ownership();

}

fn print_value2(){
    println!("GLOBAL_CONST: {}", GLOBAL_CONST);
}

fn add(item1: u8, item2: u8) -> u8{
    return item1 + item2;
}

fn print_value(item: u8){
    println!("My name is anish");
    println!("item: {}", item);
}

fn ab1(){
    let a = 5;
    let b = a;
    println!("a: {}", a);
    println!("b: {}", b);
}

fn str1str2(){
    let str1 = String::from("Hello"); //str1 is the owner of hello value
    // str1.push_str(" world");
    let str2 = str1; // transfer of ownership because as per rules there can be only 1 owner
    //println!("str1 : {}", str1); //borrow of moved value: `str1`
    println!("str2 : {}", str2);
}

fn process_integer_main(){
    let x:u8 = 10;
    process_integer(x);
    println!("process_integer_main : x: {}", x);
}

fn process_integer(x: u8){
    println!("x: {}", x);
}

fn process_string_main(){
    let x:String = String::from("Hello");
    process_string(x);//transfer of ownership
    //println!("process_string_main : x: {}", x); //x in nothing here as ownership is already transferred.
}

fn process_string(item: String){ //hello new owner is item
    println!("item: {}", item);
}

fn s2_s3_s4(){
    let s1:String = get_string(); //s1 is the owner of  "hello"
    println!("this is s1: {}", s1);

    let s2: String = String::from("World"); //s2 is owner of "world"
    let s3: String = send_get_string(s2);//transfer of ownership from s2 to received_string

    println!("s3: {}", s3);
}

fn get_string() -> String {
    let new_string = String::from("Hello"); //new_string is owner here.
    return new_string; //transferring ownership
}

fn send_get_string(received_string: String)->String {
    return received_string;//transfer of ownership from received_string to s3
}

fn avoiding_ownership(){
    let s1: String = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("s2: {}", s2);
    println!("len: {}", len);
}

fn calculate_length(s1: String) -> (String, usize) {
    let length: usize = s1.len();
    return (s1, length);
}
