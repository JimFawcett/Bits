/*---------------------------------------------------------
  Bits::Cpp::Cpp_Data::src::Cpp_Data.cpp
  - Display creation and simple use of basic types
*/
#include <iostream>  // std::cout
#include <typeinfo>  // typeid
#include <memory>    // std::unique_ptr
#include <utility>   // move()
#include <vector>
#include <unordered_map>
#include "Bits_DataAnalysis.h"
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
  Note:
  Find all Bits code, including this in
  https://github.com/JimFawcett/Bits
  You can clone the repo from this link.
-----------------------------------------------*/
/*-----------------------------------------------
  create and display values of primitive types
  in stack frame
*/
void initialize_primitives_from_literals() {

    showLabel("initialize primitives in stack from literals");
    
    /*-- objects live in stack frame, may have values in heap --*/
    
    int t1{3}; 
    showTypeScalar(t1, "t1");

    // double t1 = 3.1415927; // not allowed to redefine type of t1
        
    long long int t1a = 3;
    showTypeScalar(t1a, "t1a");

    double d{3.1415927};
    showTypeScalar(d, "d");

    std::string s {"a literal string"};  // char values in heap
    showTypeScalar(s, "s");
    nl();

    showOp("copy construct");
    
    auto s2 = s;
    showTypeScalar(s2, "s2");
    nl();
}
/*-----------------------------------------------
  create and display values of primitive types
  in heap
*/
void create_primitives_in_heap() {

    showLabel("store instances of primitives in heap");
    nl();

    /*-- values live in heap when using new --*/
    
    double* dptr = new double{3.14159};
    showTypeScalar(dptr, "dptr");
    showTypeScalar(*dptr, "*dptr");
    delete(dptr);  // see unique_ptr, below
    nl();

    /*-- control block lives in stack, char data live in heap --*/
    
    showOp("string: control block in stack, data in heap");
    
    auto s2 = std::string("Hello Data");  // move ctor - rhs is temp
    showTypeScalar(s2, "s2");
    nl();
    
    showOp("C++ reference");
    auto& rs2 = s2;      // create reference, no copy or move
    showTypeScalar(rs2, "rs2");
    nl();
    
    /*-- unique_ptr may not be copied but move allowed --*/
    
    showOp("unique_ptr owns data");
    
    auto ptr = std::unique_ptr<int>(new int{-3});
    *ptr += 1;
    showTypeScalar(std::move(ptr), "ptr");  // unique_ptr can't be copied so move
    
    /*-- original heap value deleted when ptr assigned --*/

    ptr = std::unique_ptr<int>(new int{-4});
    showTypeScalar(*ptr, "*ptr"); // contents can be copied
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
  nl();
}
/*-----------------------------------------------
  create and display values of user-defined type
  - If class members have correct copy and assignment
    semantics it is appropriate to let compiler define
    copy ctor and assign, move ctor and assign
    and dtor.
  - More details about classes in next bit.
*/
class Demo {
  public:
    Demo() = delete;
    Demo(const Demo&) = default;
    Demo(const std::string& name) : _name(name) {}
    Demo& operator=(const Demo&) = default;
    ~Demo() = default;
    std::string& name() {
      return _name;
    }
    void show();
  private:
    std::string _name;
};

