/*---------------------------------------------------------
  Bits::Cpp::Cpp_Data::src::Cpp_Data.cpp
  - Display creation and simple use of basic types
*/
// import std;
#include <iostream>       // std::cout
#include <sstream>        // std::stringstream
#include <iomanip>        // std::setw, std::left 
#include <typeinfo>       // typeid
#include <memory>         // std::shared_ptr
#include <utility>        // move()
#include <array>          // std::array
#include <vector>         // std::vector
#include <deque>          // std::deque
#include <unordered_map>  // std::unordered_map
#include <optional>       // std::option
#include <functional>     // std::func
#include <format>         // std::format
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

  Requires std::C++20
-----------------------------------------------*/

/*-- convert hexidecimal to string ----------------------*/
template<typename T>
std::string hexToString(T h) {
  // format in-memory stringstream so formats are temporary
  std::ostringstream out;
  out << std::showbase << std::hex << int(h);
  return out.str();
}
/*-- wrap string-convertable in quotes-------------------  
  - works for literal and std::string instances
*/
template <typename T>
std::string quotedString(const T& t) {
  std::string str = t;
  return "\"" + str +"\"";
}
/*-- convert scalar to string ---------------------------*/
template <typename T>
std::string scalarToString(const T& scalar) {
  /* format in-memory stringstream so formats are temporary */
  std::ostringstream out;
  out.precision(3);
  out << std::showpoint;
  out << std::boolalpha;
  out << scalar;
  return out.str();
}
/*-- convert float to string ----------------------------*/
template <typename T>
std::string floatToString(const T& scalar) {
  /* format in-memory stringstream so formats are temporary */
  std::ostringstream out;
  out.precision(7);
  out << std::showpoint;
  out << std::boolalpha;
  out << scalar;
  return out.str();
}
/*-- convert sequential collection to string ------------*/
template <typename C>
std::string seq_collectionToString(const C& coll) {
  /* format in-memory stringstream so formats are temporary */
  std::ostringstream out;
  out.precision(3);
  out << std::showpoint;
  out << std::boolalpha;
  out << "[ ";
  /*-- show comma only in interior of sequence --*/
  bool first = true;
  for(auto item : coll) {
    if(first) {
      out << item;
      first = false;
    }
    else {
      out << ", " << item;
    }
  }
  out << " ]";
  return out.str();
}
/*-- convert associative collection to string -----------*/
template <typename C>
std::string assoc_collectionToString(const C& coll) {
  /* format in-memory stringstream so formats are temporary */
  std::ostringstream out;
  out.precision(3);       // formats floats, ignores all else
  out << std::showpoint;  //   "        
  out << std::boolalpha;
  out << "{ ";
  /*-- show comma only in interior of sequence --*/
  bool first = true;
  for(auto item : coll) {
    if(first) {
      out << "{" << item.first << ", " << item.second << "}";
      first = false;
    }
    else {
      out << ", " << "{" << item.first << ", " << item.second << "}";
    }
  }
  out << " }";
  return out.str();
}
/*-- format output --------------------------------------
  - last argument is the standard lambda std::function
  - intent is to pass in formatting customized for type T
  - lots of examples of that below
*/
template <typename T>
std::string formatOutput(
  const T& t,                               // variable being formatted
  const std::string& nm,                    // caller name
  std::function<std::string(const T&)> f,   // convert to formatted string
  bool showtype = true                      // default to show type
){
  std::stringstream out;
  out << "  " << std::setw(WIDTH) << std::left << nm + ": "
      << f(t) << "\n";
  if(showtype) {
    out << getType(t, nm);
  }
  return out.str();
}
/*-- demonstrate initialization of primitive types --------
  - Note how formatOutput function works for most types
    and eliminates a lot of repetitive code.
*/

