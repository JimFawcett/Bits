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
#include <unordered_map>  // unordered_map<k,V> class
#include <set>            // set<T> class
#include <thread>         // this_thread
#include "AnalysisGen.h"  // Analysis functions
#include "HelloTemplates.h"        // Stats class declaration
#include "Stats.h"
#include "PointsGen.h"    // Point<T, N> class declaration

using namespace Analysis;
using namespace Points;


/*-----------------------------------------------
  Overload operator<< for std::vector, 
  required for demo_std_generic_types() 
  and testformats()
*/
template<typename T>
std::ostream& operator<<(std::ostream& out, std::vector<T>& v) {
  out << format(v, "vector<T>", "", DisplayParams.left, DisplayParams.width);
  return out;
}

/*-----------------------------------------------
  demonstrate use of std generic types
*/
void demo_std_generic_types() {
  
  showNote("Demo std generic types", 30);
  showOp("array<int,4>",nl);
  auto a = std::array<int, 4> { 1, 2, 3, 4 };
  showArray(a);     // works only for arrays
  showSeqColl(a);   // works for any sequential collection

  showOp("vector<double>", nl);
  std::vector<double> v = { 1.0, 1.5, 2.0, 2.5 };
  std::cout << v << "\n"; // uses operator<< overload for vectors, above
  showSeqColl(v);         // any sequential collection
  std::cout << formatColl(v, "v", "\n"); // any STL collection

  showOp("std::map<string,int>", nl);
  std::map<std::string, int> m {
    {"zero", 0}, {"one", 1}, {"two", 2}, {"three", 3}
  };
  std::cout << formatColl(m, "m", "\n"); // any STL collection
  showAssocColl(m);  // coll elements must be std::pair<K,V>

  showOp("std::unordered_map<string,int>", nl);
  std::unordered_map<std::string, int> um {
    {"zero", 0}, {"one", 1}, {"two", 2}, {"three", 3}
  };
  std::cout << formatColl(um, "um", "\n"); // any STL collection
  showAssocColl(um);  // coll elements must be std::pair<K,V>
  println();
}

/*-----------------------------------------------
  demonstrate use of generic functions
*/
void demo_generic_functions() {

  println();
  showNote("demo generic functions", 35);

  showOp("showType for std::string");
  std::string s = "a string";
  showType(s, "s", nl);

  showOp("showType for std::vector");
  std::vector<int> v {1, 2, 3, 2, 1, 0, -1, -2};
  showType(v, "v", nl);

  /*-------------------------------------------------------
    showSeqColl works for any collection with 
    iterator, integer indexing, and size() function
  */
  showOp("showSeqColl for std::string", nl);
  showSeqColl(s);

  showOp("showSeqColl for std::vector", nl);
  showSeqColl(v);

  /*-------------------------------------------------------
    showAssocColl works for any collection with 
    interator and std::pair<key, Value> elements
  */
  showOp("showAssocColl for std::map", "\n");
  std::map<std::string, int> m { 
    {"zero", 0}, {"one", 1}, {"two", 2}, {"three", 3}, 
    {"four", 4}, {"five", 5} 
  };
  showAssocColl(m);

  showOp("showAssocColl for std::unordered_map", "\n");
  std::unordered_map<std::string, int> um1 { 
    {"zero", 0}, {"one", 1}, {"two", 2}, {"three", 3}, 
    {"four", 4}, {"five", 5} 
  };
  showAssocColl(um1);

  /*-------------------------------------------------------
    Analysis::format works for any collection with 
    interator and std::pair<key, Value> elements
  */
  showOp("Analysis::format for int");
  int mol = 42;
  std::cout << Analysis::format(42, "mol", "\n");
  // need to distinguish Analysis::format from std::format

  showOp("Analysis::format for std::string");
  std::cout << Analysis::format(s, "s", "\n");

  showOp("Analysis::format for std::vector");
  std::cout << Analysis::format(v, "v", "\n", 2, 5);

  showOp("Analysis::format for std::map");
  std::cout << Analysis::format(m, "m", "\n", 2, 4); 

  showOp("Analysis::format for std::unordered_map");
  std::unordered_map<std::string, int> um2 {
    {"zero", 0}, {"one", 1}, {"two", 2}, {"three", 4}, 
    {"four", 4}, {"five", 5} 
  };
  std::cout << Analysis::format(um2, "um", "\n", 2, 4);
}

/*-----------------------------------------------
  Demo execution starts here
*/
void testFormats();

int main() {

    showNote("Demonstrate C++ Generics", 30, nl);
  
    demo_std_generic_types();
    demo_custom_type_HelloTemplates();
    demo_custom_type_Stats();
    demo_custom_type_Point();
    demo_generic_functions();

    testtime();
    
    for(size_t i=0; i<4; ++i) {
      testtimer();
      std::this_thread::sleep_for(std::chrono::seconds(1));
    }

    // #define TEST
    #ifdef TEST
      testFormats();
    #endif

    print("\n  That's all Folks!\n\n");
}
/*-- testFormats adds details to the main demonstration --*/
void testFormats() {

    println();
    showNote("Test and demonstrate formatting functions", 50);
    
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
    std::cout << formatScalar(adouble, "adouble", nl) << "\n";

    showNote("Using consolidated format function", 40);
    
    std::cout << format<double>(adouble, "adouble", nl);
    //std::cout << format<std::string>(astring, "astring", nl);
    std::vector<double> avec{ 1, 2, 3, 4.5, -3.14159 };
    std::cout << format(avec, "avec", nl);
    std::cout << format(amap, "amap", nl);
}