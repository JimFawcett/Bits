using System;
using System.Reflection;
using System.Reflection.Emit;
using System.Collections;
using System.Collections.Generic;
using System.Text;

/* Cs_Data::Cs_Objects.cs */
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
// https://stackoverflow.com/questions/8173239/c-getting-size-of-a-value-type-variable-at-runtime/8173293#8173293
// 
namespace CSharpObjects
{
  interface IShow {
    void show();              // show rows of elements
    int Length { get; }       // total number of elements
    int width { get; set; }   // number of elements per row
    int left { get; set; }    // offset from terminal left
  }
  /*--- Point1 is a user-defined type --*/
  public class Point1 : IShow  // value type with value type members
  { 
    public Point1() {}
    public int x { get; set; } = 0;
    public int y { get; set; } = 0;
    public int z { get; set; } = 0;
    public int Length { get; } = 3;
    public int width { get; set; } = 3;
    public int left { get; set; } = 2;
    public void show() {
      printSelf("Point1");
    }
    public void printSelf(string name) {
      string ofstr = Program.indent(left);
      Console.WriteLine(
        "{4}{0}  {{ x:{1}, y:{2}, z:{3} }}", name, x, y, z, ofstr
      );
    }
  }
  /*--- Point2<T> is a generalization of Point1 ---*/
  public class Point2<T> : IEnumerable<T> , IShow {
    public Point2(int N) {
      coor = new List<T>();
      for(int i = 0; i<N; ++i) {
        T? test = default(T);
        if(test != null) {
          coor.Add(test);
        }
      }
    }
    public void show() {
      printSelf("Point2<T>");
    }
    public void printSelf(string name) {
      Console.Write(Program.indent(left));
      Console.Write("{0} {{ ", name);
      for(int i=0; i<coor.Count; ++i) {
        Console.Write("{0}", coor[i]);
        if(i < coor.Count - 1) {
          Program.print(", ");
        }
      }
      Console.Write(" }\n");
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
    public int width { get; set; } = 5;
    public int left { get; set; } = 2;
  }
  
  /*-- type demonstration --*/

  class Program
  {
    /*-- demonstration begins here --*/
    const string nl = "\n";

