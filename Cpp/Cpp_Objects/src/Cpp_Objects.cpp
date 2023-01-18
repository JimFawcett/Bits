// Cpp_Objects.cpp

#include <iostream>  // std::cout
#include <typeinfo>  // typeid
#include <memory>    // std::unique_ptr
#include <utility>   // move()
#include <vector>    // vector class

/*
  Static Data Types:
    byte, bool, int, char, char16_t, char32_t, wchar_t
    float, double
    array, pointer, reference
    struct, class, enum class, using
  Qualifiers:
    short, long, const, unsigned

  Operations:
    Primitive types can all be copied.
    Most library and user-defined types can be copied, moved, and deleted
    by providing member constructors and destructor.  Often compiler 
    generation works well, but for classes with pointer members developers 
    must provide them.

  Processing:
    All types are static, operations run as native code, and no garbage
    collection is needed. Resources are returned at end of declr scope.
*/
#define STRINGIFY_(x) #x
#define STRINGIFY(x) STRINGIFY_(x)

const std::string nl = "\n";

/* C++ requires declaration before use */
template<typename T>
void showType(T t, const std::string &nm, const std::string& suffix = "");
void showNote(const std::string& txt, const std::string& suffix = "");
void print(const std::string& txt = "");
void println(const std::string& txt = "");
std::string truncate(size_t N, const char* pStr);

/* alias type name */
template<typename T>
using pU = std::unique_ptr<T>;

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
  std::cout << "\n  " << STRINGIFY(Point1);
  std::cout << " { " << x << ", " << y << ", " << z << " }";
}
/* required for showType(T t, const std::string& nm) */
std::ostream& operator<<(std::ostream& out, Point1& t1) {
  out << STRINGIFY(Point1);
  out << " { " << t1.xCoor() << ", " << t1.yCoor() << ", " 
               << t1.zCoor() << " }";
  return out;
}

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
private:
  std::vector<T> coord;
};

template<typename T>
Point2<T>::Point2(size_t N) {
  for(size_t i=0; i<N; i++) {
    coord.push_back(T{0});
  }
}
template<typename T>
void Point2<T>::show() {
  std::cout << "\n  " << STRINGIFY(Point2);
  std::cout << " { ";
  for(size_t i=0; i<coord.size(); ++i) {
    std::cout << coord[i];
    if(i < coord.size() - 1) {
      std::cout << ", ";
    }
  }
  std::cout << " }";
}
/* required for showType(T t, const std::string& nm) */
template<typename T>
std::ostream& operator<<(std::ostream& out, Point2<T>& t2) {
  out << STRINGIFY(Point2);
  out << " { ";
  for(size_t i=0; i < t2.coords().size(); ++i) {
    std::cout << t2.coords()[i];
    if(i < t2.coords().size() - 1) {
      std::cout << ", ";
    }
  }
  std::cout << " }";  
  return out;
}

int main() {
    print(" Demonstrate C++ Objects");

    showNote("stack based instances");

    /* standard type std::string */
    print("\n--- auto str = std::string(\"Wile E. Coyote\") ---");
    auto str = std::string("Wile E. Coyote");
    auto out = std::string("contents of str = ") + str;
    print(out);
    showType(str, "str", nl);

    /* custom type Point1 */
    Point1 p1;
    // int x = p1.xCoor();
    p1.show();
    p1.xCoor() = 42;
    p1.zCoor() = -3;
    p1.show();
    showType(p1, "p1", nl);
    std::cout << "  p1.xCoor() = " << p1.xCoor() << "\n";

    Point2<double> p2(5);
    p2.show();
    p2.coords() = std::vector<double>{1.0, -2.0, 3.0, 4.5, -42.0 };
    p2.show();
    showType(p2, "p2", nl);
    std::cout << "  p2.coords()[2] = " << p2.coords()[2] << "\n";
    
    showNote("heap-based instances");

    /* standard type std::string */
    /* uses alias pU for std::unique_ptr, defined above */
    print("\n--- pU<std::string> pStr(new std::string(\"Road Runner\") ---");
    pU<std::string> pStr(new std::string("Road Runner"));
    std::cout << "\n  pStr contents = " << *pStr;
    showType(*pStr, "*pStr");
    showType(move(pStr), "pStr");

    /* custom types */
    print("\n--- pU<Point1> pPoint1(new Point1()) ---");
    pU<Point1> pPoint1(new Point1());
    print("\n--- pPoint1->show() ---");
    pPoint1->show();
    pPoint1->xCoor() = 1;
    pPoint1->yCoor() = 2;
    pPoint1->zCoor() = -3;
    pPoint1->show();
    std::cout << "\n  pPoint1->zCoor() = " << pPoint1->zCoor() << "\n";
    showType(*pPoint1, "*pPoint1");
    showType(std::move(pPoint1), "pPoint1");
    /* pPoint1 moved, so invalid */

    print("\n--- pU<Point2> pPoint2(new Point2(4)) ---");
    pU<Point2<double>> pPoint2(new Point2<double>(4));
    print("\n--- pPoint2->show() ---");
    pPoint2->show();
    pPoint2->coords() = std::vector<double>{ 1.0, 3.5, -2.0, 42.0 };
    pPoint2->show();
    std::cout << "\n  pPoint2->coords()[1] = " << pPoint2->coords()[1] << "\n";
    showType(*pPoint2, "*pPoint2");
    showType(std::move(pPoint2), "pPoint2");
    /* pPoint2 moved, so invalid */

    print("\n\n  That's all Folks!\n\n");
}

template<typename T>
void showType(T t, const std::string &nm, const std::string& suffix) {
  std::cout << "\n  " << nm;                // show name at call site
  std::cout << " type: " << truncate(60,typeid(t).name());  // show type
  std::cout << "\n  value: " << t;          // show value
  std::cout << ",  size:  " << sizeof(t);   // show size on stack
  std::cout << suffix;
}
void showNote(const std::string& txt, const std::string& suffix) {
  print("-------------------------");
  print(txt);
  print("-------------------------");
  std::cout << suffix;
}
std::string truncate(size_t N, const char* pStr) {
  std::string temp(pStr);
  if(temp.length() > N) {
    temp.resize(N);
    return temp + "...";
  }
  return temp;
}
void print(const std::string& txt) {
  std::cout << "\n  " << txt;
}
void println(const std::string& txt) {
  std::cout << "\n  " << txt << "\n";
}
