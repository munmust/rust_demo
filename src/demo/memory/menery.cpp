#include <iostream>
#include <memory>

using namespace std;

char* memory()
{
    unique_ptr<int> p1(new int(5));
    cout << *p1 << endl;
    auto stolen = move(p1); // 指针的转移，所有权转移
    cout << *p1 << endl;    // p1指向的内存已经被转移，运行时（编译期不会抛错）这里会报错

    char *a = "123"; // 变量a的作用域开始
    char *c = "xyz"; // 变量c的作用域开始
    return a;       // 变量a和c的作用域结束，却将a的地址返回
}
