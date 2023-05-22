/*-------------------------------------------------------------------
  AnalysisObj.h
  - Provides functions that analyze types, display results
    and other program defined information.
  - Some of this code requires complex template operations.
    Those will be discussed in the generics bit.
  - You can skip the hard parts until then, without loss
    of understanding.
*/

#include <typeinfo>     // typeid
#include <utility>      // move()
#include <sstream>      // stringstream
#include <type_traits>  // is_scalar, if constexpr
#include <iostream>     // cout
#include <vector>       // vector

/*-------------------------------------------------------------------
  Analysis function declarations are provided here so that
  definitions below may be placed in any order. That's
  needed because C++ requires declaration before use.
*/
template<typename T>
void showType(T t, const std::string &nm, const std::string& suffix = "");
void showNote(const std::string& txt, const std::string& suffix = "");
void showOp(const std::string& opstr, const std::string& suffix = "");
void print(const std::string& txt = "");
void println(const std::string& txt = "");
std::string truncate(size_t N, const char* pStr);
std::string indent(size_t n);
template<typename T>
std::string formatScalar(
  const T& t, const std::string& nm, 
  const std::string& suffix = "", size_t left = 2
);
template<typename T>
std::string formatString(
  const T& t, const std::string& nm, const std::string& suffix,
  size_t left = 2
);
/*-------------------------------------------------------------------
  Display and Analysis function and globals definitions
---------------------------------------------------------------------
*/
const std::string nl = "\n";
/*-------------------------------------------------------------------
  Mutable globals are a common source of bugs.  We try not
  to use them, but will use DisplayParams here to control how
  the insertion operator sends instances to standard output.
*/
struct displayParams {
  size_t left = 2;    // number of spaces to indent
  size_t width = 7;   // width of display row
  size_t trunc = 40;  // replace text after trunc with ...
} DisplayParams;      // global object

/*-----------------------------------------------
  Overload operator<< required for 
  showType(std::vector<T> v, const std::vector<T>& nm) 
*/
template<typename T>
std::ostream& operator<< (std::ostream& out, const std::vector<T>& vec)
{
    out << "[";
    for (auto it = vec.begin(); it != vec.end(); ++it)
    {
        out << *it;
        if (it != vec.end() - 1)
            out << ", ";
    }
    out << "]";
    return out;
}
/*-----------------------------------------------
  Display calling name, static class, and size
*/
template<typename T>
void showType(T t, const std::string &callname, const std::string& suffix) {
  std::cout << "\n  " << callname;          // show name at call site
  std::cout << " type: " 
            << truncate(DisplayParams.trunc,typeid(t).name());  // show type
  std::cout << "\n  size:  " << sizeof(t);  // show size on stack
  std::cout << suffix;
}
/*-----------------------------------------------
  Display emphasized text
*/
inline void showNote(const std::string& txt, const std::string& suffix) {
  print("--------------------------------------------------");
  print("  " + txt);
  print("--------------------------------------------------");
  std::cout << suffix;
}
/*-----------------------------------------------
  Display emphasized line
*/
inline void showOp(const std::string& opstr, const std::string& suffix) {
  std::cout << "\n  --- " << opstr << " ---" << suffix;
}
/*-----------------------------------------------
  Helper function for formatting output
  - truncates line to N chars and adds ellipsis
*/
inline std::string truncate(size_t N, const char* pStr) {
  std::string temp(pStr);
  if(temp.length() > N) {
    temp.resize(N);
    return temp + "...";
  }
  return temp;
}
/*-----------------------------------------------
  Helper function for formatting output
  - generates string of n blanks to offset text
*/
inline std::string indent(size_t n) {
  return std::string(n, ' ');
}
/*-----------------------------------------------
  Format output for scalar types like primitives
*/
template<typename T>
std::string formatScalar(
  const T& t, const std::string& nm, const std::string& suffix,
  size_t left
) {
  std::stringstream out;
  out << "\n" << indent(left) << nm << ": " << t << suffix;
  return out.str();
}
/*-----------------------------------------------
  Format output for strings
  - indent and embed in quotation marks
*/
template<typename T>
std::string formatString(
  const T& t, const std::string& nm, const std::string& suffix,
  size_t left
) {
  std::stringstream out;
  out << "\n" << indent(left) << nm << ": \"" << t << "\"" << suffix;
  return out.str();
}
/*-----------------------------------------------
  Display text after newline and indentation
*/
inline void print(const std::string& txt) {
  std::cout << "\n  " << txt;
}
/*-----------------------------------------------
  Display text after newline and indentation
  - provides trailing newline
*/
inline void println(const std::string& txt) {
  std::cout << "\n  " << txt << "\n";
}
