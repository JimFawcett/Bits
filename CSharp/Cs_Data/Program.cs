using System;
using System.Reflection;
using System.Reflection.Emit;
using System.Text;
using System.Collections;
using System.Collections.Generic;
using Analysis;

/* Cs_Data::Program.cs */
/*
  Static Data Types:
    value types => assignment copies value
    --------------------------------------
      sbyte, byte, short, int, long, ushort, uint, ulong
      float, double
      decimal, char, bool
      struct
    Reference types => assignment copies reference
    ----------------------------------------------
      object, string, class
*/
//#pragma warning disable CS8602  // possibly null reference warning
// https://stackoverflow.com/questions/8173239/c-getting-size-of-a-value-type-variable-at-runtime/8173293#8173293
// 
namespace CSharpData
{
  using Dict = Dictionary<string,int>;
  using KVPair = KeyValuePair<string,int>;

//  using AnalysisData
  /*-- 
    Svt is a value type used for type demo example.
    It consists of three public properties.
  --*/
  public struct Svt   // value type with value type members
  { 
    public Svt() {}
    public int I { get; set; } = 0;
    public double D { get; set; } = 1.0;
    public char C { get; set; }  = 'a';
    public readonly void PrintSelf(string name) {
      Console.Write("{0}  {{ I:{1}, D:{2}, C:{3} }}\n", name, I, D, C);
    }
  }
  /*-- 
    Crt is a reference type used for type demo example.
    It consists of three public properties.
  --*/
  public class Crt   // reference type with a reference type member
  { 
    public Crt() {}
    public int I { get; set; } = 0;
    public double D { get; set; } = 1.0;
    public string S { get; set; }  = "a string";
    public void PrintSelf(string name) {
      Console.Write("{0}  {{ I:{1}, D:{2}, S:{3} }}\n", name, I, D, S);
    }
  }
  /*-- type demonstration --*/
  
  class Program
  {
    /*-- demonstration begins here --*/
  
    const string nl = "\n";

    public struct S {
      public int I {get;}
      public double D {get;}
      public S(int i, double d) {
        this.I = i;
        this.D = d;
      }
      public string ToStringRep() {
        return String.Format("S {{ {0}, {1} }}", this.I, this.D);
      }
    }

    public class X {
      public X(int ia, double da) {
        i = ia;
        d = da;
      }
      public int i {get; set;} = 0;
      public double d {get; set;} = 0.0;
      public string ToStringRep() {
        return String.Format("X {{ {0}, {1} }}", this.i, this.d);
      }
    }

