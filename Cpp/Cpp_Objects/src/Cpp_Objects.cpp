/*-------------------------------------------------------------------
  Cpp_Objects.cpp
  - depends on Points.h to provide two user-defined point classes
  - depends on Analysis.h for several display and analysis functions
*/
#include <iostream>     // std::cout
#include <memory>       // std::unique_ptr
#include <vector>       // vector<T> class
#include <array>        // array<T> class
#include <map>          // map<K,V> class
#include <set>          // set<T> class
#include "Analysis.h"   // Analysis functions for this demo
#include "Points.h"     // Two Point class declarations
/*
  This demo uses the std::string and std::vector<T> classes
  and two user defined classes, Point1 and Point2<T>, to 
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

/*-------------------------------------------------------------------
  Demonstration starts here 
*/
int main() {

    print("Demonstrate C++ Objects\n");

    showNote("primitive C++ types size_t and double");
    size_t st = 42;
    std::cout << "\n  size_t st = " << st;
    showType(st, "st", nl);

    double d = 3.1415927;
    std::cout << "\n  double d = " << d;
    showType(d, "ld", nl);

    showNote("std library types string and vector<T>");

    /* create and display std::string object */
    auto str = std::string("\"Wile E. Coyote\"");
    auto out = std::string("contents of str = ") + str;
    print(out);
    print("--- showType(str, \"str\"); ---");
    showType(str, "str", nl);

    /* create and display std::vector<double> */
    auto vec = std::vector<double>{ 3.5, 3, 2.5, 2 };
    std::cout << vec;
    print("--- showType(vec, \"vec\"); ---");
    showType(vec, "vec", nl);

    showOp("vec[2] = -2.5;");
    vec[2] = -2.5;
    std::cout << "\n  vec:" << vec;

    showOp("auto vec2 = vec : copy construction");
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

    showNote("user-defined types Point1 and Point2<T>");
    Point1 p1;
    p1.show();
    p1.xCoor() = 42;
    p1.zCoor() = -3;
    p1.show();
    print();
    
    print("--- showType(p1, \"p1\", nl) ---");
    showType(p1, "p1", nl);
    std::cout << "  p1.xCoor() returns value " 
              << p1.xCoor() << "\n";

    Point2<double> p2(5);
    p2.show();
    showNote(
      "p2.coords() = std::vector<double>\n    "
      "{ 1.0, -2.0, 3.0, 4.5, -42.0 }"
    );
    p2.coords() = std::vector<double>{1.0, -2.0, 3.0, 4.5, -42.0 };
    p2.show();
    print("--- showType(p2, \"p2\", nl); ---");
    showType(p2, "p2", nl);
    std::cout << "  p2.coords()[2] = " << p2.coords()[2] << "\n";
    
    /* standard library type std::string */
    /* uses alias pU for std::unique_ptr, defined above */
    showNote("heap-based string instance");
  
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
    std::cout << "\n  *pVec = " << *pVec;
    showType(*pVec, "*pVec", nl);
    std::cout << "\n  pVec = " << pVec;
    showType(move(pVec), "move(pVec)", nl);

    /* custom point types */
    showNote("heap-based Point1 instance");
    showOp("pU<Point1> pPoint1(new Point1())");
    pU<Point1> pPoint1(new Point1());
    pPoint1->show();
    pPoint1->xCoor() = 1;
    pPoint1->yCoor() = 2;
    pPoint1->zCoor() = -3;
    pPoint1->show();

    std::cout << "\n  pPoint1->zCoor() = " << pPoint1->zCoor();
    showOp("showType(*pPoint1, \"*pPoint1\");");
    showType(*pPoint1, "*pPoint1");
    showOp("showType(std::move(pPoint1), \"pPoint1\");");
    showType(std::move(pPoint1), "pPoint1", nl);
    /* pPoint1 moved, so now invalid */

    showNote("heap-based Point2<T> instance");
    showOp("pU<Point2<double>> pPoint2(new Point2<double>(4))");
    pU<Point2<double>> pPoint2(new Point2<double>(4));
    pPoint2->show();
    showOp(
      "pPoint2->coords() = \n"
      "     std::vector<double>{ 1.0, 3.5, -2.0, 42.0 };"
    );
    pPoint2->coords() = std::vector<double>{ 1.0, 3.5, -2.0, 42.0 };
    pPoint2->show();
    std::cout << "\n  value of pPoint2->coords()[1] is " 
              << pPoint2->coords()[1];
    print("--- showType(*pPoint2, \"*pPoint2\"); ---");
    showType(*pPoint2, "*pPoint2");
    print("--- showType(std::move(pPoint2), \"pPoint2\"); ---");
    showType(std::move(pPoint2), "pPoint2");
    /* pPoint2 moved, so now invalid */

    print();
    showNote("Test and demonstrate formatting functions");
    println("\n--- demonstrate Point2 show() ---");
    print("default indent = 4 and width = 7:");
    Point2<int> p2a(15);
    p2a.show();
    size_t saveLeft = p2a.left();
    size_t saveWidth = p2a.width();
    print("\n  indent = 6, width = 12:");
    p2a.left() = 6;
    p2a.width() = 12;
    p2a.show();

    println(
      "\n--- demonstrate operator<< "
      "overload for Point2 ---"
    );
    p2a.left() = saveLeft;
    p2a.width() = saveWidth;
    print("default indent = 4 and width = 7:");
    std::cout << p2a;
    print("\n  indent = 6, width = 12:");
    p2a.left() = 6;
    p2a.width() = 12;
    std::cout << p2a;

    println(
      "\n--- demonstrate operator<< "
      "overload for vector ---"
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

    std::map<int, std::string> amap { {1, "one"}, {2, "two"}, {3, "three"} };
    std::cout << formatColl(amap, "amap", nl, 2, 4);

    std::set<std::string> aset { "one", "two", "three", "four", "five" };
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

    print("\n  That's all Folks!\n\n");
}
