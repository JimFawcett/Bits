/*-------------------------------------------------------------------
  Demos.h defines Demo<T> and Stats<T> classes
  - Demo<T> is a simple demonstration of a generic type that
    does'nt do anything very useful except to explain syntax.
  - Stats<T> holds a std::vector<T> and provides methods for
    computing max, min, average of this collection of elements
    of unspecified type T
*/
/*-------------------------------------------------------------------
  simple user-defined type to demonstrate template syntax
*/
using namespace Analysis;

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
  std::cout << "  type T: " 
            << truncate(DisplayParams.trunc,typeid(t).name());  // show type
  std::cout << ", size: " << sizeof(t);  // show size on stack
  std::cout << ", value: " << t;
  std::cout << "\n  }\n";
}
/* template method specialization */
template<>
void inline Demo<std::vector<int>>::show() {
  std::cout << "  Demo<T> {\n  ";
  std::cout << "  type T: " 
            << truncate(DisplayParams.trunc,typeid(t).name());  // show type
  std::cout << ", size: " << sizeof(t) << "\n    value: [ ";
  for (auto item : t) {
    std::cout << item << " ";
  };
  std::cout << "]\n  }\n";
}
/*-------------------------------------------------------------------
  Stats<T>
  - Stats<T> holds a std::vector<T> and provides methods for
    computing max, min, average of this collection
    of unspecified type T
  - Code builds as a template definition
  - Will fail to build instantiation if T is not a numeric type
*/
#include <iostream>
#include <vector>
#include <exception>
/*-------------------------------------------------------------------
  Stats<T> class provides several simple computational services on
  a vector of items who's type provides required arithmetic operations.
  - This class inhibits compiler generation of default constructor
    and assignment operator.
*/

template <typename T>
class Stats {
public:
    Stats() = default;
    Stats(const std::vector<T>& v);
    Stats(const Stats<T>& s) = default;
    Stats<T>& operator=(Stats<T>& s) = default;
    size_t size();
    T max();
    T min();
    T sum();
    double avg();
    void show(const std::string& name="");
private:
    bool check();
    const std::vector<T>& items;
};
/*-------------------------------------------------------------------
  Constructor initialized with vector of values
*/
template<typename T>
Stats<T>::Stats(const std::vector<T>& v) : items(v) {}

/*-------------------------------------------------------------------
  check that Stats instance contains at least one value
*/
template<typename T>
bool Stats<T>::check() {
    return items.size() > 0;
}
/*-------------------------------------------------------------------
  returns number of data items
*/
template<typename T>
size_t Stats<T>::size() {
    if(!check()) {
        throw "Stats is empty";
    }
    return items.size();
}
/*-------------------------------------------------------------------
  returns largest value (not necessarily largerst magnitude)
*/
template<typename T>
T Stats<T>::max() {
    if(!check()) {
        throw "Stats is empty";
    }
    auto max = items[0];
    for( auto item : items) {
        if (item > max) {
            max = item;
        }
    }
    return max;
}
/*-------------------------------------------------------------------
  returns smallest value (not necessarily smallest magnitude)
*/
template<typename T>
T Stats<T>::min() {
    if(!check()) {
        throw "Stats is empty";
    }
    auto min = items[0];
    for( auto item : items) {
        if (item < min) {
            min = item;
        }
    }
    return min;
}
/*-------------------------------------------------------------------
  returns sum of data values
*/
template<typename T>
T Stats<T>::sum() {
    if(!check()) {
        throw "Stats is empty";
    }
    auto sum = T{0};
    for( auto item : items) {
        sum += item;
    }
    return sum;
}
/*-------------------------------------------------------------------
  returns average of data values
*/
template<typename T>
double Stats<T>::avg() {
    if(!check()) {
        throw "Stats is empty";
    }
    auto sum = T{0};
    for( auto item : items) {
        sum += item;
    }
    return double(sum)/double(items.size());
}
/*-------------------------------------------------------------------
  displays current contents
*/
template<typename T>
void Stats<T>::show(const std::string& name) {
    if(!check()) {
        throw "Stats is empty";
    }
    std::cout << "\n  " << name << " {\n    ";
    auto iter = items.begin();
    std::cout << *iter++;
    while(iter != items.end()) {
        std::cout << ", " << *iter++;
        std::cout.flush();
    }
    std::cout << "\n  }\n";
}
