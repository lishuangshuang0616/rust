// // Rust 程序入口函数，跟其它语言一样，都是 main，该

// fn main() {
//     // let mut s = String::from("hello ");
//     // s.push_str(", world");
//     // println!("{}", s);
//     // let s1 = String::from("hello");
//     // let s2 = s1;
//     // println!("x = {}, y = {}", s2, s2);
//     // let x: &str = "hello, world";
//     // let y = x;
//     // println!("x = {}, y = {}", x, y);
    
//     // let s1 = String::from("hello, world");
//     // let s2 = s1.clone();
//     // println!("s1 = {}, s2 = {}", s1,s2)

//     // let s = String::from("hello");
//     // take_ownership(&s);
//     // println!("{}", s);
//     // let x = 5;    
//     // make_copy(x);

//     // let s1 = give_ownership();
//     // println!("s1 = {}", s1);
//     // let s2 = String::from("world");
//     // println!("s2 = {}", s2);
//     // let s3 = takes_given_ownership(s2);
//     // //println!("s2 = {}", s2);
//     // println!("s3 = {}", s3);


//     // let x = 5;
//     // let y = &x;
//     // println!("{}\n{}", x,y);

//     // assert_eq!(x, 5);
//     // assert_eq!(*y, 5);

//     // let s1 = String::from("hello");
//     // let len = calculate_length(&s1);
    
//     // println!("The length of '{}' is {}", s1, len.1);

//     // let mut s = String::from("hello");
//     // change(&mut s);
//     // println!("{}",s);


// }

// fn main() {
    
//     let mut s = String::from("hello, world");
    
//     let word = first_word(&s);

//     println!("The first word is {}", word);

//     s.clear();

//     // println!("The first word is {}", word);
// }

// fn first_word(s: &mut String) -> &str {
//     &s[..4]
// }







// fn change(s: &mut String){
//     s.push_str(",world");
// }

// fn calculate_length(s: &String) -> (&String, usize) {
//     let length = s.len();
//     (s, length)
// }

// fn give_ownership() -> String {
//     let s = String::from("hello");
//     return s;
// }
// fn takes_given_ownership(a_string: String) -> String {
//     println!("{}", a_string);
//     return a_string;
// }

// fn take_ownership(some_string: &String) {
//     println!("{}", &some_string);
// }

// fn make_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }



// // 定义一个函数，输入两个i32类型的32位有符号整数，返回它们的和
// fn add(i: i32, j: i32) -> i32 {
//     // 返回相加值，这里可以省略return
//     i + j
// }


// fn another_function(x: i32, y: char) {
//     // 函数体
//     println!("The value of x is: {}", x);
//     println!("The value of y is: {}", y);
// }

// // fn report<T: debug>(item: T){
// //     println!("{:?}", item);
// // }

// fn clear(text: &mut String) -> () {
//     *text = String::from("woshi yi ge zi");
// }


// 数值类型 
// 整数
// i8, i16, i32, i64, i128, isize
// u8, u16, u32, u64, u128, usize
// f32, f64
// 

// String 类型是复杂类型，由存储在栈中的堆指针、字符串长度、字符串容量共同组成；
// 其中堆指针是最重要的，指向了真实储存字符串内容的堆内存

// 任何基本类型的组合可以 Copy ，不需要分配内存或某种形式资源的类型是可以 Copy 的。如下是一些 Copy 的类型：

// 所有整数类型，比如 u32
// 布尔类型，bool，它的值是 true 和 false
// 所有浮点数类型，比如 f64
// 字符类型，char
// 元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是
// 不可变引用 &T ，例如转移所有权中的最后一个例子，但是注意：可变引用 &mut T 是不可以 Copy的


// 复合类型，结构体struct, 枚举enum



// fn main() {
    
//     let mut s = String::from("hello");

//     s.push_str(", world");
//     println!("{}", s);

//     s.push('!');
//     println!("{}", s);

//     s.insert(6, ',');
//     println!("{}", s);

//     s.insert_str(6, " world");
//     println!("{}", s);

//     let s1 = s.replace_range(..6, "Hello");

//     let s2 = s.replace("world", "World");
//     println!("{:?}", s1);

//     println!("{:?}", s2);

    
    
// }


// fn main() {

//     let string_appends = String::from("hello");
//     let string_rust = String::from("rust");

//     let result = string_appends + &string_rust;

//     println!("{}", result);

//     let mut result = result + "!";
//     result = result + "!!!!!";
    
//     println!("{}", result);
// }

// fn main() {
//     let s1 = String::from("hello");
//     let h = &s1[0..2];
//     println!("{}",h)
//   }
  
//   fn greet(name: &str) {
//     println!("Hello, {}!", name);
//   }

// fn main() {
//     let tup = (500, 6.4, 1);

//     let (_, y, _) = tup;

//     println!("The value of y is: {}", y);
// }



// fn main() {
//     // let mut user2 = User {
//     //     email: String::from("someone@example.com"),
//     //     username: String::from("someusername123"),
//     //     active: true,
//     //     sign_in_count: 1,
//     //  };
//     // user2.email = String::from("anotheremail@example.com");

//     // let user3 = User {
//     //     email: String::from("another1111@example.com"),
//     //     ..user2
//     // };
    
    
//     // // println!("{:?}", user2);
//     // println!("{:?}", user3);

//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
//     println!("{}", black.1);
//     println!("{}", origin.2);
    
// }

// #[derive(Debug)]
// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }


// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// let black = Color(0, 0, 0);
// let origin = Point(0, 0, 0);
// println!("{:?}", black);
// println!("{:?}", origin);

// #[derive(Debug)]
// struct User {
//     username: Box<str>,
//     email: Box<str>,
//     sign_in_count: u64,
//     active: bool,
// }

// fn main() {
//     let user1 = User {
//         email: "someone@example.com".into(),
//         username: "someusername123".into(),
//         active: true,
//         sign_in_count: 1,
//     };
//     // println!("{:?}", user1);
//     dbg!(&user1);
// }

// struct Unit;
// trait SomeTrait {
//     // ...定义一些行为
// }

// // 我们并不关心结构体中有什么数据( 字段 )，但我们关心它的行为。
// // 因此这里我们使用没有任何字段的单元结构体，然后为它实现一些行为
// impl SomeTrait for Unit {  }
// fn main() {
//     let u = Unit;
//     do_something_with_unit(u);
// } 

// // 填空，让代码工作
// fn do_something_with_unit(u: __) {   }

// 结构体 struct
// 枚举

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// enum PokerCard {
//     Spade,
//     Club,
//     Diamond,
//     Heart,
// }

// struct PokerCard1{
//     suit: PokerCard,
//     value: u8,
// }

// fn main() {
//     let card = PokerCard1{suit: PokerCard::Heart, value: 10};
//     println!("{}", card.value);
// }

// enum Message {                                                                                                                                                                                                                                                                                                                                                                                                               
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// fn main() {
//     let msg = Message::ChangeColor(0, 160, 255);
//     match msg {
//         Message::ChangeColor(_, _, _) => println!("Change the color!"),
//         _ => println!("Something else"),
//     }
// }

// enum Option<T> {
//     Some(T),
//     None,
// }

// use crate::List::*;

// enum List {
//     // Cons: 链表中包含值的节点
//     Cons(i32, Box<List>),
//     // Nil: 链表的结尾（值为空）
//     Nil,
// }

// impl List{++++++++++++++    ++++++++
//     fn new() -> List {
//         Nil
//     }

//     fn prepend(self, elem: i32) -> List {
//         Cons(elem, Box::new(
//         ))
//     }
    
//     fn len(&self) -> u32 {
//         // 递归的实现
//         match s*elf {
//             Cons(_, ref tail) => 1 + tail.len(),
//             Nil => 0,
//         }
//     }

//     fn stringify(&self) -> String {
//         match self {
//             Cons(head, tail) => format!("{}, {}", head, tail.stringify()),
//             Nil => format!("Nil"),
//         }
//     }
// }

// fn main() {
//     let a = [1, 2, 3, 4, 5];
//     let slice = &a[1..3];
//     assert_eq!(slice, &[2, 3]);
//     println!("{:?}", slice);
// }


// fn main() {
//     let months = ["January", "February", "March", "April", "May", "June", "July",
//               "August", "September", "October", "November", "December"];

//     let a: [i32;5] = [1,2,3,4,5];
//     println!("{:?}", &a[1..3]);
//     println!("{:?}", months);
    
// }

// use std::io;
// fn main() {
//     let a:[i32;5] = [1,2,3,4,5];
//     println!("Please enter an array index.");
    
