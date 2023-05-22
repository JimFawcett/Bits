/*-------------------------------------------------------------------
  Bits_Iter.cpp
  - defines functions to iterate over collections
  - depends on Points.h to provide user-defined point class
  - depends on Analysis.h for several display and analysis functions
*/
#include <iostream>         // std::cout
#include <memory>           // std::unique_ptr
#include <vector>           // vector<T> class
#include <array>            // array<T> class
#include <map>              // map<K,V> class
#include <set>              // set<T> class
#include "AnalysisIter.h"   // Analysis functions
#include "PointsIter.h"     // PointN<T> class declaration
/*
  This demo uses the std::string and std::vector<T> classes
  and two user defined classes, Point1 and PointN<T>, to 
  illustrate how objects are defined and instantiated.

  Operations:
    All the classes discussed here provide operations for:
      T t2 = t1          // copy construction
      T t3 = temporary   // move construction
      t1 = t2            // copy assignment
      t3 = temporary     // move assignment

    All instances return their resources when they go out of
    scope by implicitly calling their destructor.
    Primitive types can all be copied.

    Most library and user-defined types can be copied, moved, 
    and deleted by providing member constructors and destructor.
    Often compiler generation works well, but for classes with 
    pointer members developers must provide them.

  Processing:
    All types are static, operations run as native code, and no 
    garbage collection is needed. Resources are returned at end 
    of their declaration scope.
*/
#pragma warning(disable: 4984)  // warns about C++17 extension

/*-----------------------------------------------
  alias type name 
  - pU<T> is the same type as std::unique_ptr<T> 
  - this just provides a shorter name
*/
template<typename T>
using pU = std::unique_ptr<T>;

/*-----------------------------------------------
  forLoopVec accepts std::vector<T> instances
  by constant reference.
  - uses range-for to display PointN coordinates
*/
template<typename T>
void forLoopVec(const std::vector<T>& v) {
  std::cout << "\n    ";
  for(auto const &item : v) {
    std::cout << item << " ";
  }
}
/*-----------------------------------------------
  forLoopPoint accepts PointN<T> instances
  by constant reference.
  - uses range-for to display PointN coordinates
*/
template<typename T>
void forLoopPoint(const PointN<T>& p) {
    std::cout << "\n    ";
    for(auto const &item : p) {
      std::cout << item << " ";
    }
}
/*-----------------------------------------------
  whilerPoint accepts PointN<T> instances
  by constant reference.
  - uses iterator to display PointN coordinates
*/
template<typename T>
void whilerPoint(const PointN<T>& p) {
  auto itr = p.begin();
  std::cout << "\n    " << *itr++;
  while (itr < p.end()) {
    std::cout << ", " << *itr++;
  }
}
/*-----------------------------------------------
  whiler is flexible function that accepts any 
  iterable container
  - will use iterator on C.
*/
template<typename C>
void whiler(const C& c) {
  auto itr = c.begin();  // uses const overload
  std::cout << "\n    " << *itr++;
  while (itr != c.end()) {
    std::cout << ", " << *itr++;
  }
}
/*-----------------------------------------------
  whiler_guarded is flexible function that accepts 
  any container
  - if C is iterable will use iterator on C.
  - If non-iterable input is detected, 
    will display error msg and return.
  - decision is made at compile time.
  - is_iterable_v is defined in Analysis.h
  - max is the maximum number of items 
    to show on one line
*/
template<typename C>
void whiler_guarded(
  const C& c, const std::string& name, 
  size_t indent = 2, size_t max = 8
  ) {
  if constexpr(!is_iterable_v<C>) {  // decision at compile-time
    std::cout << "\n  whiler input type is not iterable\n";
    return;
  }
  else {
    std::cout << formatColl(c, name, "", indent, max);
    /*
      Analysis::formatColl uses range-based for loop to iterate 
      over collection.
    */
  }
}
/*-------------------------------------------------------------------
  Demonstration starts here 
*/
void testFormat();

