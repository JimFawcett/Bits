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
  Note:
  Find all Bits code, including this in
  https://github.com/JimFawcett/Bits
  You can clone the repo from this link.
-----------------------------------------------*/

static const size_t WIDTH = 8;
/*-----------------------------------------------
  truncate string to length n
  - does nothing if string length is less than n
*/
std::string truncate(const std::string& str, size_t n = 40);
/*-----------------------------------------------
  show type of t
*/
template<typename T>
void showType(T t, const std::string &nm) {
  std::cout << "  " << std::setw(WIDTH) << std::left << nm + ": ";      // show name at call site
  std::cout << "type: " << truncate(typeid(t).name());  // show type
  std::cout << "\n";
}
/*-----------------------------------------------
  return type of t
*/
template<typename T>
std::string getType(T t, const std::string &nm) {
  std::ostringstream out;
  out << "  " << std::setw(WIDTH) << std::left << nm + ": ";      // show name at call site
  out << "type: " << truncate(typeid(t).name());  // show type
  out << "\n";
  return out.str();
}
/*-----------------------------------------------
  show type and value of t
*/
template<typename T>
void showTypeScalar(T t, const std::string &nm) {
  std::cout << "  " << nm;                // show name at call site
  auto truncType = truncate(typeid(t).name());
  if(truncType.length() < 20) {
    std::cout << ": type: " << truncType;  // show type
    std::cout << ", value = " << t << "\n";
  }
  else {
    std::cout << ": type: " << truncType << "\n";  // show type
    std::cout << "      value = \"" << t << "\"\n";
  }
}
/*-----------------------------------------------
  show value of t
*/
template<typename T>
void showScalarValue(const T& t, const std::string &nm) {
  std::cout << "  " << nm << ": value = " << t << "\n";
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
  show value of sequential container t
*/
template<typename T>
void showEnum(T t, const std::string &nm) {
  std::cout << "\n  " << nm;                // show name at call site
  std::cout << ": {\n  ";
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
}
/*-----------------------------------------------
  show value of associative container t
*/
template<typename T>
void showAssoc(T t, const std::string &nm) {
  std::cout << "\n  " << nm << " {\n    ";                // show name at call site
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
}
/*-----------------------------------------------
  display text in several formats
*/
void showOp(const std::string& text, const std::string& suffix = "");
void showLabel(const std::string& text, size_t n = 50);
void print(const std::string& txt = "");
void println(const std::string& txt = "");
/*-----------------------------------------------
  emit newline
*/
static void nl() {
  std::cout << std::endl;
}
