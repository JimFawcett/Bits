/*---------------------------------------------------------
  Bits::Cpp::src::Cpp_Data.cpp
  - Display creation and simple use of basic types
*/
#include <iostream>  // std::cout
#include <typeinfo>  // typeid
#include <memory>    // std::unique_ptr
#include <utility>   // move()
#include <vector>
#include <unordered_map>
#include "Bits_DataAnalysis.h"

/*-----------------------------------------------
  Note:
  Find all Bits code, including this in
  https://github.com/JimFawcett/Bits
  You can clone the repo from this link.
-----------------------------------------------*/
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
/*-----------------------------------------------
  create and display values of primitive types
  in stack frame
*/
void initialize_primitives_from_literals() {

    showLabel("initialize primitives in stack from literals");
    
    /*-- objects live in stack frame, may have values in heap --*/
    
    int t1{3}; 
    showType(t1, "t1");
    // double t1 = 3.1415927; // not allowed to redefine type of t1
        
    long long int t1a = 3;
    showType(t1a, "t1a");

    double d{3.1415927};
    showType(d, "d");

    std::string s{"a literal string"};  // char values in heap
    showType(s, "s");
    nl();

    showOp("copy construct");
    
    auto t1b = t1a;
    showType(t1b, "t1b");
    nl();
}
/*-----------------------------------------------
  create and display values of primitive types
  in heap
*/
void create_primitives_in_heap() {

    showLabel("store instances of primitives in heap");
    
    /*-- values live in heap when using new --*/
    
    double* dptr = new double{3.14159};
    showType(dptr, "dptr");
    showType(*dptr, "*dptr");
    delete(dptr);  // see unique_ptr, below
    nl();

    /*-- control block lives in stack, char data live in heap --*/
    
    showOp("string: control block in stack, data in heap");
    
    auto s2 = std::string("Hello Data");  // move ctor - rhs is temp
    showType(s2, "s2");
    nl();
    
    showOp("C++ reference");
    auto& rs2 = s2;      // create reference, no copy or move
    showType(rs2, "rs2");
    nl();
    
    /*-- unique_ptr may not be copied but move allowed --*/
    
    showOp("unique_ptr owns data");
    
    auto ptr = std::unique_ptr<int>(new int{-3});
    *ptr += 1;
    showType(std::move(ptr), "ptr");  // unique_ptr can't be copied so move
    
    /*-- original heap value deleted when ptr assigned --*/

    ptr = std::unique_ptr<int>(new int{-4});
    showType(*ptr, "*ptr"); // contents can be copied
    nl();

    showOp("heap data deleted when unique_ptr leaves scope");
    nl();
}
/*-----------------------------------------------
  create and display values of std lib types
  - uses showTypeEnum for sequential containers
  - uses showTypeAssoc for associative containers
*/
void demo_stdlib_types() {

  showLabel("demo std library types");
  std::vector<int> v { 1, 2, 3, 2, 1};
  showTypeEnum(v, "v");

  std::unordered_map<std::string, int> m {
    {"zero", 0}, {"one", 1}, {"two", 2}
  };
  showTypeAssoc(m, "m");
}
int main() {
    showLabel(" Demonstrate C++ types");
    nl();

    initialize_primitives_from_literals();
    create_primitives_in_heap();
    demo_stdlib_types();

    println("\n  That's all Folks!\n\n");
}
