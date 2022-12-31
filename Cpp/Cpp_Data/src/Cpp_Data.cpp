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
    short, long, const, unsigned

  Operations:
    Primitive types can all be copied.
    Most library and user-defined types can be copied, moved, and deleted
    by providing member constructors and destructor.  Often compiler generation
    works well. For classes with pointer members developers must provide them.

  Processing:
    All types are static, operations run as native code, and no garbage
    collection is needed. Resources are returned at end of declr scope.
*/
/*-- macro turns name into string --*/
#define STRINGIFY_(x) #x     // # is a macro stringizer
#define TS(x) STRINGIFY_(x)  // operator

template<typename T>
void show(T t, const std::string &ts) {
  std::cout << "\n  " << typeid(t).name();  // show type
  std::cout << ", " << ts;                  // show name at call site
  std::cout << "\n  value: " << t;          // show value
  std::cout << ",  size:  " << sizeof(t);   // show size on stack
  std::cout << "\n";
}
int main() {
    std::cout << "\n  Demonstrate C++ types:";
    std::cout << "\n ------------------------";
    /*-- values live in stack frame --*/
    int t1 = int{3}; 
    show(t1, TS(t1));
    long long int t1a = 3;
    show(t1a, TS(t1a));
    // double t1 = 3.1415927; // not allowed to redefine type of t1
    
    /*-- values live in heap when using new --*/
    double* t2 = new double{3.14159};
    show(t2, TS(t2));
    show(*t2, TS(*t2));
    delete(t2);
    
    /*-- control block lives in stack, char data live in heap --*/
    auto t3 = std::string("Hello Data");  // move ctor - rhs is temp
    show(t3, TS(t3));
    
    auto& t4 = t3;      // reference
    show(t4, TS(t4));
    
    /*-- unique_ptr may not be copied but move allowed --*/
    auto t5 = std::unique_ptr<int>(new int{-3});
    show(std::move(t5), TS(t5));  // unique_ptr can't be copied so move
    t5 = std::unique_ptr<int>(new int{-3});
    show(*t5, TS(*t5)); // contents can be copied
    std::cout << std::endl;
}