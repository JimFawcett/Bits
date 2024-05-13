/*-------------------------------------------------------------------
  HelloTemplates.h defines HelloTemplates<T> class
  - HelloTemplates<T> is a simple demonstration of a template
    type that does'nt do anything very useful except to explain 
    syntax.
*/
/*-------------------------------------------------------------------
  simple user-defined type to demonstrate template syntax
*/
#include "AnalysisGen.h"
using namespace Analysis;

template<typename T>
class HelloTemplates {
public:
  HelloTemplates() = default;
  HelloTemplates(T& tin) : t(tin) {};
  HelloTemplates(const HelloTemplates<T>& t) = default;
  HelloTemplates<T>& operator=(const HelloTemplates<T>&t) = default;
  ~HelloTemplates() = default;
  T& value() { return t; }
  void show();
private:
  T t;
};

template<typename T>
void HelloTemplates<T>::show() {
  std::cout << "  HelloTemplates<T> {\n  ";
  std::cout << "  type T: " 
            << truncate(DisplayParams.trunc,typeid(t).name());  // show type
  std::cout << ", size: " << sizeof(t);  // show size on stack
  std::cout << ", value: " << t;
  std::cout << "\n  }\n";
}
/* template method specialization */
template<>
void HelloTemplates<std::vector<int>>::show() {
  std::cout << "  HelloTemplates<T> {\n  ";
  std::cout << "  type T: " 
            << truncate(DisplayParams.trunc,typeid(t).name());  // show type
  std::cout << ", size: " << sizeof(t) << "\n    value: [ ";
  for (auto item : t) {
    std::cout << item << " ";
  };
  std::cout << "]\n  }\n";
}
/* partial template specialization of HelloTemplates class */
template<template<typename> typename V, typename T>
class HelloTemplates<V<T>> {
  HelloTemplates<V<T>>& operator=(const HelloTemplates<V<T>>& v) = default;
  ~HelloTemplates() = default;
  V<T>& value() { return val; }
  void show() {
    std::cout << "  HelloTemplates<V<T>> {\n  ";
    std::cout << "  type V<T>: " 
              << truncate(DisplayParams.trunc,typeid(val).name());  // show type
    std::cout << ", size: " << sizeof(val) << "\n    value: [ ";
    for (auto item : val) {
      std::cout << item << " ";
    };
    std::cout << "]\n  }\n";
  }
private:
  V<T> val;
};
// template<template<typename> typename V, typename T>
// void HelloTemplates<V<T>>::show() {
//   std::cout << "  HelloTemplates<V<T>> {\n  ";
//   std::cout << "  type V<T>: " 
//             << truncate(DisplayParams.trunc,typeid(val).name());  // show type
//   std::cout << ", size: " << sizeof(val) << "\n    value: [ ";
//   for (auto item : val) {
//     std::cout << item << " ";
//   };
//   std::cout << "]\n  }\n";
// }

/*-- demonstrate creation and use of HelloTemplates<T> --*/
void demo_custom_type_HelloTemplates() {
  
  println();
  showNote("Demo user-defined HelloTemplates<T>", 40);

  showOp("HelloTemplates<T>", nl);
  int arg = 42;
  HelloTemplates<int> demi(arg);
  demi.show();
  std::cout << std::endl;;
  
  double pi = 3.1415927;
  HelloTemplates<double> demd(pi);
  demd.show();

  /*-------------------------------------------------------
    specialization defined in HelloTemplates<T> class header
    HelloTemplates.h and used here
  */
  auto vs = std::vector<int> { 1, 2, 3 };
  HelloTemplates<std::vector<int>> demv(vs);
  demv.show();

  // /*-------------------------------------------------------
  //   partial specialization defined in HelloTemplates<T> class header
  //   HelloTemplates.h and used here
  // */
  // auto svs = std::vector<double> { 1.0, 2.0, 3.0 };
  // HelloTemplates<std::vector<double>> sdemv(svs);
  // sdemv.show();
  //
  // compiler thinks that show is unspecialized
}