//     let mut index = String::new();
//     io::stdin().read_line(&mut index)
//         .expect("Failed to read line");

//     let index: usize = index.trim().parse()
//         .expect("Index entered was not a number");

//     let element = a[index];
//     println!("The value of the element at index {} is: {}", index, element);
// }

// use std::array;
// fn main() {
//     // let array = [String::from("rust is good!");8];
//     // println!("{:?}", array)
//     let array: [String; 8] = array::from_fn(|_i| String::from("rust is good!"));
//     println!("{:?}", array)
// }

// fn main() {
//     let a:[i32;5] = [1,2,3,4,5];
//     let slice = &a[1..3];
//     assert_eq!(slice, &[2,3]);
// }

// fn main() {
//     let one = [1,2,3];
//     let two:[u8;3] = [1,2,3];
//     let blank1 = [0;3];
//     let blank2:[u8;3] = [0;3];
//     println!("{:?}\n{:?}\n{:?}\n{:?}", one, two, blank1, blank2);
    
//     let arrays:[[u8;3];4] = [one,two,blank1,blank2];
//     println!("{:?}", arrays);
    
//     for a in arrays.iter() {
//         println!("{:?}", a);
        
//         for i in a.iter() {
//             println!("\t{} + 10 = {}", i, i+10 );
//         }

//         let mut sum = 0;
//         for i in 0..a.len() {
//             sum += a[i];
//         }
//         println!("\t{:?} = {}", a,sum);
        
//     }
// }

// fn main(){
//     let condition = true;
//     let number = if condition {5} else {6};
//     println!("{}", number)
// }

// fn main(){
//     let n = 6;
    
//     if n%4 == 0 {
//         println!("{} is divisible by 4", n);
//     } else if n%3 == 0 {
//         println!("{} is divisible by 3", n);
//     } else if n%2 == 0 {
//         println!("{} is divisible by 2", n);
//     } else {
//         println!()
//     }
// }

// fn main(){
//     for i in 1..=4 {
//         println!("{}", i);
//     }
// }

// for item in collection
// for item in &collection
// for item in &mut collection

// fn main() {
//     let a = [4, 3, 2, 1];
//     // `.iter()` 方法把 `a` 数组变成一个迭代器
//     for (i, v) in a.iter().enumerate() {
//         println!("第{}个元素是{}", i + 1, v);
//     }
//     println!("{:?}",a);
// }

// enum Direction{
//     East,
//     West,
//     North,
//     South,
// }

// fn main(){
//     let dire = Direction::South;
//     match dire {
//         Direction::East => println!("East"),
//         Direction::North | Direction::South => 
//             println!(
//                 "North or South"
//             ),
//         _ => println!("West"),
//     }
// }

// enum Coin{
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin{
//         Coin.Penny => {
//             println!("Penny");
//             1
//         },
//         Coin.Nickel => 5,
//         Coin.Dime => 10,
//         Coin.Quarter => 25,
//         }
//     }

// enum IpAddressKind{
//     V4(String),
//     V6(String),
// }

// fn main(){
//     let ip1 = IpAddressKind::V4(String::from("127.0.0.1"));
//     let ip2 = IpAddressKind::V6(String::from("::1"));
// }

// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
//     // --snip--
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState), // 25美分硬币
// }

// enum Action{
//     Say(String),
//     MoveTo {x: i32, y: i32},
//     ChangeColor(i32, i32, i32),
// }

// fn main(){
//     let actions = [
//         Action::Say("hello world".to_string()),
//         Action::MoveTo{x:1,y:2},
//         Action::ChangeColor(255,255,255),
//     ];
    
//     for action in actions.iter(){
//         match action{
//             Action::Say(s) => println!("{s}"),
//             Action::MoveTo{x,y} => println!("Move to x:{x} y:{y}"),
//             Action::ChangeColor(r,g,b) => println!("Change color r:{r} g:{g} b:{b}"),
//         }
//     }
// }

// enum Direction {
//     East,
//     West,
//     North,
//     South,
// }

// fn main(){
//     let dire = Direction::South;
//     match dire {
//         Direction::East => println!("East"),
//         Direction::North | Direction::South => 
//             println!(
//                 "North or South"
//             ),
//         _ => println!("West"),
//     }
// }

// fn main(){
//     let x = 'F';
//     assert!(matches!(x, 'A'..='Z' | 'a'..='z'));
//     println!("{}", x)
// }

// let val: Option<i32> = get_some_val();

// // match val
// match val {
//     Some(x) => println!("{}", x),
//     None => println!("None"),
// }

// // if let
// if let Some(x) = val {
//     println!("{}", x);
// } else {
//     println!("None");
// }

// let sr: Result<u32,&str> = Ok(5);

// match sr {
//     Ok(x) => println!("{}", x),
//     Err(e) => println!("{}", e),
// }

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i+1),
//     }
// }


// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);
//     println!("{:?}\n{:?}\n{:?}", five, six, none);
// }

// use std::str::FromStr;

// fn get_count_item(s: &str) -> (u64, &str){
//     let mut it = s.split(' ');
    
//     let (Some(count_str),Some(item)) = (it.next(), it.next()) else {
//         panic!("Can't segment count item pair: '{s}' ");
//     };
    
//     let Ok(count) = u64::from_str(count_str) else {
//         panic!("Can't parse count: '{count_str}' ");
//     };
//     (count, item)
// }

// fn main(){
//     let (count, item) = get_count_item("11sdfa shoes");
//     println!("{} {}", count, item);
// }

// fn main(){
//     let x =1;
//     match x{
//         1 => println!("one"),
//         _ => println!("other"),
//     }
// }

// struct Point {
//     x : i32,
//     y : i32,
// }

// // fn main(){
// //     let p = Point{x:0,y:7};
// //     let Point{x:a,y:b} = p;
// //     println!("{},{}",a,b);
// // }

// fn main(){
//     let p = Point{x:0,y:7};
    
//     match p {
//         Point{x:1,y} => println!("y:{}",y),
//         Point{x,..} => println!("x:{}",x),
//         _ => println!("other"),
//     }
// }


// enum Message {
//     Quit,
//     Move{x:i32,y:i32},
//     Write(String),
//     ChangeColor(i32,i32,i32),
// }

// fn main(){
//     let msg = Message::Write(String::from("hello"));
//     match msg{
//         Message::Quit => println!("Quit"),
//         Message::Move{x,y} => println!("Move x:{} y:{}",x,y),
//         Message::Write(s) => println!("Write {}",s),
//         Message::ChangeColor(r,g,b) => println!("ChangeColor r:{} g:{} b:{}",r,g,b),
//     }
// }

// enum Color {
//     Rgb(i32,i32,i32),
//     Hsv(i32,i32,i32),
// }

// enum Message {
//     Quit,
//     Move{x:i32,y:i32},
//     Write(String),
//     ChangeColor(Color),
// }

// fn main(){
//     let msg = Message::ChangeColor(Color::Hsv(0,160,255));

//     match msg{
//         Message::ChangeColor(Color::Rgb(r,g,b)) => println!("RGB:{},{},{}",r,g,b),
//         Message::ChangeColor(Color::Hsv(h,s,v)) => println!("HSV:{},{},{}",h,s,v),
//         _ => (),
//     }
    
// }

// fn main(){
//     let arr: &[i32;5] = &[1,2,3,4,5];

//     if let [x,..] = arr{
//         println!("x:{}",x);
//     }
    
//     if let &[..,y] = arr{
//         println!("y:{}",y);
//     }
    
//     let arr: &[u16] = &[];

//     assert!(matches!(arr, [..]));
//     assert!(!matches!(arr, [x, ..]));
// }


// fn main(){
//     let mut setting_value = Some(5);
//     let new_setting_value = Some(10);

//     match (setting_value, new_setting_value) {
//         (Some(_), Some(_)) => {
//             println!("Can't overwrite an existing customized value");
//         }
//         _ => {
//             setting_value = new_setting_value;
//         }
//     }

//     println!("setting is {:?}", setting_value);
// }

// fn main(){
//     let x = Some(5);
//     let y = 10;

//     match x {
//         Some(50) => println!("Got 50"),
//         Some(n) if n == y => println!("Matched, n={:?}", n),
//         _ => println!("Default case, x={:?}", x),
//     }
//     println!("at the end: x={:?}, y={:?}", x, y);
// }

// enum Message {
//     Hello{id:i32},
// }

// fn main(){
//     let msg = Message::Hello{id:5};
    
