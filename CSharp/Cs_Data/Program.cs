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
using System;
using System.Reflection;
using System.Reflection.Emit;

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
    Crt is a value type used for type demo example.
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
    static void Main(string[] args)
    {
      print(" Demonstrate C# types");
      label("int - value type");
      showOp("int t1 = 42");
      int t1 = 42;
      showType(t1, "t1");
      iterate(t1);
      print();
      showOp("int t1a = t1");
      int t1a = t1;
      isSameObj(t1a, "t1a", t1, "t1");

      // reference behavior - copy on write
      label("string - reference type");
      showOp("string t2 = \"a string\"");
      string t2 = "a string";
      showType(t2, "t2");
      showNote("string has many methods - uncomment next line to see them");
      //iterate(t2);
      showOp("string t3 = t2");
      string t3 = t2;
      showType(t3, "t3");
      isSameObj(t3,"t3",t2,"t2");

      showOp("t3 += \" is here\"");
      t3 += " is here";
      showType(t3, "t3");
      isSameObj(t3,"t3",t2,"t2");
      
      label("Object - base reference type");
      showOp("Object obj1 - new Object()");
      Object obj1 = new Object();
      // showIdent(obj, "obj");
      showType(obj1, "obj");
      iterate(obj1);
      showOp("Object obj2 = obj1");
      Object obj2 = obj1;
      isSameObj(obj2, "obj2", obj1, "obj1");

      label("Svt Struct value type");
      showOp("Svt t4 = new Svt()");
      Svt t4 = new Svt();
      t4.I = 3;
      t4.D = 3.1415927;
      t4.C = 'z';
      t4.printSelf("t4");
      showType(t4, "t4");
      showNote("value type: size of object in stackframe");
      iterate(t4);

      showOp("Svt t4a = t4");
      Svt t4a = t4;
      t4a.printSelf("t4a");
      t4.printSelf("t4");
      isSameObj(t4a,"t4a",t4,"t4");
      showType(t4a, "t4a");

      showOp("t4a.C = 'q'");
      t4a.C = 'q';
      t4a.printSelf("t4a");
      t4.printSelf("t4");
      isSameObj(t4, "t4", t4a, "t4a");

      label("Crt - ref type with ref member");
      showOp("Crt t5 = new Crt()");
      Crt t5 = new Crt();
      t5.S = "SomeString";
      t5.printSelf("t5");
      showType(t5, "t5");
      showNote("ref type: size of handle to object in heap");

      showOp("Crt t5a = t5");
      Crt t5a = t5;
      t5a.printSelf("t5a");
      isSameObj(t5a, "t5a", t5, "t5");
      isSameObj(t5a.S, "t5a.S", t5.S, "t5.S");

      showOp("t5a.S = \"new string\"");
      t5a.S = "new string";
      t5a.printSelf("t5a");
      t5a.printSelf("t5");
      isSameObj(t5a, "t5a", t5, "t5");
      isSameObj(t5a.S, "t5a.S", t5.S, "t5.S");
      showNote("source t5 was altered!");
      Console.WriteLine("\nThat's all Folks!\n");
    }
    /*-- simple reflection --*/
    public static void showType<T>(T t, String s)
    {
      #pragma warning disable CS8602  // possibly null reference warning
      Type tt = t.GetType();
      Console.WriteLine("{0}, {1}",tt.Name, s);
      int size = Utils.GetManagedSize(tt);
      Console.WriteLine("value: {0}, size: {1}", t, size);
    }
    /*-- beware, two distinct objects may have same hashcode --*/
    public static void showIdent<T>(T t, String n) {
      int id = t.GetHashCode();
      Console.WriteLine("{0}, {1}", n, id);
    }
    public static void isSameObj<T>(T t1, String n1, T t2, String n2) {
      if(ReferenceEquals(t1, t2)) {
        Console.WriteLine("{0} is same object as {1}", n1, n2);
      }
      else {
        Console.WriteLine("{0} is not same object as {1}", n1, n2);
      }
    }
    public static void showOp(string op) {
      Console.WriteLine("--- {0} ---", op);
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
    public static void label(string s) {
      Console.WriteLine(
        "\n---------------\n  {0}\n---------------", s
      );
    }
    public static void showNote(string s) {
      Console.WriteLine("--- Note: ---\n  {0}", s);
    }
  }
  /*-- 
    Utils uses advanced relection 
    - This is function returns the size of value types and handles
      which is part of discovering how things work.
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
