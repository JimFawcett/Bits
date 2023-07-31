/*-------------------------------------------------------------------
  Cs_Generic::Program.cs
  - demonstrates creating and using std library generic types:
      array, List<T>, Dictionary<K,V>
  - demonstrates creating and using user-defined generic types:
      Demo<T>, Stats<T>, and PointsN<T>
  - depends on PointsGeneric.cs to provide user-defined PointN class
  - depends on Stats.cs to provide user-defined Stats class
  - depends on AnalysisGeneric.cs for several display and analysis 
    functions
*/
using System;
using System.Collections;
using System.Collections.Generic;   // IEnumerable<T>, List<T>
using System.Linq;                  // IEnumerable<T>.ToArray
using System.Text;                  // StringBuilder
using Points;                       // defined in Points.cs
using Analysis;                     // defined in Analysis.cs
using Stats;                        // defined in Stats.cs
/*-----------------------------------------------
Note:
Find all Bits code, including this in
https://github.com/JimFawcett/Bits
You can clone the repo from this link.
-----------------------------------------------*/
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
    public T datum { get; set; }  // property
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
    static void Demostrate_generic_functions() {

      Display.ShowNote("Demonstrate generic functions", nl);
      
      Display.ShowOp("ShowList(List<int> l)");
      List<int> li = new List<int> {1, 2, 3, 2, 1};
      ShowList(li);
      Console.WriteLine();

      Display.ShowOp("ShowList(List<double> l)");
      List<double> ld = new List<double> {1.0, 2.25, 3.50, 4.75, 5.0};
      ShowList(ld);
      Console.WriteLine();

      Display.ShowOp("ShowTypeEnum(List<double> l, \"l\")");
      List<double> ldt = new List<double> {1.0, 2.25, 3.50, 4.75, 5.0};
      Display.ShowTypeEnum(ldt, "ldt");
      Console.WriteLine();

      Display.ShowOp("ShowTypeEnum(Dictionary<String, int> d, \"d\")");
      Dictionary<String, int> map = new Dictionary<String, int> {
        {"zero", 0}, {"one", 1}, {"two", 2}
      };
      Display.ShowTypeEnum(map, "map");
      Console.WriteLine();

    }
    static void Main(string[] args)
    {
      Display.ShowLabel(" Demonstrate C# generics");

      Demonstrate_Std_Library_Types();
      Demonstrate_User_defined_Types();
      Demostrate_generic_functions();

      Console.WriteLine("\nThat's all Folks!\n");
    }
  }
}

