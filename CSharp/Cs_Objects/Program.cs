using System;
using System.Reflection;
using System.Reflection.Emit;
using System.Collections;
using System.Collections.Generic;   // IEnumerable<T>, List<T>
using System.Linq;                  // IEnumerable<T>.ToArray
using System.Text;                  // StringBuilder

/*
  Cs_Objects::Cs_Objects.cs
  ----------------------
  Demonstrates creation and assignment of objects as well as a few
  peeks at other operations on objects.
*/
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
    ----------------------------------------------
      strings are immutable, so any change requires copy on write.
      That simulates value behavior.
*/
namespace CSharpObjects
{
  /*----------------------------------------------------------------------
    IShow allows analysis functions to operate on instances of any class 
    that implements it. See ShowTypeShowable<T>(T t, ...), below.
  */
  interface IShow {
    void Show(string name);   // show instance state as rows of elements
    int Length { get; }       // total number of elements
    int Width { get; set; }   // number of elements per row
    int Left { get; set; }    // offset from terminal Left
  }
  /*-----------------------------------------------------------------------
    Point1 is a basic point class with three integer coordinates.
    - It is a reference type because it's instances are created
      from a class
    - That affects the way assignments work - see demo near the
      end of main.
  */
  public class Point1 : IShow  // value type with value type members
  { 
    public Point1() {
      /* 
        doesn't need to do anything because its properties have
        default values
      */
    }
    public int x { get; set; } = 0;
    public int y { get; set; } = 0;
    public int z { get; set; } = 0;
    public int Length { get; } = 3;
    public int Width { get; set; } = 3;
    public int Left { get; set; } = 2;
    /* translates IShow::show to needs of class */
    public void Show(string name) {
      PrintSelf(name);
    }
    /* Display Point1 structure and state */
    public void PrintSelf(string name) {
      string ofstr = Program.Indent(Left);
      Console.WriteLine(
        "{4}{0}  {{ x:{1}, y:{2}, z:{3} }}", name, x, y, z, ofstr
      );
    }
  }
  /*----------------------------------------------------------------------
    Point2<T> is a generalization of Point1 
     - holds any finite number of generic coordinates
     - coordinates are held in a List<T>
     - implements IEnumerable<T> so it can be indexed and iterated
     - it is a reference type because it is implemented with a class.
       its List<T> is also a reference type
  */
  public class Point2<T> : IEnumerable<T> , IShow {
    /*--------------------------------------------------------------------
      Constructs a point with N coordinates each with default value
    */
    public Point2(int N) {
      coor = new List<T>();
      for(int i = 0; i<N; ++i) {
        T? test = default(T);
        if(test != null) {
          coor.Add(test);
        }
      }
    }
    /* translates IShow::show() for needs of Point2 class */
    public void Show(string name) {
      PrintSelf(name);
    }
    /*
      Displays structure and state of N-dimensional point.
      - state is a set of rows of coordinate data
      - property Width specifies number of elements in each row
      - property Left specifies offset of row from terminal Left edge
    */
    public void PrintSelf(string name) {
      Console.Write(Program.Indent(Left));
      Console.Write("{0} {{\n{1}", name, Program.Indent(Left + 2));
      for(int i=0; i<coor.Count; ++i) {
        Console.Write("{0}", coor[i]);
        if(i < coor.Count - 1) {
          Program.print(", ");
        }
        if((i+1) % Width == 0 && i != coor.Count - 1) {
          Console.Write("\n");
          Console.Write(Program.Indent(Left + 2));
        }
      }
      Console.Write("\n{0}", Program.Indent(Left));
      Console.WriteLine("}");
    }
    /* The three functions below support indexing and iterating */
    public T this[int index] {
      get { return coor[index]; }
      set { coor.Insert(index, value); }
    }
    public IEnumerator<T> GetEnumerator() {
      return coor.GetEnumerator();
    }
    IEnumerator IEnumerable.GetEnumerator() {
      return this.GetEnumerator();
    }
    public List<T> coor { get; set; }
    public int Length { get { return coor.Count; } }
    public int Width { get; set; } = 5;   // default row size
    public int Left { get; set; } = 2;    // default offset
    /* initializers */
    public Point2<T> width(int w) {
      this.Width = w;
      return this;
    } 
    public Point2<T> left(int l) {
      this.Left = l;
      return this;
    } 
  }
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