//     match msg {
//         Message::Hello{id:idref @ 3..=7} => println!("Found an id in range: {}", idref),
//         Message::Hello{id:idref} => println!("Found some other id: {}", idref),
//     }
// }

// #[derive(Debug)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// fn main() {
//     // 绑定新变量 `p`，同时对 `Point` 进行解构
//     let p @ Point {x: px, y: py } = Point {x: 10, y: 23};
//     println!("x: {}, y: {}", px, py);
//     println!("{:?}", p);


//     let point = Point {x: 10, y: 5};
//     if let p @ Point {x: 10, y} = point {
//         println!("x is 10 and y is {} in {:?}", y, p);
//     } else {
//         println!("x was not 10 :(");
//     }
// }

// fn main() {
//     match 1 {
//         num @ (1 | 2) => {
//             println!("{}", num);
//         }
//         _ => {}
//     }
// }

// struct Circle{
//     x: f64,
//     y: f64,
//     radius: f64,
// }

// impl Circle{
//     fn new(x: f64, y: f64, radius: f64) -> Circle{
//         Circle{x,y,radius}
//     }
    
//     fn area(&self) -> f64{
//         std::f64::consts::PI * (self.radius * self.radius)
//     }

//     fn intersect(&self, other: &Circle) -> bool{
//         let distance_squared = ((self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y)) as f64;
//         let radii_squared = (self.radius + other.radius) * (self.radius + other.radius);
//         distance_squared <= radii_squared
//     }
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// // impl Rectangle {
// //     fn area(&self) -> u32 {
// //         self.width * self.height
// //     }
// // }

// // fn main() {
// //     let rect1 = Rectangle { width: 30, height: 50 };
// //     println!("The area of the rectangle is {} square pixels.", rect1.area());
// // }

// impl Rectangle{
//     fn width(&self) -> bool{
//         self.width > 0
//     }
// }

// fn main(){
//     let rect1 = Rectangle { width: 30, height: 50 };
//     if rect1.width(){
//         println!("The rectangle has a nonzero width; it is {}", rect1.width);
//     }
// }

// mod my{
//     pub struct Rectangle{
//         pub width: u32,
//         pub height: u32,
//     }

//     impl Rectangle{
//         pub fn new (width: u32, height: u32) -> Rectangle{
//             Rectangle{width,height}
//         }
//         pub fn width(&self) -> u32{
//             return self.width;
//         }
//         pub fn height(&self) -> u32{
//             return self.height;
//         }
//     }
// }

// fn main(){
//     let rect1 = my::Rectangle::new(30,50);
//     println!("rect1 width:{}",rect1.width());
//     println!("rect1 height:{}",rect1.height());
//     println!("rect1 width:{}",rect1.width);
//     println!("rect1 height:{}",rect1.height);
// }


// mod my {
//     pub struct Rectangle {
//         pub width: u32,
//         pub height: u32,
//     }

//     impl Rectangle {
//         pub fn new(width: u32, height: u32) -> Self {
//             Rectangle { width, height }
//         }
        
//         pub fn area(&self) -> u32 {
//             self.width * self.height
//         }
        
//         pub fn can_hold(&self, other: &Rectangle) -> bool {
//             self.width > other.width && self.height > other.height
//         }
//     }
// }

// fn main()  {
//     let rect1 = my::Rectangle::new(30, 50);
//     let rect2 = my::Rectangle::new(10, 40);
//     let rect3 = my::Rectangle::new(60, 45);

//     println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
//     println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
//     println!("The area of the rectangle is {} square pixels.", rect1.area());
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// // impl Rectangle {
// //     fn new(width: u32, height: u32) -> Rectangle {
// //         Rectangle { width, height }
// //     }
// // }

// // fn main() {
// //     let rect1 = Rectangle::new(30, 50);
// //     println!("rect1 is {:?}", rect1);
// // }



// impl Rectangle{
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// impl Rectangle {
//     fn  can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     } 
// }


// fn main(){
//     let rect1 = Rectangle{width:30,height:50};
//     println!("The area of the rectangle is {} square pixels.", rect1.area());
//     let rect2 = Rectangle{width:10,height:40};
//     let rect3 = Rectangle{width:60,height:45};
//     println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
//     println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
// }


// #[allow(unused)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// impl Message {
//     fn call(&self) {
//         // method body would be defined here
//     }
// }

// fn main() {
//     let m = Message::Write(String::from("hello"));
//     m.call();
// }


// mod my {
//     pub struct Point{
//         pub x: f32,
//         pub y: f32,
//     }

//     impl Point{
//         pub fn new(x: f32, y: f32) -> Point{
//             Point{x,y}
//         }
        
//         pub fn origin() -> Point{
//             Point{x:0.0,y:0.0}
//         }
//     }

//     pub struct Rectangle{
//         pub p1: Point,
//         pub p2: Point,
//     }
    
//     impl Rectangle{
//         pub fn new(p1: Point, p2: Point) -> Rectangle{
//             Rectangle{p1,p2}
//         }
//         pub fn area(&self) -> f32{
//             let Point{x:x1,y:y1} = self.p1;
//             let Point{x:x2,y:y2} = self.p2;
//             (x1-x2).abs()*(y1-y2).abs()
//         }
//         pub fn perimeter(&self) -> f32{
//             let Point{x:x1,y:y1} = self.p1;
//             let Point{x:x2,y:y2} = self.p2;
//             2.0*((x1-x2).abs()+(y1-y2).abs())
//         }
        
//         pub fn translate(&mut self, x: f32, y: f32){
//             self.p1.x += x;
//             self.p2.x += x;
//             self.p1.y += y;
//             self.p2.y += y;
//         }
//     }

//     pub struct Pair(pub Box<i32>,pub Box<i32>);
    
//     impl Pair{
//         pub fn destroy(self) {
//             // Destructure `self` into two `Box` types.
//             let Pair(first, second) = self;
//             println!("Destroying Pair({}, {})", first, second);
//             // Return the original `Box` values.
//         }
//     }
// }

// fn main(){
//     let rectangle = my::Rectangle::new(
//         my::Point::new(1.0, 2.0),
//         my::Point::new(5.0, 5.0),
//     );

//     println!("rectangle perimeter: {}", rectangle.perimeter());
//     println!("rectangle area: {}", rectangle.area());
    
//     let mut square = my::Rectangle::new(my::Point::origin(), my::Point::new(5.0, 5.0));
//     square.translate(3.0, 3.0);
//     let pair = my::Pair(Box::new(1), Box::new(2));
//     pair.destroy();
// }

// fn add<T: std::ops::Add<Output = T>>(a:T,b:T) -> T {
//     a + b
// }
// fn main(){
//     println!("add i8: {}", add(2i8, 3i8));
//     println!("add i32: {}", add(20, 30));
//     println!("add f64: {}", add(1.23, 1.23));
// }   

// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }


// use std::fmt::Display;

// fn create_and_print<T>() where T:From <i32> + Display {
//     let x: T = 5i32.into();
//     println!("{}", x);
// }

// fn main() {
//     create_and_print::<i32>();
// }
// use std::fmt::Display;

// #[derive(Debug)]
// struct Point<T> {
//     x :T,
//     y :T,
// }

// fn main(){
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };
    
//     println!("integer x: {:#?},",integer);
//     println!("float x: {:#?},",float);
// }

// enum Option<T> {
//     Some(T),
//     None,
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }



// struct Point<T>{
//     x: T,
//     y: T,
// }

// impl<T> Point<T>{
//     fn x(&self) -> &T{
//         &self.x
//     }
// }

// fn main() {
//     let integer = Point { x: 5, y: 10 };
//     println!("integer x: {}", integer.x());
// }

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// impl <T, U> Point<T, U> {
//     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }

// fn main() {
//     let p1 = Point { x: 5, y: 10.4 };
//     let p2 = Point { x: "Hello", y: 'c' };

//     let p3 = p1.mixup(p2);

//     println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
// }

// impl Point<f32, f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

// const 

// fn display_array(arr: &[i32]) {
//     for item in arr {
//         println!("{}", item);
//     }
// }

// fn main() {
//     let arr = [1, 2, 3, 4, 5];
//     display_array(&arr);
// }

// fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
//     for item in &arr {
//         println!("{:?}", item);
//     }
// }

// fn main() {
//     let arr: [i32; 3] = [1, 2, 3];
//     display_array(arr);

//     let arr: [i32; 2] = [1, 2];
//     display_array(arr);
// }

// #![feature(generic_const_exprs)]
// #![allow(incomplete_features)]