void initialize_primitives() {

  showLabel("initialize fundamental types");
  nl();

  showOp("scalars");
  bool b = true;
  std::cout << formatOutput<bool>(
    b, "b", scalarToString<bool>
  );
  nl();

  std::byte byte { 0x0f };
  /*  std::byte => unsigned char {} */
  std::cout << formatOutput<std::byte>(
    byte, "byte", hexToString<std::byte>
  );
  nl();

  int i = 42;  // equiv to int i { 42 };
  std::cout << formatOutput<int>(
    i, "i", scalarToString<int>
  );
  nl();

  double d = 3.1415927;
  std::cout << formatOutput<double>(
    d, "d", floatToString<double>
  );
  nl();

  char ch = 'z';
  std::cout << formatOutput<char>(
    ch, "ch", scalarToString<char>
  );
  nl();

  const char* lst = "a literal string";
  std::cout << formatOutput<const char*>(
    lst, "lst", quotedString<const char*>
  );
  nl();

  showOp("aggregate types");
  nl();

  /*-- array --*/
  short int fa[] { 1, 2, 3, 4, 5 };
  short int fa_alt[5] { 1, 2, 3, 4, 5 };
  std::cout << formatOutput<short int[5]>(
    fa, "fa[]", seq_collectionToString<short int[5]>
  );
  nl();

  /*-- struct ---------------------------------------------
    - user-defined, so generic formatOutput function 
      is not practical
    - https://en.cppreference.com/w/cpp/utility/format/formatter
  */
  struct S { int a; char b; double c; };
  S strct { 1, 'a', 3.1415927 };
  std::string fstr = std::format("  {:8}{}", "strct: ", "S { ");
  fstr += std::format("{}, {}, {:.5f} }}\n", strct.a, strct.b, strct.c);
  std::cout << fstr;
  showType(strct, "strct");
  nl();

  /*-- tuple ----------------------------------------------
    - user-defined, so generic formatOutput function 
      is not practical
  */
  std::tuple<int, double, char> tup { 1, 3.14, 'z' };
  std::cout << "  " << std::setw(WIDTH) << std::left << "tup: " << "{ " 
            << std::get<0>(tup) << ", " 
            << std::get<1>(tup) << ", " 
            << std::get<2>(tup)
            << " }\n";
  showType(tup, "tup");
  nl();
  
  /*-- optional -------------------------------------------
    - generic format output function would only match this
      one type, so no code reuse
  */
  std::optional<double> opt1 { 3.1415927 };
  std::cout << "  " << std::setw(9) << "opt1: " << opt1.value_or(42.0) << "\n"; 
  std::optional<double> opt2; // { std::nullopt };
  std::cout << "  " << std::setw(WIDTH) << std::left << "opt2: ";
  if(opt2 == std::nullopt) {
    std::cout << "empty\n";
  }
  else {
    std::cout << *opt2 << "\n";
  }
  showType(opt1, "opt1");
  nl();

  showOp("initialized in heap memory");
  nl();

  showOp("raw pointer");
  int* tmol = new int{42};
  std::cout << formatOutput<int>(
                 *tmol, "*tmol", scalarToString<int>
               );
  std::cout << formatOutput<int*>(
                 tmol, "tmol", scalarToString<int*>
               );
  delete tmol;  // forgetting this causes memory leak
  nl();

  showOp("std::shared_ptr");
  std::shared_ptr<int> utmol = std::make_shared<int>(42);
  int tmp = *utmol;
  std::cout << formatOutput<int>(
    *utmol, "*utmol", scalarToString<int>
  );
  std::cout << formatOutput<std::shared_ptr<int>>(
    utmol, "utmol", scalarToString<std::shared_ptr<int>>
  );
  // std::shared_ptr deallocates heap storage when
  // all references go out of scope. So no memory leak.
  nl();

  std::shared_ptr<std::vector<int>> pVec = 
  make_shared<std::vector<int>>(std::vector<int>{ 1, 2, 3 });
  std::cout << formatOutput<std::vector<int>>(
                 *pVec, "*pVec", 
                 seq_collectionToString<std::vector<int>>
               );
  std::cout << formatOutput<std::shared_ptr<std::vector<int>>>(
                 pVec, "pVec", 
                 scalarToString<std::shared_ptr<std::vector<int>>>
               );
  nl();

  showOp("using aliases to simplify");
  using VecI = std::vector<int>;
  using SPtr = std::shared_ptr<std::vector<int>>;
  SPtr pVec2 = make_shared<VecI>(VecI{ 1, 2, 3 });
  std::cout << formatOutput<VecI>(*pVec2, "*pVec2", seq_collectionToString<VecI>);
  std::cout << formatOutput<SPtr>(
                 pVec2, "pVec2", 
                 scalarToString<SPtr>
               );
  nl();
}
/*-- initialize std::library types ----------------------*/

