/*-------------------------------------------------------------------
  Stats.h defines Stats<T> class
  - Stats<T> holds a std::vector<T> and provides methods for
    computing max, min, average of this collection
    of unspecified type T
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
    Stats(const std::vector<T>& v);
    Stats() = delete;
    Stats<T>& operator=(Stats<T>& s) = delete;
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
  displays results of all class operations
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
