fn c() {
    let p1 =Box::new(5);
    println!("p1 = {}", *p1);
    let stolen=p1; //p1进行了移动，所有权发生了改变
    println!("stolen = {}", *p1); // 发现空指针，编译报错
}

