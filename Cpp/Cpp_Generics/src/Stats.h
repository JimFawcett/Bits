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
#include <concepts>
#include "AnalysisGen.h"
using namespace Analysis;

/*-------------------------------------------------------------------
  Stats<T> class provides several simple computational services on
  a vector of items who's type provides required arithmetic operations.
  - This class inhibits compiler generation of default constructor
    and assignment operator.
*/

template <typename T>
  concept Number = std::integral<T> || std::floating_point<T>;

template <typename T>
requires Number<T>
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
requires Number<T>
Stats<T>::Stats(const std::vector<T>& v) : items(v) {}

/*-------------------------------------------------------------------
  check that Stats instance contains at least one value
*/
template<typename T>
requires Number<T>
bool Stats<T>::check() {
    return items.size() > 0;
}
/*-------------------------------------------------------------------
  returns number of data items
*/
template<typename T>
requires Number<T>
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
requires Number<T>
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
requires Number<T>
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
requires Number<T>
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
requires Number<T>
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
requires Number<T>
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
/*-- demonstrate custom type Stats<T> --*/
void demo_custom_type_Stats() {

  println();
  showNote("Demo user-defined Stats<T>", 35);

  showOp("Stats<double> s(v)", nl);
  std::vector<double> v { 1.0, 2.5, -3.0, 4.5 };
  showSeqColl(v);
  Stats<double> s(v);
  std::cout << "  min: " << s.min();
  std::cout << ", max: " << s.max();
  std::cout << ", sum: " << s.sum();
  std::cout << ", avg: " << s.avg() << std::endl;

  showOp("Stats<double> s2 = s", nl);
  Stats<double> s2 = s;  // copy construction
  std::cout << "  min: " << s2.min();
  std::cout << ", max: " << s2.max();
  std::cout << ", sum: " << s2.sum();
  std::cout << ", avg: " << s2.avg() << std::endl;

  // showOp("Stats<std::string> ss", nl);
  // std::vector<std::string> vstr { "ab", "cd", "ef" };
  // Stats<std::string> ss(vstr);
  // std::cout << "  min: " << ss.min();
  // std::cout << ", max: " << ss.max();
  // std::cout << ", sum: " << ss.sum();
  //--------------------------------------------------
  // first compile phase:
  //   Stats<T>::avg() passess
  // second compile phase:
  //   Stats<std::string>::avg() fails to compile.
  //   No way to divide sum string by size integer in
  //   std::cout << ", avg: " << ss.avg() << std::endl; 
  //   All the other methods compile successfully. 
  println();
}
