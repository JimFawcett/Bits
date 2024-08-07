/*-------------------------------------------------------------------
  PointsGen.h defines point classe Point<T, N>
  - Point<T, N> represents points with N coordinates of
    unspecified type T and a Time t.
*/
#ifndef PointsHeader
#define PointsHeader

#include <iostream>
#include <vector>
#include <string>
#include <initializer_list>
#include <concepts>
#include "AnalysisIter.h"
#include "Time.h"

namespace Points {

  //using namespace Analysis;
  
  /*-------------------------------------------------------------------
    Point<T, N> class represents a point in an N-Dimensional hyperspace.
    It uses a template parameter to support a variety of coordinate 
    types, and uses a vector to hold any finite number of 
    coordinates, specified by N.

    It also carries a Time t instance which conceptually is the time
    at which something was at that point in space. Time is a class
    defined for this demonstration in Time.h.

    All its special members, ctors, assignment, ... with the exception 
    of constructor Point(), are declared default to indicate to a maintainer 
    that compiler generated methods are correct and should not be provided.

    It does not provide an iterator nor begin() and end() members.
    Those will added in the iteration bit.
  */
  template<typename T, const size_t N>
  class Point {
  public:
    using iterator = typename std::vector<T>::iterator;
    using const_iterator = typename std::vector<T>::const_iterator;
    using value_type = T;
    
    Point();                                      // default ctor
    Point(std::initializer_list<T> il);           // construct from list
    Point(const Point& pt) = default;             // copy ctor
    Point(Point&& pt) = default;                  // move ctor
    Point& operator=(const Point& pt) = default;  // copy assignment
    Point& operator=(Point&& pt) = default;       // move assignemnt
    ~Point() = default;                           // dtor

    void init(const std::vector<T>& v);

    std::string timeToString();
    void updateTime();
    Time& time();
    const size_t size() const;

    iterator begin();
    iterator end();
    const_iterator begin() const;
    const_iterator end() const;

    T& operator[](size_t index);                  // index oper
    const T operator[](size_t index) const;       // const index oper
    
    std::vector<T>& coords() { return coord; }    // accessor

    void show(const std::string& name);           // display contents
    size_t& left() { return _left; }              // display indent
    size_t& width() { return _width; }            // display width
  private:
    std::vector<T> coord;
    Time tm;
    size_t _left = 2;   // default display indent
    size_t _width = 7;  // default display row width
  };
  /*-----------------------------------------------
    Point<T, N> constructor with size Template
    parameter
  */
  template<typename T, size_t N>
  Point<T, N>::Point() 
    : tm(Time()) {
    for(size_t i=0; i<N; i++) {
      coord.push_back(T{0});
    }
  }
  /*-----------------------------------------------
    Fill coor with elements from initializer list li
    - if li is smaller than N then fill remainder with 
      default values of T
    - if li is larger use first N elements of li
  */
  template<typename T, size_t N>
  Point<T, N>::Point(std::initializer_list<T> il) 
    : tm(Time()) {
    size_t sz = std::min(N, il.size());
    size_t i = 0;
    for(auto item : il) {
      coord.push_back(item);
      if(++i == sz) {
        break;
      }
    }
    for(size_t i = il.size(); i<N; i++) {
      coord.push_back(T{0});
    }
  }
  /*---------------------------------------------
    Always returns N
  */
  template<typename T, size_t N>
  const size_t Point<T, N>::size() const {
    return coord.size();
  }
  /*---------------------------------------------
    index returns mutable value
  */
  template<typename T, size_t N>
  T& Point<T, N>::operator[](size_t index) {
    if (index < 0 || coord.size() <= index) {
      throw "Point<T, N> indexing error";
    }
    return coord[index];
  }
  /*---------------------------------------------
    index returns immutable value
  */
  template<typename T, size_t N>
  const T Point<T, N>::operator[](size_t index) const {
    if (index < 0 || coord.len() <= index) {
      throw "Point<T, N> indexing error";
    }
    return coord[index];
  }