// fn something<T>(val: T) where Assert<{core::mem::size_of::<T>() < 768}> : IsTrue {
//     println!("{}", val);
// }

// fn main() {
//     something([0u8; 0]);
//     something([0u8; 1]);
//     something([0u8; 767]);
//     something([0u8; 768]);
// }

// pub enum Assert<const CHECK: bool> {
//     Assertion,
// }

// pub trait IsTrue {
//     fn is_true() -> bool {
//         CHECK
//     }
// }

// impl IsTrue for Assert<true> {
//     fn is_true() -> bool {
//         true
//     }
// }

// const fn add(a: usize, b: usize) -> usize {
//     a + b
// }

// const RESULT: usize = add(5, 10);

// fn main() {
//     println!("The result is: {}", RESULT);
// }

// struct Buffer<const N:usize> {
//     data: [u8; N],
// }

// const fn compute_buffer_size(factor: usize) -> usize {
//     factor * 1024
// }

// fn main(){
//     const SIZE: usize = compute_buffer_size(4);
//     let buffer = Buffer::<SIZE>{data:[0; SIZE]};
//     println!("buffer size: {}", buffer.data.len());
// }

// struct ArrayPair<T, const N:usize>{
//     first: [T; N],
//     second: [T; N],
// }

// impl<T, const N: usize> ArrayPair<T, N>{
//     fn swap()
// }

// fn foo<const N: usize>(a: [i32; N]) -> [i32; N] {
//     a
// }


// pub struct MinSlice<T, const N: usize> {
//     pub head: [T; N],
//     pub tail: [T],
// }

// fn main(){
//     let slice: &[u8] = b"Hello, world";
//     let reference: Option<&u8> = slice.get(6);
//     match reference {
//         Some(&b',') => println!("Found a comma at index 6"),
//         Some(&b'!') => println!("Found an exclamation mark at index 6"),
//         Some(&b' ') => println!("Found a space at index 6"),
//         Some(&b'w') => println!("Found a 'w' at index 6"),
//         Some(&b'l') => println!("Found a 'l' at index 6"),
//         Some(&b'o') => println!("Found a 'o' at index 6"),
//         Some(&b'r') => println!("Found a 'r' at index 6"),
//         Some(&b'd') => println!("Found a 'd' at index 6"),
//         Some(&b'H') => println!("Found a 'H' at index 6"),
//         _ => println!("No match found"),
//     }

//     let slice: &[u8] = b"Hello, world";
//     let minslice = MinSlice::<u8,12>::from_slice(slice).unwrap();
//     let value:u8 = minslice.head[6];
//     println!("{}",value);
//     println!("{:?}",minslice);
// }



// fn main(){
//     let tweet = Tweet{
//         username: String::from("horse_ebooks"),
//         content: String::from("of course, as you probably already know, people"),
//         reply: false,
//     };
//     println!("{}",tweet.summarize())
// }

// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T,u: &U) -> i32{
//     1
// }

// fn some _function<T, U>(t: T, u: U) -> i32
// where T: Display + Clone,
//       U: Clone + Debug{
//     1
// }

// use std::fmt::Display;

// struct Pair<T>{
//     x: T,
//     y: T,
// }

// impl<T> Pair<T> {
//     fn new(x:T ,y:T) -> Self{
//         Self{
//             x,
//             y,
//         }
//     }
// }

// impl<T: Display + PartialOrd> Pair<T>{
//     fn cmp_display(&self){
//         if self.x >= self.y{
//             println!("The largest member is x = {}",self.x);
//         }else{
//             println!("The largest member is y = {}",self.y);
//         }
//     }
// }

// fn main(){
//     let pair1 = Pair::new(1,2);
//     pair1.cmp_display();
//     let pair2 = Pair::new("hello","world");
//     pair2.cmp_display();
// }

// pub trait Summary {
//     fn summarize_author(&self) -> String;

//     fn summarize(&self) -> String {
//         format!("(Read more from {}...)", self.summarize_author())
//     }
// }
// pub struct Post{
//     pub title: String,
//     pub author: String,
//     pub content: String,
// }

// impl Summary for Post {
//     fn summarize_author(&self) -> String {
//         format!("@{}", self.author)
//     }
// }

// pub struct Tweet{
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub retweet: bool,
// }

// impl Summary for Tweet {
//     fn summarize_author(&self) -> String {
//         format!("@{}", self.username)
//     }
// }

// fn returns_summarizable() -> impl Summary{
//     Tweet{
//         username: String::from("horse_ebooks"),
//         content: String::from("of course, as you probably already know, people"),
//         reply: false,
//         retweet: false,
//     }
// }

// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         Post {
//             title: String::from("Post title"),
//             author: String::from("Author name"),
//             content: String::from("Post content"),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

// fn main(){
//     let tweet = returns_summarizable();
//     println!("{}",tweet.summarize());
// }


// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T{
//     let mut largest = list[0];
//     for &item in list.iter(){
//         if item > largest{
//             largest = item;
//         }
//     }
//     largest
// }

// fn main(){
//     let number_list = vec![34,50,25,100,65];
//     let result = largest(&number_list);
//     println!("The largest number is {}",result);

//     let char_list = vec!['y','m','a','q'];
//     let result = largest(&char_list);
//     println!("The largest char is {}",result);
// }


// fn main() {
//     let a: i32 = 10;
//     let b: u16 = 100;
  
//     let b_ = b.try_into()
//               .unwrap();
  
//     if a < b_ {
//       println!("Ten is less than one hundred.");
//     }
//   }


// use std::ops::Add;

// #[derive(Debug)]
// struct Point<T: Add<T, Output = T>> {
//     x: T,
//     y: T,
// }

// impl<T:Add<T, Output= T>> Add for Point<T>{
//     type Output = Point<T>;
//     fn add(self, p : Point<T>) -> Point<T>{
//         Point{
//             x: self.x + p.x,
//             y: self.y + p.y,
//         }
//     }
// }

// fn add<T: Add<T, Output=T>>(a:T, b:T) -> T{
//     a + b
// }

// fn main(){
//     let p1 = Point{x: 1, y: 2};
//     let p2 = Point{x: 3, y: 4};
//     println!("{:?}",p1 + p2);
    
//     let p3 = Point{x: 1.1f32,y: 2.2f32};
//     let p4 = Point{x: 3.3f32,y: 4.4f32};
//     println!("{:?}",p3 + p4);
// }


// #![allow(dead_code)]
// use std::fmt;
// use std::fmt::Display;
// use std::cmp::PartialEq;

// #[derive(Debug, PartialEq)]
// enum FileState {
//     Open,
//     Closed,
// }

// #[derive(Debug)]
// struct File {
//     name: String,
//     data: Vec<u8>,
//     state: FileState,
// }

// impl Display for File {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "file state for {} is {}", self.name, self.state)
//     }
// }

// impl Display for FileState {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             FileState::Open => write!(f, "OPEN"),
//             FileState::Closed => write!(f, "CLOSED"),
//         }
//     }
// }

// impl File {
//     fn new(name: &str) -> File {
//         File {
//             name: String::from(name),
//             data: Vec::new(),
//             state: FileState::Closed,
//         }
//     }

//     fn open(&mut self) -> Result<isize, FileError> {
//         if self.state == FileState::Closed {
//             self.state = FileState::Open;
//             Ok(self.data.len() as isize)
//         } else {
//             Err(FileError::AlreadyOpen)
//         }
//     }

//     fn close(&mut self) -> Result<String, FileError> {
//         if self.state == FileState::Open {
//             self.state = FileState::Closed;
//             Ok(String::from("File closed"))
//         } else {
//             Err(FileError::AlreadyClosed)
//         }
//     }
// }

// #[derive(Debug)]
// enum FileError {
//     AlreadyOpen,
//     AlreadyClosed,
// }

// impl Display for FileError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             FileError::AlreadyOpen => write!(f, "File already open"),
//             FileError::AlreadyClosed => write!(f, "File already closed"),
//         }
//     }
// }

// impl std::error::Error for FileError {}

// fn main() {
//     let mut f6 = File::new("sixth.txt");
//     println!("{:?}", f6);
//     println!("{}", f6);
//     match f6.open() {
//         Ok(len) => println!("File opened with length: {}", len),
//         Err(e) => println!("Error: {}", e),
//     }
//     let mut f7 = File::new("seventh.txt");
//     match f7.close() {
//         Ok(msg) => println!("{}", msg),
//         Err(e) => println!("Error: {}", e),
//     }
//     // match f7.close() {
//     //     Ok(msg) => println!("{}", msg),
//     //     Err(e) => println!("Error: {}", e),
//     // }
//     println!("{:?}", f7);
// }

