/*-------------------------------------------------------------------
  Analysis.cs
  - provides several type analysis and display methods in the
    class Display
*/

using System.Reflection;
using System.Reflection.Emit;
using System.Collections;
using System.Collections.Generic;   // IEnumerable<T>, List<T>
using System.Linq;                  // IEnumerable<T>.ToArray
using System.Text;

/* - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    cut here

    The functions below are used for analysis and display. They
    have varying complexities. It is not essential to understand 
    them in order to follow the demonstrations in this demo.  
    We will look at them in some detail in Bits_Generic and
    Bits_Iter. 
*/
namespace Analysis {
  /*----------------------------------------------------------------------
      IShow allows analysis functions to operate on instances of any class 
      that implements it. See ShowTypeShowable<T>(T t, ...), below.
  */
  public interface IShow {
    void Show(string name);   // show instance state as rows of elements
    int Length { get; }       // total number of elements
    int Width { get; set; }   // number of elements per row
    int Left { get; set; }    // offset from terminal Left
    int Indent {get; set; }   // element indent
  }
      
  public class Display {
    /*-------------------------------------------------
      Show static type with some formatting adjustments
      and name at callsite.
    */
    public static String Spaces(int i) {
      return new String(' ', i);
    }
    public static void ShowType<T>(T t, string nm) {
      Type tt = t!.GetType();
      string tnm = tt.Name;
      if(tt.IsGenericType) {
        if(tnm.Length > 1) {
          tnm = tnm.Remove(tnm.Length - 2) + "<T>";
        }
      }
      Console.Write("{0}: {1}, ", nm, tnm);
      int size = GetManagedSize(tt);
      Console.Write("size: {0}, ", size);
      if (tt.IsValueType) {
        Console.WriteLine("value type");
      }
      else {
        Console.WriteLine("reference type");
      }
    }
    /*-------------------------------------------------
    Display information about the type of any scalar type.
    - scalar types are those with a single value like:
      int, double, string, ...
    - This function directly uses only simple reflection
    */
    public static void ShowTypeScalar<T>(
      T t, string nm, string suffix = ""
    )
    {
      ShowType(t, nm);
      Console.WriteLine("value: \"{0}\"{1}", t, suffix);
    }
    /*-------------------------------------------------
    Show type information for any type that implements
    the IShow interface
    */
    public static void ShowTypeShowable<T>(
      T t, string nm, string suffix=""
    )
    where T:IShow
    {
      ShowType(t, nm);
      Console.WriteLine("value:");
      t.Show(nm);     // guaranteed by IShow implementation
      Console.Write(suffix);
    }
    /*-------------------------------------------------
      In Cs_Generic we will show how to implement a
      show function for the enumerable types below.
    */
    public static void ShowIntArray(int[] arr, string suffix="")
    {
      Console.Write("[ {0}", arr[0]);
      for (int i = 1; i < arr.Length; i++)
      {
        Console.Write(", " + arr[i] + " ");
      }
      Console.Write("]\n" + suffix);
    }
    public static void ShowDoubleList(List<double> list, string suffix="")
    {
      Console.Write("[ {0}", list[0]);
      for (int i = 1; i < list.Count; i++)
      {
        Console.Write(", " + list[i] + " ");
      }
      Console.Write("]\n" + suffix);
    }
    public static void ShowDictionary(
      Dictionary<int, string> dict, string suffix=""
    )
    {
      Dictionary<int, string>.KeyCollection keyColl = dict.Keys;
      Console.Write("[ ");
      foreach(KeyValuePair<int, string> entry in dict)
      {
        Console.Write("{{{0},{1}}} ", entry.Key, entry.Value);
      }
      Console.Write("]\n" + suffix);
    }
    /*-------------------------------------------------
      Provides name of caller, nm, as label for IShow() information.
      - works for all Showable instances
    */
    public static void ShowLabeledObject<T>(
      T t, string nm
    ) where T:IShow {
      t.Show(nm);
    }
    /*-------------------------------------------------
      Provides name of caller, nm, as label for value.
      - works for all T with ToString.
    */
    public static void DisplayLabeledObject<T>(T t, string nm) {
      Console.WriteLine("{0}: {1}", nm, t!.ToString());
    }
    /*-------------------------------------------------
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
    /*-------------------------------------------------
      Beware, two distinct objects may have same hashcode.
      - Not used in this demo for that reason.
    */
    public static void showIdent<T>(
      T t, String n, string suffix = ""
    ) {
      int id = t!.GetHashCode();
      Console.WriteLine("{0}, {1}{2}", n, id, suffix);
    }
    /*-------------------------------------------------
      Display function call or operation to help turn 
      output data into information 
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
    /*-------------------------------------------------
      Emphasize text with borders
    */
    public static void ShowNote(string s, string suffix = "") {
      Console.WriteLine(
        "--------------------------------------------------"
      );
      Console.WriteLine("{0}", s);  
      Console.WriteLine(
        "--------------------------------------------------{0}", 
        suffix
      );
    }
    /*-------------------------------------------------
      Surround note with empty lines
    */
    public static void ShowLabel(string s) {
      Console.WriteLine();
      ShowNote(s);
      Console.WriteLine();
    }
    /*
      Show fields and methods for either reference or value types
      using reflection
    */
    // https://stackoverflow.com/questions/7613782/iterating-through-struct-members
    public static void iterate<T>(T t) /*where T:new()*/ {
      Console.WriteLine("fields:");
      foreach(
        var field in typeof(T).GetFields(
        BindingFlags.Instance | 
        BindingFlags.NonPublic | 
        BindingFlags.Public
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
    /*----------------------------------------------------------------
      Utils uses advanced relection 
      - GetMangedSize(Type type) is function that returns the size of 
      value types and handles, used to help discover how things work.
      It is placed here because it uses advanced techniques that
      will eventually be covered elsewhere in this site.  Knowing
      how it works is not essential for the things we are examining
      in this demo.
    */
    // https://stackoverflow.com/questions/8173239/c-getting-size-of-a-value-type-variable-at-runtime/8173293#8173293
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

