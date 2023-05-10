/*-------------------------------------------------------------------
  Cpp_Generics.cpp
  - depends on PointsGen.h to provide user-defined point class
  - depends on Analysis.h for several display and analysis functions
*/
#include <iostream>     // std::cout
#include <memory>       // std::unique_ptr
#include <vector>       // vector<T> class
#include <array>        // array<T> class
#include <map>          // map<K,V> class
#include <set>          // set<T> class
#include "Analysis.h"   // Analysis functions for this demo
#include "PointsGen.h"  // PointN<T> class declaration
/*
  This demo uses std::basic_string<char> and std::vector<T>
  classes and user defined class, PointN<T>, to 
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
  alias type names
  - pU<T> is the same type as std::unique_ptr<T> 
  - this just provides a shorter name
*/
template<typename T>
using pU = std::unique_ptr<T>;
/*
  The std library uses aliases:
  using string = basic_string<char>;
  using wstring = basic_string<wchar_t>;
*/
/*-------------------------------------------------------------------
  Demonstration starts here 
*/
void testFormats();

int main() {

    print("Demonstrate C++ Generic Objects\n");

    showNote("std library types string and vector<T>");
    /* create and display std::basic_string<char> object */
    auto str = std::basic_string<char>("\"Wile E. Coyote\"");
    auto out = std::basic_string<char>("contents of str = ") + str;
    print(out);
    print("--- showType(str, \"str\"); ---");
    showType(str, "str", nl);

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
      "has no affect on source vec.", nl
    );

    showNote("user-defined type PointN<T>");

    PointN<double> p1(5);
    p1.show("p1");

    showNote(
      "p1.coords() = std::vector<double>\n    "
      "{ 1.0, -2.0, 3.0, 4.5, -42.0 }"
    );
    p1.coords() = std::vector<double>{1.0, -2.0, 3.0, 4.5, -42.0 };
    p1.show("p1");
    showOp("showType(p1, \"p1\", nl);");
    showType(p1, "p1", nl);
    std::cout << "  p1.coords()[2] = " << p1.coords()[2] << "\n";
    
    /*-- copy construction --*/

    auto p2 = p1;
    showOp("auto p2 = p1");
    p2.show("p2");
    showOp("p2[1] = 5.5");
    p2[1] = 5.5;
    p2.show("p2");
    p1.show("p1");
    print();

    showNote("heap-based string instance");
  
    /* standard library type std::basic_string<char> */
    /* uses alias pU for std::unique_ptr, defined above */
    showOp(
      "pU<std::basic_string<char>> "
      "pStr(new std::basic_string<char>(\"\\\"Road Runner\\\"\")"
    );
    pU<std::basic_string<char>> pStr(new std::basic_string<char>("\"Road Runner\""));
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
    std::cout << "\n  *pVec = " << *pVec;
    showType(*pVec, "*pVec", nl);
    std::cout << "\n  pVec = " << pVec;
    showType(move(pVec), "move(pVec)", nl);

    /* custom point type */

    showNote("heap-based PointN<T> instance");
    
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
    
    showOp("showType(std::move(pPointN), \"pPointN\");");
    showType(std::move(pPointN), "pPointN");
    /* pPointN moved, so now invalid */
    print();

    //#define TEST
    #ifdef TEST
      testFormats();
    #endif

    print("\n  That's all Folks!\n\n");
}

void testFormats() {

    showNote("Test and demonstrate formatting functions");
    
    showOp("demonstrate PointN show()");
    print("default indent = 4 and width = 7:");
    PointN<int> p1a(15);
    p1a.show("p1a");
    size_t saveLeft = p1a.left();
    size_t saveWidth = p1a.width();
    print("\n  indent = 6, width = 12:");
    p1a.left() = 6;
    p1a.width() = 12;
    p1a.show("p1a");

    showOp(
      "demonstrate operator<< overload for PointN ---"
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
    
    std::cout << format(adouble, "adouble", nl);
    std::cout << format(astring, "astring", nl);
    std::vector<double> avec{ 1, 2, 3, 4.5, -3.14159 };
    std::cout << format(avec, "avec", nl);
    std::cout << format(amap, "amap", nl);
}