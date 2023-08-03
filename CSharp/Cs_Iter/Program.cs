/*-------------------------------------------------------------------
  Cs_Iter::Program.cs
  - Demonstrates IEnumerable interface and iteration
*/
using System;
using System.Collections;
using System.Collections.Generic;   // IEnumerable<T>, List<T>
using System.Linq;                  // IEnumerable<T>.ToArray, select
using System.Text;                  // StringBuilder
using Points;                       // defined in Points.cs
using Analysis;                     // defined in Analysis.cs

/*-----------------------------------------------
  Note:
  Find all Bits code, including this in
  https://github.com/JimFawcett/Bits
  You can clone the repo from this link.
-----------------------------------------------*/
/*
  This demo uses Indexing and Enumerator to iterate 
  through std collections:
  - String, List<T>, Dictionary<K,V>
  and through user-defined type:
  - PointN<T> 
*/
namespace CSharpIter
{
  using Analysis;

  /*-- Iter demonstration begins here --*/

  class Program
  {
    const string nl = "\n";

    /*-----------------------------------------------------
      void ListIndexer<T>(List<T>)
      - Display list items with indexing
    */
    static void ListIndexer<T>(List<T> lst) {
      Console.Write("  ");
      bool first = true;
      for(int i=0; i < lst.Count; ++i) {
        if(first) {
          Console.Write("{0}", lst[i]);
          first = false;
        }
        else
          Console.Write(", {0}", lst[i]);
      }
      Display.Println("");
    }
    static void executeListIndexer() {
      Display.ShowNote("  execute ListIndexer<T>(List<T> lst)", nl);
      Display.ShowOp("index over List<int>");
      List<int> lint = new List<int> { 1, 2, 3, 4, 3, 2 };
      ListIndexer(lint);
      Console.WriteLine();
    }
    /*-----------------------------------------------------
      void GenericIndexer<T>(List<T>)
      - Display list items with indexing
    */
    static void GenericIndexer<C, T>(C coll) 
      where C:IEnumerable, ICollection<T>, IList
    {
      Console.Write("  {0}", coll[0]);
      for(int i=1; i < coll.Count(); ++i) {
        Console.Write(", {0}", coll[i]);
      }
      Display.Println("");
    }
    static void executeGenericIndexer() {
      Display.ShowNote("  GenericIndexer<C, T>(C coll)", nl);

      Display.ShowOp("index over int[]");
      int[] ai = new int[] {1, 2, 3, 4, 5};
      GenericIndexer<int[], int>(ai);
      Console.WriteLine();
      
      Display.ShowOp("index over List<int>");
      List<int> li = new List<int>{1, 2, 3, 2, 1};
      GenericIndexer<List<int>, int>(li);
      Console.WriteLine();
    }
    /*-----------------------------------------------------
      void GenericEnumerator<T>(IEnumerable<T> enm)
      - Display list items with enumerator
    */
    static void GenericEnumerator<T>(IEnumerable<T> enm) {
      var lenum = enm.GetEnumerator();
      lenum.MoveNext();
      Console.Write("  {0}", lenum.Current);
      while(lenum.MoveNext()) {
        var item = lenum.Current;
        Console.Write(", {0}", item);
      }
      Display.Println("");
    }
    static void executeGenericEnumerator() {
      Display.ShowNote("  GenericEnumerator<T>(IEnumerable<T> enm)");
      Console.WriteLine();

      Display.ShowOp("enumerate over int[]");
      int[] ai = new int[] {1, 2, 3, 4, 5};
      GenericEnumerator<int>(ai);
      Console.WriteLine();

      Display.ShowOp("enumerate over String");
      String s = "a string";
      GenericEnumerator<char>(s);
      Console.WriteLine();

      Display.ShowOp("enumerate over List<double>");
      List<double> ld = new List<double>{1.0, 2.25, 3.5, 2.75, 1.0};
      GenericEnumerator<double>(ld);
      Console.WriteLine();

      Display.ShowOp("enumerate over Dictionary<String, int>");
      Dictionary<String, int> d = new Dictionary<String, int> {
        {"zero", 0}, {"one", 1}, {"two", 2}
      };
      GenericEnumerator<KeyValuePair<String,int>>(d);
      Console.WriteLine();
    }
    /*-----------------------------------------------------
      void GenericForEach<T>(IEnumerable<T> enm)
      - Display list items with foreach
    */
    static void GenericForEach<T>(IEnumerable<T> enm, Action<T> lambda) {
      foreach (T item in enm) {
        lambda(item);
      }
    }
    static void ExecuteGenericForEach() {
      Display.ShowNote(
        @"  GenericForEach<T>(
    IEnumerable<T> enm, Action<T> lambda
  )"
      );
      Console.WriteLine();

