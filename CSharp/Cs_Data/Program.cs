/*---------------------------------------------------------
  Cs_Data::Program.cs
  - demonstrate C# type system
---------------------------------------------------------*/
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
  // using System;
  // using System.Runtime.InteropServices;
  using Dict = Dictionary<string,int>;
  using KVPair = KeyValuePair<string,int>;

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
  
    /*-- emit count newlines to stdout --*/
    static void nl(int count = 1) {
      for(int i=0; i<count; ++i) {
        Console.Write("\n");
      }
    }

    /*-----------------------------------------------------
      User-defined value type used in Aggregate Types 
      demo.
    -----------------------------------------------------*/
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
    /*-----------------------------------------------------
      User-defined reference type used in Language Defined
      Reference Types demo.
    -----------------------------------------------------*/
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
    /*-----------------------------------------------------
      Not used in this demo
    -----------------------------------------------------*/
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
    /*-- Demonstrate Value Types Initialization --*/
    static void DemoValueTypes() {

      // Display.Print();
      Display.ShowNote("Value Types", "\n", 25);

      /*-- bool --*/
      Display.ShowOp("bool b = true;");
      bool b = true;
      Anal.ShowType(b, "b", "\n");

      /*-- int --*/
      Display.ShowOp("int i = 42");
      int i = 42;
      Anal.ShowType(i, "i", "\n");

      /*-- char --*/
      Display.ShowOp("char c = 'z'");
      char c = 'z';
      Anal.ShowType(c, "c", "\n");

      /*-- double --*/
      Display.ShowOp("double d = 3.1415927;");
      double d = 3.1415927;
      Anal.ShowType(d, "d", "\n");

      /*-- decimal --*/
      Display.ShowOp("decimal dc = 100_000_000.00m;");
      decimal dec = 100_000_000.00m;
      Anal.ShowType(dec, "dec", "\n");
    }
    /*-----------------------------------------------------
      Demonstrate initialization of aggregate types
    -----------------------------------------------------*/
    static void DemoAggregateTypes() {
      
      Display.ShowNote("Aggregate Types", "\n", 25);

      /*-- array --*/
      Display.ShowOp("int[]array = { 1, 2, 3, 2, 1}");
      int[]?array = { 1, 2, 3, 2, 1 };
      int first = array[0];
      Anal.ShowType(array, "array");
      Display.Print(Display.ToStringRepIEnumerable<int[], int>(array));

      /*-- tuple --*/
      Display.ShowOp("(int, double, char)tup = (42, 3.14159, 'z');");
      (int, double, char)tup = (42, 3.14159, 'z');
      double second = tup.Item2;
      Anal.ShowType(tup, "tup");
      Anal.ShowType(42, "first arg");
      Anal.ShowType(3.1415927, "second arg");
      Anal.ShowType('z', "third arg", "\n");

      /*-- struct --*/
      Display.ShowOp("S s = new S(42, 3.1415927);");
      S s = new S(42, 3.1415927);
      int sfirst = s.I;
      Anal.ShowType(s, "s");
      Display.Print(s.ToStringRep() + "\n");
    }
    /*-----------------------------------------------------
      Demonstrate initialization of reference types
    -----------------------------------------------------*/
    static void DemoRefTypes() {

      Display.ShowNote("Language Defined Reference Types", "\n", 40);
      
      Display.ShowOp("object o = new object();");
      object o = new object();
      Anal.ShowType(o, "o", "\n");

      Display.ShowOp("string str = \"astring\"");
      string str = "a string";
      string str_alt = new("another string");
      Anal.ShowType(str, "str", "\n");

      Display.ShowOp("dynamic dyn = 42;");
      dynamic dyn = 42;
      Anal.ShowType(dyn, "dyn");
      dyn = 3.1415927;
      Anal.ShowType(dyn, "dyn", "\n");
      
      Display.ShowOp("X x = new X(42, 3.1415927);");
      X x = new X(42, 3.1415927);
      int xFirst = x.i;
      Anal.ShowType(x, "x");
      Console.WriteLine(x.ToStringRep() + "\n");
    }
    /*-----------------------------------------------------
      Demonstrate initialization of library types
    -----------------------------------------------------*/
    static void DemoLibTypes() {

      Display.ShowNote("Library Defined Reference Types", "\n", 40);

      Display.ShowOp("List<int> li = new List<int> { 1, 2, 3, 2, 1 };");
      List<int> li = new List<int> { 1, 2, 3, 2, 1 };
      int lfirst = li[0];
      Anal.ShowType(li, "li");
      Console.Write(
        "List<int> " +
        Display.ToStringRepIEnumerable<List<int>, int>(li)
      );
      li.Insert(5, 0);
      Display.ShowOp("li.Insert(5, 0);");
      Display.Print(
        "List<int> " +
        Display.ToStringRepIEnumerable<List<int>, int>(li)
      );

      /*---------------------------------------------------
        These declarations must immediately follow 
        namespace declaration (see top of file)
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
        - this is here just to show how to retrieve
          an element from an associative collection
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
      Anal.ShowType(dict, "dict");
      string rep = Display.ToStringRepAssocCont<Dict,string,int>(dict);
      Display.Print("dict: " + rep);
      /*---------------------------------------------------
        Can also use alternate display: 
        string rep = 
          Display.ToStringRepIEnumerable<Dict,KVPair>(dict);
      */
    }
    static unsafe void DemoCopy() {

      nl();
      Display.ShowNote(
        "Demonstrate Copy Operations", "\n"
      );
      Display.ShowOp("int i = 42;");
      int i = 42;
      string addri = Anal.ToStringAddress<int>(&i);
      Console.WriteLine("i: " + addri);
      Display.ShowOp("int j = i; // copy of value");
      int j = i;  // copy
      string addrj = Anal.ToStringAddress<int>(&j);
      Console.WriteLine("j: " + addrj);
      Display.ShowNote(
        "Addresses of i and j are unique, demonstrating\n" +
        "  value of i was copied to new j location.", "\n", 50
      );
    
      Display.ShowOp("List<int> li = new List<int> { 1, 2, 3, 2, 1 }");
      List<int> li = new List<int> { 1, 2, 3, 2, 1 };
    #pragma warning disable 8500
    /*
      Suppresses warning about taking address of managed type.
      The pointer is used only to show the address of ptr
      as part of analysis of copy operations.
    */
      string addrli = Anal.ToStringAddress<List<int>>(&li);
      Console.WriteLine("li: " + addrli);

      Display.ShowOp("List<int> lj = lj  // copy of reference");
      List<int> lj = li;  // copy of ref
      string addrlj = Anal.ToStringAddress<List<int>>(&lj);
      Console.WriteLine("lj: " + addrli);
      #pragma warning restore 8500
      Display.ShowNote(
        "Addresses of li and lj are the same, demonstrating\n" +
        "  reference of li was copied to new reference lj.\n" +
        "  Both references point to the same managed heap-based\n" +
        "  instance.", "\n", 55
      );
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
