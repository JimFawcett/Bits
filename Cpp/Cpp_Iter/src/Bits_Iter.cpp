/*-------------------------------------------------------------------
  Bits_Iter.cpp
  - defines functions to iterate over collections
  - depends on Points.h to provide user-defined point class
  - depends on Analysis.h for several display and analysis functions
*/
#pragma warning(disable: 4038)

#include <iostream>         // std::cout
#include <iomanip>          // std::fixed, std::setprecision
#include <sstream>          // std::stringstream
#include <memory>           // std::unique_ptr
#include <vector>           // vector<T> class
#include <array>            // array<T> class
#include <map>              // map<K,V> class
#include <set>              // set<T> class
#include <concepts>         // supports C++20 concepts
#include <algorithm>        // STL algorithms
#include "AnalysisIter.h"   // Analysis functions
#include "PointsIter.h"     // PointN<T> class declaration

using namespace Points;
/*-----------------------------------------------
  Note:
  Find all Bits code, including this in
  https://github.com/JimFawcett/Bits
  You can clone the repo from this link.
-----------------------------------------------*/
/*
  This demo uses the std::string and std::vector<T> classes
  and a user-defined class, Point<T, N>, to illustrate how
  types support indexing and iteration.
  - Each standard container, C, provides C::iterator, 
    C::const_iterator, C::reverse_iterator, and
    begin() and end() methods that return iterators to
    the first and one past the last element of the 
    collection, respectively. 
*/
#pragma warning(disable: 4984)  // warns about C++17 extension

