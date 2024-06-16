/*-------------------------------------------------------------------
  AnalysisIter.h
  - Provides functions that analyze types, display results
    and other program defined information.
  - Some of this code requires complex template operations.
    Those will be discussed in the generics bit.
  - You can skip the hard parts until then, without loss
    of understanding.
*/
#ifndef AnalysisHeader
#define AnalysisHeader

#include <typeinfo>     // typeid
#include <utility>      // move()
#include <sstream>      // stringstream
#include <type_traits>  // is_scalar, if constexpr
#include <iostream>     // cout
#include <vector>       // vector

namespace Analysis {
  /*-------------------------------------------------------------------
    Analysis function declarations are provided here so that
    definitions below may be placed in any order. That's
    needed because C++ requires declaration before use.
  */
  template<typename T>
  void showType(T t, const std::string &nm, const std::string& suffix = "");
  void showNote(const std::string& txt, const size_t width = 40, const std::string& suffix = "");
  void showOp(const std::string& opstr, const std::string& suffix = "");
  void print(const std::string& txt = "");
  void println(const std::string& txt = "");
  std::string truncate(size_t N, const char* pStr);
  std::string indent(size_t n);
  template<typename T>
  std::string fold(std::vector<T>& v, size_t left, size_t width);
  template<typename T>
  std::string formatColl(
    const T& t, const std::string& nm,
    const std::string& suffix, size_t left, size_t width
  );
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
  template<typename T>
  std::string format(
    const T& t, const std::string& nm, const std::string& suffix = "",
    size_t left = 2, size_t width = 7
  );

  /* end of function declarations 
  -------------------------------------------------------------------*/

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
  std::ostream& operator<<(std::ostream& out, std::vector<T>& v) {
    out << format(v, "vector<T>", "", DisplayParams.left, DisplayParams.width);
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
  inline void showNote(
    const std::string& txt, 
    const size_t width, 
    const std::string& suffix
  ) {
    auto fill = std::string(width, '-');
    print(fill);
    print("  " + txt);
    print(fill);
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
    Helper function for formatting output
    - folds lines after width elements
  */
  template<typename T>
  std::string fold(std::vector<T>& v, size_t left, size_t width) {
    std::stringstream out("\n");
    out << indent(left);
    for(size_t i=0; i<v.size(); ++i) {
      if((i % width) == 0 && i != 0 && i != width - 1) {
        out << "\n" << indent(left);
      }
      if(i < v.size() - 1) {
        out << v[i] << ", ";
      }
      else {
        out << v[i] << "\n";
        break;
      }
    }
    return out.str();
  }
  /*-----------------------------------------------
    Helper function for formatColl
    - defines out << std::pair<K,V>
    - used in formatColl for associative containers
  */
  template<typename K, typename V>
  std::stringstream& operator<<(
    std::stringstream& out, const std::pair<K,V>& p
  ) {
    out << "{" << p.first << ", " << p.second << "}";
    return out;
  }
  /*-----------------------------------------------
    Format output for Collection types
    - any type with begin() and end() like
      all the STL containers.
    - if nm is larger than empty str displays nm : { + ...
    - if nm is empty str displays { + ...
    - if c.size() <= width displays on one line
    - if c.size() > width displays on folded stack of lines
  */
  template<typename Coll>
  std::string formatColl(
    const Coll& c, const std::string& nm = "", const std::string& suffix = "",
    size_t left = 2, size_t width = 7
  ) {
    std::string nameStr;
    std::string prologue;
    std::string epilogue;
    if(nm.size() == 0) {
      nameStr = "{ ";
    }
    else {
      nameStr = nm + " : { ";
    }
    if(c.size() <= width) {
      prologue = indent(left) + nameStr;
      epilogue = " }";
    }
    else {
      prologue = indent(left) + nameStr + "\n" + indent(left + 2);
      epilogue = "\n" + indent(left) + "}\n";
    }
    std::stringstream out;
    out << "\n" + prologue;
    size_t i = 0;
    for(const Coll::value_type& elem : c) {
      if((i % width) == 0 && i != 0 && i != width - 1) {
        out << "\n" << indent(left + 2);
      }
      if(i < c.size() - 1) {
        out << elem << ", ";
      }
      else {
        out << elem << epilogue << suffix;
        break;
      }
      ++i;
    }
    return out.str();
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
    Defines is_iterable trait
    - detects STL containers and user-defined types
      that provide iteration
    - uses template metaprogramming, e.g., user code
      that runs at compile-time
  https://stackoverflow.com/questions/13830158/check-if-a-variable-type-is-iterable
  */
  template <typename T, typename = void>
  struct is_iterable : std::false_type {};

  // this gets used only when we can call 
  // std::begin() and std::end() on that type
  template <typename T>
  struct is_iterable<
    T, 
    std::void_t
      <decltype(std::begin(std::declval<T>())),
      decltype(std::end(std::declval<T>()))>
  > : std::true_type {};

  template <typename T>
  constexpr bool is_iterable_v = is_iterable<T>::value;

  /*-----------------------------------------------
    Displays almost everything.
    - strings work better with formatString(...)
    https://www.cppstories.com/2018/03/ifconstexpr/
    Iteration is discussed in Bit Cpp_iter
  */
  template<typename T>
  std::string format(
    const T& t, const std::string& nm, const std::string& suffix,
    size_t left, size_t width
  ) {
    if constexpr(is_iterable_v<T>) {  // decision at compile-time
      return formatColl(t, nm, suffix, left, width);
    }
    else {
      return formatScalar(t, nm, suffix, left);
    }
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
}
#endif