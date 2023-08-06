/*---------------------------------------------------------
  Bits::Cpp::src::Cpp_DataAnalysis.h
  - Display creation and simple use of basic types
*/
#include <iostream>  // std::cout
#include <typeinfo>  // typeid
// #include <memory>    // std::unique_ptr
// #include <utility>   // move()

/*-----------------------------------------------
  Note:
  Find all Bits code, including this in
  https://github.com/JimFawcett/Bits
  You can clone the repo from this link.
-----------------------------------------------*/
/*-----------------------------------------------
  truncate string to length n
  - does nothing if string length is less than n
*/
std::string truncate(const std::string& str, size_t n = 40);
/*-----------------------------------------------
  show type and value of scalar t
*/
template<typename T>
void showType(T t, const std::string &nm) {
  std::cout << "\n  " << nm;                // show name at call site
  std::cout << ": type: " << truncate(typeid(t).name());  // show type
  std::cout << "\n  value: " << t;          // show value
  std::cout << ",  size:  " << sizeof(t);   // show size on stack
  std::cout << "\n";
}
/*-----------------------------------------------
  show type and value of sequential container t
*/
template<typename T>
void showTypeEnum(T t, const std::string &nm) {
  std::cout << "\n  " << nm;                // show name at call site
  std::cout << ": type: " << truncate(typeid(t).name());  // show type
  std::cout << "\n  value: {\n    ";
  bool first = true;
  for(auto &item : t) {
    if(first) {
      std::cout << "  " << item;
      first = false;
    }
    else {
      std::cout << ", " << item;
    }
  }
  std::cout << "\n  }\n";
  std::cout << "  size:  " << sizeof(t);   // show size on stack
  std::cout << "\n";
}
/*-----------------------------------------------
  show type and value of associative container t
*/
template<typename T>
void showTypeAssoc(T t, const std::string &nm) {
  std::cout << "\n  " << nm;                // show name at call site
  std::cout << ": type: " << truncate(typeid(t).name());  // show type
  std::cout << "\n  value: {\n    ";
  bool first = true;
  for(auto &item : t) {
    if(first) {
      std::cout << "{" << item.first << ", " << item.second << "}";
      first = false;
    }
    else {
      std::cout << ", {" << item.first << ", " << item.second << "}";
    }
  }
  std::cout << "\n  }\n";
  std::cout << "  size:  " << sizeof(t);   // show size on stack
  std::cout << "\n";
}
/*-----------------------------------------------
  display text in several formats
*/
void showOp(const std::string& text);
void showLabel(const std::string& text, size_t n = 50);
void print(const std::string& txt = "");
void println(const std::string& txt = "");
/*-----------------------------------------------
  emit newline
*/
static void nl() {
  std::cout << std::endl;
}
