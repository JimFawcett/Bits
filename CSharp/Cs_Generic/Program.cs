/*-------------------------------------------------------------------
  Cs_Generic::Program.cs
  - Demonstrates creation and assignment of objects from classes with
    generic type parameter(s).
*/
using System;
using System.Collections;
using System.Collections.Generic;   // IEnumerable<T>, List<T>
using System.Linq;                  // IEnumerable<T>.ToArray
using System.Text;                  // StringBuilder
using Points;                       // defined in Points.cs
using Analysis;                     // defined in Analysis.cs
using Stats;                        // defined in Stats.cs
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
      object, string, array, class
    ----------------------------------------------
      strings are immutable, so any change requires copy on write.
      That simulates value behavior.
*/
namespace CSharpGenerics
{
  public class Demo<T> where T:new() {
    public Demo() {
      datum = new T();
    }
    public Demo(T t) {
      datum = t;
    }
    public void show(String nm) {
      Console.Write("  {0}: {{ {1} }}\n", nm, datum);
    }
    public T datum { get; set; }
  }

  /*-- Generic demonstration begins here --*/

  class Program
  {
    const string nl = "\n";

    static void ShowList<T>(List<T> lst) {
      bool first = true;
      Console.Write("  [ ");
      foreach(T item in lst) {
        if(first) {
          first = false;
          Console.Write("{0}", item);
        }
        else {
          Console.Write(", {0}", item);
        }
      }
      Console.WriteLine(" ]");
    }

    /*-- demonstration starts here --*/
    static void Demonstrate_Std_Library_Types() {

      Display.ShowNote("Demonstrate std library generic types", nl);

      Display.ShowOp("String[] array processed by generic function");
      String[] array = {"one", "two", "three"};
      Display.ShowTypeEnum(array, "array", 5, nl);

      Display.ShowOp("List<double>");
      List<double> aList = 
      new List<double>{ 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0 };
      Display.ShowTypeEnum(aList, "aList", 5, nl);

      Display.ShowOp("Dictionary<int, String>");
      var d1 = new Dictionary<int, String>
      {
        { 0, "zero"}, {1, "one"}, {2, "two"}
      };
      Display.ShowTypeEnum(d1, "d1", 5, nl);
      
    }
    static void Demonstrate_User_defined_Types() {

      Display.ShowNote("Demonstrate user-defined generic types", nl);

      Display.ShowOp("Demo<int>");
      Demo<int> d = new Demo<int>(42);
      Display.ShowType(d, "d");
      Console.WriteLine("value:");
      d.show("d");
      Console.WriteLine();
      
      Display.ShowOp("Stats<double>");
      List<double> tmpl = new List<double> { 1.0, 1.5, 2.0, 1.5, 1.0 };
      Stats<double> s = new Stats<double>(tmpl);
      ShowList(tmpl);
      Console.WriteLine("  max: {0:0.0}", s.max());
      Console.WriteLine("  min: {0:0.0}", s.min());
      Console.WriteLine("  sum: {0:0.0}", s.sum());
      Console.WriteLine("  avg: {0:0.0}", s.avg());
      Console.WriteLine();

      Display.ShowOp("PointN<double>");
      PointN<double> p = new PointN<double>(0);
      List<double> tmp = new List<double>{1.0, 2.5, 3.0, -2.5, 1.0 };
      p.coor = tmp;
      Display.ShowTypeEnum(p, "p");
      Console.WriteLine();
    }
    static void Main(string[] args)
    {
      Display.ShowLabel(" Demonstrate C# generics");

      Demonstrate_Std_Library_Types();
      Demonstrate_User_defined_Types();

      // Display.ShowNote(
      //   "Examples of creation and display of Library Types\n" + 
      //   "- size is the size of reference, not instance\n" +
      //   "- String is not generic, but ShowTypeScalar(...),\n" +
      //   "  used below to display String, is generic\n" +
      //   "- Generic function ShowTypeEnum(...) is used, below,\n" +
      //   "  for both List<T> and Dictionary<K,V>", nl
      // );
      // /*-- ShowTypeScalar is a generic function --*/
      // string aString = "a string";  // hides construction
      // Display.ShowTypeScalar(aString, "aString", nl);

      // string another = new string("another string");
      // Display.ShowTypeScalar(another, "another", nl);

      // List<double> aList = 
      //   new List<double>{ 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0 };
    
      // Display.ShowTypeEnum(aList, "aList", 5, nl);

      // var d1 = new Dictionary<int, string>
      // {
      //   { 0, "zero"}, {1, "one"}, {2, "two"}
      // };
      // Display.ShowTypeEnum(d1, "d1", 5, nl);
  
      // Display.ShowNote(
      //   "Example of user-defined type:\n" + 
      //   "- PointN<T>, a point with N generic coordinates", nl
      // );

      // Display.ShowOp("Demo<int>", nl);
      // Demo<int> d = new(42);
      // d.show("d");
      // Console.WriteLine();

      // Display.ShowOp("PointN<double>", nl);
      // PointN<double> p1 = new PointN<double>(7);
      // p1.coor = new List<double> {
      //   1.5, 2.0, 3.5, 4.0, 5.5, 6.0, 7.5
      // };
      // p1.Show("p1");
      // Console.WriteLine();

      // Display.ShowNote(
      //   "PointN<T> is both Enumerable and Showable. That is,\n" +
      //   "it implements both IEnumerable<T> and IShow.\n" +
      //   "- so both ShowTypeEnum and ShowTypeShowable work"
      // );
      // Display.ShowOp("ShowTypeShowable(p1, \"p1\")");
      // p1.Width = 6;
      // Display.ShowTypeShowable(p1, "p1", nl);

      // Display.ShowOp("ShowTypeEnum(p1, \"p1\")");
      // Display.ShowTypeEnum<double>(p1, "p1", p1.Width, nl);

      // Display.ShowOp("set left(2) and width(7)");
      // PointN<int> p2 = new PointN<int>(7);
      // p2.coor = new List<int>{1, 2, 3, 4, 5, 6, 7};
      // Display.ShowOp("p2.Show(\"p2\")");
      // p2.left(2).width(7);
      // p2.Show("p2");
      // Console.WriteLine();

      // Display.ShowTypeShowable(p2, "p2", nl);

      // Display.ShowNote(
      //   "Test formatting for Enumerable types", nl
      // );

      // int[] testarr = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9 };
      // Display.ShowTypeEnum(testarr, "testarr", 5, nl);

      /*
        Commented function calls below left to let viewers
        uncomment and walk through with debugger.
      */
      #region 
      // ShowLabel("Test code for functions used above");
      // print("--- double[] to folded CSV ---");
      // int[] arr2 = { 0,1,2,3,4,5,6,7,8 };
      // string tmp = FoldArray(arr2, 3, 2);
      // Console.WriteLine("\n{0}",tmp);

      // ShowLabel("Alternate function for generating CSVs");
      // print("--- coor to folded CSV ---");
      // double[] arr3 = p2.coor.ToArray();
      // tmp = FoldArray(arr3, 4, 2);
      // Console.WriteLine("\n{0}",tmp);

      // string tmp1 = ToCSV<double>(p2.coor);
      // string tmp2 = ToCSV(p2.coor);  // demo that type inference works here
      // ShowTypeScalar(tmp2, "tmp2");
      #endregion

      Console.WriteLine("\nThat's all Folks!\n");
    }
  }
}

