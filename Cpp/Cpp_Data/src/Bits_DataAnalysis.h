/*---------------------------------------------------------
  Bits::Cpp::Cpp_Data::src::Bits_DataAnalysis.h
  - Display creation and simple use of basic types
*/
#include <iostream>  // std::cout
#include <iomanip>
#include <sstream>
#include <typeinfo>  // typeid
#include <concepts>

/*-----------------------------------------------
  Notes:
  Find all Bits code, including this in
  https://github.com/JimFawcett/Bits
  You can clone the repo from this link.

  I've made non-template functions inline so
  the header contains all implementation code
  and users need only consume this one file. 
-----------------------------------------------*/

/*-----------------------------------------------
  truncate string to length n
  - does nothing if string length is less than n
*/
inline std::string truncate(const std::string& str, size_t n = 40) {
  std::string tmp = str;
  if(tmp.size() < n) {
    return tmp;
  }
  tmp.resize(n);
  tmp += "...";
  return tmp;
}
/*-----------------------------------------------
  return type of t
*/
template<typename T>
std::string getType(T t, const std::string &nm) {
  std::ostringstream out;
  out << "  " << nm + ": ";      // show name at call site
  out << "type: " << truncate(typeid(t).name());  // show type
  out << "\n";
  return out.str();
}
/*-----------------------------------------------
  display text in several formats
*/
inline void showOp(const std::string& text, const std::string& suffix = "") {
  std::cout << "  --- " << text << " ---" << std::endl << suffix;
}
inline void showLabel(const std::string& text, size_t n = 50) {
  auto line = std::string(n, '-');
  std::cout << line << std::endl;
  std::cout << "  " << text << std::endl;
  std::cout << line << std::endl;
}
inline void print(const std::string& txt) {
  std::cout << "\n  " << txt;
}
inline void println(const std::string& txt) {
  std::cout << "\n  " << txt << "\n";
}
/*-----------------------------------------------
  emit newline
*/
static void nl() {
  std::cout << std::endl;
}
