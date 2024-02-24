/*-------------------------------------------------------------------
  Cpp_Generics.cpp
  - demonstrates creating and using std::library generic types:
      array, basic_string, vector, and map
  - demonstrates creating and using user-defined generic types:
      Stats, Demo, and PointsN
  - depends on PointsGen.h to provide user-defined point class
  - depends on Analysis.h for several display and analysis functions
*/
/*-----------------------------------------------
  Note:
  Find all Bits code, including this in
  https://github.com/JimFawcett/Bits
  You can clone the repo from this link.
-----------------------------------------------*/
#include <iostream>       // std::cout
#include <memory>         // std::unique_ptr
#include <vector>         // vector<T> class
#include <array>          // array<T> class
#include <map>            // map<K,V> class
#include <set>            // set<T> class
#include "AnalysisGen.h"  // Analysis functions
#include "PointsGen.h"    // Point<T, N> class declaration
#include "Stats.h"        // Stats class declaration
/*
  
*/
#pragma warning(disable: 4984)  // warns about C++17 extension

using namespace Points;

/*-------------------------------------------------------------------
  Building types and functions for demonstration 
*/
/*-- simple user-defined type --*/
template<typename T>
class Demo {
public:
  Demo() = default;
  Demo(T& tin) : t(tin) {};
  Demo(const Demo<T>& t) = default;
  Demo<T>& operator=(const Demo<T>&t) = default;
  ~Demo() = default;
  T& value() { return t; }
  void show();
private:
  T t;
};

template<typename T>
void Demo<T>::show() {
  std::cout << "  Demo<T> {\n  ";
  std::cout << "  type: " 
            << truncate(DisplayParams.trunc,typeid(t).name());  // show type
  std::cout << ", size: " << sizeof(t);  // show size on stack
  std::cout << ", value: " << t;
  std::cout << "\n  }\n";
}
/*-----------------------------------------------
  Demonstration functions
*/
/*-- showArray function --*/
template<typename T, int N>
void showArray(std::array<T,N> &a) {
  std::cout << "  array<T,N> [";
  std::cout << a[0];
  for(int i=1; i<N; ++i) {
    std::cout << ", " << a[i];
  }
  std::cout << "]" << std::endl;
}
/*-- show collection function --*/
template<typename C>
void showColl(const C& c) {
  std::cout << "  Collection [";
  std::cout << c[0];
  for(size_t i=1; i<c.size(); ++i) {
    std::cout << ", " << c[i];
  }
  std::cout << "]" << std::endl;
}
/*-- showMap function --*/
template<typename K, typename V>
void showMap(const std::map<K,V> &m) {
  std::cout << "  map<K,V> {\n    ";
  bool first = true;
  for(const auto& pair : m) {
    if(first) {
      std::cout << "{" << pair.first << "," << pair.second << "}";
      first = false;
    }
    else {
      std::cout << ", {" << pair.first << "," << pair.second << "}";
    }
  }
  std::cout << "\n  }\n";
}
/*-- demonstrate use of std generic types --*/
void demo_std_generic_types() {
  
  showNote("Demo std generic types", nl);
  showOp("array<int,4>",nl);
  auto a = std::array<int, 4> { 1, 2, 3, 4 };
  showArray(a);
  showColl(a);

  showOp("vector<double>");
  std::vector<double> v = { 1.0, 1.5, 2.0, 2.5 };
  std::cout << v << "\n";
  showColl(v);

  showOp("map<string,int>", nl);
  std::map<std::string, int> m {
    {"zero", 0}, {"one", 1}, {"two", 2}, {"three", 3}
  };
  showMap(m);  // std::map<K,V> only
  std::cout << formatColl(m, "m", "\n"); // any STL collection
}
/*-- demonstrate use of user-defined types --*/
void demo_user_defined_generic_types() {
  
  showNote("Demo user-defined generic types", nl);
  showOp("Stats<double>", nl);
  std::vector<double> v { 1.0, 2.5, -3.0, 4.5 };
  showColl(v);
  Stats<double> s(v);
  std::cout << "  min: " << s.min();
  std::cout << ", max: " << s.max();
  std::cout << ", sum: " << s.sum();
  std::cout << ", avg: " << s.avg() << std::endl;

  showOp("Demo<int>", nl);
  int arg = 42;
  Demo<int> dem(arg);
  dem.show();

  showOp("Point<double, 3> p1 {1.0, 1.5, 2.0}");
  Point<double, 3> p1 {1.0, 1.5, 2.0};
  p1.show("p1");
  showOp("Point<double, 3> p2 {1.0, 1.5}");
  Point<double, 3> p2 {1.0, 1.5};
  p2.show("p2");
  showOp("Point<double, 3> p3 {1.0, 1.5, 2.0, 2.5}");
  Point<double, 3> p3 {1.0, 1.5, 2.0, 2.5};
  p3.show("p3");
  //auto t = p3.time();
  std::cout << "\n  p3.timeToString() = \"" << p3.timeToString() << "\"";
  //std::cout << "\n  p3.time().localtime()."
  println();
}
/*-- demonstrate use of generic functions --*/
void demo_generic_functions() {

showNote("demo generic functions", nl);

showOp("showType for std::string");
std::string s = "a string";
showType(s, "s", nl);

showOp("showType for std::vector");
std::vector<int> v {1, 2, 3, 2, 1};
showType(v, "v", nl);

showOp("showColl for std::string", nl);
showColl(s);

showOp("showColl for std::vector", nl);
showColl(v);
}
void testtime() {
  showNote("test Time");
  Time t;
  t.getLocalTime();
  std::cout << "\n  datetime = " << t.toString() << std::endl;
  std::cout << "\n  epoch in secs = " << t.getTime();
  std::cout << "\n  year:     " << t.year();
  std::cout << "\n  month:    " << t.month();
  std::cout << "\n  day:      " << t.day();
  std::cout << "\n  hour:     " << t.hour();
  std::cout << "\n  minutes:  " << t.minutes();
  std::cout << "\n  seconds:  " << t.seconds();
  std::cout << "\n  timezone: " << t.getTimeZone();
  std::cout << std::endl;

  t.getGMTTime();
  std::cout << "\n  datetime = " << t.toString() << std::endl;
  std::cout << "\n  epoch in secs = " << t.getTime();
  std::cout << "\n  year:     " << t.year();
  std::cout << "\n  month:    " << t.month();
  std::cout << "\n  day:      " << t.day();
  std::cout << "\n  hour:     " << t.hour();
  std::cout << "\n  minutes:  " << t.minutes();
  std::cout << "\n  seconds:  " << t.seconds();
  std::cout << "\n  timezone: " << t.getTimeZone();
}
/*-----------------------------------------------
  Demo execution starts here
*/
void testFormats();