// struct Sheep {
//     naked: bool,
//     name:  String,
// }

// impl Sheep {
//     fn is_naked(&self) -> bool {
//         self.naked
//     }
//     fn shear(&mut self) {
//         if self.is_naked() {
//             println!("{} is already naked...", self.name());
//         } else {
//             println!("{} gets a haircut!", self.name);
//             self.naked = true;
//         }
//     }
// }

// trait Animal {
//     fn new(name: String) -> Self;
//     fn name(&self) -> String;
//     fn noise(&self) -> String;
//     fn talk(&self) {
//         println!("{} says {}", self.name(), self.noise());
//     }
    
// }

// impl Animal for Sheep {
//     fn new(name: String) -> Sheep {
//         Sheep {
//             name: name,
//             naked: false,
//         }
//     }
//     fn name(&self) -> String {
//         self.name.clone()
//     }
//     fn noise(&self) -> String {
//         if self.is_naked() {
//             String::from("baaaaah?")
//         } else {
//             String::from("baadasfadfadfh!")
//         }
//     }
//     fn talk(&self) {
//         println!("{} pauses briefly... {}", self.name(), self.noise());
//     }
// }

// fn main() {
//     let mut dolly: Sheep = Animal::new(String::from("Dolly"));
//     dolly.talk();
//     dolly.shear();
//     dolly.talk();
// }

// fn main(){
//     println!(
//         "\n▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓\
//          ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓\n"
//     );
// }

// 特征对象

// pub struct Button{
//     pub width: u32,
//     pub height: u32,
//     pub label: String,
// }

// impl Draw for Button{
//     fn draw(&self){
//         println!("draw button");
//     }
// }

// pub struct SelectBox{
//     pub width: u32,
//     pub height: u32,
//     pub options: Vec<String>,
// }

// impl Draw for SelectBox{
//     fn draw(&self){
//         println!("draw select box");
//     }
// }

// pub trait Draw{
//     fn draw(&self);
// }

// pub struct Screen{
//     pub components: Vec<Box<dyn Draw>>,
// }

// impl Screen{
//     pub fn run(&self){
//         for component in self.components.iter(){
//             component.draw();
//         }
//     }
// }

// fn main(){
//     let screen = Screen{
//         components: vec![
//             Box::new(SelectBox{
//                 width: 75,
//                 height: 10,
//                 options: vec![
//                     String::from("yes"),
//                     String::from("no"),
//                 ],
//             }),
//             Box::new(Button{
//                 width: 50,
//                 height: 10,
//                 label: String::from("ok"),
//             }),
//         ],
//     };
//     screen.run();
// }


// use std::fmt::Display;

// fn main() {
//     let mut v = vec![1, 2, 3];
//     for i in &mut v {
//         *i += 10
//     }

//     println!("{:?}", v);    
// }

// #[derive(Debug)]
// enum IpAddr{
//     V4(String),
//     V6(String),
// }

// fn show_addr(ip: &IpAddr){
//     println!("{:?}", ip);
// }

// fn main(){
//     let v = vec![
//         IpAddr::V4(String::from("127.0.0.1")),
//         IpAddr::V6(String::from("::1")),
//     ];
    
//     for ip in &v {
//         show_addr(ip);
//     }
    
// }

// trait IpAddr {
//     fn display(&self)-> String;
// }

// struct V4(String);
// struct V6(String);
// impl IpAddr for V4 {
//     fn display(&self) -> String {
//         format!("V4: {}", self.0)
//     }
// }

// impl IpAddr for V6 {
//     fn display(&self) -> String {
//         format!("V6: {}", self.0)
//     }
// }

// fn main(){
//     let V : Vec<Box<dyn IpAddr>> = vec![
//         Box::new(V4(String::from("127.0.0.1"))),
//         Box::new(V6("::1".to_string())),
//      ];

//      for ip in V {
//          println!("{}", ip.display());
//      }
// }

// fn main(){
//     let mut v = Vec::with_capacity(10);
//     v.extend(0..=9);
    
//     println!("{}\t{}", v.len(),v.capacity());

//     v.reserve(100);        // 调整 v 的容量，至少要有 100 的容量
//     println!("Vector（reserve） 长度是: {}, 容量是: {}", v.len(), v.capacity());

//     v.shrink_to_fit();     // 释放剩余的容量，一般情况下，不会主动去释放容量
//     println!("Vector（shrink_to_fit） 长度是: {}, 容量是: {}", v.len(), v.capacity());
// }


// fn main() {
//     let v = vec![11, 22, 33, 44, 55];
//     let slice = v[1..=3];
//     assert_eq!(slice, [22, 33, 44]);
// }

// fn main(){
//     let mut vec = vec![1,5,10,2,15];
//     vec.sort_unstable_by(|a,b|b.cmp(a));
//     println!("{:?}", vec);
// }


// #[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
// struct Person {
//     name: String,
//     age: u32,
// }

// impl Person {
//     fn new(name: String, age: u32) -> Person {
//         Person { name, age }
//     }
// }

// fn main() {
//     let mut people = vec![
//         Person::new("Zoe".to_string(), 25),
//         Person::new("Al".to_string(), 60),
//         Person::new("Al".to_string(), 30),
//         Person::new("John".to_string(), 1),
//         Person::new("John".to_string(), 25),
//     ];

//     people.sort_unstable();

//     println!("{:?}", people);
// }

// fn main() {
//     let arr: [u8; 3] = [1, 2, 3];
    
//     let v = Vec::from(arr);
//     is_vec(&v);

//     let v = vec![1, 2, 3];
//     is_vec(&v);

//     // vec!(..) and vec![..] are same macros, so
//     let v = vec!(1, 2, 3);
//     is_vec(&v);
    
//     // in code below, v is Vec<[u8; 3]> , not Vec<u8>
//     // USE `for` to rewrite the below code 
//     let mut v1 = Vec::new();
//     for i in &v {
//         v1.push(*i)
//     }
//     is_vec(&v1);
 
//     assert_eq!(format!("{:?}",v), format!("{:?}",v1));

//     println!("Success!")
// }

// fn is_vec(_v: &Vec<u8>) {}


// fn main() {
//     // array -> Vec
//     // impl From<[T; N]> for Vec
//     let arr = [1, 2, 3];
//     let v1 = Vec::from(arr);
//     let v2: Vec<i32> = arr.into();
 
//     assert_eq!(v1, v2);
 
    
//     // String -> Vec
//     // impl From<String> for Vec
//     let s = "hello".to_string();
//     let v1: Vec<u8> = s.into_bytes();

//     let s = "hello".to_string();
//     let v2 = s.into_bytes();
//     assert_eq!(v1, v2);

//     // impl<'_> From<&'_ str> for Vec
//     let s = "hello";
//     let v3 = Vec::__(s);
//     assert_eq!(v2, v3);

//     // 迭代器 Iterators 可以通过 collect 变成 Vec
//     let v4: Vec<i32> = [0; 10].into_iter().collect();
//     assert_eq!(v4, vec![0; 10]);

//     println!("Success!")
//  }


// fn main(){
//     let mut my_gems = HashMap::new();

//     my_gems.insert("A".to_string(),1);
//     my_gems.insert("B".to_string(),2);
//     my_gems.insert("C".to_string(),3);
//     println!("{:?}", my_gems);
// }

// use std::collections::{HashMap, HashSet};

// fn main() {
//     use std::collections::HashMap;

//     let name = String::from("Sunface");
//     let age = 18;

//     let mut handsome_boys = HashMap::new();
//     handsome_boys.insert(&name, &age);

//     println!("因为过于无耻，{}已经被从帅气男孩名单中除名", name);
//     println!("还有，他的真实年龄远远不止{}岁", age);
//     println!("{:?}",handsome_boys)
// }

// fn main(){
//     use std::collections::HashMap;

//     let mut scores = HashMap::new();
//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);
    
//     let team_name = String::from("Blue");
//     let score = scores.get(&team_name).copied().unwrap_or(0);
    
//     println!("{:?}", score);
// }

// fn main(){
//     use std::collections::HashMap;

//     let mut scores = HashMap::new();
//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);
    
//     let old_score = scores.insert(String::from("Blue"), 25);
//     println!("The old score of Blue is: {:?}", old_score);
//     println!("{:?}", scores);

//     let new = scores.entry(String::from("Blue")).or_insert(50);
//     println!("{:?}", new);

//     let text = "hello world wonderful world";
//     let mut map = HashMap::new();
    