    static void Main(string[] args)
    {
      ShowLabel(" Demonstrate C# objects");

      ShowNote(
        "Examples of creation and display of Primitive Types"
      );
      short s = 123;
      ShowTypeScalar(s, "s", nl);
      long l = 12345;
      ShowTypeScalar(l, "l", nl);
      float f = 3.1415927f;
      ShowTypeScalar(f, "f", nl);
      double d = 3.1415927;
      ShowTypeScalar(d, "d", nl);
      int[] arr = new int[]{ 4, 3, 2, 1, 0, -1};
      ShowTypeEnum(arr, "arr", 10, nl);

      ShowNote(
        "Examples of creation and display of Library Types\n" + 
        "- size is the size of reference, not instance"
      );
      string aString = "a string";  // hides construction
      ShowTypeScalar(aString, "aString", nl);

      string another = new string("another string");
      ShowTypeScalar(another, "another", nl);

      List<double> aList = 
        new List<double>{ 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0 };
    
      ShowTypeEnum(aList, "aList", 5, nl);

      var d1 = new Dictionary<int, string>
      {
        { 0, "zero"}, {1, "one"}, {2, "two"}
      };
      ShowTypeEnum(d1, "d1", 5, nl);
  
      ShowNote(
        "Examples of user-defined types:\n" + 
        "- Point1, a point with x,y,z coordinates\n" +
        "- Point2, a point with N generic coordinates"
      );
      Point1 p1 = new Point1();
      p1.x = 3;
      p1.y = -42;
      p1.z = 1;
      ShowOp("ShowTypeShowable(p1, \"p1\")");
      ShowTypeShowable(p1, "p1", nl);

      ShowNote(
        "Point2<T> is both Enumerable and Showable. That is,\n" +
        "it implements both IEnumerable<T> and IShow.\n" +
        "- so both ShowTypeEnum and ShowTypeShowable work"
      );
      Point2<double> p2 = new Point2<double>(7);
      p2.coor = new List<double> {
        1.5, 2.0, 3.5, 4.0, 5.5, 6.0, 7.5
      };
      ShowOp("ShowTypeShowable(p2, \"p2\")");
      p2.Width = 6;
      ShowTypeShowable(p2, "p2", nl);

      ShowOp("ShowTypeEnum(p2, \"p2\")");
      ShowTypeEnum<double>(p2, "p2", p2.Width, nl);

      ShowOp("ShowTypeShowable(p3, \"p3\")");
      Point2<int> p3 = new Point2<int>(7);
      p3.left(4).width(7);
      ShowOp("set left(4) and width(7)");
      p3.coor = new List<int>{1, 2, 3, 4, 5, 6, 7};
      ShowTypeShowable(p3, "p3", nl);

      ShowLabel(
        "Differences between value and reference types"
      );
      ShowNote(
        "Assignment of reference types assigns their references,\n" +
        "creating two references to the same instance in managed\n" +
        "heap, so variables are coupled through single instance."
      );
      ShowOp("Point1 val1 = new Point1(), then initialize");
      Point1 val1 = new Point1();
      val1.x = 1;
      val1.y = 2;
      val1.y = 3;
      ShowLabeledObject(val1, "val1");
      ShowOp("Point1 val2 = val1");
      Point1 val2 = val1;
      ShowLabeledObject(val2, "val2");
      IsSameObj(val2, "val2", val1, "val1");
      ShowOp("val2.z = 42;");
      val2.z = 42;
      IsSameObj(val2, "val2", val1, "val1");
      ShowLabeledObject(val2, "val2");
      ShowLabeledObject(val1, "val1");
      ShowNote(
        "Note! Source of assignment, val1, changed when val2 changed.\n" +
        "Point1 is ref type, so assignment just assigns references."
      );
      println();

      ShowNote(
        "Value types can be independently assigned and copied."
      );
      ShowOp("var s1 = new St(1, -0.5, 'z');");
      var s1 = new St(1, -0.5, 'z');
      ShowOp("var s2 = s1");
      var s2 = s1;
      IsSameObj(s2, "s2", s1, "s1");
      s2.Show("s2");
      s1.Show("s1");
      ShowOp("s2.c = 'q'");
      s2.c = 'q';
      s2.Show("s2");
      s1.Show("s1");
      ShowNote(
        "Change in destination, s2, did not alter source, s1.\n" +
        "Assignment of value types creates independent objects."
      );
      ShowLabel(
        "Instances of string are reference types, but simulate\n" +
        "some value behaviors.  String objects are immutable.\n" +
        "To modify an instance, a new string is created with\n" +
        "copies of the source string characters inclucing any\n" +
        "modifications.  This is called copy on write."
      );
      var str1 = "An immutable string";
      var str2 = str1;
      IsSameObj(str2, "str2", str1, "str1");
      DisplayLabeledObject<string>(str1, "str1");
      DisplayLabeledObject(str2, "str2");
      str2 = str2.Remove(0,3);
      IsSameObj(str2, "str2", str1, "str1");
      DisplayLabeledObject(str1, "str1");
      DisplayLabeledObject(str2, "str2");
      ShowNote(
        "There is no way for change in str2 to affect str1."
      );

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

      Console.WriteLine("\nThat's all Folks!\n");
    }
    /* - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        cut here

        The functions below are used for analysis and display. They
        have varying complexities. It is not essential to understand 
        them in order to follow the demonstrations above.  We will 
        look at them in some detail in Bits_Functions. 
    */
    public static void ShowType<T>(T t, string nm) {
      Type tt = t!.GetType();
      string tnm = tt.Name;
      if(tt.IsGenericType) {
        if(tnm.Length > 1) {
          tnm = tnm.Remove(tnm.Length - 2) + "<T>";
        }
      }
      Console.Write("{0}: {1}, ", nm, tnm);
      int size = Utils.GetManagedSize(tt);
      Console.Write("size: {0}, ", size);
      if (tt.IsValueType) {
        Console.WriteLine("value type");
      }
      else {
        Console.WriteLine("reference type");
      }
    }
    /*--------------------------------------------------------------------
      Display information about the type of any scalar type.
      - scalar types are those with a single value like:
        int, double, string, ...
      - This function directly uses only simple reflection
    */
    public static void ShowTypeScalar<T>(T t, string nm, string suffix = "")
    {
      // Type? tt = t!.GetType();
      // int size = 0;
      // if(tt != null) {
      //   Console.WriteLine("{0}, {1}", nm, tt.Name);
      //   size = Utils.GetManagedSize(tt);
      // }
      ShowType(t, nm);
      Console.WriteLine("value: \"{0}\"{1}", t, suffix);
    }
    /*--------------------------------------------------------------------
      Show type information about any type that implements the IShow
      interface
    */
    public static void ShowTypeShowable<T>(T t, string nm, string suffix="")
      where T:IShow
    {
      ShowType(t, nm);
      // Type tt = t.GetType();
      // Console.WriteLine("{0}, {1}", nm, tt.Name);
      // int size = Utils.GetManagedSize(tt);
      Console.WriteLine("value:");
      t.Show(nm);  // guaranteed availability by IShow implementation
      // Console.WriteLine("size: {0}{1}", size, suffix);
      Console.Write(suffix);
    }
    /*--------------------------------------------------------------------
      Provides name of caller, nm, as label for IShow() information.
      - works for all Showable instances
    */
    public static void ShowLabeledObject<T>(T t, string nm) where T:IShow {
      Console.Write(nm);
      t.Show(nm);
    }
    /*--------------------------------------------------------------------
      Provides name of caller, nm, as label for value.
      - works for all T with ToString.
    */
    public static void DisplayLabeledObject<T>(T t, string nm) {
      Console.WriteLine("{0}: {1}", nm, t!.ToString());
    }
    /*--------------------------------------------------------------------
      Show type information for any type that implements IEnumerable<T> 
      interface.
    */
    public static void ShowTypeEnum<T> (
      IEnumerable<T> t, string nm, int w = 5, string suffix = ""
    )
    {
      ShowType(t, nm);
      Console.WriteLine("value:\n{0}{1} {{", "  ", nm);
      /* 
        beautify value list into rows of w elements 
        indented by 4 spaces
      */
      string tmp = FoldArray(t.ToArray(), w, 4);
      Console.Write(tmp);
      Console.WriteLine("\n  }");
      Console.Write(suffix);
    }
    /*--------------------------------------------------------------------
      create string of count spaces, used to offset output 
    */
    public static string Indent(int count) {
      StringBuilder sb = new StringBuilder();
      sb.Append(' ', count);
      return sb.ToString();
    }
    /*--------------------------------------------------------------------
      Truncate string to length of N, but only
      if its length is greater than N
    */
    string truncate(int N, string bigStr) {
      if(bigStr.Length <= N) {
        return bigStr;
      }
      StringBuilder sb =  new StringBuilder();
      sb.Append(bigStr);
      sb.Length = N;  // move back pointer to desired length
      return sb.ToString();
    }
    /*-------------------------------------------------------------------- 
      fold array elements into rows of w elements 
    */
    public static string FoldArray<T>(T[] arr, int w, int Left) {
      StringBuilder tmp = new StringBuilder();
      tmp.Append(Indent(Left));
      for(int i=0; i< arr.Length; ++i) {
        tmp.Append(arr[i]!.ToString());
        tmp.Append(", ");
        if((i+1) % w == 0 && i != arr.Length - 1) {
          tmp.Append("\n");
          tmp.Append(Indent(Left));
        }
      }
      if(tmp.Length > 1) {
        tmp.Length -= 2;  // don't return last comma and space
      }
      return tmp.ToString();
    }
    /*--------------------------------------------------------------------
      do t1 and t2 share the same address?
    */
    public static void IsSameObj<T>(
      T t1, String n1, T t2, String n2, string suffix = ""
    ) {
      if(ReferenceEquals(t1, t2)) {
        Console.WriteLine(
          "{0} is same object as {1}{2}", n1, n2, suffix
        );
      }
      else {
        Console.WriteLine(
          "{0} is not same object as {1}{2}", n1, n2, suffix);
      }
    }
    /*--------------------------------------------------------------------
      Beware, two distinct objects may have same hashcode.
      Not used in this demo for that reason.
    */
    public static void showIdent<T>(T t, String n, string suffix = "") {
      int id = t!.GetHashCode();
      Console.WriteLine("{0}, {1}{2}", n, id, suffix);
    }
    /*--------------------------------------------------------------------
      Display function call or operation to help turn output data 
      into information 
    */
    public static void ShowOp(string op, string suffix = "") {
      Console.WriteLine("--- {0} ---{1}", op, suffix);
    }
    public static void print(String s = "") {
      Console.Write(s);
    }
    public static void println(String s = "") {
      Console.WriteLine(s);
    }
    /*--------------------------------------------------------------------
      Emphasize text with borders
    */
    public static void ShowNote(string s, string suffix = "") {
      Console.WriteLine(
        "--------------------------------------------------"
      );
      Console.WriteLine("{0}", s);  
      Console.WriteLine(
        "--------------------------------------------------{0}", suffix
      );
    }
    /*--------------------------------------------------------------------
      Surround note with empty lines
    */
    public static void ShowLabel(string s) {
      Console.WriteLine();
      ShowNote(s);
      Console.WriteLine();
    }
    /*
      Show fields and methods for either reference or value types using reflection
    */
    // https://stackoverflow.com/questions/7613782/iterating-through-struct-members
    public static void iterate<T>(T t) /*where T:new()*/ {
      Console.WriteLine("fields:");
      foreach(
        var field in typeof(T).GetFields(
          BindingFlags.Instance | BindingFlags.NonPublic | BindingFlags.Public
        )
      ) {
        Console.WriteLine(
          "  {0} = {1}", field.Name, field.GetValue(t)
        );
      }
      Console.WriteLine("methods:");
      foreach(
        var method in typeof(T).GetMethods(
          BindingFlags.Instance | BindingFlags.Public
        )
      ) {
        Console.WriteLine(
          "  {0}", method.Name
        );
      }
    }
    /*--------------------------------------------------------------------
      Build string of comma separated values from Enumerable collection
      - no longer used here, but will be useful so kept
    */
    // https://stackoverflow.com/questions/330493/join-collection-of-objects-into-comma-separated-string
    public static string ToCSV<T>(IEnumerable<T> coll) {
      StringBuilder sb = new StringBuilder();
      foreach(T elem in coll) {
        sb.Append(elem!.ToString()).Append(", ");
      }
      return sb.ToString(0, sb.Length - 2);
    }
    /*--------------------------------------------------------------------
      Returns value of T for IEnumerable<T> at runtime.  Needed for some functions
      that operate on generic collections.
      - at this time, not used in this demo
    */
    // https://www.codeproject.com/Tips/5267157/How-To-Get-A-Collection-Element-Type-Using-Reflect
    public static Type? GetTypeOfCollection(Object coll) {
      Type type = (coll).GetType();
      var etype = typeof(IEnumerable<>);
      foreach (var bt in type.GetInterfaces()) {
        if (bt.IsGenericType && bt.GetGenericTypeDefinition() == etype) {
          return (bt.GetGenericArguments()[0]);
        }
      }
      return null;
    }
  }
  /*
    Utils uses advanced relection 
    - GetMangedSize(Type type) is function that returns the size of 
      value types and handles, used to help discover how things work.
      It is placed here because it uses advanced techniques that
      will eventually be covered elsewhere in this site.  Knowing
      how it works is not essential for the things we are examining
      in this demo.
  */
  // https://stackoverflow.com/questions/8173239/c-getting-size-of-a-value-type-variable-at-runtime/8173293#8173293
  public static class Utils {
    /*-- more advanced reflection --*/
    // https://stackoverflow.com/questions/7613782/iterating-through-struct-members
    public static int GetManagedSize(Type type)
    {
      // all this just to invoke one opcode with no arguments!
      var method = new DynamicMethod(
        "GetManagedSizeImpl", typeof(uint), new Type[0], 
        typeof(TypeExtensions), false
      );

      ILGenerator gen = method.GetILGenerator();
      gen.Emit(OpCodes.Sizeof, type);
      gen.Emit(OpCodes.Ret);

      var func = (Func<uint>)method.CreateDelegate(typeof(Func<uint>));
      return checked((int)func());
    }
  }
}