void Demo::show() {
  std::cout << "  Demo {" << _name << " }\n";
}
/*-----------------------------------------------
  demonstrate user-defined type
*/
void demo_user_defined_type() {

  showLabel("demonstrate user-defined type");
  nl();

  Demo demo("demo");
  showType(demo, "demo");
  demo.show();
  nl();
}
/*-----------------------------------------------
  pass argument by value 
    - copies argument to function's stack frame
    - value changed to default in function
      has no side effects
-----------------------------------------------*/
template<typename T>
void pass_by_value(T t) {
  /*-- uses copy constructor --*/
  showType(t, "t:T");
  /* 
    statement below won't affect caller's t value
    since that was copied into function
  */
  t = T();  // value change uses T copy assignment
}
/*-----------------------------------------------
  pass argument by reference 
    - copies reference to argument to function's
      stack frame
    - value changed to default in function
      has side effects
-----------------------------------------------*/
template<typename T>
void pass_by_ref(T& t) {
  /*-- direct copy of reference --*/
  showType(t, "T");
  /*
    statement below changes caller's t value 
    since a reference to that was copied into
    the function
  */
 t = T();  // value change uses T copy assignment
}
/*-----------------------------------------------
  demonstrate copy and pass by value
-----------------------------------------------*/
void demo_copy() {

  showLabel("demonstrate copy and pass-by-value");
  nl();

  showOp("direct integer copy");
  int i_src = 42;
  int i_dst = i_src;
  std::cout << "  source: " << i_src << "\n";
  std::cout << "  destin: " << i_dst << "\n";
  nl();

  showOp("direct vector copy");
  /*-- uses vector copy constructor --*/
  auto v_src = std::vector<int>{1, 2, 3, 4, 5};
  auto v_dst = v_src;
  std::cout << "  source:";
  showEnum(v_src, "v_src");
  std::cout << "  destin:";
  showEnum(v_dst, "v_dst");
  nl();

  showOp("pass-by-value copies integer");
  int i = 42;
  std::cout << "  before pass_by_value: " << i << "\n";
  pass_by_value(i);
  std::cout << "  after pass_by_value:  " << i << "\n";
  nl();

  showOp("pass-by-value copies vector using ctor");
  auto v = std::vector<int>{ 1, 2, 3, 2, 1 };
  std::cout << "  before pass_by_value:";
  showEnum(v, "v");
  pass_by_value(v);
  std::cout << "  after pass_by_value:";
  showEnum(v, "v");
  nl();

  showOp("pass-by-value copies unordered_map using ctor");
  auto map = std::unordered_map<std::string, int> { 
    {"zero", 0}, {"one", 1}, {"two", 2}  };
  std::cout << "  before pass_by_value:";
  showAssoc(map, "map");
  pass_by_value(map);
  std::cout << "  after pass_by_value:";
  showAssoc(map, "map");
  nl();
}
/*-----------------------------------------------
  demonstrate pass by reference
-----------------------------------------------*/
void demo_ref() {

  showLabel("demonstrate pass-by-reference");
  nl();

  showLabel("all demos have side effects", 35);
  nl();

  showOp("copy ref of integer");
  int i = 42;
  std::cout << "  before pass_by_ref: " << i << "\n";
  pass_by_ref(i);
  std::cout << "  after pass_by_ref:  " << i << "\n";
  nl();

  showOp("copy ref of vector");
  auto v = std::vector<int>{ 1, 2, 3, 2, 1 };
  std::cout << "  before pass_by_ref:";
  showEnum(v, "v");
  pass_by_ref(v);
  std::cout << "  after pass_by_ref:";
  showEnum(v, "v");
  nl();

  showOp("copy ref of unordered_map");
  auto map = std::unordered_map<std::string, int> { 
    {"zero", 0}, {"one", 1}, {"two", 2}  };
  std::cout << "  before pass_by_ref:";
  showAssoc(map, "map");
  pass_by_ref(map);
  std::cout << "  after pass_by_ref:";
  showAssoc(map, "map");
  nl();
}
/*-----------------------------------------------
  demonstrate move
-----------------------------------------------*/
void demo_move() {

  showLabel("demonstrate move");
  nl();

  showOp("move string");
  std::string s1 { "a string" };
  std::string s2 { " plus more" };
  std::string s = s1 + s2;  // rhs constructs sum then moves to lhs
  std::cout << "  s1 = " << "\"" << s1 << "\"" << "\n";
  std::cout << "  s2 = " << "\"" << s2 << "\"" << "\n";
  std::cout << "  s = s1 + s2 => " << "\"" << s << "\"" << "\n";
  nl();

  showOp("move vector");
  auto v = std::vector<int>{ 1, 2, 3, 2, 1 };
  std::cout << "  before move:";
  showEnum(v, "v");
  auto vm = std::move(v);
  std::cout << "  after move of v into vm:";
  showEnum(vm, "vm");
  /*-- v has been moved so not guarenteed to be in viable state --*/
  try {
    showEnum(v, "v");   // undefined behavior since resources moved, 
  }                     // works without exception using VS compiler
  catch(std::exception& ex) {
    std::cout << "\n  bad things happened\n";
  }
}
/*-----------------------------------------------
  demonstration starts here
*/
int main() {
    showLabel(" Demonstrate C++ types");
    nl();

    initialize_primitives_from_literals();
    create_primitives_in_heap();
    demo_stdlib_types();
    demo_user_defined_type();
    demo_copy();
    demo_ref();
    demo_move();

    println("\n  That's all Folks!\n\n");
}