/*-----------------------------------------------
  demoIndexer(const std::vector<T>& v)
  - accepts std::vector<T>
  - creates comma separated list
  - uses indexing, not using iterator
*/
template<typename T>
void demoIndexerVec(const std::vector<T>& v) {
  if(v.size() < 1)
    return;
  std::cout << "\n  " << v[0];
  for(size_t i = 1; i<v.size(); ++i) {
    std::cout << ", "  << v[i];
  }
}
void executeDemoIndexerVec() {
  std::cout << "\nexecute demoIndexerVec(v)";
  auto v = std::vector<int> { 1, 2, 3, 2, 1 };
  demoIndexerVec(v);
}
/*-----------------------------------------------
  demoIteratorVec
  - accepts std::vector<T> instances
  - uses iterator
*/
template<typename T>
void demoIteratorVec(const std::vector<T>& v) {
  auto itr = v.begin();
  std::cout << "\n  " << *itr;
  while(++itr != v.end()) {
    std::cout << ", " << *itr; 
  }
}
void executeDemoIteratorVec() {
  std::cout << "\nexecute demoIteratorVec(v)";
  auto v = std::vector<int> { 1, 2, 3, 2, 1 };
  demoIteratorVec(v);
}
/*-----------------------------------------------
  demoForLoopVec
  - accepts std::vector<T> instances
  - uses range-for to display elements
  - that uses iterator implicitly
*/
template<typename T>
void demoForLoopVec(const std::vector<T>& v) {
  std::cout << "\n  ";
  for(auto const &item : v) {
    std::cout << item << " ";
  }
}
void executeDemoForLoopVec() {
  std::cout << "\nexecute demoForLoopVec(v)";
  auto v = std::vector<int> { 1, 2, 3, 2, 1 };
  demoForLoopVec(v);
}
/*-----------------------------------------------
  forLoopPoint 
  - accepts Point<T, N> instances by constant reference.
  - uses range-for to display Point coordinates
  - creates comma separated list
*/
template<typename T, const size_t N>
void demoForLoopPoint(const Point<T, N>& p) {
  auto s = std::stringstream();
  s << "\n  ";
  for(auto const &item : p) {
    s << item << ", ";
  }
  auto str = s.str();
  /* remove last ", " */
  str.pop_back();
  str.pop_back();
  std::cout << str;
}
void executeDemoForLoopPoint() {
  std::cout << "\nexecute demoForLoopPoint(v)";
  /* using initialization list */
  auto p = Point<int, 5> { 1, 2, 3, 2, 1 };
  demoForLoopPoint(p);
}
/*-----------------------------------------------
  whilerPoint 
  - accepts Point<T, N> instances by constant reference.
  - explicit use of iterator to display PointN coordinates
  - creates comma separated list
*/
template<typename T, const size_t N>
void demoWhilerPoint(const Point<T, N>& p) {
  auto itr = p.begin();
  std::cout << "\n  " << *itr++;
  while (itr < p.end()) {
    std::cout << ", " << *itr++;
  }
}
void executeDemoWhilerPoint() {
  std::cout << "\nexecute demoWhilerPoint(v)";
  auto p = Point<int, 5>();
  /* using indexer */
  p[0] = 1;
  p[1] = 2;
  p[2] = 3;
  p[3] = 2;
  p[4] = 1;
  demoWhilerPoint(p);
}
/*-----------------------------------------------
  whiler
  - is flexible function that accepts any 
    iterable container
  - uses iterator on C.
  - creates comma separated list
*/
template<typename C>
void demoWhiler(const C& c) {
  auto itr = c.begin();  // uses const overload
  std::cout << "\n  " << *itr++;
  while (itr != c.end()) {
    std::cout << ", " << *itr++;
  }
}
void executeDemoWhiler() {
  std::cout << "\nexecute demoWhiler(c) with string";
  auto s = std::string("a string");
  demoWhiler(s);
  std::cout << "\nexecute demoWhiler(c) with vector";
  auto v = std::vector<double> { 1.0, 1.5, -1.5, -1.0, 0 };
  demoWhiler(v);
  std::cout << "\nexecute demoWhiler(c) with Point";
  auto p = Point<double, 5> { 1.0, 2.0, 3.0, 2.0, 1.0 };
  demoWhiler(p);
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
void demoWhilerGuarded(
  const C& c, const std::string& name, 
  size_t indent = 2, size_t max = 8
  ) {
  if constexpr(!is_iterable_v<C>) {  // decision at compile-time
    std::cout << "\n  whiler input type is not iterable";
    return;
  }
  else {
    std::cout << formatColl(c, name, "", indent, max);
    /*
      Analysis::formatColl uses range-based for loop to iterate 
      over collection, folding output into rows of max items.

      Could have used the same iteration as in demoWhiler
      but wanted to show nicer formatting.
    */
  }
}
void executeDemoWhilerGuarded() {
  std::cout << "\nexecute demoWhilerGuarded(c) with vector";
  auto v = std::vector<int> { 1, 2, 3, 2, 1, 0, -1, -2, -3, -4 };
  demoWhilerGuarded(v, "vector");
  std::cout << "\nexecute demoWhiler(c) with double will fail";
  demoWhilerGuarded(3.5, "double");
}
/*-----------------------------------------------
  collectionWithOperation(C& c, F f)
  - accepts collection c and lamda f
  - applies lamda to each element of c
  - supports modifying collection elements
  - demonstrated with lambdas:
      square(item), plus_one(item), plus_one_mod(item)
*/
template<typename C, typename F>
void collectionWithOperation(C& c, F f) {
  for(auto &item : c) {
    f(item);
  }
}
void executeCollectionWithOperation() {
  /*------------------------------*/
  auto v = std::vector<int> { 1, 2, 3, 2, 1 };
  std::cout << "\noriginal vector";
  demoWhiler(v);

  std::cout << "\nexecute collectionWithOperation(v, square)";
  std::cout << "\n  ";
  /*-- lambda --*/
  auto square = [](int item) { 
    std::cout << item*item << " "; 
  };
  collectionWithOperation(v, square);
  /*------------------------------*/
  std::cout << "\nexecute collectionWithOperation(v, plus_one)";
  std::cout << "\n  ";
  /*-- lambda --*/
  auto plus_one = [](int item) { 
    std::cout << item + 1 << " "; 
  };
  collectionWithOperation(v, plus_one);
  std::cout << "\nno side effect of plus_one:";
  demoForLoopVec(v);
  /*------------------------------*/
  std::cout << "\nexecute collectionWithOperation(v, plus_one_mod)";
  std::cout << "\n  ";
  /*-- lambda --*/
  auto plus_one_mod = [](int& item) { 
    item += 1;
    std::cout << item << " "; 
  };
  collectionWithOperation(v, plus_one_mod);
  std::cout << "\nside effect of plus_one_mod:";
  demoForLoopVec(v);
}
/*-----------------------------------------------
  executeForEachAlgorithm()
  - uses std::for_each with lambda plus_one_mod
  - same functionality as collectionWithOperation
*/
void executeForEachAlgorithm() {
  auto v = std::vector<int> { 1, 2, 3, 2, 1 };
  std::cout << "\noriginal vector:";
  demoWhiler(v);  // display v
 
  /* define lambda plus_one_mod */
  auto plus_one_mod = [](int& item) { 
    item += 1;
  };
  std::cout << "\nmodified vector:";
  std::for_each(v.begin(), v.end(), plus_one_mod);
  demoWhiler(v);  // display v
  std::cout << "\n";

  auto p = Point<int, 5> { 1, 2, 3, 4, 5 };
  std::cout << "\noriginal point";
  demoWhiler(p);
  std::cout << "\nmodified point";
  std::for_each(p.begin(), p.end(), plus_one_mod);
  demoWhiler(p);
  std::cout << "\n";
 
  /* define lambda sumer */
  auto sumer = [](int item) -> int {
    static int sum = 0;
    sum += item;
    return sum; 
  };
  auto result = std::for_each(p.begin(), p.end(), sumer);
  std::cout << "\nsum = " << sumer(0) << " - using supplied lambda";
  std::cout << "\nsum = " << result(0) << " - using returned lambda";
}
/*-------------------------------------------------------------------
  Demonstration starts here 
*/
void testFormat();

int main() {

    showNote("Demonstrate C++ Iteration", 30, "\n"); 

    showOp("collection specific iterations", nl);
    std::cout << std::fixed;
    std::cout << std::setprecision(1);
    executeDemoIndexerVec();
    executeDemoIteratorVec();
    executeDemoForLoopVec();
    executeDemoForLoopPoint();
    executeDemoWhilerPoint();
    print();

    showOp("accepts any iterable collection", nl);
    executeDemoWhiler();
    print();

    showOp("detects non-iterable input at compile-time", nl);
    executeDemoWhilerGuarded();
    print();

    showOp("using lambda to operate on items", nl);
    executeCollectionWithOperation();
    std::cout << "\n";

    showOp("using std::for_each algorithm to modify items", nl);
    executeForEachAlgorithm();

    // #define TEST
    #ifdef TEST
    testFormat();
    #endif

    print("\n  That's all Folks!\n\n");
}
/*-----------------------------------------------
  Test Analysis formatting functions
  - turned off by default
*/
void testFormat() {

  showNote("Test and demonstrate formatting functions");
  
  showOp("demonstrate Point show()");
  print("default indent = 4 and width = 7:");
  Point<int, 15> p2;
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

  showNote("Using consolidated format function", 40, nl);
  
  /* qualification needed to avoid ambiguity with std::format */
  std::cout << Analysis::format(adouble, "adouble", nl);
  std::cout << Analysis::format(astring, "astring", nl);
  std::vector<double> avec{ 1, 2, 3, 4.5, -3.14159 };
  std::cout << Analysis::format(avec, "avec", nl);
  std::cout << Analysis::format(amap, "amap", nl);
}
