/*---------------------------------------------------------
  Bits::Cpp::src::Cpp_DataAnalysis.cpp
  - Functions to analyze and display data
*/
#include <iostream>  // std::cout
#include <typeinfo>  // typeid
#include "Bits_DataAnalysis.h"
//#include <memory>    // std::unique_ptr
//#include <utility>   // move()

/*-----------------------------------------------
  Note:
  Find all Bits code, including this in
  https://github.com/JimFawcett/Bits
  You can clone the repo from this link.
-----------------------------------------------*/
/*---------------------------------------------------------
  Truncates input string to n chars. Beware: side-effects.
  - used in showType which may generate long type names.
*/
std::string truncate(const std::string& str, size_t n) {
  std::string tmp = str;
  if(tmp.size() < n) {
    return tmp;
  }
  tmp.resize(n);
  tmp += "...";
  return tmp;
}
/*---------------------------------------------------------
  Show call name, static type, value, and size
*/
// template<typename T>
// void showType(T t, const std::string &nm) {
//   std::cout << "\n  " << nm;                // show name at call site
//   std::cout << ": type: " << truncate(typeid(t).name());  // show type
//   std::cout << "\n  value: " << t;          // show value
//   std::cout << ",  size:  " << sizeof(t);   // show size on stack
//   std::cout << "\n";
// }
/*---------------------------------------------------------
  Show operation text surrounded by "---" strings
*/
void showOp(const std::string& text) {
  std::cout << "  --- " << text << " ---" << std::endl;
}
/*---------------------------------------------------------
  Show text surrounded by long lines of '-' characters
*/
void showLabel(const std::string& text, size_t n) {
  auto line = std::string(n, '-');
  std::cout << line << std::endl;
  std::cout << "  " << text << std::endl;
  std::cout << line << std::endl;
}
void print(const std::string& txt) {
  std::cout << "\n  " << txt;
}
void println(const std::string& txt) {
  std::cout << "\n  " << txt << "\n";
}