void initialize_std_library_types() {
  showLabel("initialize std::library types");
  nl();

  /*-- generic array --*/
  std::array<double, 7> 
  sarr { 1.0, 1.5, 2.0, 2.5, 2.0, 1.5, 1.0 };
  std::cout << formatOutput<std::array<double, 7>>(
    sarr, "sarr", 
    seq_collectionToString<std::array<double, 7>>
  );
  nl();

  /*-- expandable string --*/
  std::string sstr = "a string";   // initializes from literal
  std::cout << formatOutput<std::string>(
    sstr, "sstr", quotedString<std::string>
  );
  nl();

  /*-- expandable indexable array --*/
  std::vector<int> vec { 1, 2, 3, 4 };
  vec.push_back(5);
  std::cout << formatOutput<std::vector<int>>(
    vec, "vec", seq_collectionToString<std::vector<int>>
  );
  nl();

  /*-- expandable double-ended queue --*/
  std::deque<int> dq { 1, 2, 3 };
  dq.push_front(0);
  dq.push_back(4);
  std::cout << formatOutput<std::deque<int>>(
    dq, "dq", seq_collectionToString<std::deque<int>>
  );
  nl();

  /*-- expandable associative container --*/
  std::unordered_map<std::string, int> umap
  {{"one", 1}, {"two", 2}, {"three", 3}};
  umap.insert({"zero", 0});
  std::cout << formatOutput<std::unordered_map<std::string, int>>(
    umap, "umap", 
    assoc_collectionToString<std::unordered_map<std::string, int>>
  );
  nl();
}
/*---------------------------------------------------------
  Demonstrate copy and move operations for various types
*/
/*-- display address of t in T --*/
template<typename T>
std::string formatAddress(const T& t, const std::string& nm) {
  const T* ptrToArg = &t;
  std::stringstream out;
  out.precision(7);
  out << "  " << std::showpoint;
  out << std::boolalpha;
  out << std::setw(WIDTH) << std::left << nm + ": " << "address: ";
  out << std::showbase << std::hex << ptrToArg << "\n";
  return out.str();
}
void demoCopyAndMove() {

  showLabel("copy operations for primitives");
  nl();

  /*-- primitive copy construction - bit-wise copy --*/
  showOp("copy construction");
  int ival = 42;
  /*-- demonstrate bit-wise copy --*/
  std::cout << formatOutput<int>(ival, "ival", scalarToString<int>);
  std::cout << formatAddress(ival, "ival");
  showOp("int jval = ival");
  int jval = ival;
  std::cout << formatOutput<int>(jval, "jval", scalarToString<int>);
  std::cout << formatAddress(jval, "jval");
  nl();

  showOp("copy assignment");
  double dval = 3.1415927;
  std::cout << formatOutput<double>(dval, "dval", floatToString<double>);
  /*-- demonstrate bit-wise copy --*/
  std::cout << formatAddress(dval, "dval");
  double eval = 1.33333;
  std::cout << formatOutput<double>(eval, "eval", floatToString<double>);
  std::cout << formatAddress(eval, "eval");
  showOp("dval = eval");
  dval = eval;
  std::cout << formatOutput<double>(dval, "dval", floatToString<double>);
  std::cout << formatAddress(dval, "dval");
  nl();

  /*-- library type copy construction ---------------------
    - uses copy constructor and assignment operator 
  */

  showLabel("copy operations for std::library types");
  nl();

  showOp("vector copy construction");
  std::vector<double> vec { 1.0, 1.5, 2.0 };
  std::cout << formatOutput<std::vector<double>>(
    vec, "vec", seq_collectionToString<std::vector<double>>, false
  );
  /*-- demonstrate copy construction --*/
  std::cout << formatAddress(vec, "vec");
  std::cout << formatAddress(vec[0], "vec[0]");
  showOp("auto uec = vec");
  auto uec = vec;
  std::cout << formatOutput<std::vector<double>>(
    uec, "uec", seq_collectionToString<std::vector<double>>, false
  );
  std::cout << formatAddress(uec, "uec");
  std::cout << formatAddress(uec[0], "uec[0]");
  
  showLabel(
    "Note:\n  Both uec and vec and their resources are unique.\n"
    "  That's because the vector copy constructor\n"
    "  copies each element of vec to uec.\n\n"
    "  Managed languages copy handles to instances,\n"
    "  not the instances themselves, so construction\n"
    "  does not create a new instance in those\n"
    "  languages.  Resources are shared."
  );
  nl();
  
 showOp("vector copy assignment");
  std::vector<double> tec { 1.0, 1.5, 2.0, 2.5 };
  std::cout << formatOutput<std::vector<double>>(
    tec, "tec", seq_collectionToString<std::vector<double>>, false
  );
  /*-- demonstrate copy assignment --*/
  std::cout << formatAddress(tec, "tec");
  std::cout << formatAddress(tec[0], "tec[0]");
  showOp("uec = tec");
  uec = tec;
  std::cout << formatOutput<std::vector<double>>(
    uec, "uec", seq_collectionToString<std::vector<double>>, false
  );
  std::cout << formatAddress(uec, "uec");
  std::cout << formatAddress(uec[0], "uec[0]");
  
  showOp("original source vec has not changed");
  std::cout << formatOutput<std::vector<double>>(
    vec, "vec", seq_collectionToString<std::vector<double>>, false
  );
  std::cout << formatAddress(vec, "vec");
  std::cout << formatAddress(vec[0], "vec[0]");
  
  showLabel(
    "Note:\n  Both uec and tec and their resources are unique.\n"
    "  That's because vector copy assignment operator\n"
    "  copies each element of tec to uec.\n\n"
    "  Managed languages copy handles to instances,\n"
    "  not the instances themselves, so assignment\n"
    "  causes sharing of resources in those languages."
  );
  nl();

  showLabel("std::string moves");
  nl();

  showOp("move temporary string");
  auto first = std::string("first part");
  auto mid = std::string(" and ");
  auto last = std::string("last part");
  std::cout << formatOutput<std::string>(first, "first", quotedString<std::string>, false);
  std::cout << formatOutput<std::string>(mid, "mid", quotedString<std::string>, false);
  std::cout << formatOutput<std::string>(last, "last", quotedString<std::string>, false);
  showOp("auto aggr = first + mid + last");
  auto aggr = first + mid + last;
  std::cout << formatOutput<std::string>(aggr, "aggr", quotedString<std::string>, false);
  showLabel(
    "first + mid + last is a temporary string that\n  "
    "is moved to aggr using move constructor."
  );
  nl();

  showOp("forced string move");
  auto src = std::string("src string");
  std::cout << formatOutput<std::string>(src, "src", quotedString<std::string>);
  showOp("auto dst = std::move(src)");
  auto dst = std::move(src);
  std::cout << formatOutput<std::string>(dst, "dst", quotedString<std::string>);
  std::cout << formatOutput<std::string>(src, "src", quotedString<std::string>);
  showLabel(
    "There is no guarantee that src will have a valid\n  "
    "state after move, so the src display, above, has\n  "
    "undefined behavior - just happens to work on MSVC."
  );
  nl();
}
template<typename T>
void pass_seq_by_value(T t) {
  std::cout << formatOutput<T>(t, "   passed", seq_collectionToString<T>);
  std::cout << formatAddress<T>(t, "   passed");
  std::cout << formatAddress<typename T::value_type>(t[0], "passed[0]");
}
template<typename T>
void pass_seq_by_reference(T& t) {
  std::cout << formatOutput<T>(t, "   passed", seq_collectionToString<T>);
  std::cout << formatAddress<T>(t, "   passed");
  std::cout << formatAddress<typename T::value_type>(t[0], "passed[0]");
}
void pass_by_value_and_ref() {

  showLabel("pass-by-value and pass-by-reference");
  nl();

  showOp("std::vector<int> pass-by-value");
  nl();
  using VECI = std::vector<int>;
  auto v = std::vector<int>() = { 1, 2, 3 };
  std::cout << formatOutput<VECI>(v, "     call", seq_collectionToString<VECI>);
  std::cout << formatAddress<VECI>(v, "     call");
  std::cout << formatAddress<int>(v[0], "  call[0]");
  pass_seq_by_value(v);
  showLabel(
    "passed has the same value as call.\n"
    "  call and its resources are different from\n"
    "  passed and its resources.  That demonstrates\n"
    "  passed was copy constructed from call."
  );
  nl();

  showOp("std::vector<int> pass-by-reference");
  nl();
  using VECI = std::vector<int>;
  auto rv = std::vector<int>() = { 1, 2, 3 };
  std::cout << formatOutput<VECI>(rv, "     call", seq_collectionToString<VECI>);
  std::cout << formatAddress<VECI>(rv, "     call");
  std::cout << formatAddress<int>(rv[0], "  call[0]");
  pass_seq_by_reference(rv);
  showLabel(
    "call and its resources are the same as\n"
    "  passed and its resources.  That demonstrates\n"
    "  that only reference was copied."
  );

}
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
// /*-----------------------------------------------
//   create and display values of primitive types
//   in heap
// */
// void create_primitives_in_heap() {

