/*---------------------------------------------------------
  Bits::Cpp::Cpp_Data::src::Bits_Data.cpp
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
  /* format in-memory stringstream so formats are temporary */
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
  - Third argument is a lambda (aka closure) std::function
  - intent is to pass in formatting customized for type T
  - examples of that are shown below
*/
template <typename T>
std::string formatOutput(
  const T& t,                               // variable being formatted
  const std::string& nm,                    // caller name
  std::function<std::string(const T&)> f,   // convert to formatted string
  bool showtype = true                      // default to show type
){
  std::stringstream out;
  out << "  " << nm + ": "
      << f(t) << "\n";
  if(showtype) {
    out << getType(t, nm);
    out << "  " << nm + ": size = " << sizeof(t) << "\n";
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

  showOp("scalars", "\n");

  showOp("bool");
  bool b = true;
  std::cout << formatOutput<bool>(
    b, "b", scalarToString<bool>
  );
  size_t bsize = sizeof(b);

  showOp("byte");
  std::byte byte { 0x0f };
  /*  std::byte => unsigned char {} */
  std::cout << formatOutput<std::byte>(
    byte, "byte", hexToString<std::byte>
  );

  showOp("int");
  int i = 42;  // equiv to int i { 42 };
  std::cout << formatOutput<int>(
    i, "i", scalarToString<int>
  );

  showOp("double");
  double d = 3.1415927;
  std::cout << formatOutput<double>(
    d, "d", floatToString<double>
  );

  showOp("char");
  char ch = 'z';
  std::cout << formatOutput<char>(
    ch, "ch", scalarToString<char>
  );

  showOp("const char*");
  const char* lst = "a literal string";
  std::cout << formatOutput<const char*>(
    lst, "lst", quotedString<const char*>
  );
  std::cout << "  lst: char count = " << std::strlen(lst) << "\n\n";

  showOp("aggregate types");
  nl();

  /*-- array --*/
  showOp("native array");
  short int fa[] { 1, 2, 3, 4, 5 };
  short int fa_alt[5] { 1, 2, 3, 4, 5 };
  auto afirst = fa[0];
  std::cout << formatOutput<short int[5]>(
    fa, "fa[]", seq_collectionToString<short int[5]>
  );
  std::cout << "  afirst = " << afirst << "\n";

  /*-- struct ---------------------------------------------
    - user-defined, so generic formatOutput function 
      is not practical
    - https://en.cppreference.com/w/cpp/utility/format/formatter
  */
  showOp("struct");
  struct S { int a; char b; double c; };
  S strct { 42, 'a', 3.1415927 };
  auto sfirst = strct.a;
  std::string fstr = std::format("  {}{}", "strct: ", "S { ");
  fstr += std::format("{}, {}, {:.5f} }}\n", strct.a, strct.b, strct.c);
  std::cout << fstr;
  std::cout << getType(strct, "strct");
  std::cout << "  strct: size = " << sizeof(strct) << "\n";
  std::cout << "  sizeof(strct.a) = " << sizeof(strct.a) << "\n";
  std::cout << "  sizeof(strct.b) = " << sizeof(strct.b) << "\n";
  std::cout << "  sizeof(strct.c) = " << sizeof(strct.c) << "\n";
  std::cout << "  sfirst = " << sfirst << "\n";

  /*-- tuple ----------------------------------------------
    - user-defined, so generic formatOutput function 
      is not practical
  */
  showOp("tuple");
  std::tuple<int, double, char> tup { -1, 3.14, 'z' };
  std::cout << "  " << "tup: " << "{ " 
            << std::get<0>(tup) << ", " 
            << std::get<1>(tup) << ", " 
            << std::get<2>(tup)
            << " }\n";
  std::cout << getType(tup, "tup");
  std::cout << "  size = " << sizeof(tup) << "\n";
  auto tfirst = get<0>(tup);
  std::cout << "  tfirst = " << tfirst << "\n";

  /*-- optional -------------------------------------------
    - generic format output function would only match this
      one type, so no code reuse
  */
  showOp("optional");
  std::optional<double> opt1 { 3.1415927 };
  std::cout << "  " << "opt1: " << opt1.value_or(42.0) << "\n"; 
  std::optional<double> opt2; // { std::nullopt };
  std::cout << "  " << "opt2: ";
  if(opt2 == std::nullopt) {
    std::cout << "empty\n";
  }
  else {
    std::cout << *opt2 << "\n";
  }
  std::cout << getType(opt1, "opt1");
  std::cout << "  opt1: size = " << sizeof(opt1) << "\n";
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

  showOp("std::unique_ptr<int>");
  std::unique_ptr<int> suni = std::make_unique<int>(-1);
  int utmp = *suni;
  std::cout << formatOutput<int>(
    *suni, "*suni", scalarToString<int>
  );
  // can't use formatOutput<T>(t, ...) function here, 
  // because that requires making a copy of t internally, 
  // but std::unique_ptr disallows copies to preserve
  // uniqueness
  std::cout << "  suni: " << suni << "\n";
  size_t sz = sizeof(suni);
  std::cout << getType(std::move(suni), "suni");
  // suni no longer valid after move
  std::cout << "  suni: size = " << sz << "\n";

  showOp("std::shared_ptr<int>");
  std::shared_ptr<int> stmol = std::make_shared<int>(42);
  int stmp = *stmol;
  std::cout << formatOutput<int>(
    *stmol, "*stmol", scalarToString<int>
  );
  std::cout << formatOutput<std::shared_ptr<int>>(
    stmol, "stmol", scalarToString<std::shared_ptr<int>>
  );
  // std::shared_ptr deallocates heap storage when
  // all references go out of scope. So no memory leak.

  showOp("std::shared_ptr<std::vector<int>>");
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
  auto vtmp = *pVec;

  showOp("using aliases to simplify");
  using VecI = std::vector<int>;
  using SPtr = std::shared_ptr<VecI>;
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
  showOp("std::array<double>");
  std::array<double, 7> 
  sarr { 1.0, 1.5, 2.0, 2.5, 2.0, 1.5, 1.0 };
  std::cout << formatOutput<std::array<double, 7>>(
    sarr, "sarr", 
    seq_collectionToString<std::array<double, 7>>
  );

  /*-- expandable string --*/
  showOp("std::string => std::basic_string<char>");
  std::string sstr = "a string";   // initializes from literal
  std::cout << formatOutput<std::string>(
    sstr, "sstr", quotedString<std::string>
  );
  auto sfirst = sstr[0];
  std::cout << "  sfirst = " << sfirst << "\n";

  /*-- expandable indexable array --*/
  showOp("std::vector<int>");
  std::vector<int> vec { 1, 2, 3, 4 };
  vec.push_back(5);
  std::cout << formatOutput<std::vector<int>>(
    vec, "vec", seq_collectionToString<std::vector<int>>
  );

  /*-- expandable double-ended queue --*/
  showOp("std::deque<int>");
  std::deque<int> dq { 1, 2, 3 };
  dq.push_front(0);
  dq.push_back(4);
  std::cout << formatOutput<std::deque<int>>(
    dq, "dq", seq_collectionToString<std::deque<int>>
  );

  /*-- expandable associative container --*/
  showOp("std::unordered_map<std::string, int>");
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
  return formatted address as string
*/
static const size_t WIDTH = 8;

/*-- return address of t --*/
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
/*---------------------------------------------------------
  Demonstrate copy and move operations for various types
*/
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
  demonstration starts here
*/
int main() {

    // std::cout << std::showbase << std::hex << short int(-42) << "\n";
    std::cout << std::setprecision(12) << double(3.141592653589793) << "\n";

    showLabel("Demonstrate C++ types");
    nl();

    initialize_primitives();
    initialize_std_library_types();
    demoCopyAndMove();
    pass_by_value_and_ref();

    println("\n  That's all Folks!\n\n");
}
