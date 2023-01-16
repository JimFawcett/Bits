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
    by providing member constructors and destructor.  Often compiler 
    generation works well, but for classes with pointer members developers 
    must provide them.

  Processing:
    All types are static, operations run as native code, and no garbage
    collection is needed. Resources are returned at end of declr scope.
*/
/* C++ requires declaration before use */
template<typename T>
void showType(T t, const std::string &nm);
void print(const std::string& txt = "");
void println(const std::string& txt = "");

int main() {
    print(" Demonstrate C++ types");
    println("-----------------------");
    /*-- values live in stack frame --*/
    println("-- initialize from literals --");
    int t1 = int{3}; 
    showType(t1, "t1");
    long long int t1a = 3;
    showType(t1a, "t1a");

    println("-- copy construct --");
    auto t1b = t1a;
    showType(t1b, "t1b");
    // double t1 = 3.1415927; // not allowed to redefine type of t1
    
    /*-- values live in heap when using new --*/
    println("-- store in heap --");
    double* t2 = new double{3.14159};
    showType(t2, "t2");
    showType(*t2, "*t2");
    delete(t2);  // see unique_ptr, below
    
    /*-- control block lives in stack, char data live in heap --*/
    println("-- string: control block in stack, data in heap --");
    auto t3 = std::string("Hello Data");  // move ctor - rhs is temp
    showType(t3, "t3");
    println("-- C++ reference --");
    auto& t4 = t3;      // create reference, no copy or move
    showType(t4, "t4");
    
    /*-- unique_ptr may not be copied but move allowed --*/
    println("-- unique_ptr owns data --");
    auto t5 = std::unique_ptr<int>(new int{-3});
    showType(std::move(t5), "t5");  // unique_ptr can't be copied so move
    t5 = std::unique_ptr<int>(new int{-3});
    showType(*t5, "*t5"); // contents can be copied
    println("-- t5 deleted when unique_ptr leaves scope");
    std::cout << std::endl;
}

template<typename T>
void showType(T t, const std::string &nm) {
  std::cout << "\n  " << nm;                // show name at call site
  std::cout << ": type: " << typeid(t).name();  // show type
  std::cout << "\n  value: " << t;          // show value
  std::cout << ",  size:  " << sizeof(t);   // show size on stack
  std::cout << "\n";
}
void print(const std::string& txt) {
  std::cout << "\n  " << txt;
}
void println(const std::string& txt) {
  std::cout << "\n  " << txt << "\n";
}
