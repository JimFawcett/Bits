// Cpp_Objects.cpp

#include <iostream>  // std::cout
#include <typeinfo>  // typeid
#include <memory>    // std::unique_ptr
#include <utility>   // move()

/*
  Static Data Types:
    byte, bool, int, char, char16_t, char32_t, wchar_t
    float, double
    array, pointer, reference
    struct, class, enum class, using
  Qualifiers:
    short, long, const, unsigned

  Operations:
    Primitive types can all be copied.
    Most library and user-defined types can be copied, moved, and deleted
    by providing member constructors and destructor.  Often compiler 
    generation works well, but for classes with pointer members developers 
    must provide them.

  Processing:
    All types are static, operations run as native code, and no garbage
    collection is needed. Resources are returned at end of declr scope.
*/
/* C++ requires declaration before use */
template<typename T>
void showType(T t, const std::string &nm);
void showNote(const std::string& txt);
void print(const std::string& txt = "");
void println(const std::string& txt = "");

const char nl = '\n';

class Test1 {
public:
  Test1() = default;   // compiler generated default ctor
  Test1(const std::string& name);
  std::string& name(); 
private:
  std::string _name;
};
/*-- promotion constructor --*/
Test1::Test1(const std::string& name) : _name(name) { }
/*-- get and set name --*/
std::string& Test1::name() {
  return _name;
}
/*-- make class instance printable --*/
std::ostream& operator<<(std::ostream& out, Test1& t1) {
  out << t1.name();
  return out;
}

int main() {
    print(" Demonstrate C++ Objects");

    showNote("stack based instances");

    /* standard type std::string */
    print("--- auto str = std::string(\"Wiley\") ---");
    auto str = std::string("Wiley");
    auto out = std::string("contents of str = ") + str;
    print(out);
    showType(str, "str");

    /* custom type Test1 */
    print("--- auto test1 = Test1(\"tom\") ---");
    auto test1 = Test1("Tom");
    std::cout << "\n  Test1 name = " << test1.name();
    showType(test1, "test1");
    print("-- test1.name() = \"Jerry\" ---");
    test1.name() = std::string("Jerry");
    std::cout << "\n  Test1 name = " << test1.name();
    showType(test1, "test1");

    showNote("heap-based instances");

    /* standard type std::string */
    print("--- std::unique_ptr<std::string> pStr(new std::string(\"Bugs\") ---");
    std::unique_ptr<std::string> pStr(new std::string("Bugs"));
    std::cout << "\n  pStr contents = " << *pStr;
    showType(*pStr, "*pStr");

    /* custom type Test1 */
    print("--- std::unique_ptr<Test1> pTest1(new Test1(\"Elmer\") ---");
    std::unique_ptr<Test1> pTest1(new Test1("Elmer"));
    std::cout << "\n  Test1 name = " << *pTest1;
    showType(*pTest1, "*pTest1");
    showType(move(pTest1), "move(pTest1)");

    // /*-- values live in stack frame --*/
    // println("-- initialize from literals --");
    // int t1 = int{3}; 
    // showType(t1, "t1");
    // long long int t1a = 3;
    // showType(t1a, "t1a");

    // println("-- copy construct --");
    // auto t1b = t1a;
    // showType(t1b, "t1b");
    // // double t1 = 3.1415927; // not allowed to redefine type of t1
    
    // /*-- values live in heap when using new --*/
    // println("-- store in heap --");
    // double* t2 = new double{3.14159};
    // showType(t2, "t2");
    // showType(*t2, "*t2");
    // delete(t2);  // see unique_ptr, below
    
    // /*-- control block lives in stack, char data live in heap --*/
    // println("-- string: control block in stack, data in heap --");
    // auto t3 = std::string("Hello Data");  // move ctor - rhs is temp
    // showType(t3, "t3");
    // println("-- C++ reference --");
    // auto& t4 = t3;      // create reference, no copy or move
    // showType(t4, "t4");
    
    // /*-- unique_ptr may not be copied but move allowed --*/
    // println("-- unique_ptr owns data --");
    // auto t5 = std::unique_ptr<int>(new int{-3});
    // showType(std::move(t5), "t5");  // unique_ptr can't be copied so move
    // t5 = std::unique_ptr<int>(new int{-3});
    // showType(*t5, "*t5"); // contents can be copied
    // println("-- t5 deleted when unique_ptr leaves scope");
    
    print("\n\n  That's all Folks!\n\n");
}

template<typename T>
void showType(T t, const std::string &nm) {
  std::cout << "\n  " << nm;                // show name at call site
  std::cout << ": type: " << typeid(t).name();  // show type
  std::cout << "\n  value: " << t;          // show value
  std::cout << ",  size:  " << sizeof(t);   // show size on stack
  std::cout << "\n";
}
void showNote(const std::string& txt) {
  print("-------------------------");
  print(txt);
  print("-------------------------");
}

void print(const std::string& txt) {
  std::cout << "\n  " << txt;
}
void println(const std::string& txt) {
  std::cout << "\n  " << txt << "\n";
}