//     for word in text.split_whitespace() {
//         let count = map.entry(word).or_insert(0);
//         *count += 1;
//     }

//     println!("{:?}", map);

// }

// fn main(){
    
//         let r;
    
//         {
//             let x = 5;
//             r = &'x;
//         }
    
//         println!("r: {}", r);

    
// }

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// fn main() {
//     let string1 = String::from("long string is long");

//     {
//         let string2 = String::from("xyz");
//         let result = longest(&string1, &string2);
//         println!("The longest string is {}", result);
//     }
// }

// #[derive(Debug)]
// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }

// fn main(){
//     let novel = String::from("Call me Ishmael. Some years ago... ");
//     let first_sentence = novel.split('.').next().expect("Could not find a '.'");
//     let i = ImportantExcerpt { part: first_sentence };
//     println!("{:?}", i);
// }

// 生命周期，消除规则

// 1. 每一个引用参数

// fn first_word(s: &str) -> &str{}

// fn first_word<'a>(s: &'a str) -> &'a str{}

// fn longest(x: &str, y: &str) -> &str {}

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {}

// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }


// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }

// impl <'a> ImportantExcerpt<'a> {
//     fn level(&self) -> i32 {
//         3
//     }
// }

// use std::fmt::Display;

// fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
//     where T: Display
// {
//     println!("Announcement! {}", ann);
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// use std::fs::File;
// use std::io;
// use std::io::Read;

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut f = File::open("hello.txt")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }

// fn main(){
//     let _ = read_username_from_file();
// }


// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {
//             println!("Added to waitlist!");
//         }
//     }
// }

// use crate::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
// }


// fn main(){

//     eat_at_restaurant();
//     let s = String::from("hello");
//     println!("{}", s);
// }


// use serde::{Serialize, Deserialize};

// #[derive(Serialize, Deserialize, Debug)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// fn main() {
//     let point = Point { x: 1, y: 2 };

//     let serialized = serde_json::to_string(&point).unwrap();
//     println!("serialized = {}", serialized);

//     let deserialized: Point = serde_json::from_str(&serialized).unwrap();
//     println!("deserialized = {:?}", deserialized);
// }


// #[derive(Debug)]
// struct Foo;

// impl Foo {
//     fn mutate_and_return(&mut self) -> &Self {
//         &*self
//     }
//     fn share(&self){

//     }
// }

// fn main(){
//     let mut foo = Foo;
//     foo.share();
//     foo.mutate_and_return();
// }

// fn main(){
//     let x = String::from("hello");
//     let y = &*&x;
//     println!("{}",y)

// }

// fn main(){

//     let x =1 ;
//     let sum = |y| x+y;
//     println!("{}",sum(2));
// }

// use std::thread;
// use std::time::Duration;
// // 开始健身，好累，我得发出声音：muuuu...
// fn muuuuu(intensity: u32) -> u32 {
//     println!("muuuu.....");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

// fn workout(intensity: u32, random_number: u32) {
//     if intensity < 25 {
//         println!(
//             "今天活力满满，先做 {} 个俯卧撑!",
//             muuuuu(intensity)
//         );
//         println!(
//             "旁边有妹子在看，俯卧撑太low，再来 {} 组卧推!",
//             muuuuu(intensity)
//         );
//     } else if random_number == 3 {
//         println!("昨天练过度了，今天还是休息下吧！");
//     } else {
//         println!(
//             "昨天练过度了，今天干干有氧，跑步 {} 分钟!",
//             muuuuu(intensity)
//         );
//     }
// }

// fn main() {
//     // 强度
//     let intensity = 10;
//     // 随机值用来决定某个选择
//     let random_number = 7;

//     // 开始健身
//     workout(intensity, random_number);
// }

 
// #[cfg(test)]

// mod tests{
//     #[test]

//     fn Iterator_sum() {
//         let v1 = vec![1, 2, 3];
//         // let mut v1_iter = v1.iter();

//         // let total: i32 = v1_iter.sum();

//         // assert_eq!(total, 6);

//         let v2 : Vec<_> 
//     }
// }

// use rand::prelude::*;
// use rand_chacha::ChaCha20Rng;

// fn main(){
//     let mut rng = ChaCha20Rng::from_entropy();
//     println!("{:?}", rng.gen::<f64>());
// }


// use std::ops::Add;

// #[derive(Debug)]
// struct Point<T: Add<T, Output = T>> {
//     x: T,
//     y: T,
// }

// impl<T: Add<T, Output = T>> Add for Point<T> {
//     type Output = Point<T>;

//     fn add(self, rhs: Self) -> Self::Output {
//         Point {
//             x: self.x + rhs.x,
//             y: self.y + rhs.y,
//         }
//     }
// }

// fn add<T: Add<T, Output = T>>(a: T, b: T) -> T {
//     a + b
// }

// fn main() {
//     let p1 = Point { x: 1, y: 2 };
//     let p2 = Point { x: 3, y: 4 };
//     let p3 = add(p1,p2);
//     println!("{:?}", p3);

//     let p4 = Point { x: 1.0, y: 2.0 };
//     let p5 = Point { x: 3.0, y: 4.0 };
//     let p6 = add(p4,p5);
//     println!("{:?}", p6);
// }

// pub struct Button {
//     pub width: u32,
//     pub height: u32,
//     pub label: String,
// }

// struct SelectBox {
//     width: u32,
//     height: u32,
//     options: Vec<String>,
// }

// trait Draw {
//     fn draw(&self) -> String;
// }

// impl Draw for u8 {
//     fn draw(&self) -> String {
//         format!("u8: {}", *self)
//     }
// }

// impl Draw for f64 {
//     fn draw(&self) -> String {
//         format!("f64: {}", *self)
//     }
// }

// fn draw1(x: Box<dyn Draw>) {
//     x.draw();
// }

// fn draw2(x: &dyn Draw) {
//     x.draw();
// }


// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> Screen<T>
//     where T: Draw {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }


// fn main(){
//     let screen = Screen {
//         components: vec![
//             Box::new(
//                 SelectBox {
//                     width: 75,
//                     height: 10,
//                     options: vec![
//                         String::from("Yes"),
//                         String::from("Maybe"),
//                         String::from("No")
//                     ],
//                 }
//             ),
//             Box::new(
//                 Button {
//                     width: 50,
//                     height: 10,
//                     label: String::from("OK"),
//                 }
//             ),
//         ]
//     };

//     screen.run();
// }

// 定义了两个方法，但 eat 方法有默认的实现
// trait Animal {
//     // 有声明，有实现（函数体）
//     fn eat(&self) {
//         println!("Animal 在吃东西")
//     }
//     // 只有声明，没有实现（不存在函数体）
//     fn drink(&self);
// }

// struct Dog {
//     name: String,
//     category: &'static str
// }

// struct Cat {
//     name: String,
//     category: &'static str
// }

// impl Animal for Dog {
//     fn eat(&self) {
//         println!("{} 在吃东西，它是一只 {}", self.name, self.category);
//     }
//     fn drink(&self) {
//         println!("{} 在喝饮料，它是一只 {}", self.name, self.category);
//     }
// }

// // 我们没有为 Cat 实现 eat 方法，但由于 eat 方法有默认实现，不实现也没关系
// // 因此一个类型如果要实现某个 trait，那么必须实现该 trait 里面所有没有默认实现的方法
// impl Animal for Cat {
//     fn drink(&self) {
//         println!("{} 在喝饮料，它是一只 {}", self.name, self.category);
//     }
// }

// fn main() {
//     let dog = Dog{name: "旺财".to_string(), category: "小狗"};
//     let cat = Cat{name: "翠花".to_string(), category: "小猫"};
//     // eat(&dog);  // 旺财 在吃东西，它是一只 小狗
//     // // Cat 没有实现 eat 方法，此时调用的是 trait 的默认实现
//     // eat(&cat);  // Animal 在吃东西
//     // drink(&dog);  // 旺财 在喝饮料，它是一只 小狗
//     // drink(&cat);  // 翠花 在喝饮料，它是一只 小猫
//     dog.eat();
//     cat.drink();
// }


// pub trait People {
//     fn get_name(&self) -> &str;
//     fn set_name(&self);
//     fn transfer_ownership(self);
//     fn new(name: String) -> Self;
// }

// struct Girl {
//     name: String,
// }

// impl People for Girl {
//     fn get_name(&self) -> &str {
//         &self.name
//     }
//     fn set_name(&self) {
//         println!("Girl set_name");
//     }
//     fn transfer_ownership(self) {
//         println!("Girl transfer_ownership");
//     }
//     fn new(name: String) -> Self {
//         Girl { name }
//     }
// }

