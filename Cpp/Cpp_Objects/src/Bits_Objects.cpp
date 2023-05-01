/*-------------------------------------------------------------------
  Cpp_Objects.cpp
  - depends on Points.h to provide user-defined point class
  - depends on Analysis.h for several display and analysis functions
*/
#include <iostream>     // std::cout
#include <memory>       // std::unique_ptr
#include <vector>       // vector<T> class
#include <array>        // array<T> class
#include <map>          // map<K,V> class
#include <set>          // set<T> class
#include "Analysis.h"   // Analysis functions for this demo
#include "Points.h"     // Point4D class declaration
/*
  This demo uses the std::string and std::vector<T> classes
  and a user defined class, Point4D, to illustrate how objects 
  are defined and instantiated.

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

void testFormats();  // declare function for testing formats (see below)

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

    showNote("user-defined type Point4D");
    Point4D p1;
    p1.show();
    p1.xCoor() = 42;
    p1.zCoor() = -3.5;
    /*- t contains time of construction -*/
    p1.show();
    print();
    
    print("--- showType(p1, \"p1\", nl) ---");
    showType(p1, "p1", nl);
    std::cout << "  p1.xCoor() returns value " 
              << p1.xCoor() << "\n";

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
    std::cout << "\n  *pVec = " << *pVec;
    showType(*pVec, "*pVec", nl);
    std::cout << "\n  pVec = " << pVec;
    showType(move(pVec), "move(pVec)", nl);

    /* custom point types */
    showNote("heap-based Point4D instance");
    showOp("pU<Point4D> pPoint4D(new Point4D())");
    pU<Point4D> pPoint4D(new Point4D());
    pPoint4D->show();
    pPoint4D->xCoor() = 1;
    pPoint4D->yCoor() = 2;
    pPoint4D->zCoor() = -3;
    pPoint4D->updateTime();
    pPoint4D->show();

    std::cout << "\n  pPoint4D->zCoor() = " << pPoint4D->zCoor();
    showOp("showType(*pPoint4D, \"*pPoint4D\");");
    showType(*pPoint4D, "*pPoint4D");
    showOp("showType(std::move(pPoint4D), \"pPoint4D\");");
    showType(std::move(pPoint4D), "pPoint4D", nl);
    /* pPoint4D moved, so now invalid */

//#define TEST
#ifdef TEST
    testFormats();
#endif
    print("\n  That's all Folks!\n\n");
}

void testFormats() {

    showNote("Test and demonstrate formatting functions");
    
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

    std::array<double, 5> array = { 1, 2, 3, 4.5, -3.14159 };
    std::cout << formatColl(array, "array", nl, 2, 4);

    std::map<int, std::string> map {
       {1, "one"}, {2, "two"}, {3, "three"} 
    };
    std::cout << formatColl(map, "map", nl, 2, 4);

    std::set<std::string> set { 
      "one", "two", "three", "four", "five" 
    };
    std::cout << formatColl(set, "set", nl, 2, 4);

    std::string astring = "this is a string";
    std::cout << formatString(astring, "astring", nl, 2);

    double adouble { 3.1415927 };
    std::cout << formatScalar(adouble, "adouble", nl);

    showNote("Using consolidated format function", nl);
    
    std::cout << format(adouble, "adouble", nl);
    std::cout << format(astring, "astring", nl);
    std::vector<double> vect{ 1, 2, 3, 4.5, -3.14159 };
    std::cout << format(vect, "vect", nl);
    std::cout << format(map, "map", nl);
}