int main() {

    print("Demonstrate C++ Iteration\n");

    showNote("std library types string and vector<T>");
    /* create and display std::string object */
    auto str = std::string("\"Wile E. Coyote\"");
    auto out = std::string("contents of str = ") + str;
    print(out);
    print("--- showType(str, \"str\"); ---");
    showType(str, "str", nl);

    showNote("Iterate over string");
    whiler_guarded(out, "out");
    print();

    /* create and display std::vector<double> */
    auto vec = std::vector<double>{ 3.5, 3, 2.5, 2 };
    std::cout << vec;
    showOp("showType(vec, \"vec\");");
    showType(vec, "vec", nl);

    showOp("vec[2] = -2.5;");
    vec[2] = -2.5;
    std::cout << "\n  vec:" << vec;

    showOp("auto vec2 = vec : copy construction");
    /* copy construction */
    auto vec2 = vec;
    std::cout << "\n  vec2:" << vec2;
    
    showOp("vec2[0] = 42;");
    vec2[0] = 42;
    std::cout << "\n  vec2: " << vec2;
    std::cout << "\n  vec: " << vec;

    showNote(
      "Copy construction, auto vec2 = vec, creates\n    " 
      "independent instance. So changing target vec2\n    "
      "has no affect on source vec."
    );

    showNote(
      "Iterate over vector"
    );
    showOp("function using range-for taking vector");
    forLoopVec(vec);
    showOp("function using iterator taking iterable container");
    whiler(vec);
    print();

    showNote("user-defined type PointN<T>");

    PointN<double> p1(5);
    p1.show("p1");

    showNote(
      "p1.coords() = std::vector<double>\n    "
      "{ 1.0, -2.0, 3.0, 4.5, -42.0 }"
    );
    p1.coords() = std::vector<double>{1.0, -2.0, 3.0, 4.5, -42.0 };
    p1.show("p1");
    #pragma region
    showOp("showType(p1, \"p1\", nl);");
    showType(p1, "p1", nl);
    std::cout << "  p1.coords()[2] = " << p1.coords()[2] << "\n";
    
    showNote(
      "iterate over PointN<T>"
    );
    showOp("function using range-for taking Point");
    forLoopPoint(p1);
    showOp("function using iterator taking Point");
    whilerPoint(p1);
    showOp("function using iterator taking iterable container");
    whiler_guarded(p1, "p1");
    showOp("same function attempting to take non-iterable");
    struct S { int i; };
    auto s = S{3};
    whiler_guarded(s, "s");
    print();

    showNote("heap-based string instance");
  
    /* standard library type std::string */
    /* uses alias pU for std::unique_ptr, defined above */
    showOp(
      "pU<std::string> "
      "pStr(new std::string(\"\\\"Road Runner\\\"\")"
    );
    pU<std::string> pStr(new std::string("\"Road Runner\""));
    std::cout << "\n  pStr contents = " << *pStr << "\n";
    
    showOp("showType(*pStr, \"*pStr\")");
    showType(*pStr, "*pStr", nl);

    /* std::unique_ptr<T> cannot be copied but can be moved */
    showOp("showType(move(pStr), \"pStr\")");
    showType(move(pStr), "pStr", nl);

    /* standard library type std::vector<T> */
    showNote("heap-based vector instance");
    showOp(
      "pU<std::vector<double>>\n "
      "     pVec(new std::vector<double>{ 1.5, 2.5, 3.5 });"
    );
    pU<std::vector<double>> pVec(
      new std::vector<double>{ 1.5, 2.5, 3.5 }
    );
    print();
    showOp("iterating over *pVec");
    whiler_guarded(*pVec, "*pVec");
    print();
    std::cout << "\n  *pVec = " << *pVec;
    showType(*pVec, "*pVec", nl);
    std::cout << "\n  pVec = " << pVec;
    showType(move(pVec), "move(pVec)", nl);

    /* custom point type */

    showNote("heap-based PointN instance");
    
    showOp("pU<PointN<double>> pPointN(new PointN<double>(4))");
    pU<PointN<double>> pPointN(new PointN<double>(4));
    pPointN->show("*pPointN");
    
    showOp(
      "pPointN->coords() = \n"
      "      std::vector<double>{ 1.0, 3.5, -2.0, 42.0 };"
    );
    pPointN->coords() = std::vector<double>{ 1.0, 3.5, -2.0, 42.0 };
    pPointN->show("*pPointN");
    std::cout << "\n  value of pPointN->coords()[1] is " 
              << pPointN->coords()[1];
    
    showOp("showType(*pPointN, \"*pPointN\");");
    showType(*pPointN, "*pPointN");
    print();

    showOp("iterating over *pPointN");
    whiler_guarded(*pPointN, "*pPointN");
    print();

    showOp("showType(std::move(pPointN), \"pPointN\");");
    showType(std::move(pPointN), "pPointN");
    /* pPointN moved, so now invalid */
    print();

    showNote(
      "Iterate over map, using Analysis::format"
    );

    std::map<std::string, int> map{{"zero", 0}, {"one", 1}};
    map.insert({"two", 2});
    std::cout << format(map, "map");

    // #define TEST
    #ifdef TEST
    testFormat();
    #endif

    print("\n  That's all Folks!\n\n");
}

