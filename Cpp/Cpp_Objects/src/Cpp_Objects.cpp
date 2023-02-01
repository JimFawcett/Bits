// Cpp_Objects.cpp

#include <iostream>     // std::cout
#include <typeinfo>     // typeid
#include <memory>       // std::unique_ptr
#include <utility>      // move()
#include <vector>       // vector<T> class
#include <sstream>      // stringstream
#include <array>        // array<T> class
#include <map>          // map<K,V> class
#include <set>          // set<T> class
#include <type_traits>  // is_scalar, if constexpr
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
const std::string nl = "\n";
/*-------------------------------------------------------------------
  Mutable globals are a common source of bugs.  We try not
  to use them, but will use DisplayParams here to control how
  the insertion operator sends instances to standard output.
*/
struct displayParams {
  size_t left = 2;    // number of spaces to indent
  size_t width = 7;   // width of display row
  size_t trunc = 55;  // replace text after trunc with ...
} DisplayParams;      // global object

/*------------------------------------------------------------------- 
  Display and analysis functions are defined at the end
  but C++ requires declaration before use.
*/
template<typename T>
void showType(T t, const std::string &nm, const std::string& suffix = "");
void showNote(const std::string& txt, const std::string& suffix = "");
void showOp(const std::string& opstr, const std::string& suffix = "");
void print(const std::string& txt = "");
void println(const std::string& txt = "");
std::string truncate(size_t N, const char* pStr);
std::string indent(size_t n);
template<typename T>
std::string fold(std::vector<T>& v, size_t left, size_t width);
template<typename T>
std::string formatColl(
  const T& t, const std::string& nm,
  const std::string& suffix = "", size_t left = 2, size_t width = 7
);
template<typename T>
std::string formatScalar(
  const T& t, const std::string& nm, 
  const std::string& suffix = "", size_t left = 2
);
template<typename T>
std::string formatString(
  const T& t, const std::string& nm, const std::string& suffix,
  size_t left = 2
);
template<typename T>
std::string format(
  const T& t, const std::string& nm, const std::string& suffix = "",
  size_t left = 2, size_t width = 7
);

/*-----------------------------------------------
  alias type name 
  - pU<T> is the same type as std::unique_ptr<T> 
*/
template<typename T>
using pU = std::unique_ptr<T>;

/*-------------------------------------------------------------------
  Point1 class represents a point in an integral 3-Dimensional
  lattice. Simple enough for illustration, but still useful.

  It declares all of the special class methods, most with
  default qualifiers to indicate that the compiler will
  generate them as needed.

  The word ctor is an abbreviation of constructor and dtor
  an abbreviation for destructor.
*/
class Point1 {
public:
  Point1();                                       // default ctor
  Point1(const Point1& pt) = default;             // copy ctor
  Point1(Point1&& pt) = default;                  // move ctor
  Point1& operator=(const Point1& pt) = default;  // copy assignment
  Point1& operator=(Point1&& pt) = default;       // move assignment
  ~Point1() = default;                            // dtor
  void show();
  int& xCoor() { return x; }
  int& yCoor() { return y; }
  int& zCoor() { return z; }
private:
  int x;
  int y;
  int z;
};

Point1::Point1() {
  x = y = z = 0;
}
void Point1::show() {
  std::cout << "\n  " << "Point1";
  std::cout << " { " << x << ", " << y << ", " << z << " }";
}
/* required for showType(T t, const std::string& nm) */
std::ostream& operator<<(std::ostream& out, Point1& t1) {
  out << "Point1";
  out << " { " << t1.xCoor() << ", " << t1.yCoor() << ", " 
               << t1.zCoor() << " }";
  return out;
}

