/* Cs_Data::Cs_Data.cs */
/*
  Static Data Types:
    sbyte, byte, short, int, long, ushort, uint, ulong
    float, double
    decimal, char, bool
    object, string
    pointer
*/
#pragma warning disable CS8602  // possibly null reference warning
// https://stackoverflow.com/questions/8173239/c-getting-size-of-a-value-type-variable-at-runtime/8173293#8173293
// 
using System;
using System.Reflection;
using System.Reflection.Emit;

namespace HelloData
{
  /*-- 
    T4 is a value type used for type demo example.
    It consists of three public properties.
  --*/
  public struct T4   // value type
  { 
    public T4() {}
    public int I { get; set; } = 0;
    public double D { get; set; } = 1.0;
    public char C { get; set; }  = 'a';
    public void printSelf(string name) {
      Console.Write("{0}  {{ I:{1}, D:{2}, C:{3} }}\n", name, I, D, C);
    }
  }
  /*-- 
    T4 is a value type used for type demo example.
    It consists of three public properties.
  --*/
  public struct T5   // value type with reference type member
  { 
    public T5() {}
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
    /*-- simple reflection --*/
    public void showType<T>(T t, String s)
    {
      Type tt = t.GetType();
      Console.WriteLine("{0}, {1}",tt.Name, s);
      int size = Utils.GetManagedSize(tt);
      Console.WriteLine("value: {0}, size: {1}", t, size);
    }
    /*-- beware, two distinct objects may have same hashcode --*/
    public void showIdent<T>(T t, String n) {
      int id = t.GetHashCode();
      Console.WriteLine("{0}, {1}", n, id);
    }
    public void isSameObj<T>(T t1, String n1, T t2, String n2) {
      if(ReferenceEquals(t1, t2)) {
        Console.WriteLine("{0} is same object as {1}", n1, n2);
      }
      else {
        Console.WriteLine("{0} is not same object as {1}", n1, n2);
      }
    }
    // https://stackoverflow.com/questions/7613782/iterating-through-struct-members
    public void iterate<T>(T t) where T:new() {
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
    public static void print(String s) {
      Console.WriteLine(s);
    }
    public static void separator() {
      Console.WriteLine("----------");
    }
    /*-- demonstration begins here --*/
    static void Main(string[] args)
    {
      Console.WriteLine(" Demonstrate C# types");
      Console.WriteLine("----------------------");
      print("   Double value type");
      print("----------");
      Program p = new Program();
      print("--- double t1 = 3.5 ---");
      double t1 = 3.5;
      //p.showIdent(t1, "t1");
      p.showType(t1, "t1");
      p.iterate(t1);
      print("--- double t1a = t1 ---");
      double t1a = t1;
      p.isSameObj(t1a, "t1a", t1, "t1");

      // reference behavior - copy on write
      print("----------");
      print("   String reference type:");
      print("----------");
      String t2 = "a string";
      //p.showIdent(t2, "t2");
      p.showType(t2, "t2");
      
      print("--- String t3 = t2 ---");
      String t3 = t2;
      //p.showIdent(t3, "t3");
      p.showType(t3, "t3");
      p.isSameObj(t3,"t3",t2,"t2");

      print("--- t3 += \", ok\" ---");
      t3 += ", ok";
      p.showType(t3, "t3");
      p.isSameObj(t3,"t3",t2,"t2");
      
      
      print("--- t3 = \"another string\" ---");
      t3 = "another string";
      p.isSameObj(t3,"t3",t2,"t2");

      //   
      print("----------");
      print("   Object obj1 = new Object()");
      print("----------");
      Object obj1 = new Object();
      // p.showIdent(obj, "obj");
      p.showType(obj1, "obj");
      p.iterate(obj1);
      print("--- Object obj2 = obj1 ---");
      Object obj2 = obj1;
      p.isSameObj(obj2, "obj2", obj1, "obj1");

      print("----------");
      print("   T4 t4 - new T4()");
      print("----------");
      T4 t4 = new T4();
      t4.I = 3;
      t4.D = 3.1415927;
      t4.C = 'z';
      t4.printSelf("t4");
      // p.showIdent(t4, "t4");
      p.showType(t4, "t4");
      p.iterate(t4);

      print("--- t4a = t4 ---");
      T4 t4a = t4;
      t4a.printSelf("t4a");
      t4.printSelf("t4");
      p.isSameObj(t4a,"t4a",t4,"t4");
      p.showType(t4a, "t4a");

      print("--- t4a.C = 'q' ---");
      t4a.C = 'q';
      t4a.printSelf("t4a");
      // p.showIdent(t4a, "t4a");
      t4.printSelf("t4");
      p.isSameObj(t4, "t4", t4a, "t4a");

      print("----------");
      print("   T5 t5 - new T5()");
      print("----------");
      T5 t5 = new T5();
      t5.S = "SomeString";
      t5.printSelf("t5");
      p.showType(t5, "t5");

      print("--- T t5a = t5 ---");
      T5 t5a = t5;
      t5a.printSelf("t5a");
      p.isSameObj(t5a, "t5a", t5, "t5");
      p.isSameObj(t5a.S, "t5a.S", t5.S, "t5.S");

      Console.WriteLine("\nThat's all Folks!\n");
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
