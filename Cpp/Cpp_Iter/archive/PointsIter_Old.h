/*-------------------------------------------------------------------
  PointsIter.h defines point class PointN<T>
  - PointN<T> represents points with many coordinates of
    unspecified type T
  - Added iterator, begin(), end()
*/
#include <iostream>
#include <vector>
/*-------------------------------------------------------------------
  PointN<T> class represents a point in an n-Dimensional hyperspace.
  It uses a template parameter to support a variety of coordinate 
  types, and uses a vector to hold any finite number of 
  coordinates.

  Its default constructor PointN() is declared delete so it won't
  be provided here nor generated by the compiler.

  It provides iterators and begin() and end() members.
  These are used for iterating over PointN<T>'s elements.
*/
template<typename T>
class PointN {
public:
  using iterator = typename std::vector<T>::iterator;
  using const_iterator = typename std::vector<T>::const_iterator;
  using value_type = T;
  
  PointN() = delete;                              // default ctor
  PointN(size_t N);
  PointN(const PointN& pt) = default;             // copy ctor
  PointN(PointN&& pt) = default;                  // move ctor
  PointN& operator=(const PointN& pt) = default;  // copy assignment
  PointN& operator=(PointN&& pt) = default;       // move assignemnt
  ~PointN() = default;                            // dtor
  
  iterator begin();
  iterator end();
  const_iterator begin() const;
  const_iterator end() const;
  size_t size() const;
  T& operator[](size_t index);
  T operator[](size_t index) const;

  void push_back(T r);
  T pop_back();
  std::vector<T>& coords() { return coord; }

  void show(const std::string& name);             // display contents
  size_t& left() { return _left; };               // display indent
  size_t& width() { return _width; };             // display width
private:
  std::vector<T> coord;
  size_t _left = 2;   // default display indent
  size_t _width = 7;  // default display row width
};
/*-----------------------------------------------
  PointN<T> constructor with size 
*/
template<typename T>
PointN<T>::PointN(size_t N) {
  for(size_t i=0; i<N; i++) {
    coord.push_back(T{0});
  }
}
template<typename T>
size_t PointN<T>::size() const {
  return coord.size();
}
template<typename T>
T& PointN<T>::operator[](size_t index) {
  if (index < 0 || coord.size() <= index) {
    throw "indexing error";
  }
  return coord[index];
}
template<typename T>
T PointN<T>::operator[](size_t index) const {
  if (index < 0 || coord.len() <= index) {
    throw "indexing error";
  }
  return coord[index];
}
template<typename T>
typename PointN<T>::iterator PointN<T>::begin() {
  return coord.begin();
}
template<typename T>
typename PointN<T>::iterator PointN<T>::end() {
  return coord.end();
}
template<typename T>
typename PointN<T>::const_iterator PointN<T>::begin() const {
  return coord.begin();
}
template<typename T>
typename PointN<T>::const_iterator PointN<T>::end() const {
  return coord.end();
}
template<typename T>
void PointN<T>::push_back(T t) {
  coord.push_back(t);
}
template<typename T>
T PointN<T>::pop_back() {
  coord.pop_back(t);
}
/*-----------------------------------------------
  PointtN<T> display function 
*/
template<typename T>
void PointN<T>::show(const std::string& name) {
  std::cout << "\n" << indent(_left) << name << ": " << "PointN<T>";
  std::cout << " {\n";
  std::cout << fold(coord, _left + 2, _width);
  std::cout << indent(_left) << "}";
}
/*-----------------------------------------------
  Overload operator<< required for 
  showType(PointN<T> t, const std::string& nm) 
*/
template<typename T>
std::ostream& operator<<(std::ostream& out, PointN<T>& t2) {
  out << "\n" << indent(t2.left()) << "PointN<T>";
  out << " {\n";
  out << fold(t2.coords(), t2.left() + 2, t2.width());
  out << indent(t2.left()) << "}";
  return out;
}