// fn main(){
//     let girl = Girl::new("小花".to_string());
//     println!("{}", girl.get_name());
//     girl.set_name();
//     girl.transfer_ownership();
// }


// pub trait SomeTrait {
//     const LEN: u32 = 123;
// }

// struct A;
// struct B;

// impl SomeTrait for A {
//     const LEN: u32 = 456;
// }

// impl SomeTrait for B {}

// fn main() {
//     println!("{}", A::LEN);
//     println!("{}", B::LEN);
// }

// fn bar<T, E, W> ( a: T ,b: E, c: W) -> i32
//     where
//         T: SomeTrait,
//         E: SomeTrait,
//         W: SomeTrait,
// {
    
// }


// fn main() {
//     let mark_twain: &str = "Samuel Clemens";
//     print_author(mark_twain);
//   }
//   fn print_author(author: & str) {
//     println!("{}", author);
//   }

// use std::fmt::Display;
// fn main() {
//     let mark_twain = "Samuel Clemens".to_string();
//     print(&mark_twain);
// }

// fn print<T: Display + 'static>(message: &T) {
//     println!("{}", message);
// }

// use std::{slice::from_raw_parts, str::from_utf8_unchecked};

// fn get_memory_loction() -> (usize, usize) {
//     let string = "hello world";
//     let ptr = string.as_ptr() as usize;
//     let len = string.len();
//     (ptr, len)
// }

// fn get_str_at_location(ptr: usize, len: usize) -> &'static str {
//     unsafe { from_utf8_unchecked(from_raw_parts(ptr as *const u8, len)) }
//     // unsafe { from_utf8_unchecked(slice) }
// }

// fn main(){
//     let (ptr, len) = get_memory_loction();
//     let message = get_str_at_location(ptr, len);
    
//     println!(
//         "{ptr}\n{len}\n{message}"
//     )
// }


// fn main()  {
//     {
//         let static_string = "read only mem";
//         println!("static string: {static_string}")
//     }
//     println!("static1111 string: {static_string}");
// }

// fn main() {
//     let x = 1;
//     let sum = |y| x+y;
//     assert_eq!(sum(2),3);
// }


// use std::thread;
// use std::time::Duration;

// fn muuuuu(intensity: u32) -> u32 {
//     println!("muuuuuuuuuuuuuuuuuuuuuuuuuuuuuu");
//     thread::sleep(Duration::from_secs(5));
//     intensity
// }

// fn workout(intensity: u32, random_number: u32) {
//     if intensity < 25 {
//         println!(
//             "今天活力满满，先做 {} 个俯卧撑!",
//             muuuuu(intensity)
//         );
//         println!(
//             "旁边有妹子在看，俯卧撑太low，再来 {} 组卧推!",
//             muuuuu(intensity)
//         );
//     } else if random_number == 3 {
//         println!("昨天练过度了，今天还是休息下吧！");
//     } else {
//         println!(
//             "昨天练过度了，今天干干有氧，跑步 {} 分钟!",
//             muuuuu(intensity)
//         );
//     }
// }

// fn main() {
//     let intensity = 10;
//     let random_number = 7;
//     workout(intensity, random_number);
// }


// use std::thread;
// use std::time::Duration;

// fn workout(intensity: u32, random_number: u32) {
//     let action = || {
//         println!("do something");
//         thread::sleep(Duration::from_secs(1));
//         intensity
//     };
    
//     if intensity < 25 {
//         println!(
//             "今天活力满满，先做 {} 个俯卧撑!",
//             action()
//         );
//         println!(
//             "旁边有妹子在看，俯卧撑太low，再来 {} 组卧推!",
//             action()
//         );
//     } else if random_number == 3 {
//         println!("昨天练过度了，今天还是休息下吧！");
//     } else {
//         println!(
//             "昨天练过度了，今天干干有氧，跑步 {} 分钟!",
//             action()
//         );
//     }
    
// }

// fn main() {
//     let intensity = 10;
//     let random_number = 7;
//     workout(intensity, random_number);
// }

// fn main() {
//     let sum::<T> = |x: T, y: T| x + y;
//     let v = sum(1,2);
//     let s = sum(1.1, 2.3);
//     println!("{v}, {s}")
// }



// #[derive(Debug, PartialEq, Copy, Clone)]
// enum ShirtColor {
//     Red,
//     Blue,
// }

// struct Inventory {
//     shirts: Vec<ShirtColor>,
// }

// impl Inventory {
//     fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
//         user_preference.unwrap_or_else(|| self.most_stocked())
//     }

//     fn most_stocked(&self) -> ShirtColor {
//         let mut num_red = 0;
//         let mut num_blue = 0;
        
//         for &shirt in self.shirts.iter() {
//             match shirt {
//                 ShirtColor::Red => num_red += 1,
//                 ShirtColor::Blue => num_blue += 1,
//             }
//         }
        
//         if num_red > num_blue {
//             ShirtColor::Red
//         } else {
//             ShirtColor::Blue
//         }
//     }
// }

// fn main() {
//     let store = Inventory {
//         shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
//     };
//     let user_pref1 = Some(ShirtColor::Red);
//     let giveaway1 = store.giveaway(user_pref1);
//     println!(
//         "用户 {:?} 得到的颜色是 {:?}",
//         user_pref1, giveaway1
//     );
//     let user_pref2 = None;
//     let giveaway2 = store.giveaway(user_pref2);
//     println!(
//         "用户 {:?} 得到的颜色是 {:?}",
//         user_pref2, giveaway2
//     );
//     assert_eq!(giveaway1, ShirtColor::Red);
// }

// fn main() {
//     let v1 = vec![1,2,3,4];
//     for val in v1.iter() {
//         print!("{val}")
//     }
// }

// pub trait Iterator {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>;
// }

// fn main() {
//     // let arr = [1, 2, 3];
//     // let mut arr_iter = arr.into_iter();

//     // assert_eq!(arr_iter.next(), Some(1));
//     // assert_eq!(arr_iter.next(), Some(2));
//     // assert_eq!(arr_iter.next(), Some(3));
//     // assert_eq!(arr_iter.next(), None);

//     let values = vec![1, 2, 3];

//     {
//     let result = match IntoIterator::into_iter(values) {
//         mut iter => loop {
//             match iter.next() {
//                 Some(x) => { println!("{}", x); },
//                 None => break,
//             }
//         },
//     };
//     result
// }
    
// }

// impl<I: Iterator> IntoIterator for I {
//     type Item = I::Item;
//     type IntoIter = I;

//     fn into_iter(self) -> Self::IntoIter {
//         self
//     }
// }


// fn main() {
//     let v1: Vec<i32> = vec![1,2,3,4,5];
//     // v1.iter().for_each(|x| println!("{}", x+1));
//     let v2:Vec<_> = v1.iter().map(|x| x+1).collect();
//     println!("{:?}", v2);
    
// }

// use std::collections::HashMap;

// fn main() {
//     let names:Vec<&str> = vec!["2232","dfasdfas"];
//     let ages:Vec<i32> = vec![15, 14];
//     let folks: HashMap<&str, i32> = names.into_iter().zip(ages.into_iter()).collect();
    
//     println!("{:?}", folks);
// }


// struct Shoe {
//     size: u32,
//     style: String,
// }

// fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
//     shoes.into_iter().filter(|s| s.size == shoe_size).collect()
// }

// struct Counter {
//     count: u32,
// }

// impl Counter {
//     fn new() -> Counter {
//         Counter { count: 0 }
//     }
// }

// impl Iterator for Counter {
//     type Item = u32;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.count += 1;
//         if self.count < 6 {
//             Some(self.count)
//         } else {
//             None
//         }
//     }
// }

// fn main(){
//     let sum: u32 = Counter::new().zip(Counter::new().skip(1)).map(|(a, b)| a * b).filter(|x| x % 3 == 0).sum();
//     println!("{sum}")
// }


// fn main() {
//     let v = vec![1u64, 2, 3, 4, 5, 6];
//     let val = v.iter()
//         .enumerate()
//         // 每两个元素剔除一个
//         // [1, 3, 5]
//         .filter(|&(idx, _)| idx % 2 == 0)
//         .map(|(_, val)| val)
//         // 累加 1+3+5 = 9
//         .fold(0u64, |sum, acm| sum + acm);

//     println!("{}", val);

// }


// #![feature(test)]

// extern crate rand;
// extern crate test;

