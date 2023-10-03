using System;
using System.Reflection;
using System.Reflection.Emit;

/* Cs_Data::AnalysisData.cs */

namespace Analysis
{
  class Display
  {
    /*-- simple reflection --*/
    public static void ShowType<T>(T t, String nm, String suffix = "")
    {
      #pragma warning disable CS8602  // possibly null reference warning
      Type tt = t.GetType();
      Console.WriteLine("{0}: Type: {1}", nm, tt.Name);
      int size = Utils.GetManagedSize(tt);
      Console.WriteLine("value: {0}\nsize: {1}{2}", t, size, suffix);
    }
    /*-- beware, two distinct objects may have same hashcode --*/
    public static void showIdent<T>(T t, String n, string suffix = "") {
      int id = t.GetHashCode();
      Console.WriteLine("{0}, {1}{2}", n, id, suffix);
    }
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
    public static void ShowOp(String op, String suffix = "") {
      Console.WriteLine("--- {0} ---{1}", op, suffix);
    }
    // https://stackoverflow.com/questions/7613782/iterating-through-struct-members
    public static void Iterate<T>(T t) /*where T:new()*/ {
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
    public static void Print(String s = "") {
      Console.WriteLine(s);
    }
    public static void ShowNote(string s, string suffix = "", int length = 35) {
      string line = new string('-', length);
      Console.WriteLine(line);
      Console.WriteLine("  {0}", s);  
      Console.WriteLine(line + suffix);
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