    static void Main(string[] args)
    {
      print(" Demonstrate C# objects");
      Type? t = GetTypeOfCollection(new List<double>());
      if(t != null) {
        Console.WriteLine("\n{0}", t.Name);
      }
      Double d = 3.1415927;
      showTypeScalar(d, "d");
      Point1 p1 = new Point1();
      println("--- showTypeShowable(p1, \"p1\") ---");
      showTypeShowable(p1, "p1");
      Point2<double> p2 = new Point2<double>(10);
      println("--- showTypeShowable(p2, \"p2\") ---");
      showTypeShowable(p2, "p2");
      println("--- showTypeEnum(p2, \"p2\") ---");
      showTypeEnum<double>(p2, "p2", 7);

      print("--- double[] to folded CSV ---");
      // double[] arr = p2.coor.ToArray();
      int[] arr = { 0,1,2,3,4,5,6,7,8 };
      string tmp = FoldArray(arr, 3, 2);
      Console.WriteLine("\n{0}",tmp);
      print("--- coor to folded CSV ---");
      double[] arr2 = p2.coor.ToArray();
      tmp = FoldArray(arr2, 4, 2);
      Console.WriteLine("\n{0}",tmp);
      // string tmp = ToCSV<double>(p2.coor);
      // string tmp1 = ToCSV(p2.coor);
      // showType(p1, "p1");
      // // showNote("int - value type");
      // // showOp("int t1 = 42");
      // // int t1 = 42;
      // // showType(t1, "t1", nl);
      // // showOp("interate over val type members using reflection");
      // // iterate(t1);
      // // print();
      // // showOp("int t1a = t1 : copy of value type");
      // // int t1a = t1;
      // // isSameObj(t1a, "t1a", t1, "t1", nl);

      // // reference behavior - copy on write
      // showNote("Library types string and List<T>");
      // showOp("string t2 = \"a string\"");
      // string t2 = "a string";
      // showType(t2, "t2");
      // showNote("string has many methods - uncomment next line to see them");
      // //iterate(t2);
      // showOp("string t3 = t2 : copy handle of ref type");
      // string t3 = t2;
      // showType(t3, "t3");
      // isSameObj(t3,"t3",t2,"t2");
      // showOp("t3 += \" is here\" : copy on write");
      // t3 += " is here";
      // showType(t2, "t2");
      // showType(t3, "t3");
      // isSameObj(t3,"t3",t2,"t2");
      // showNote("t2 not modified by change in t3 due to copy on write", nl);
      
      // List<int> nums = new List<int>();
      // nums.Add(1);
      // nums.Add(2);
      // nums.Add(3);
      // showType(nums, "nums");

      // showNote("Object - base reference type");
      // showOp("Object obj1 - new Object()");
      // Object obj1 = new Object();
      // // showIdent(obj, "obj");
      // showType(obj1, "obj");
      // showOp("interate over ref type members using reflection");
      // iterate(obj1);
      // print();
      // showOp("Object obj2 = obj1");
      // Object obj2 = obj1;
      // isSameObj(obj2, "obj2", obj1, "obj1", nl);

      // showNote("Point1 user-defined reference type");
      // showOp("Point1 t4 = new Point1()");
      // Point1 t4 = new Point1();
      // t4.x = 3;
      // t4.y = 42;
      // t4.z = -2;
      // t4.printSelf("t4");
      // showType(t4, "t4");
      // showNote("value type: size of object in stackframe", nl);

      // showOp("iterate over val type members using reflection");
      // iterate(t4);
      // print();

      Console.WriteLine("\nThat's all Folks!\n");
    }
    /*-- simple reflection --*/
    public static void showTypeScalar<T>(T t, string nm, string suffix = "")
    {
      Type tt = t!.GetType();
      int size = 0;
      if(tt != null) {
        Console.WriteLine("{0}, {1}", nm, tt.Name);
        size = Utils.GetManagedSize(tt);
      }
      Console.WriteLine("value: {0}, size: {1}{2}", t, size, suffix);
    }
    public static void showTypeEnum<T> (IEnumerable<T> t, string nm, int w = 5, string suffix="")
    {
      Type tt = t.GetType();
      Console.WriteLine("{0}, {1}", nm, tt.Name);
      int size = Utils.GetManagedSize(tt);
      // string tmp = ToCSV<T>(t);
      string tnm = tt.Name;
      if(tnm.Length > 1) {
        tnm = tnm.Remove(tnm.Length - 2) + "<T>";
      }
      Console.WriteLine("value:\n{0}{1} {{", "  ", tnm);
      // Console.WriteLine("    {0}", tmp);
      string tmp = FoldArray(t.ToArray(), w, 4);
      Console.Write(tmp);
      // foreach(T elem in t) {
      //   Console.Write("{0}, ", elem);   // change to string instead of write
      // }                                 // fold and remove last comment
      Console.WriteLine("\n  }");
      Console.WriteLine("size: {0}{1}", size, suffix);
    }
    public static void showTypeShowable<T>(T t, string nm, string suffix="")
      where T:IShow
    {
      Type tt = t.GetType();
      Console.WriteLine("{0}, {1}", nm, tt.Name);
      int size = Utils.GetManagedSize(tt);
      Console.WriteLine("value:");
      t.show();
      Console.WriteLine("size: {0}{1}", size, suffix);
    }
    // public static void showType<T>(T t, String nm, string suffix = "")
    // {
    //   #pragma warning disable CS8602  // possibly null reference warning
    //   Type tt = t.GetType();
    //   if(tt.IsPrimitive) {
    //     Console.WriteLine("{0}, {1}", nm, tt.Name);
    //   } 
    //   else {
    //     Type? tc = GetTypeOfCollection(t);
    //     if(tc != null) {
    //       Console.WriteLine("\n{0}", tc.Name);
    //     }
        
