// Cpp_Data.cpp

#include <iostream>  // std::cout
#include <typeinfo>  // typeid
#include <memory>    // std::unique_ptr
#include <utility>   // move()

/*
  Static Data Types:
    byte, bool, int, char, char16_t, char32_t, wchar_t
    float, double
    array, pointer, reference
    struct, class, enum class, using
  Qualifiers:
    short, long, const
*/
#define STRINGIFY_(x) #x     // # is a macro stringizer
#define TS(x) STRINGIFY_(x)  // operator

template<typename T>
void show(T t, const std::string &ts) {
  std::cout << "\n  " << typeid(t).name();
  std::cout << ", " << ts;
  std::cout << "\n  value: " << t;
  std::cout << ",  size:  " << sizeof(t);
  std::cout << "\n";
}
int main() {
    std::cout << "\n  Demonstrate C++ types:";
    std::cout << "\n ------------------------" << "\n";

    int t1 = int{3};     // lives in stack frame
    show(t1, TS(t1));
    long long int t1a = 3;
    show(t1a, TS(t1a));
    
    double* t2 = new double{3.14159}; // value lives in heap
    show(t2, TS(t2));
    show(*t2, TS(*t2));
    
    auto t3 = std::string("Hello Data");
    show(t3, TS(t3));
    
    auto& t4 = t3;      // reference
    show(t4, TS(t4));
    
    auto t5 = std::unique_ptr<int>(new int{-3});
    show(move(t5), TS(t5));  // can't be copied
    std::cout << std::endl;
}