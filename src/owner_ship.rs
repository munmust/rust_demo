#[derive(Debug)]
// #[derive(Debug, Copy, Clone)]
struct A {
    a: u32,
    b: i32,
}
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}
pub fn owner_ship() {
    // 所有权
    // let a1 = A { a: 1, b: 2 };
    // let mut a2: &str = "aa";
    // let _b = a1;
    // println!("{:?}", a1);

    // Box的所有权
    // let x = Box::new(5);
    // let y = x;
    // println!("{:?}", x);

    // // 当Option和元组中元素都是复制语义类型，元组也可以自动Copy
    // let a2 = ("a".to_string(), "b".to_string());
    // let b2 = a2;
    // let c2 = (1, 2, 3);
    // let d2 = c2;
    // println!("{:?}", a2);

    // 一个大的例子
    // let mut v = Vec::new(); // v获得Vec空间的所有权
    // v.push(Person {
    //     name: "Alice".to_string(), // Person的name获得String空间的所有权
    //     age: 20,
    // }); // Person的所有权转移到v中

    // println!("{:?}", v[0]);

    // 控制流
    // let mut x = vec![1, 2, 3];
    // let c = true;
    // if c {
    //     fn1(x);
    // } else {
    //     fn2(x);
    // }
    // fn3(x); // 报错，x的所有权已经转移

    // while f() {
    //     X(x);
    //    // x = vec![4, 5, 6];
    // }

    // 索引
    // let mut v = Vec::new();
    // for i in 101..105 {
    //     v.push(i.to_string());
    // }
    // let v1 = v[0];
    // let v2 = v[1];
    // println!("{:?}", v[0]);

    // 函数
    // let St=String::from("hello");
    // T1(St);
    // println!("{}",St);

    // /* 闭包 */
    // let a = "hello".to_string();
    // let b = |i: &str| a + i;
    // print!("{}", a);

    // 限制
    // let mut j = 5;
    // let l = &j;
    // let mut k = &mut j;
    // let mut k = &mut j;
    // let mut l = &mut j;
    // println!("{}, {}", k, l);

    // 规则
    let i = 20;
    let mut j = 5;
    // comp1(&j, &mut j);
    // comp1(&i, &mut j);

    // 解引用
    // let str1 = String::from("hello");
    // join(&str1);

    // let x = 10;
    // let r = &x;
    // println!("{:?}", r);

    // /* 所有权转移 */
    // let s = String::from("hello");
    // let ss: String = f1(s);
    // println!("{}", s);
}

fn fn1(x: Vec<i32>) {
    println!("{:?}", x);
}

fn fn2(x: Vec<i32>) {
    println!("{:?}", x);
}

fn fn3(x: Vec<i32>) {
    println!("{:?}", x);
}
fn f() -> bool {
    true
}
fn X(x: Vec<i32>) {
    println!("{:?}", x);
}

fn T1(strs: String) {
    println!("{}", strs);
}

fn T2(strs: &String, str2: &mut String) -> usize {
    println!("{}", strs);
    let _s = String::from("hello");
    // strs.push('a');
    // let mut str3 = *strs;
    // str3.push('a');
    // str2.push('a');
    strs.len()
    // &_s
}

fn f1(s: String) -> String {
    let w = " world".to_string();
    s + &w
}

fn comp1(i: &i32, j: &mut i32) {
    if *i > 10 {
        *j = 1;
    }
    if *i > 5 {
        *j *= 2
    }
}

// fn join(s: &String) -> String {
//     let appendStr = *s; // 会转移所有权
//     "hello".to_string() + &appendStr
// }
