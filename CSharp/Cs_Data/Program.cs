using System;
using System.Reflection;
using System.Reflection.Emit;

/* Cs_Data::Cs_Data.cs */
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
    public void printSelf(string name) {
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
    public void printSelf(string name) {
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
      print(" Demonstrate C# types");
      showNote("int - value type");
      showOp("int t1 = 42");
      int t1 = 42;
      showType(t1, "t1", nl);
      showOp("interate over val type members using reflection");
      iterate(t1);
      print();
      showOp("int t1a = t1 : copy of value type");
      int t1a = t1;
      isSameObj(t1a, "t1a", t1, "t1", nl);

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

      showNote("Svt Struct value type");
      showOp("Svt t4 = new Svt()");
      Svt t4 = new Svt();
      t4.I = 3;
      t4.D = 3.1415927;
      t4.C = 'z';
      t4.printSelf("t4");
      showType(t4, "t4");
      showNote("value type: size of object in stackframe", nl);

      showOp("iterate over val type members using reflection");
      iterate(t4);
      print();

      showOp("Svt t4a = t4 : copy of val type");
      Svt t4a = t4;
      t4a.printSelf("t4a");
      t4.printSelf("t4");
      isSameObj(t4a,"t4a",t4,"t4");
      showType(t4a, "t4a");

      showOp("t4a.C = 'q'");
      t4a.C = 'q';
      t4a.printSelf("t4a");
      t4.printSelf("t4");
      isSameObj(t4, "t4", t4a, "t4a", nl);

      showNote("Crt - ref type with ref member");
      showOp("Crt t5 = new Crt()");
      Crt t5 = new Crt();
      t5.S = "SomeString";
      t5.printSelf("t5");
      showType(t5, "t5");
      showNote("ref type: size of handle to object in heap", nl);

      showOp("Crt t5a = t5 : copy handle of ref type");
      Crt t5a = t5;
      t5a.printSelf("t5a");
      isSameObj(t5a, "t5a", t5, "t5");
      isSameObj(t5a.S, "t5a.S", t5.S, "t5.S");

      showOp("t5a.S = \"new string\" : no copy on write");
      t5a.S = "new string";
      t5a.printSelf("t5a");
      t5a.printSelf("t5");
      isSameObj(t5a, "t5a", t5, "t5");
      isSameObj(t5a.S, "t5a.S", t5.S, "t5.S");
      showNote("source t5 was altered!");

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
        "-------------------------"
      );
      Console.WriteLine("{0}", s);  
      Console.WriteLine(
        "-------------------------{0}", suffix
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
