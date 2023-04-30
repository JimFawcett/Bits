/*---------------------------------------------------------
  Bits::Cpp::src::Cpp_Data.cpp
  - Display creation and simple use of basic types
*/
#include <iostream>  // std::cout
#include <typeinfo>  // typeid
#include <memory>    // std::unique_ptr
#include <utility>   // move()

/*
  Static Data Types:
    byte, bool, int, char, char16_t, char32_t, wchar_t
    float, double
    array, pointer, reference
    struct, class, enum class
  Qualifiers:
    short, long, const, unsigned

  Library Types:
    std::string, std::vector<T>, std::unordered_map<K,V>, 
    std::unique_ptr<T>, std::shared_ptr<T>, and many more

  Operations:
    Primitive types can all be copied.
    Most library and user-defined types can be copied, moved, 
    and deleted by providing member constructors and destructor.
    Often compiler generated functions work well, but for 
    classes with pointer members developers must provide them.

  Processing:
    All types are static, operations run as native code, and no garbage
    collection is needed. Resources are returned at end of declr scope.
*/
/* C++ requires declaration before use */
template<typename T>
void showType(T t, const std::string &nm);
void showOp(const std::string& text);
void showLabel(const std::string& text, size_t n = 50);
void print(const std::string& txt = "");
void println(const std::string& txt = "");

void nl() {
  std::cout << std::endl;
}

int main() {
    showLabel(" Demonstrate C++ types");
    nl();

    /*-- objects live in stack frame, may have values in heap --*/
    
    showOp("initialize from literals");
    
    int t1{3}; 
    showType(t1, "t1");
    
    long long int t1a = 3;
    showType(t1a, "t1a");

    double d{3.1415927};
    showType(d, "d");

    std::string s{"a literal string"};  // char values in heap
    showType(s, "s");

    println("-- copy construct --");
    
    auto t1b = t1a;
    showType(t1b, "t1b");
    // double t1 = 3.1415927; // not allowed to redefine type of t1
    
    /*-- values live in heap when using new --*/
    
    println("-- store in heap --");
    
    double* dptr = new double{3.14159};
    showType(dptr, "dptr");
    showType(*dptr, "*dptr");
    delete(dptr);  // see unique_ptr, below
    
    /*-- control block lives in stack, char data live in heap --*/
    
    println("-- string: control block in stack, data in heap --");
    
    auto s2 = std::string("Hello Data");  // move ctor - rhs is temp
    showType(s2, "s2");
    
    println("-- C++ reference --");
    auto& rs2 = s2;      // create reference, no copy or move
    showType(rs2, "rs2");
    
    /*-- unique_ptr may not be copied but move allowed --*/
    
    println("-- unique_ptr owns data --");
    
    auto ptr = std::unique_ptr<int>(new int{-3});
    *ptr += 1;
    showType(std::move(ptr), "ptr");  // unique_ptr can't be copied so move
    
    /*-- original heap value deleted when ptr assigned --*/

    ptr = std::unique_ptr<int>(new int{-4});
    showType(*ptr, "*ptr"); // contents can be copied
    nl();

    showOp("heap data deleted when unique_ptr leaves scope");

    println("\n  That's all Folks!\n\n");
}
/*---------------------------------------------------------
  Truncates input string to n chars. Beware: side-effects.
  - used in showType which may generate long type names.
*/
std::string truncate(const std::string& str, size_t n = 40) {
  std::string tmp = str;
  if(tmp.size() < n) {
    return tmp;
  }
  tmp.resize(n);
  tmp += "...";
  return tmp;
}
/*---------------------------------------------------------
  Show call name, static type, value, and size
*/
template<typename T>
void showType(T t, const std::string &nm) {
  std::cout << "\n  " << nm;                // show name at call site
  std::cout << ": type: " << truncate(typeid(t).name());  // show type
  std::cout << "\n  value: " << t;          // show value
  std::cout << ",  size:  " << sizeof(t);   // show size on stack
  std::cout << "\n";
}
/*---------------------------------------------------------
  Show operation text surrounded by "---" strings
*/
void showOp(const std::string& text) {
  std::cout << "  --- " << text << " ---" << std::endl;
}
/*---------------------------------------------------------
  Show text surrounded by long lines of '-' characters
*/
void showLabel(const std::string& text, size_t n) {
  auto line = std::string(n, '-');
  std::cout << line << std::endl;
  std::cout << "  " << text << std::endl;
  std::cout << line << std::endl;
}
void print(const std::string& txt) {
  std::cout << "\n  " << txt;
}
void println(const std::string& txt) {
  std::cout << "\n  " << txt << "\n";
}
