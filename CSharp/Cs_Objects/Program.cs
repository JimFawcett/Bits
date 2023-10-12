/*-------------------------------------------------------------------
  Cs_Objects::Program.cs
  - Demonstrates creation and assignment of objects as well as a few
    peeks at other operations on objects.
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
namespace CSharpObjects
{
  /* 
    Demonstration composite value type.
    - St is a value type because it is implemented with
      a struct and all its data members are value types. 
  */
  public struct St { 
    public St(int ai, double bi, char ci) {
      a=ai; b=bi; c=ci;
    }
    public void Show(string name) {
      Console.WriteLine(
        "  {0} {{ a:{1}, b:{2}, c:{3} }}", name, a, b, c 
      );
    }
    /*
     These data members are public instead of public properties.
     - we don't need that ceremony because St is simply a data
       aggregator. It doesn't have any significant behavior.
    */
    public int a; 
    public double b; 
    public char c; 
  }

  /*-- Object demonstration begins here --*/
  class Program
  {
    const string nl = "\n";
        static unsafe void Pass_by_value<T>(T? t, string nm) {
      string ts = Anal.GetTypeString(t, nm);
      Console.WriteLine(ts);
      /*
        Suppresses warning about taking address of managed type.
        The pointer is used only to show the address of ptr
        as part of analysis of copy operations.
      */
      #pragma warning disable 8500
      string addrd = Anal.ToStringAddress<T>(&t);
      #pragma warning restore 8500
      Console.WriteLine("{0}: {1}", nm, addrd);
      t = default(T);
      /*
        caller sees this change if and only if T is a reference type
        in which case t is null.
      */
    }
    static void TestForNullValue<T>(T? t, string nm) {
      if(t == null) {
        Console.WriteLine(nm + " is null");
      }
      else {
        Console.WriteLine(nm + " is {0}", t);
      }
    }
    static void DemoPassValAndRef() {
      Display.ShowNote("Pass by value", "\n");
      double d = 3.1415927;
      Pass_by_value<double>(d, "d");
      TestForNullValue(d, "d");

      List<int> li = new List<int>{ 1, 2, 3, 2, 1 };
      Pass_by_value<List<int>>(li, "li");
      TestForNullValue(li, "li");
    }

    static void Main(string[] args)
    {
      // https://www.pluralsight.com/guides/csharp-in-out-ref-parameters
      // https://www.pluralsight.com/guides/csharp-passing-reference-vs-value-objective
      DemoPassValAndRef();

      Display.ShowLabel(" Demonstrate C# objects");

      Display.ShowNote(
        "Examples of creation and display of Primitive Types"
      );
      short s = 123;
      Display.ShowTypeScalar(s, "s", nl);
      long l = 12345;
      Display.ShowTypeScalar(l, "l", nl);
      float f = 3.1415927f;
      Display.ShowTypeScalar(f, "f", nl);
      double d = 3.1415927;
      Display.ShowTypeScalar(d, "d", nl);
      int[] arr = new int[]{ 4, 3, 2, 1, 0, -1};
      Display.ShowTypeScalar(arr, "arr");
      Display.ShowIntArray(arr, nl);

      Display.ShowNote(
        "Examples of creation and display of Library Types\n" + 
        "- size is the size of reference, not instance"
      );
      string aString = "a string";  // hides construction
      Display.ShowTypeScalar(aString, "aString", nl);

      string another = new string("another string");
      Display.ShowTypeScalar(another, "another", nl);

      List<double> aList = 
        new List<double>{ 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0 };
    
      Display.ShowTypeScalar(aList, "aList");
      Display.ShowDoubleList(aList, nl);
      // Display.ShowTypeEnum(aList, "aList", 5, nl);

      var d1 = new Dictionary<int, string>
      {
        { 0, "zero"}, {1, "one"}, {2, "two"}
      };
      Display.ShowTypeScalar(d1, "d1");
      Display.ShowDictionary(d1, nl);
      // Display.ShowTypeEnum(d1, "d1", 5, nl);
  
      Display.ShowNote(
        "Examples of user-defined type:\n" + 
        "- Point4D, a point with x,y,z,t coordinates"
      );
      Point4D p1 = new Point4D();
      p1.x = 3;
      p1.y = -42;
      p1.z = 1;
      p1.Show("p1");
      Display.ShowOp("ShowTypeShowable(p1, \"p1\")");
      Display.ShowTypeShowable(p1, "p1");

      Display.ShowLabel(
        "Differences between value and reference types"
      );
      Display.ShowNote(
        "Assignment of reference types assigns their references,\n" +
        "creating two references to the same instance in managed\n" +
        "heap, so variables are coupled through single instance."
      );

      Display.ShowOp("Point4D val1 = new Point4D(), then initialize", "\n");
      Point4D val1 = new Point4D();
      val1.x = 1;
      val1.y = 2;
      val1.z = 3;
      Display.ShowLabeledObject(val1, "val1");

      Display.ShowOp("Point4D val2 = val1");
      Point4D val2 = val1;
      Display.ShowLabeledObject(val2, "val2");
      Display.IsSameObj(val2, "val2", val1, "val1");
      
      Display.println();
      Display.ShowOp("val2.z = 42;");
      Display.println();
      val2.z = 42;
      Display.IsSameObj(val2, "val2", val1, "val1");
      Display.ShowLabeledObject(val2, "val2");
      Display.ShowLabeledObject(val1, "val1");
      Display.ShowNote(
        "Note! Source of assignment, val1, changed when val2 changed.\n" +
        "Point4D is ref type, so assignment just assigns references."
      );
      Display.println();

      Display.ShowNote(
        "Value types can be independently assigned and copied."
      );
      Console.WriteLine();

      Display.ShowOp("var s1 = new St(1, -0.5, 'z');");
      var s1 = new St(1, -0.5, 'z');

      Display.ShowOp("var s2 = s1");
      var s2 = s1;
      Display.IsSameObj(s2, "s2", s1, "s1");
      s2.Show("s2");
      s1.Show("s1");

      Display.ShowOp("s2.c = 'q'");
      s2.c = 'q';
      s2.Show("s2");
      s1.Show("s1");
      Display.ShowNote(
        "Change in destination, s2, did not alter source, s1.\n" +
        "Assignment of value types creates independent objects."
      );

      Display.ShowLabel(
        "Instances of string are reference types, but simulate\n" +
        "some value behaviors.  String objects are immutable.\n" +
        "To modify an instance, a new string is created with\n" +
        "copies of the source string characters inclucing any\n" +
        "modifications."
      );
      var str1 = "An immutable string";
      var str2 = str1;  // copy handle not instance
      Display.IsSameObj(str2, "str2", str1, "str1");
      Display.DisplayLabeledObject<string>(str1, "str1");
      Display.DisplayLabeledObject(str2, "str2");

      str2 = str2.Remove(0,3);
      Display.IsSameObj(str2, "str2", str1, "str1");
      Display.DisplayLabeledObject(str1, "str1");
      Display.DisplayLabeledObject(str2, "str2");
      Display.ShowNote(
        "There is no way for change in str2 to affect str1."
      );
      
      Console.WriteLine("\nThat's all Folks!\n");
    }
  }
}

