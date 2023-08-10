using System;
using System.Reflection;
using System.Reflection.Emit;
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
namespace HelloData
{
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

    static void Main(string[] args)
    {
      Display.Print(" Demonstrate C# types");
  
      Display.ShowNote("int - value type");
      
      Display.ShowOp("int t1 = 42");
      int t1 = 42;
      Display.ShowType(t1, "t1", nl);
      
      Display.ShowOp("interate over val type members using reflection");
      Display.Iterate(t1);
      Display.Print();

      Display.ShowOp("int t1a = t1 : copy of value type");
      int t1a = t1;
      Display.IsSameObj(t1a, "t1a", t1, "t1", nl);

      // reference behavior - copy on write
      Display.ShowNote("string - reference type");

      Display.ShowOp("string t2 = \"a string\"");
      string t2 = "a string";
      Display.ShowType(t2, "t2");

      Display.ShowNote("string has many methods - uncomment next line to see them", "", 60);
      //Display.Iterate(t2);

      Display.ShowOp("string t3 = t2 : copy handle of ref type");
      string t3 = t2;
      Display.ShowType(t3, "t3");
      Display.IsSameObj(t3,"t3",t2,"t2");

      Display.ShowOp("t3 += \" is here\" : copy on write");
      t3 += " is here";
      Display.ShowType(t2, "t2");
      Display.ShowType(t3, "t3");
      Display.IsSameObj(t3,"t3",t2,"t2");
      Display.ShowNote("t2 not modified by change in t3 due to copy on write", nl, 55);
      Console.WriteLine();

      Display.ShowNote("Object - base reference type");

      Display.ShowOp("Object obj1 - new Object()");
      Object obj1 = new Object();
      Display.ShowType(obj1, "obj");

      Display.ShowOp("interate over ref type members using reflection");
      Display.Iterate(obj1);
      Display.Print();

      Display.ShowOp("Object obj2 = obj1");
      Object obj2 = obj1;
      Display.IsSameObj(obj2, "obj2", obj1, "obj1", nl);

      Display.ShowNote("Svt Struct value type");

      Display.ShowOp("Svt t4 = new Svt()");
      Svt t4 = new() { I=3, D=3.1415927, C='z' };
      t4.PrintSelf("t4");
      Display.ShowType(t4, "t4");
      Display.ShowNote("value type: size of object in stackframe", nl, 45);

      /* Iterate over val type members using reflection */
      Display.ShowOp("Iterate over val type members using reflection");
      Display.Iterate(t4);
      Display.Print();

      /* copy of value type */
      Display.ShowOp("Svt t4a = t4 : copy of val type");
      Svt t4a = t4;
      t4a.PrintSelf("t4a");
      t4.PrintSelf("t4");
      Display.IsSameObj(t4a,"t4a",t4,"t4");
      Display.ShowType(t4a, "t4a");

      Display.ShowOp("t4a.C = 'q'");
      t4a.C = 'q';
      t4a.PrintSelf("t4a");
      t4.PrintSelf("t4");
      Display.IsSameObj(t4, "t4", t4a, "t4a", nl);

      Display.ShowNote("Crt - ref type with ref member");

      Display.ShowOp("Crt t5 = new Crt()");
      Crt t5 = new() { S = "SomeString" };
      // using default values for I and D
      t5.PrintSelf("t5");
      Display.ShowType(t5, "t5");
      Display.ShowNote("ref type: size of handle to object in heap", nl, 45);

      /* copy handle of reference type */
      Display.ShowOp("Crt t5a = t5 : copy handle of ref type");
      Crt t5a = t5;
      t5a.PrintSelf("t5a");
      Display.IsSameObj(t5a, "t5a", t5, "t5");
      Display.IsSameObj(t5a.S, "t5a.S", t5.S, "t5.S");

      /* literal strings are not immutable */
      Display.ShowOp("t5a.S = \"new literal string\" : no copy on write");
      t5a.S = "new string";
      t5a.PrintSelf("t5a");
      t5a.PrintSelf("t5");
      Display.IsSameObj(t5a, "t5a", t5, "t5");
      Display.IsSameObj(t5a.S, "t5a.S", t5.S, "t5.S");
      Display.ShowNote("source t5 was altered!");

      Console.WriteLine("\nThat's all Folks!\n");
    }
  }
}