    public interface Foo {
      void Bar(int i);
    }
    public class Demo:Foo {
      public void Bar(int i) {
        Console.WriteLine("argument is {0}", i);
      }
    }
    static void doOp<T>(int i, T t)
    where T:Foo {
      t.Bar(i);
    }
    static void DemoValueTypes() {

      // Display.Print();
      Display.ShowNote("Value Types", "\n", 25);

      /*-- bool --*/
      Display.ShowOp("bool b = true;");
      bool b = true;
      Display.ShowType(b, "b", "\n");

      /*-- int --*/
      Display.ShowOp("int i = 42");
      int i = 42;
      Display.ShowType(i, "i", "\n");

      /*-- char --*/
      Display.ShowOp("char c = 'z'");
      char c = 'z';
      Display.ShowType(c, "c", "\n");

      /*-- double --*/
      Display.ShowOp("double d = 3.1415927;");
      double d = 3.1415927;
      Display.ShowType(d, "d", "\n");

      /*-- decimal --*/
      Display.ShowOp("decimal dc = 100_000_000.00m;");
      decimal dec = 100_000_000.00m;
      Display.ShowType(dec, "dec", "\n");
    }
    static string ToStringRepArray<T>(T[] arr) {
      StringBuilder sb = new StringBuilder();
      sb.Append("{ ");
      bool first = true;
      foreach(T item in arr) {
        if(item == null) {
          break;
        }
        if(first) {
          sb.Append(item.ToString());
          first = false;
        }
        else {
          sb.AppendFormat(", {0}", item);
        }
      }
      sb.Append(" }\n");
      return sb.ToString();
    }
    static string ToStringRepIEnumerable<T,U>(T enu) 
      where T:IEnumerable<U>
    {
      StringBuilder sb = new StringBuilder();
      sb.Append("{ ");
      bool first = true;
      foreach(U item in enu) {
        if(item == null) {
          break;
        }
        if(first) {
          sb.Append(item.ToString());
          first = false;
        }
        else {
          sb.AppendFormat(", {0}", item);
        }
      }
      sb.Append(" }\n");
      return sb.ToString();
    }
    // static string ToStringRepAssocCont<T,K,V>(T assoc) 
    //   where T:IEnumerable<K,V>
    // {
    //   StringBuilder sb = new StringBuilder();
    //   sb.Append("{ ");
    //   bool first = true;
    //   foreach(U item in enu) {
    //     if(item == null) {
    //       break;
    //     }
    //     if(first) {
    //       sb.Append(item.ToString());
    //       first = false;
    //     }
    //     else {
    //       sb.AppendFormat(", {0}", item);
    //     }
    //   }
    //   sb.Append(" }\n");
    //   return sb.ToString();
    // }
    static void DemoAggregateTypes() {
      
      Display.ShowNote("Aggregate Types", "\n", 25);

      /*-- array --*/
      Display.ShowOp("int[]array = { 1, 2, 3, 2, 1}");
      int[]?array = { 1, 2, 3, 2, 1 };
      int first = array[0];
      Display.ShowType(array, "array");
      Display.Print(ToStringRepIEnumerable<int[], int>(array));

      /*-- tuple --*/
      Display.ShowOp("(int, double, char)tup = (42, 3.14159, 'z');");
      (int, double, char)tup = (42, 3.14159, 'z');
      double second = tup.Item2;
      Display.ShowType(tup, "tup", "\n");

      /*-- struct --*/
      Display.ShowOp("S s = new S(42, 3.1415927);");
      S s = new S(42, 3.1415927);
      int sfirst = s.I;
      Display.ShowType(s, "s");
      Display.Print(s.ToStringRep() + "\n");
    }
    static void DemoRefTypes() {

      Display.ShowNote("Language Defined Reference Types", "\n", 40);
      
      Display.ShowOp("object o = new object();");
      object o = new object();
      Display.ShowType(o, "o", "\n");

      Display.ShowOp("string str = \"astring\"");
      string str = "a string";
      string str_alt = new("another string");
      Display.ShowType(str, "str", "\n");

      Display.ShowOp("dynamic dyn = 42;");
      dynamic dyn = 42;
      Display.ShowType(dyn, "dyn");
      dyn = 3.1415927;
      Display.ShowType(dyn, "dyn", "\n");
      
      Display.ShowOp("X x = new X(42, 3.1415927);");
      X x = new X(42, 3.1415927);
      int xFirst = x.i;
      Display.ShowType(x, "x");
      Console.WriteLine(x.ToStringRep() + "\n");
    }
    static void DemoLibTypes() {

      Display.ShowNote("Library Defined Reference Types", "\n", 40);

      Display.ShowOp("List<int> li = new List<int> { 1, 2, 3, 2, 1 };");
      List<int> li = new List<int> { 1, 2, 3, 2, 1 };
      int lfirst = li[0];
      Display.ShowType(li, "li");
      Display.Print(
        "List<int> " +
        ToStringRepIEnumerable<List<int>, int>(li)
      );

      /*---------------------------------------------------
        These declarations must immediately follow 
        namespace declaration
        - using Dict = Dictionary<string,int>;
        - using KVPair = KeyValuePair<string,int>;
      */
      Display.ShowOp("Dict dict = new Dict();");
      Dict dict = new Dict();
      dict.Add("three", 3);
      dict["zero"] = 0;
      dict["one"] = 1;
      dict["two"] = 22;
      dict["two"] = 2;    // overrites previous value
      int oneval = dict["one"];
      /*---------------------------------------------------
        Find first key and value
      */
      IDictionaryEnumerator enumr = dict.GetEnumerator();
      if(enumr.MoveNext()) {  // returns false at end
        string key = (string)enumr.Key;
        int? value = null;
        if(enumr.Value != null) {
          value = (int)enumr.Value;
        }
        // do something with key and value
      }
      /*-- alternate evaluation --*/
      List<string> keys = dict.Keys.ToList();
      if(keys.Count > 0) {
        string keyfirst = keys[0];
        int valfirst = dict[keyfirst];
        // do something with key and value
      }
      Display.ShowType(dict, "dict");
      string rep = ToStringRepIEnumerable<Dict,KVPair>(dict);
      Display.Print("dict: " + rep);
    }
    static unsafe string ToStringAddress<T>(T t) {
      T* ptr = &t;
      IntPtr addr = (IntPtr)ptr;
      string addrstr = string.Format("address of i: 0x" + addr.ToString("x"));
      return addrstr;
    }
    static unsafe void DemoCopy() {
      int i = 42;
      int* ptr = &i;
      IntPtr addr = (IntPtr)ptr;
      Console.WriteLine("address of i: 0x" + addr.ToString("x"));
      List<int> li = new List<int> { 1, 2, 3, 2, 1 };
      List<int>* ptrli = &li;
      IntPtr addrli = (IntPtr)ptrli;
      Console.WriteLine("address if li: 0x" + addrli.ToString("x"));
      int li0 = li[0];
      int* ptrli0= &li0;
      IntPtr addrli0 = (IntPtr)ptrli0;
      Console.WriteLine("address 0f li[0]: 0x" + addrli0.ToString("x"));
      // Console.WriteLine(addr.ToString("addr: {0:X}", addr));
      // Console.WriteLine(&i.ToString("x"));
    }
    static void Main(string[] args)
    {
      Display.Print();
      DemoValueTypes();
      DemoAggregateTypes();
      DemoRefTypes();
      DemoLibTypes();
      DemoCopy();
    }
  }
}

