/*-------------------------------------------------------------------
  Points.h defines a space-time point class:
  - Point4D represents points with three double spatial coordinates
    and std::time_t time coordinate.
*/
#pragma warning(disable:4996) // warning about ctime use - see below
#include <iostream>
#include <vector>
#include <chrono>
#include <ctime>
/*-------------------------------------------------------------------
  Point4D class represents a point in a 4-Dimensional space-time
  lattice. Simple enough for illustration, but still useful.

  It declares all of the special class methods, most with
  default qualifiers to indicate that the compiler will
  generate them as needed.

  The word ctor is an abbreviation of constructor and dtor
  an abbreviation for destructor.
*/
class Point4D {
public: 
  Point4D();                                        // default ctor
  Point4D(const Point4D& pt) = default;             // copy ctor
  Point4D(Point4D&& pt) = default;                  // move ctor
  Point4D& operator=(const Point4D& pt) = default;  // copy assignment
  Point4D& operator=(Point4D&& pt) = default;       // move assignment
  ~Point4D() = default;                             // dtor
  std::string timeToString();
  void updateTime();
  void show();
  double& xCoor() { return x; }
  double& yCoor() { return y; }
  double& zCoor() { return z; }
  std::time_t& tCoor() { return t; }
private:
  double x;
  double y;
  double z;
  std::time_t t;
};

Point4D::Point4D() {
  x = y = z = 0.0;
  t = std::time(0);
}
std::string Point4D::timeToString() {
  return ctime(&t);
  /* 
    ctime is depricated due to a thread-safety issue.
    That's not a problem here. Compiler warning
    recommends converting to ctime_s, which, unfortunately
    has a different and awkward interface.
  */
}
void Point4D::updateTime() {
  t = std::time(0);
}

void Point4D::show() {
  std::cout << "\n  " << "Point4D {";
  std::cout << "\n    " << x << ", " << y << ", " << z << ", ";
  std::cout << "\n    " << timeToString();
  std::cout << "  }";
}
/* required for showType(T t, const std::string& nm) */
std::ostream& operator<<(std::ostream& out, Point4D& t1) {
  out << "Point4D {";
  out << "    " << t1.xCoor() << ", " << t1.yCoor() << ", " 
               << t1.zCoor() << std::endl
               << "    " << t1.timeToString() << std::endl
               << "  }" << std::endl;
  return out;
}