void testFormat() {

  showNote("Test and demonstrate formatting functions");
  
  showOp("demonstrate PointN show()");
  print("default indent = 4 and width = 7:");
  PointN<int> p2(15);
  p2.show("p2");
  size_t saveLeft = p2.left();
  size_t saveWidth = p2.width();
  print("\n  indent = 6, width = 12:");
  p2.left() = 6;
  p2.width() = 12;
  p2.show("p2");

  showOp(
    "demonstrate operator<< overload for PointN ---"
  );
  p2.left() = saveLeft;
  p2.width() = saveWidth;
  print("default indent = 4 and width = 7:");
  std::cout << p2;
  print("\n  indent = 6, width = 12:");
  p2.left() = 6;
  p2.width() = 12;
  std::cout << p2;

  showOp(
    "demonstrate operator<< overload for vector"
  );
  auto vtest = std::vector<int>{1, 2, 3, 4, 5, 6, 7, 8, 9 };
  print("default indent = 4 and width = 7:");
  std::cout << vtest;
  DisplayParams.left = 2;
  DisplayParams.width = 5;
  print("indent = 2, width = 5:");
  std::cout << vtest;

  std::cout << formatColl(vtest, "vtest", nl, 2, 5);
  std::cout << formatColl(vtest, "vtest", nl, 4, 7);
  std::cout << formatColl(vtest, "vtest", nl, 2, 9);
  std::cout << formatColl(vtest, "vtest: vector<int>", nl, 2, 10);

  std::array<double, 5> arrtest = { 1, 2, 3, 4.5, -3.14159 };
  std::cout << formatColl(arrtest, "arrtest", nl, 2, 4);

  std::map<int, std::string> amap {
    {1, "one"}, {2, "two"}, {3, "three"} 
  };
  std::cout << formatColl(amap, "amap", nl, 2, 4);

  std::set<std::string> aset { 
    "one", "two", "three", "four", "five" 
  };
  std::cout << formatColl(aset, "aset", nl, 2, 4);

  std::string astring = "this is a string";
  std::cout << formatString(astring, "astring", nl, 2);

  double adouble { 3.1415927 };
  std::cout << formatScalar(adouble, "adouble", nl);

  showNote("Using consolidated format function", nl);
  
  std::cout << format(adouble, "adouble", nl);
  std::cout << format(astring, "astring", nl);
  std::vector<double> avec{ 1, 2, 3, 4.5, -3.14159 };
  std::cout << format(avec, "avec", nl);
  std::cout << format(amap, "amap", nl);
}