//       Display.Print(" Demonstrate C# types");
  
//       Display.ShowNote("int - value type");
      
//       Display.ShowOp("int t1 = 42");
//       int t1 = 42;
//       Display.ShowType(t1, "t1", nl);
      
//       Display.ShowOp("interate over val type members using reflection");
//       Display.Iterate(t1);
//       Display.Print();

//       Display.ShowOp("int t1a = t1 : copy of value type");
//       int t1a = t1;
//       Display.IsSameObj(t1a, "t1a", t1, "t1", nl);

//       // reference behavior - copy on write
//       Display.ShowNote("string - reference type");

//       Display.ShowOp("string t2 = \"a string\"");
//       string t2 = "a string";
//       Display.ShowType(t2, "t2");

//       Display.ShowNote("string has many methods - uncomment next line to see them", "", 60);
//       //Display.Iterate(t2);

//       Display.ShowOp("string t3 = t2 : copy handle of ref type");
//       string t3 = t2;
//       Display.ShowType(t3, "t3");
//       Display.IsSameObj(t3,"t3",t2,"t2");

//       Display.ShowOp("t3 += \" is here\" : copy on write");
//       t3 += " is here";
//       Display.ShowType(t2, "t2");
//       Display.ShowType(t3, "t3");
//       Display.IsSameObj(t3,"t3",t2,"t2");
//       Display.ShowNote("t2 not modified by change in t3 due to copy on write", nl, 55);
//       Console.WriteLine();

//       Display.ShowNote("Object - base reference type");

//       Display.ShowOp("Object obj1 - new Object()");
//       Object obj1 = new Object();
//       Display.ShowType(obj1, "obj");

//       Display.ShowOp("interate over ref type members using reflection");
//       Display.Iterate(obj1);
//       Display.Print();

//       Display.ShowOp("Object obj2 = obj1");
//       Object obj2 = obj1;
//       Display.IsSameObj(obj2, "obj2", obj1, "obj1", nl);

//       Display.ShowNote("Svt Struct value type");

//       Display.ShowOp("Svt t4 = new Svt()");
//       Svt t4 = new() { I=3, D=3.1415927, C='z' };
//       t4.PrintSelf("t4");
//       Display.ShowType(t4, "t4");
//       Display.ShowNote("value type: size of object in stackframe", nl, 45);

//       /* Iterate over val type members using reflection */
//       Display.ShowOp("Iterate over val type members using reflection");
//       Display.Iterate(t4);
//       Display.Print();

//       /* copy of value type */
//       Display.ShowOp("Svt t4a = t4 : copy of val type");
//       Svt t4a = t4;
//       t4a.PrintSelf("t4a");
//       t4.PrintSelf("t4");
//       Display.IsSameObj(t4a,"t4a",t4,"t4");
//       Display.ShowType(t4a, "t4a");

//       Display.ShowOp("t4a.C = 'q'");
//       t4a.C = 'q';
//       t4a.PrintSelf("t4a");
//       t4.PrintSelf("t4");
//       Display.IsSameObj(t4, "t4", t4a, "t4a", nl);

//       Display.ShowNote("Crt - ref type with ref member");

//       Display.ShowOp("Crt t5 = new Crt()");
//       Crt t5 = new() { S = "SomeString" };
//       // using default values for I and D
//       t5.PrintSelf("t5");
//       Display.ShowType(t5, "t5");
//       Display.ShowNote("ref type: size of handle to object in heap", nl, 45);

//       /* copy handle of reference type */
//       Display.ShowOp("Crt t5a = t5 : copy handle of ref type");
//       Crt t5a = t5;
//       t5a.PrintSelf("t5a");
//       Display.IsSameObj(t5a, "t5a", t5, "t5");
//       Display.IsSameObj(t5a.S, "t5a.S", t5.S, "t5.S");

//       /* literal strings are not immutable */
//       Display.ShowOp("t5a.S = \"new literal string\" : no copy on write");
//       t5a.S = "new string";
//       t5a.PrintSelf("t5a");
//       t5a.PrintSelf("t5");
//       Display.IsSameObj(t5a, "t5a", t5, "t5");
//       Display.IsSameObj(t5a.S, "t5a.S", t5.S, "t5.S");
//       Display.ShowNote("source t5 was altered!");

//       Console.WriteLine("\nThat's all Folks!\n");
//     }
//   }
// }
