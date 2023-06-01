/*-------------------------------------------------------------------
  Cs_Iter::Program.cs
  - Demonstrates IEnumerable interface and iteration
*/
using System;
using System.Collections;
using System.Collections.Generic;   // IEnumerable<T>, List<T>
using System.Linq;                  // IEnumerable<T>.ToArray
using System.Text;                  // StringBuilder
using Points;                       // defined in Points.cs
using Analysis;                     // defined in Analysis.cs

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
namespace CSharpIter
{
  /*-- Iter demonstration begins here --*/

  class Program
  {
    const string nl = "\n";
    static void Main(string[] args)
    {
      Display.ShowLabel(" Demonstrate C# iteration");

      Display.ShowNote(
        "Display string as a scalar value"
      );
      string aString = "a string";  // hides construction
      Display.ShowTypeScalar(aString, "aString", nl);

      Display.ShowNote(
        "Iterate over string characters"
      );
      string another = new string("another string");
      Display.ShowTypeEnum(another, "another", 10, nl);

      Display.ShowNote(
        "Iterate over List and Dictionary elements"
      );
      List<double> aList = 
        new List<double>{ 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0 };
    
      Display.ShowTypeEnum(aList, "aList", 5, nl);

      var d1 = new Dictionary<int, string>
      {
        { 0, "zero"}, {1, "one"}, {2, "two"}
      };
      Display.ShowTypeEnum(d1, "d1", 5, nl);
  
      Display.ShowNote(
        "Example of user-defined type:\n" + 
        "- PointN<T>, a point with N generic coordinates"
      );

      PointN<double> p1 = new PointN<double>(7);
      p1.coor = new List<double> {
        1.5, 2.0, 3.5, 4.0, 5.5, 6.0, 7.5
      };
      Console.WriteLine();
      Display.ShowOp("Using p1.iter():", nl);
      IEnumerable<double> iter = p1.iter();
      Console.Write("  ");
      foreach(double item in iter) {
        Console.Write("{0} ", item);
      }
      Console.WriteLine("\n");

      Console.WriteLine();
      Display.ShowOp("Using p1 enumerator():", nl);
      IEnumerator<double> enumer = p1.GetEnumerator();
      enumer.Reset();
      enumer.MoveNext();
      Console.Write("  {0}", enumer.Current);
      while(enumer.MoveNext()) {
        Console.Write(", {0}", enumer.Current);
      }
      Console.WriteLine("\n");

      Display.ShowNote(
        "PointN<T> is both Enumerable and Showable. That is,\n" +
        "it implements both IEnumerable<T> and IShow.\n" +
        "- so both ShowTypeEnum and ShowTypeShowable work", nl
      );
      Display.ShowOp("ShowTypeShowable(p1, \"p1\")");
      p1.Width = 6;
      Display.ShowTypeShowable(p1, "p1", nl);

      Display.ShowOp("ShowTypeEnum(p1, \"p1\")");
      Display.ShowTypeEnum<double>(p1, "p1", p1.Width, nl);

      Display.ShowOp("set left(2) and width(7)");
      PointN<int> p3 = new PointN<int>(7);
      p3.coor = new List<int>{1, 2, 3, 4, 5, 6, 7};
      Display.ShowOp("p3.Show(\"p3\")");
      p3.left(2).width(7);
      p3.Show("p3");
      Console.WriteLine();

      Display.ShowTypeShowable(p3, "p3", nl);

      Display.ShowNote(
        "Test formatting for Enumerable types", nl
      );

      int[] testarr = { 0,1, 2, 3, 4, 5, 6, 7, 8, 9 };
      Display.ShowTypeEnum(testarr, "testarr", 5, nl);

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
      // double[] arr3 = p1.coor.ToArray();
      // tmp = FoldArray(arr3, 4, 2);
      // Console.WriteLine("\n{0}",tmp);

      // string tmp1 = ToCSV<double>(p1.coor);
      // string tmp1 = ToCSV(p1.coor);  // demo that type inference works here
      // ShowTypeScalar(tmp2, "tmp2");
      #endregion

      Console.WriteLine("\nThat's all Folks!\n");
    }
  }
}