// fn sum_for(x: &[f64]) -> f64 {
//     let mut result: f64 = 0.0;
//     for i in 0..x.len() {
//         result += x[i];
//     }
//     result
// }

// fn sum_iter(x: &[f64]) -> f64 {
//     x.iter().sum::<f64>()
// }

// #[cfg(test)]
// mod bench {
//     use test::Bencher;
//     use rand::{Rng,thread_rng};
//     use super::*;

//     const LEN: usize = 1024*1024;

//     fn rand_array(cnt: u32) -> Vec<f64> {
//         let mut rng = thread_rng();
//         (0..cnt).map(|_| rng.gen::<f64>()).collect()
//     }

//     #[bench]
//     fn bench_for(b: &mut Bencher) {
//         let samples = rand_array(LEN as u32);
//         b.iter(|| {
//             sum_for(&samples)
//         })
//     }

//     #[bench]
//     fn bench_iter(b: &mut Bencher) {
//         let samples = rand_array(LEN as u32);
//         b.iter(|| {
//             sum_iter(&samples)
//         })
//     }
// }

// fn main() {
//     println!("Hello, world!");
// }

// fn main() {
//     let a: i32 = 10;
//     let b: u16 = 100;
  
//     if a < (b as i32) {
//       println!("Ten is less than one hundred.");
//     }
//   }

// fn main() {

//     let mut values: [i32;2] = [1,2];
//     let p1: *mut i32 = values.as_mut_ptr();
//     println!("{:?}", p1);
    
//     let first_address = p1 as usize;
//     let second_address = first_address + std::mem::size_of::<i32>();
    
//     println!("{:?}", first_address);
//     println!("{:?}", second_address);
    
//     let p2 = second_address as *mut i32;
    
//     unsafe {
//         *p2 += 1;

//     println!("{:?}", values);
//     }
// }


// 智能指针
// Box<T>

// fn main() {
//     let b = foo("world");
//     println!("{}", b);
// }

// fn foo(x: &str) -> String{
//     let a = "Hello, ".to_string() + x;
//     a
// }

// 智能指针实现了Deref 和Drop 特征；
// fn main() {
//     let a = Box::new(3);
//     println!("{:?}", a);
    
//     let b = *a + 1;
//     println!("{:?}", b);
// }

// fn main() {
//     let arr = [0;1000];
//     let arr1 = arr;
//     println!("{:?}", arr1.len());
//     println!("{:?}", arr.len());

//     let arr = Box::new([0;1000]);
//     let arr1 = arr;
    
//     println!("{:?}", arr1.len());

    
// }

// enum List {
//     Cons (i32, Box<List>),
//     Nil,
// }

// trait Draw{
//     fn draw(&self);
// }

// struct Button {
//     id: u32
// }

// impl Draw for Button {
//     fn draw(&self) {
//         println!("Button {}", self.id);
//     }
// }

// struct SelectBox {
//     width: u32,
// }

// impl Draw for SelectBox {
//     fn draw(&self) {
//         println!("SelectBox");
//     }
// }

// fn main() {
//     let elems: Vec<Box <dyn Draw>> = 
//         vec![
//         Box::new(Button{
//             id : 1
//         }),
//         Box::new(SelectBox{
//             width: 100,
//         })
//         ];
    
//     for e in elems.iter() {
//         e.draw();
//     }
    
// }

// fn main() {
//     let arr = vec![Box::new(1), Box::new(2)];
//     let (first, second) = (&arr[0], &arr[1]);
//     let sum = **first + **second;
//     println!("{:?}", sum);
// }

// #[derive(Debug)]
// struct Person {
//     name: String,
//     age: u8
// }

// impl Person {
//     fn new(name: String, age: u8) -> Self {
//         Person { name, age}
//     }

//     fn display(self: &mut Person, age: u8) {
//         let Person{name, age} = &self;
//     }
// }
// fn main() {
//     let mut p = Person::new("John".to_string(), 30);
//     p.display(30);
//     println!("{:?}", p);
// }




    
use serde::{Deserialize, Serialize};
use shardio::*;
use std::fs::File;
use anyhow::Error;

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, PartialOrd, Ord, Debug)]
struct DataStruct {
    a: u64,
    b: u32,
}

fn main() -> Result<(), Error>
{
    let filename = "test.shardio";
    {
        // Open a shardio output file
        // Parameters here control buffering, and the size of disk chunks
        // which affect how many disk chunks need to be read to
        // satisfy a range query when reading.
        // In this example the 'built-in' sort order given by #[derive(Ord)]
        // is used.
        let mut writer: ShardWriter<DataStruct> =
            ShardWriter::new(filename, 64, 256, 1<<16)?;

        // Get a handle to send data to the file
        let mut sender = writer.get_sender();

        // Generate some test data
        for i in 0..(2 << 16) {
            sender.send(DataStruct { a: (i%25) as u64, b: (i%100) as u32 });
        }

        // done sending items
        sender.finished();

        // Write errors are accessible by calling the finish() method
        writer.finish()?;
    }

    // Open finished file & test chunked reads
    let reader = ShardReader::<DataStruct>::open(filename)?;

     
    let mut all_items = Vec::new();

    // Shardio will divide the key space into 5 roughly equally sized chunks.
    // These chunks can be processed serially, in parallel in different threads,
    // or on different machines.
    let chunks = reader.make_chunks(5, &Range::all());

    for c in chunks {
        // Iterate over all the data in chunk c.
        let mut range_iter = reader.iter_range(&c)?;
        for i in range_iter {
            all_items.push(i?);
        }
    }

    // Data will be return in sorted order
    let mut all_items_sorted = all_items.clone();
    all_items.sort();
    assert_eq!(all_items, all_items_sorted);

    // If you want to iterate through the items in unsorted order.
    let unsorted_items: Vec<_> = UnsortedShardReader::<DataStruct>::open(filename)?.collect();
    // You will get the items in the order they are written to disk.
    assert_eq!(unsorted_items.len(), all_items.len());

    std::fs::remove_file(filename)?;
    Ok(())
}


// use std::fs::File;
// use std::io::{self, Write};

// #[derive(Debug)]
// struct DataStruct {
//     a: u64,
//     b: u32,
// }

// fn main() -> io::Result<()> {
//     let filename = "D:/Files/share/lishuangshuang/pipeline/learning/rust/learn1/test.txt";

//     // 创建或打开文件，覆盖已有内容
//     let mut file = File::create(filename)?;

//     // 生成并写入测试数据
//     for i in 0..(2 << 16) {
//         let data = DataStruct { a: (i % 25) as u64, b: (i % 100) as u32 };
//         writeln!(file, "{:?}, {:?}", data.a, data.b)?;
//     }

//     Ok(())
// }


// use std::fs::File;
// use std::io::{self, Write};

// struct DataStruct {
//     a: u8,
//     b: u16,
// }

// fn main(){
//     let filename = "D:/Files/share/lishuangshuang/pipeline/learning/rust/learn1/test.txt";
//     let mut file = File::create(filename).unwrap_or_else(
//         |err| {
//             eprintln!("Failed to create file: {}",err);
//             std::process::exit(1);
//         }
//     );
    
//     for i in 0..(power_of_two(16)) {
//         let data = DataStruct {
//             a: (i % 25) as u8,
//             b: (i % 100) as u16,
//         };

//         writeln!(file, "{:?}, {:?}", data.a, data.b).unwrap_or_else(|err| {
//             eprintln!("Failed to write to file: {}", err);
//             std::process::exit(1);
//         });
//     }
// }


// fn power_of_two(n: u32) -> u32 {
//     2 << n - 1
// }


// use std::fs::OpenOptions;
// use std::io;
// use std::io::prelude::*;

// fn append_to_file(
//     file_path: &str,
//     content: &str,
// ) -> io::Result<()> {
//     let mut file = OpenOptions::new()
//         .write(true)
//         .append(true)
//         .create(true)
//         .open(file_path).unwrap_or_else(|err| {
//             eprintln!("Failed to open file: {}", err);
//             std::process::exit(1);
//         });

//     file.write_all(content.as_bytes()).unwrap_or_else(|err| {
//         eprintln!("Failed to write to file: {}", err);
//         std::process::exit(1);
//     });

//     Ok(())
// }

// fn main() {
//     let file_path = "D:/Files/share/lishuangshuang/pipeline/learning/rust/learn1/test.txt";
//     let content = "This is some new content.";

//     match append_to_file(file_path, content) {
//         Ok(_) => println!("Content appended successfully."),
//         Err(err) => eprintln!("Error appending content: {}", err),
//     }
// }