//     showLabel("store instances of primitives in heap");
//     nl();

//     /*-- values live in heap when using new --*/
    
//     double* dptr = new double{3.14159};
//     showTypeScalar(dptr, "dptr");
//     showTypeScalar(*dptr, "*dptr");
//     delete(dptr);  // see unique_ptr, below
//     nl();

//     /*-- control block lives in stack, char data live in heap --*/
    
//     showOp("string: control block in stack, data in heap");
    
//     auto s2 = std::string("Hello Data");  // move ctor - rhs is temp
//     showTypeScalar(s2, "s2");
//     nl();
    
//     showOp("C++ reference");
//     auto& rs2 = s2;      // create reference, no copy or move
//     showTypeScalar(rs2, "rs2");
//     nl();
    
//     /*-- unique_ptr may not be copied but move allowed --*/
    
//     showOp("unique_ptr owns data");
    
//     auto ptr = std::unique_ptr<int>(new int{-3});
//     *ptr += 1;
//     showTypeScalar(std::move(ptr), "ptr");  // unique_ptr can't be copied so move
    
//     /*-- original heap value deleted when ptr assigned --*/

//     ptr = std::unique_ptr<int>(new int{-4});
//     showTypeScalar(*ptr, "*ptr"); // contents can be copied
//     nl();