/*-------------------------------------------------------------------
  Point2<T> class represents a point in an n-Dimensional hyperspace.
  It is more flexible than Point1 by using a template to support
  a variety of coordinate types, and by using a vector to hold
  any finite number of coordinates.

  Its default constructor Point2() is declared delete so it won't
  be provided here nor generated by the compiler.
*/
template<typename T>
class Point2 {
public:
  Point2() = delete;                              // default ctor
  Point2(size_t N);
  Point2(const Point2& pt) = default;             // copy ctor
  Point2(Point2&& pt) = default;                  // move ctor
  Point2& operator=(const Point2& pt) = default;  // copy assignment
  Point2& operator=(Point2&& pt) = default;       // move assignemnt
  ~Point2() = default;                            // dtor
  void show();
  std::vector<T>& coords() { return coord; }
  size_t& left() { return _left; }
  size_t& width() { return _width; }
private:
  std::vector<T> coord;
  size_t _left = 2;   // default display indent
  size_t _width = 7;  // default display row width
};
/*-----------------------------------------------
  Point2<T> constructor with size 
*/
template<typename T>
Point2<T>::Point2(size_t N) {
  for(size_t i=0; i<N; i++) {
    coord.push_back(T{0});
  }
}
/*-----------------------------------------------
  Pointt2<T> display function 
*/
template<typename T>
void Point2<T>::show() {
  std::cout << "\n" << indent(_left) << "Point2<T>";
  std::cout << " {\n";
  std::cout << fold(coord, _left + 2, _width);
  std::cout << indent(_left) << "}";
}
/*-----------------------------------------------
  Overload operator<< required for 
  showType(Point2<T> t, const std::string& nm) 
*/
template<typename T>
std::ostream& operator<<(std::ostream& out, Point2<T>& t2) {
  out << "\n" << indent(t2.left()) << "Point2<T>";
  out << " {\n";
  out << fold(t2.coords(), t2.left() + 2, t2.width());
  out << indent(t2.left()) << "}";
  return out;
}
/*-----------------------------------------------
  Overload operator<< required for 
  showType(std::vector<T> v, const std::vector<T>& nm) 
*/
template<typename T>
std::ostream& operator<<(std::ostream& out, std::vector<T>& v) {
  out << format(v, "vector<T>", "", DisplayParams.left, DisplayParams.width);
  return out;
}
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
      "Copy construction, auto vec2 = vec, creates independent instance.\n  "
      "So changing target vec2 has no affect on source vec.", nl
    );

    showNote("user-defined types Point1 and Point2<T>");
    Point1 p1;
    p1.show();
    p1.xCoor() = 42;
    p1.zCoor() = -3;
    p1.show();
    print("--- showType(p1, \"p1\", nl) ---");
    showType(p1, "p1", nl);
    std::cout << "  p1.xCoor() returns value " 
              << p1.xCoor() << "\n";

    Point2<double> p2(5);
    p2.show();
    showNote(
      "p2.coords() = std::vector<double>"
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

    /* custom types */
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
    /* pPoint1 moved, so invalid */

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
    /* pPoint2 moved, so invalid */

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
/*-------------------------------------------------------------------
  Display and Analysis function definitions
---------------------------------------------------------------------
*/
/*-----------------------------------------------
  Display calling name, static class, and size
*/
template<typename T>
void showType(T t, const std::string &callname, const std::string& suffix) {
  std::cout << "\n  " << callname;          // show name at call site
  std::cout << " type: " 
            << truncate(DisplayParams.trunc,typeid(t).name());  // show type
  // std::cout << "\n  value: " << t;          // show value
  std::cout << "\n  size:  " << sizeof(t);  // show size on stack
  std::cout << suffix;
}
/*-----------------------------------------------
  Display emphasized text
*/
void showNote(const std::string& txt, const std::string& suffix) {
  print("--------------------------------------------------");
  print(txt);
  print("--------------------------------------------------");
  std::cout << suffix;
}
/*-----------------------------------------------
  Display emphasized line
*/
void showOp(const std::string& opstr, const std::string& suffix) {
  std::cout << "\n  --- " << opstr << " ---" << suffix;
}
/*-----------------------------------------------
  Helper function for formatting output
  - truncates line to N chars and adds ellipsis
*/
std::string truncate(size_t N, const char* pStr) {
  std::string temp(pStr);
  if(temp.length() > N) {
    temp.resize(N);
    return temp + "...";
  }
  return temp;
}
/*-----------------------------------------------
  Helper function for formatting output
  - generates string of n blanks to offset text
*/
std::string indent(size_t n) {
  return std::string(n, ' ');
}
/*-----------------------------------------------
  Helper function for formatting output
  - folds lines after width elements
*/
template<typename T>
std::string fold(std::vector<T>& v, size_t left, size_t width) {
  std::stringstream out("\n");
  out << indent(left);
  for(int i=0; i<v.size(); ++i) {
    if((i % width) == 0 && i != 0 && i != width - 1) {
      out << "\n" << indent(left);
    }
    if(i < v.size() - 1) {
      out << v[i] << ", ";
    }
    else {
      out << v[i] << "\n";
      break;
    }
  }
  return out.str();
}
/*-----------------------------------------------
  Helper function for formatColl
  - defines out << std::pair<K,V>
  - used in formatColl for associative containers
*/
template<typename K, typename V>
std::stringstream& operator<<(
  std::stringstream& out, const std::pair<K,V>& p
) {
  out << "{" << p.first << ", " << p.second << "}";
  return out;
}
/*-----------------------------------------------
  Format output for Collection types
  - any type with begin() and end() like
    all the STL containers.
*/
template<typename Coll>
std::string formatColl(
  const Coll& c, const std::string& nm, const std::string& suffix,
  size_t left, size_t width
) {
  std::stringstream out;
  out << "\n" << indent(left) << nm << ": {\n" << indent(left + 2);
  size_t i = 0;
  for(const Coll::value_type& elem : c) {
    if((i % width) == 0 && i != 0 && i != width - 1) {
      out << "\n" << indent(left + 2);
    }
    if(i < c.size() - 1) {
      out << elem << ", ";
    }
    else {
      out << elem << "\n" << indent(left) << "}" << suffix;
      break;
    }
    ++i;
  }
  return out.str();
}
/*-----------------------------------------------
  Format output for scalar types like primitives
*/
template<typename T>
std::string formatScalar(
  const T& t, const std::string& nm, const std::string& suffix,
  size_t left
) {
  std::stringstream out;
  out << "\n" << indent(left) << nm << ": " << t << suffix;
  return out.str();
}
/*-----------------------------------------------
  Format output for strings
  - indent and embed in quotation marks
*/
template<typename T>
std::string formatString(
  const T& t, const std::string& nm, const std::string& suffix,
  size_t left
) {
  std::stringstream out;
  out << "\n" << indent(left) << nm << ": \"" << t << "\"" << suffix;
  return out.str();
}
/*-----------------------------------------------
  Defines is_iterable trait
  - detects STL containers and user-defined types
    that provide iteration
*/
template <typename T, typename = void>
struct is_iterable : std::false_type {};

// this gets used only when we can call 
// std::begin() and std::end() on that type
template <typename T>
struct is_iterable<
  T, 
  std::void_t
    <decltype(std::begin(std::declval<T>())),
     decltype(std::end(std::declval<T>()))>
> : std::true_type {};

template <typename T>
constexpr bool is_iterable_v = is_iterable<T>::value;

/*-----------------------------------------------
  Displays almost everything.
  - strings work better with formatString(...)
*/
template<typename T>
std::string format(
  const T& t, const std::string& nm, const std::string& suffix,
  size_t left, size_t width
) {
  if constexpr(is_iterable_v<T>) {
    return formatColl(t, nm, suffix, left, width);
  }
  else {
    return formatScalar(t, nm, suffix, left);
  }
}
/*-----------------------------------------------
  Display text after newline and indentation
*/
void print(const std::string& txt) {
  std::cout << "\n  " << txt;
}
/*-----------------------------------------------
  Display text after newline and indentation
  - provides trailing newline
*/
void println(const std::string& txt) {
  std::cout << "\n  " << txt << "\n";
}
