using System;
using System.Reflection;
using System.Reflection.Emit;

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
//#pragma warning disable CS8602  // possibly null reference warning
// https://stackoverflow.com/questions/8173239/c-getting-size-of-a-value-type-variable-at-runtime/8173293#8173293
// 
namespace HelloObjects
{
  /*--- Point is a user-defined type --*/
  public class Point1   // value type with value type members
  { 
    public Point1() {}
    public int x { get; set; } = 0;
    public int y { get; set; } = 0;
    public int z { get; set; } = 0;
    public void printSelf(string name) {
      Console.Write("{0}  {{ x:{1}, y:{2}, z:{3} }}\n", name, x, y, z);
    }
  }
  
  /*-- type demonstration --*/

  class Program
  {
    /*-- demonstration begins here --*/
    const string nl = "\n";

    static void Main(string[] args)
    {
      print(" Demonstrate C# objects");
      // showNote("int - value type");
      // showOp("int t1 = 42");
      // int t1 = 42;
      // showType(t1, "t1", nl);
      // showOp("interate over val type members using reflection");
      // iterate(t1);
      // print();
      // showOp("int t1a = t1 : copy of value type");
      // int t1a = t1;
      // isSameObj(t1a, "t1a", t1, "t1", nl);

      // reference behavior - copy on write
      showNote("string - reference type");
      showOp("string t2 = \"a string\"");
      string t2 = "a string";
      showType(t2, "t2");
      showNote("string has many methods - uncomment next line to see them");
      //iterate(t2);
      showOp("string t3 = t2 : copy handle of ref type");
      string t3 = t2;
      showType(t3, "t3");
      isSameObj(t3,"t3",t2,"t2");
      showOp("t3 += \" is here\" : copy on write");
      t3 += " is here";
      showType(t2, "t2");
      showType(t3, "t3");
      isSameObj(t3,"t3",t2,"t2");
      showNote("t2 not modified by change in t3 due to copy on write", nl);
      
      showNote("Object - base reference type");
      showOp("Object obj1 - new Object()");
      Object obj1 = new Object();
      // showIdent(obj, "obj");
      showType(obj1, "obj");
      showOp("interate over ref type members using reflection");
      iterate(obj1);
      print();
      showOp("Object obj2 = obj1");
      Object obj2 = obj1;
      isSameObj(obj2, "obj2", obj1, "obj1", nl);

      showNote("Point1 user-defined reference type");
      showOp("Point1 t4 = new Point1()");
      Point1 t4 = new Point1();
      t4.x = 3;
      t4.y = 42;
      t4.z = -2;
      t4.printSelf("t4");
      showType(t4, "t4");
      showNote("value type: size of object in stackframe", nl);

      showOp("iterate over val type members using reflection");
      iterate(t4);
      print();

      Console.WriteLine("\nThat's all Folks!\n");
    }
    /*-- simple reflection --*/
    public static void showType<T>(T t, String nm, string suffix = "")
    {
      #pragma warning disable CS8602  // possibly null reference warning
      Type tt = t.GetType();
      Console.WriteLine("{0}, {1}", nm, tt.Name);
      int size = Utils.GetManagedSize(tt);
      Console.WriteLine("value: {0}, size: {1}{2}", t, size, suffix);
    }
    /*-- beware, two distinct objects may have same hashcode --*/
    public static void showIdent<T>(T t, String n, string suffix = "") {
      int id = t.GetHashCode();
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
    public static void print(String s = "") {
      Console.WriteLine(s);
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