//     showOp("heap data deleted when unique_ptr leaves scope");
//     nl();
// }
// /*-----------------------------------------------
//   create and display values of std lib types
//   - uses showTypeEnum for sequential containers
//   - uses showTypeAssoc for associative containers
// */
// void demo_stdlib_types() {

//   showLabel("demo std library types");
//   std::vector<int> v { 1, 2, 3, 2, 1};
//   showTypeEnum(v, "v");

//   std::unordered_map<std::string, int> m {
//     {"zero", 0}, {"one", 1}, {"two", 2}
//   };
//   showTypeAssoc(m, "m");
//   nl();
// }
// /*-----------------------------------------------
//   create and display values of user-defined type
//   - If class members have correct copy and assignment
//     semantics it is appropriate to let compiler define
//     copy ctor and assign, move ctor and assign
//     and dtor.
//   - More details about classes in next bit.
// */
// class Demo {
//   public:
//     Demo() = delete;
//     Demo(const Demo&) = default;
//     Demo(const std::string& name) : _name(name) {}
//     Demo& operator=(const Demo&) = default;
//     ~Demo() = default;
//     std::string& name() {
//       return _name;
//     }
//     void show();
//   private:
//     std::string _name;
// };

// void Demo::show() {
//   std::cout << "  Demo {" << _name << " }\n";
// }
// /*-----------------------------------------------
//   demonstrate user-defined type
// */
// void demo_user_defined_type() {

//   showLabel("demonstrate user-defined type");
//   nl();

//   Demo demo("demo");
//   showType(demo, "demo");
//   demo.show();
//   nl();
// }
// /*-----------------------------------------------
//   pass argument by value 
//     - copies argument to function's stack frame
//     - value changed to default in function
//       has no side effects
// -----------------------------------------------*/
// template<typename T>
// void pass_by_value(T t) {
//   /*-- uses copy constructor --*/
//   showType(t, "t:T");
//   /* 
//     statement below won't affect caller's t value
//     since that was copied into function
//   */
//   t = T();  // value change uses T copy assignment
// }
// /*-----------------------------------------------
//   pass argument by reference 
//     - copies reference to argument to function's
//       stack frame
//     - value changed to default in function
//       has side effects
// -----------------------------------------------*/
// template<typename T>
// void pass_by_ref(T& t) {
//   /*-- direct copy of reference --*/
//   showType(t, "T");
//   /*
//     statement below changes caller's t value 
//     since a reference to that was copied into
//     the function
//   */
//  t = T();  // value change uses T copy assignment
// }
// /*-----------------------------------------------
//   demonstrate copy and pass by value
// -----------------------------------------------*/
// void demo_copy() {

//   showLabel("demonstrate copy and pass-by-value");
//   nl();

//   showOp("direct integer copy");
//   int i_src = 42;
//   int i_dst = i_src;
//   std::cout << "  source: " << i_src << "\n";
//   std::cout << "  destin: " << i_dst << "\n";
//   nl();

