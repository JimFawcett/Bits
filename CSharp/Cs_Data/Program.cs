using System;
#pragma warning disable CS8602  // possibly null reference warning
// https://stackoverflow.com/questions/8173239/c-getting-size-of-a-value-type-variable-at-runtime/8173293#8173293
// 
using System;
using System.Reflection;
using System.Reflection.Emit;

namespace HelloData
{
  /*-- Utils uses advanced relection --*/
  // https://stackoverflow.com/questions/8173239/c-getting-size-of-a-value-type-variable-at-runtime/8173293#8173293
  public static class Utils {
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
  /*-- type used for type demo example --*/
  public struct T4 
  { 
    public T4() {}
    public int I { get; set; } = 0;
    public double D { get; set; } = 1.0;
    public char C { get; set; }  = 'a';
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
    public void showIdent<T>(T t, String n) {
      int id = t.GetHashCode();
      Console.WriteLine("{0}, {1}", n, id);
    }
    /*-- more advanced reflection --*/
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
    static void Main(string[] args)
    {
      Console.WriteLine(" Demonstrate C# types");
      Console.WriteLine("----------------------");
      Program p = new Program();
      double t1 = 3.5;
      p.showIdent(t1, "t1");
      p.showType(t1, "t1");
      p.iterate(t1);
      
      // reference behavior - copy on write
      separator();
      String t2 = "a string";
      p.showIdent(t2, "t2");
      p.showType(t2, "t2");
      
      print("\nt3 = t2\n");
      String t3 = t2;
      p.showIdent(t3, "t3");
      p.showType(t3, "t3");
      
      print("\nt3 = \"another string\"\n");
      t3 = "another string";
      p.showIdent(t2, "t2");
      p.showIdent(t3, "t3");
      p.showType(t3, "t3");

      separator();
      Object obj = new Object();
      p.showIdent(obj, "obj");
      p.showType(obj, "obj");
      p.iterate(obj);
      print("");
      T4 t4 = new T4();
      t4.I = 3;
      t4.D = 3.1415927;
      t4.C = 'z';
      //p.t4 = t4;
      p.showIdent(t4, "t4");
      p.showType(t4, "t4");
      p.iterate(t4);

      Console.WriteLine("\nThat's all Folks!\n");
    }
  }
}