      Display.ShowOp("Add 1.0 to items of List<double>");
      List<double> ld = new List<double>{1.0, 2.25, 3.5, 2.75, 1.0};
      Console.Write("  ");
      Action<double> plus_one = x => Console.Write("{0} ",x + 1.0);
      GenericForEach<double>(ld, plus_one);
      Console.WriteLine("\n");
    }
    /*-----------------------------------------------------
      IEnumerable<T> GenericModifier<T>(IEnumerable<T> enm, Func<T,T> lambda)
      - return modified IEnumerable<T> transformed by lambda Func<T,T>
      - uses Linq Select
    */
    static IEnumerable<T> GenericModifier<T>(IEnumerable<T> enm, Func<T,T> lambda) {
      /* 
        IEnumerable<T> is immutable so make new modified collection 
        using Linq Select method
      */
      return enm.Select(lambda);
    }
    static void ExecuteGenericModifier() {
      Display.ShowNote(
        "  IEnumerable<T> GenericModifier<T>(\n    Ienumerable<T> enm, Func<T,T> lambda\n  )"
      );
      Console.WriteLine();

      Display.ShowOp("GenericModifier for List<int> with plus_one");
      /*-- original --*/
      List<int> li = new List<int>{1, 2, 3, 4, 3, 2};
      Console.WriteLine("Original:");
      GenericEnumerator<int>(li);
      /*-- modified --*/
      Func<int,int> plus_one = x => x+1;
      var coll = GenericModifier<int>(li, plus_one);  // modify
      Console.WriteLine("Modified:");
      GenericEnumerator<int>(coll);                   // display
      Console.WriteLine();

      Display.ShowOp("GenericModifier for array<double> with square");
      /*-- original --*/
      double[] da = new double[]{ 1.0, 2.5, 3.0, 4.5 };
      Console.WriteLine("Original:");
      GenericEnumerator<double>(da);
      /*-- modified --*/
      Func<double,double> square = x => x*x;
      var coll2 = GenericModifier<double>(da, square);  // modify
      Console.WriteLine("Modified:");
      GenericEnumerator<double>(coll2);                 // display
      Console.WriteLine();
    }
    /*-----------------------------------------------------
      Begin demonstration
    */
    static void Main(string[] args)
    {
      Display.ShowLabel(" Demonstrate C# iteration");

      executeListIndexer();
      executeGenericIndexer();
      executeGenericEnumerator();
      ExecuteGenericForEach();
      ExecuteGenericModifier();

      Display.ShowNote(
        "  Test formatting for Enumerable types", nl
      );

      int[] testarr = { 0,1, 2, 3, 4, 5, 6, 7, 8, 9 };
      Display.ShowTypeEnum(testarr, "testarr", 5, nl);

      /*
        Commented function calls below left to let viewers
        uncomment and walk through with debugger.
      */
      #region 
      // Display.ShowLabel("  Test code for functions used above");
      // Display.print("--- double[] to folded CSV ---");
      // int[] arr2 = { 0,1,2,3,4,5,6,7,8 };
      // string tmp = Display.FoldArray(arr2, 3, 2);
      // Console.WriteLine("\n{0}",tmp);

      // Display.ShowLabel("  Alternate function for generating CSVs");
      // Display.print("--- coor to folded CSV ---");
      // PointN<int> p1 = new PointN<int>(0);
      // List<int> tmpl = new List<int>{ 1, 2, 3, 4, 5, 6, 7 };
      // p1.coor = tmpl;
      // int[] arr3 = p1.coor.ToArray();
      // tmp = Display.FoldArray(arr3, 4, 2);
      // Console.WriteLine("\n{0}",tmp);

      // string tmp1 = Display.ToCSV<int>(p1.coor);
      // tmp1 = Display.ToCSV(p1.coor);  // demo that type inference works here
      // Display.ShowTypeScalar(tmp1, "tmp1");
      #endregion

      Console.WriteLine("\nThat's all Folks!\n");
    }
  }
}