//   showOp("direct vector copy");
//   /*-- uses vector copy constructor --*/
//   auto v_src = std::vector<int>{1, 2, 3, 4, 5};
//   auto v_dst = v_src;
//   std::cout << "  source:";
//   showEnum(v_src, "v_src");
//   std::cout << "  destin:";
//   showEnum(v_dst, "v_dst");
//   nl();

//   showOp("pass-by-value copies integer");
//   int i = 42;
//   std::cout << "  before pass_by_value: " << i << "\n";
//   pass_by_value(i);
//   std::cout << "  after pass_by_value:  " << i << "\n";
//   nl();

//   showOp("pass-by-value copies vector using ctor");
//   auto v = std::vector<int>{ 1, 2, 3, 2, 1 };
//   std::cout << "  before pass_by_value:";
//   showEnum(v, "v");
//   pass_by_value(v);
//   std::cout << "  after pass_by_value:";
//   showEnum(v, "v");
//   nl();

//   showOp("pass-by-value copies unordered_map using ctor");
//   auto map = std::unordered_map<std::string, int> { 
//     {"zero", 0}, {"one", 1}, {"two", 2}  };
//   std::cout << "  before pass_by_value:";
//   showAssoc(map, "map");
//   pass_by_value(map);
//   std::cout << "  after pass_by_value:";
//   showAssoc(map, "map");
//   nl();
// }
// /*-----------------------------------------------
//   demonstrate pass by reference
// -----------------------------------------------*/
// void demo_ref() {

//   showLabel("demonstrate pass-by-reference");
//   nl();

//   showLabel("all demos have side effects", 35);
//   nl();

//   showOp("copy ref of integer");
//   int i = 42;
//   std::cout << "  before pass_by_ref: " << i << "\n";
//   pass_by_ref(i);
//   std::cout << "  after pass_by_ref:  " << i << "\n";
//   nl();

//   showOp("copy ref of vector");
//   auto v = std::vector<int>{ 1, 2, 3, 2, 1 };
//   std::cout << "  before pass_by_ref:";
//   showEnum(v, "v");
//   pass_by_ref(v);
//   std::cout << "  after pass_by_ref:";
//   showEnum(v, "v");
//   nl();

//   showOp("copy ref of unordered_map");
//   auto map = std::unordered_map<std::string, int> { 
//     {"zero", 0}, {"one", 1}, {"two", 2}  };
//   std::cout << "  before pass_by_ref:";
//   showAssoc(map, "map");
//   pass_by_ref(map);
//   std::cout << "  after pass_by_ref:";
//   showAssoc(map, "map");
//   nl();
// }
// /*-----------------------------------------------
//   demonstrate move
// -----------------------------------------------*/
// void demo_move() {

//   showLabel("demonstrate move");
//   nl();

//   showOp("move string");
//   std::string s1 { "a string" };
//   std::string s2 { " plus more" };
//   std::string s = s1 + s2;  // rhs constructs sum then moves to lhs
//   std::cout << "  s1 = " << "\"" << s1 << "\"" << "\n";
//   std::cout << "  s2 = " << "\"" << s2 << "\"" << "\n";
//   std::cout << "  s = s1 + s2 => " << "\"" << s << "\"" << "\n";
//   nl();

//   showOp("move vector");
//   auto v = std::vector<int>{ 1, 2, 3, 2, 1 };
//   std::cout << "  before move:";
//   showEnum(v, "v");
//   auto vm = std::move(v);
//   std::cout << "  after move of v into vm:";
//   showEnum(vm, "vm");
//   /*-- v has been moved so not guarenteed to be in viable state --*/
//   try {
//     showEnum(v, "v");   // undefined behavior since resources moved, 
//   }                     // works without exception using VS compiler
//   catch(std::exception& ex) {
//     std::cout << "\n  bad things happened\n";
//   }
// }
/*-----------------------------------------------
  demonstration starts here
*/
int main() {
    showLabel("Demonstrate C++ types");
    nl();

    initialize_primitives();
    initialize_std_library_types();
    demoCopyAndMove();
    pass_by_value_and_ref();
    // initialize_primitives_from_literals();
    // create_primitives_in_heap();
    // demo_stdlib_types();
    // demo_user_defined_type();
    // demo_copy();
    // demo_ref();
    // demo_move();

    println("\n  That's all Folks!\n\n");
}
