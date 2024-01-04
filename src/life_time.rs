// #[derive(Debug)]

struct A<'a> {
    a: &'a str,
}

impl<'a> A<'a> {
    fn split_first(s: &'a str) -> &'a str {
        s.split(',').next().expect("could not find a ','")
    }
    fn new(s: &'a str) -> Self {
        Self {
            a: Self::split_first(s),
        }
    }
}

pub fn life_time() {
    /* 悬垂指针 */
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("r: {}", r);

    /* 块作用域 */
    // let a = 10;
    // {
    //     let a2 = 12;
    // }
    // println!("{}", a2);

    /* Match */
    // let a: Option<String> = Some("hello".to_string());
    // match a { // a的所有权被移动到match中
    //     Some(s) => println!("{}", s),
    //     None => {}
    // }
    // println!("{:?}", a); // a的所有权已经转移，这里不能再使用a

    /* 循环 */
    // let a = vec![1, 2, 3];
    // for i in a {
    //     // a的所有权被移动到for中
    //     println!("{:?}", a) // a的所有权已经转移，这里不能再使用a
    // }

    /* if */
    // let a = Some("hello".to_string());
    // if let Some(s) = a {
    //     print!("{:?}", a)
    // }

    // 为什么需要
    // let s1 = String::from("hello");
    // let s1_r = &s1;
    // {
    //     let s2 = String::from("world");
    //     let res = comp(s1_r, &s2);
    //     println!("{}", res);
    // }

    // 无参数返回
    // let a = no_param_return();
    // println!("{:?}", a);

    //返回参数必须和参数匹配
    // let a = "hello";
    // let b = "world";
    // let c = no_couple(a, b);

    // 结构体
    // 方法定义的生命周期
    // let w = String::from("hello,rust");
    // let first = w.split(',').next().expect("could not find a ','");
    // let s = A::new(w.as_str());
    // assert_eq!("hello", s.a);

    // 静态生命周期
    let x = "12";
    let y = x; // x给y，x还能接着用，这边是按位复制，不是移动语义
    assert_eq!(x, y);
}

// fn longest(x: &str, y: &str) -> &str {
//     let S = x + y;
//     &S
// }

// fn no_param_return<'a>() -> &'a String {
//     let mut s = "hello".to_string();
//     for i in 0..3 {
//         s.push_str(" rust");
//     }
//     &s
// }

// fn no_couple<'a>(a: &'a str, b: &'a str) -> &'a str {
//     let strs = String::from("hello");
//     return strs.as_str();
// }

// fn comp(s1: &str, s2: &str) -> &str {
// fn comp<'a>(s1: &'a str, s2: &'a str) -> &'a str {
// fn comp<'a, 'b: 'a>(s1: &'a str, s2: &'b str) -> &'a str {
//     if s1.len() > s2.len() {
//         s1
//     } else {
//         s2
//     }
// }