  template<typename T, const size_t N>
  typename Point<T, N>::iterator Point<T, N>::begin() {
    return coord.begin();
  }
  template<typename T, const size_t N>
  typename Point<T, N>::iterator Point<T, N>::end() {
    return coord.end();
  }
  template<typename T, const size_t N>
  typename Point<T, N>::const_iterator Point<T, N>::begin() const {
    return coord.begin();
  }
  template<typename T, const size_t N>
  typename Point<T, N>::const_iterator Point<T, N>::end() const {
    return coord.end();
  }

  /*-----------------------------------------------
    Fill coor with elements from vector v
    - if v is smaller fill remainder with default
      values of T
    - if v is larger use first N elements of v
  */
  template<typename T, size_t N>
  void Point<T, N>::init(const std::vector<T>& v) {
    size_t sz = std::min(N, v.size());
    for(size_t i=0; i<sz; i++) {
      coord[i] = v[i];
    }
    for(size_t i = v.size(); i<N; i++) {
      coord[i] = T{0};
    }
  }
  /*---------------------------------------------
    returns string datetime
  */
  template<typename T, size_t N>
  std::string Point<T, N>::timeToString() {
    std::string ts = tm.toString();
    return ts;
  }
  /*---------------------------------------------
    set time to current time
  */
  template<typename T, size_t N>
  void Point<T, N>::updateTime() {
    tm = std::time(0);
  }
  /*---------------------------------------------
    returns current number of seconds in clock's 
    epoch
  */
  template<typename T, size_t N>
  Time& Point<T, N>::time() {
    return tm;
  }
  /*-----------------------------------------------
    PointtN<T> display function 
  */
  template<typename T, size_t N>
  void Point<T, N>::show(const std::string& name) {
    std::cout << "\n" << indent(_left) << name << ": " << "Point<T, N>";
    std::cout << " {\n";
    std::cout << fold(coord, _left + 2, _width);
    std::cout << indent(_left) << "}";
    std::cout << "\n" << indent(_left) << tm.toString() << std::endl;
  }
  /*-----------------------------------------------
    Overload operator<< required for 
    showType(Point<T, N> t, const std::string& nm) 
  */
  template<typename T, size_t N>
  std::ostream& operator<<(std::ostream& out, Point<T, N>& t2) {
    out << "\n" << indent(t2.left()) << "Point<T, N>";
    out << " {\n";
    out << fold(t2.coords(), t2.left() + 2, t2.width());
    out << indent(t2.left()) << "}";
    return out;
  }
}
/*-- demonstrate iteration over Point type --*/

void demo_custom_type_Point_iteration() {
  using namespace Analysis;
  using namespace Points;

  showNote("iteration over user-defined Point<T, N>", 45, "\n");

  /*-- demonstrate Point<double 3> initialization lists --*/
  showOp("Point<double, 3> p1 {1.0, 1.5, 2.0, 1.5, 1.0 }");  // equal to N
  Point<double, 5> p1 {1.0, 1.5, 2.0, 1.5, 1.0};
  p1.left() = 0;
  p1.show("p1");
  std::cout << "\np1[1] = " << p1[1];           // indexing
  std::cout << "\np1.time().day() = " 
            << p1.time().day();
  std::cout << "\np1.time().seconds() = " 
            << p1.time().seconds() << "\n";

  showOp("iteration using basic loop", "\n");
  auto itr = p1.begin();
  while(true) {
    std::cout << *itr++ << " ";
    if(itr == p1.end()) {
      break;
    }
  }
  std::cout << std::endl;

  showOp("iteration using range-for", "\n");
  for(auto item : p1) {
    std::cout << item << " ";
  }
  std::cout << std::endl;

  showOp("Point<int, 10> p2 { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 }");
  Point<int, 10> p2 { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 };
  p2.left() = 0;
  p2.show("p2");
}
#endif