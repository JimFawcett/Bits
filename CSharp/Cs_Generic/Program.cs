/*-------------------------------------------------------------------
  Cs_Generic::Program.cs
  - demonstrates creating and using std library generic types:
      array, List<T>, Dictionary<K,V>
  - demonstrates creating and using user-defined generic types:
      Demo<T>, Stats<T>, and Point<T, N>
  - depends on PointsGeneric.cs to provide user-defined Point class
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
  public class Hello<T> where T:new() {
    public Hello() {
      datum = new T();
    }
    public Hello(T t) {
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

      Display.ShowOp("Hello<int>");
      Hello<int> hi = new Hello<int>(42);
      Display.ShowType(hi, "hi");
      Console.WriteLine("value:");
      hi.show("h");
      Display.ShowOp("Hello<double>");
      Hello<double> hd = new Hello<double>(3.1415927);
      Console.WriteLine("access hd.datum: {0}", hd.datum);
      hd.show("hd");
      Console.WriteLine();
      
      Display.ShowOp("Stats<int>");
      List<int> tmp1 = new List<int> { 1, 2, 3, 2, 1 };
      Stats<int> si = new Stats<int>(tmp1);
      ShowList(tmp1);
      Console.WriteLine("  max: {0}", si.max());
      Console.WriteLine("  min: {0}", si.min());
      Console.WriteLine("  sum: {0}", si.sum());
      Console.WriteLine("  avg: {0:0.00}", si.avg());
      Console.WriteLine();

      Display.ShowOp("Stats<double>");
      List<double> tmp2 = new List<double> { 1.0, 1.5, 2.0, 1.5, 1.0 };
      Stats<double> sd = new Stats<double>(tmp2);
      ShowList(tmp2);
      Console.WriteLine("  max: {0:0.0}", sd.max());
      Console.WriteLine("  min: {0:0.0}", sd.min());
      Console.WriteLine("  sum: {0:0.0}", sd.sum());
      Console.WriteLine("  avg: {0:0.00}", sd.avg());
      Console.WriteLine();

      Display.ShowOp("Point<double>");

      Point<double> p = new Point<double>(5);
      List<double> tmp = new List<double>{1.0, 2.5, 3.0, -2.5, 1.0 };
      p.coor = tmp;
      Display.ShowTypeEnum(p, "p");
      Display.ShowOp("use indexing to retrieve and modify coordinates");
      Console.WriteLine("  value of p[1] is {0}", p[1]);
      p[1] = -2.5;
      Display.ShowOp("using p.show(callName)");
      p.Show("p");
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

      Display.ShowOp("ShowTypeEnum(List<double> ldt, \"ldt\")");
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

