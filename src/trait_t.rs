use std::ops::Add;
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Duck;
#[derive(Debug)]
struct Pig;
trait Fly {
    fn fly(&self) -> bool;
}
impl Fly for Duck {
    fn fly(&self) -> bool {
        true
    }
}
impl Fly for Pig {
    fn fly(&self) -> bool {
        false
    }
}

fn fly_static<T: Fly>(s: T) -> bool {
    s.fly()
}
fn fly_dyn(s: &dyn Fly) -> bool {
    s.fly()
}

// impl 
fn fly_2(s: impl Fly) -> bool {
    s.fly()
}

fn fly_3(s: impl Fly) -> impl Fly {
    s
}

// 关联类型
// pub trait Add<RHS = Self> {
//     // 关联起来了
//     type Output;
//     fn add(self, rhs: RHS) -> Self::Output;
// }

// impl Add for i32 {
//     type Output = i32;
//     fn add(self, rhs: Self) -> Self::Output {
//         self + rhs
//     }
// }

// impl Add<&str> for String {
//     type Output = String;
//     fn add(mut self, other: &str) -> Self::Output {
//         self.push_str(other);
//         self
//     }
// }

// 孤儿规则

// impl Add<u64> for i32 {
//     type Output = u64;
//     fn add(self, rhs: u64) -> Self::Output {
//         (self as u64) + rhs
//     }
// }

impl Add for Point<i32> {
    type Output = Point<i32>;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub fn trait_t() {
    let p = Point::new(1, 2);
    let p1 = Point::new('1', '2');
}

pub fn trait_main() {
    // 抽象

    // let duck = Duck;
    // let pig = Pig;
    // duck.fly();
    // pig.fly();
    // // 静态分发：编译器会生成特殊代码
    // fly_static::<Duck>(duck);
    // // 动态分发；运行时进行类型的查找
    // fly_dyn(&pig);

    // 孤儿规则
    let p1 = Point::new(1, 2);
    let p2 = Point::new(3, 4);
    let p3 = p1 + p2;
    println!("{:?}", p3);
}

fn fn1<T>(x: T) {
    x;
}
fn fn2<T>(x: T) -> T {
    x
}
fn fn2_1(x: i32) -> i32 {
    x
}
fn fn2_2(x: &'static str) -> &'static str {
    x
}
fn main() {
    fn2_1(1);
    fn2_2("hello");
}

struct A;
trait ATrait {
    fn fnA(&self);
}

impl ATrait for A {
    fn fnA(&self) {
        println!("A");
    }
}

fn static_dispatch<T>(t: &A) {
    t.fnA();
}
fn dynamic_dispatch(t: &dyn ATrait) {
    t.fnA();
}

pub struct TraitObject {
    pub data: *mut (),
    pub vtable: *mut (),
}