    //     string tmp = ToCSV((IEnumerable<T>)t);
    //     // Object obj = t;
    //     // string tmp = ToCSV2(obj);
    //     // string tmp = "test";
    //     Console.WriteLine("\n{0}, {1}", nm, tmp);
    //   }
    //   int size = Utils.GetManagedSize(tt);
    //   Console.WriteLine("value: {0}, size: {1}{2}", t, size, suffix);
    // }
    /*-- beware, two distinct objects may have same hashcode --*/
    public static void showIdent<T>(T t, String n, string suffix = "") {
      int id = t!.GetHashCode();
      Console.WriteLine("{0}, {1}{2}", n, id, suffix);
    }
    public static void isSameObj<T>(
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
    public static void showOp(string op, string suffix = "") {
      Console.WriteLine("--- {0} ---{1}", op, suffix);
    }
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
    // https://stackoverflow.com/questions/330493/join-collection-of-objects-into-comma-separated-string
    public static string ToCSV<T>(IEnumerable<T> coll) {
      StringBuilder sb = new StringBuilder();
      foreach(T elem in coll) {
        sb.Append(elem!.ToString()).Append(", ");
      }
      return sb.ToString(0, sb.Length - 2);
    }
    public static string ToCSV2(IEnumerable<Object> coll) {
      StringBuilder sb = new StringBuilder();
      foreach(Object elem in coll) {
        sb.Append(elem.ToString()).Append(", ");
      }
      return sb.ToString(0, sb.Length - 2);
    }
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
    // public static void test1() {
    //   List<double> ld = new List<double>();
    //     Type type = (ld).GetType();
    //     var etype = typeof(IEnumerable<>);
    //     foreach (var bt in type.GetInterfaces())
    //         if (bt.IsGenericType && bt.GetGenericTypeDefinition() == etype)
    //             Console.WriteLine(bt.GetGenericArguments()[0]);
    //   // List<double> ld = new List<double>();
    //   // Type t = ld.GetType();
    //   // Type tt = t.GetGenericTypeDefinition();
    //   // Console.WriteLine(tt.Name);
    // }
    public static void print(String s = "") {
      Console.Write(s);
    }
    public static void println(String s = "") {
      Console.WriteLine(s);
    }
    public static string indent(int count) {
      StringBuilder sb = new StringBuilder();
      sb.Append(' ', count);
      return sb.ToString();
    }
    public static void showNote(string s, string suffix = "") {
      Console.WriteLine(
        "\n-------------------------"
      );
      Console.WriteLine("{0}", s);  
      Console.WriteLine(
        "-------------------------{0}\n", suffix
      );
    }
    /* fold array elements into rows of w elements */
    public static string FoldArray<T>(T[] arr, int w, int left) {
      StringBuilder tmp = new StringBuilder();
      tmp.Append(indent(left));
      for(int i=0; i< arr.Length; ++i) {
        tmp.Append(arr[i]!.ToString());
        tmp.Append(", ");
        if((i+1) % w == 0 && i != arr.Length - 1) {
          tmp.Append("\n");
          tmp.Append(indent(left));
        }
        // tmp += arr[i].ToString();
        // tmp += ", ";
      }
      if(tmp.Length > 1) {
        tmp.Length -= 2;
      }
      return tmp.ToString();
    }
    public static void ShowFold<T>(T[] arr, uint w, uint left) {
      // print!("\n");
      // print_offset(left);
      // for(int i=0; i <= arr.GetLength(); ++i) {
      //   if (i < arr.GetLength) {
      //     print!("{:?}, ", arr[i - 1]);
      //   }
      //   else {
      //     print!("{:?}", arr[i - 1]);
      //     print!("\n");
      //     break;
      //   }
      //   if (i % w == 0  && i != 0 && i != w - 1) {
      //     print!("\n");
      //     print_offset(left);
      //   }
      // }
    }
    string truncate(int N, string bigStr) {
      // string temp = bigStr;
      // if(temp.length() > N) {
      //   temp.resize(N);
      //   return temp + "...";
      // }
      // return temp;
      return "truncate test";
    }
  }
  /*-- 
    Utils uses advanced relection 
    - GetMangedSize(Type type) is function that returns the size of 
      value types and handles.
      It is used to help discover how things work.
      It is placed here because it uses advanced techniques that
      will eventually be covered elsewhere in this site.  Knowing
      how it works is not essential for the things we are examining
      in this demo.
  --*/
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
