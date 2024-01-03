/*---------------------------------------------------------
  Cs_Data::AnalysisData.cs 
    - class Anal contains static functions that analyze
      types, identities, sizes, and addresses
    - class Display contains static functions that
      provide console displays of headings, notes,
      and operations.
---------------------------------------------------------*/

using System;
using System.Reflection;
using System.Reflection.Emit;
using System.Runtime.InteropServices;
using System.Text;
using System.Collections;
using System.Collections.Generic;

namespace Analysis
{
  using Dict = Dictionary<string,int>;
  using KVPair = System.Collections.Generic.KeyValuePair<string,int>;
  
  public interface IShow {
    void Show(string name);   // show instance state as rows of elements
    int Length { get; }       // total number of elements
    int Width { get; set; }   // number of elements per row
    int Left { get; set; }    // offset from terminal Left
    int Indent {get; set; }   // element indent
  }

  class Anal {
    
    /*-- simple reflection --*/
    public static void ShowType<T>(T t, String nm, String suffix = "")
    {
      #pragma warning disable CS8602  // possibly null reference warning
      Type tt = t.GetType();
      Console.WriteLine("{0}: Type: {1}", nm, tt.Name);
      int size = Anal.GetManagedSize(tt);
      Console.WriteLine("value: {0}\nsize: {1}{2}", t, size, suffix);
      #pragma warning restore CS8602
    }
    public static string GetTypeString<T>(T t, String nm, String suffix = "")
    {
      #pragma warning disable CS8602  // possibly null reference warning
      Type tt = t.GetType();
      string typeInfo = String.Format("{0}: Type: {1}\n", nm, tt.Name);
      int size = Anal.GetManagedSize(tt);
      string instanceInfo = String.Format("value: {0}\nsize: {1}{2}", t, size, suffix);
      return typeInfo + instanceInfo;
      #pragma warning restore CS8602
    }
    /*-- beware, two distinct objects may have same hashcode --*/
    public static void showIdent<T>(T t, String n, string suffix = "") {
      #pragma warning disable CS8602  // possibly null reference warning
      int id = t.GetHashCode();
      #pragma warning restore CS8602
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
    /*-- 
      uses advanced relection 
      - GetMangedSize(Type type) is function that returns the size of 
        value types and handles.
        It is used to help discover how things work.
        It is placed here because it uses advanced techniques that
        will eventually be covered elsewhere in this site.  Knowing
        how it works is not essential for the things we are examining
        in this demo.
    --*/
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

    #pragma warning disable 8500
    /*
      Suppresses warning about taking address of managed type.
      The pointer is used only to show the address of ptr
      as part of analysis of copy operations.
    */
    public static unsafe string ToStringAddress<T>(T* ptr) {
      if(ptr == null) {
        return "";
      }
      IntPtr addr = (IntPtr)ptr;
      string addrstr = string.Format("address: 0x" + addr.ToString("x"));
      return addrstr;
    }
    #pragma warning restore 8500
    // public static unsafe string ToStringAdddressFromHandle<T>(T t) {
    //   string addrstr = "for handle\n";
    //   try {
    //     GCHandle handle = GCHandle.Alloc(t, GCHandleType.Pinned);
    //     IntPtr address = handle.AddrOfPinnedObject();
    //     addrstr = "address: " + String.Format("0x" + address.ToString("x"));
    //     handle.Free();
    //     return addrstr + "\n";
    //   }
    //   catch {
    //     Console.WriteLine("GCHandle exception thrown");
    //   }
    //   return addrstr;
    // }
  }
  class Display
  {
    public static String Spaces(int i) {
      return new String(' ', i);
    }
    public static void ShowOp(String op, String suffix = "") {
      Console.WriteLine("--- {0} ---{1}", op, suffix);
    }
    /*-- more advanced reflection --*/
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
    /*-----------------------------------------------------
      Build string representation of array of type T
    -----------------------------------------------------*/
    public static string ToStringRepArray<T>(T[] arr) {
      StringBuilder sb = new StringBuilder();
      sb.Append("{ ");
      bool first = true;
      foreach(T item in arr) {
        if(item == null) {
          break;
        }
        if(first) {
          sb.Append(item.ToString());
          first = false;
        }
        else {
          sb.AppendFormat(", {0}", item);
        }
      }
      sb.Append(" }\n");
      return sb.ToString();
    }
    /*-----------------------------------------------------
      Build string representation of IEnumerable 
      collection T<U>. Works for array too.
    -----------------------------------------------------*/
    public static string ToStringRepIEnumerable<T,U>(T enu) 
      where T:IEnumerable<U>
    {
      StringBuilder sb = new StringBuilder();
      sb.Append("{ ");
      bool first = true;
      foreach(U item in enu) {
        if(item == null) {
          break;
        }
        if(first) {
          sb.Append(item.ToString());
          first = false;
        }
        else {
          sb.AppendFormat(", {0}", item);
        }
      }
      sb.Append(" }\n");
      return sb.ToString();
    }
    /*-----------------------------------------------------
      Direct implementation of enumerating associative
      collection.  Code below illustrates that this can also
      be done with ToStringRepIEnumerable<Dict,KVPair>(dict).
    -----------------------------------------------------*/
    public static string ToStringRepAssocCont<Dict,Key,Value>(Dict assoc) 
      where Dict:IDictionary<Key,Value>
    {
      StringBuilder sb = new StringBuilder();
      sb.Append("{ ");
      bool first = true;
      foreach(var item in assoc) {
        if(first) {
          var sf = String.Format("{{ {0}, {1} }}", item.Key, item.Value);
          sb.Append(sf);
          first = false;
        }
        else {
          sb.AppendFormat(", {{ {0}, {1} }}", item.Key, item.Value);
        }
      }
      sb.Append(" }\n");
      return sb.ToString();
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
      int size = Anal.GetManagedSize(tt);
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
      Surround note with empty lines
    */
    public static void ShowLabel(string s, string suffix="", int len=35) {
      Console.WriteLine();
      ShowNote(s, suffix, len);
      Console.WriteLine();
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
    public static void print(String s = "") {
      Console.Write(s);
    }
    public static void println(String s = "") {
      Console.WriteLine(s);
    }
  }
}