int main() {

    showNote("Demonstrate C++ Generics", nl);
  
    demo_std_generic_types();
    demo_user_defined_generic_types();
    demo_generic_functions();
    testtime();
    // #define TEST
    #ifdef TEST
      testFormats();
    #endif

    print("\n  That's all Folks!\n\n");
}
/*-- testFormats adds details to the main demonstration --*/
void testFormats() {

    showNote("Test and demonstrate formatting functions");
    
    showOp("demonstrate Point show()");
    print("default indent = 4 and width = 7:");
    Point<int, 15> p1a;
    p1a.show("p1a");
    size_t saveLeft = p1a.left();
    size_t saveWidth = p1a.width();
    print("\n  indent = 6, width = 12:");
    p1a.left() = 6;
    p1a.width() = 12;
    p1a.show("p1a");

    showOp(
      "demonstrate operator<< overload for Point ---"
    );
    p1a.left() = saveLeft;
    p1a.width() = saveWidth;
    print("default indent = 4 and width = 7:");
    std::cout << p1a;
    print("\n  indent = 6, width = 12:");
    p1a.left() = 6;
    p1a.width() = 12;
    std::cout << p1a;

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

    std::map<int, std::basic_string<char>> amap {
       {1, "one"}, {2, "two"}, {3, "three"} 
    };
    std::cout << formatColl(amap, "amap", nl, 2, 4);

    std::set<std::basic_string<char>> aset { 
      "one", "two", "three", "four", "five" 
    };
    std::cout << formatColl(aset, "aset", nl, 2, 4);

    std::basic_string<char> astring = "this is a string";
    std::cout << formatString(astring, "astring", nl, 2);

    double adouble { 3.1415927 };
    std::cout << formatScalar(adouble, "adouble", nl);

    showNote("Using consolidated format function", nl);
    
    std::cout << format<double>(adouble, "adouble", nl);
    //std::cout << format<std::string>(astring, "astring", nl);
    std::vector<double> avec{ 1, 2, 3, 4.5, -3.14159 };
    std::cout << format(avec, "avec", nl);
    std::cout << format(amap, "amap", nl);
}