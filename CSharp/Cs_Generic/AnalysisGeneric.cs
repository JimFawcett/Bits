/*-------------------------------------------------------------------
  Analysis.cs
  - provides several type analysis and display methods in the
    class Display
*/
using System;
using System.Collections;
using System.Collections.Generic;     // IEnumerable<T>, List<T>
using System.Linq;                    // IEnumerable<T>.ToArray
using System.Text;                    // StringBuilder
using System.Reflection;
using System.Reflection.Emit;
using System.Runtime.InteropServices; //GCHandle

namespace Analysis {
    /*----------------------------------------------------------
        IShow interface allows analysis functions to operate on 
        instances of any class that implements it. 
        - See ShowTypeShowable<T>(T t, ...), below.
    */
    public interface IShow {
        void Show(string name, int Width = 5, int Left = 2);   // show instance state as rows of elements
        int Length { get; }       // total number of elements
        int Width { get; set; }   // number of elements per row
        int Left { get; set; }    // offset from terminal Left
        int Indent {get; set; }   // element indent from braces
    }
    /*-----------------------------------------------------------------------
      Collection of static functions that present information about types
      and data values on the terminal line.
    */          
    public class Display {
        public static String Spaces(int i) {
            return new String(' ', i);
        }
        public static bool IsAssociativeColl(IEnumerable coll) {
          return coll is IDictionary;
        }
        /*-------------------------------------------------
          Show static type with some formatting adjustments
          and name at callsite.
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
        IEnumerable<T> interface.
        */
        public static void ShowTypeEnum<T> (
        IEnumerable<T> t, string nm, int w = 5, string suffix = ""
        )
        {
            Type tt = t!.GetType();
            string tnm = tt.Name;
            if(tt.IsGenericType) {
                if(tnm.Length > 1) {
                    tnm = tnm.Remove(tnm.Length - 2);
                    if(IsAssociativeColl(t)) {
                      tnm += "<K,V>";
                    }
                    else {
                      tnm += "<T>";
                    }
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
        Provides name of caller, nm, as label for IShow() information.
        - works for all Showable instances
        */
        public static void ShowLabeledObject<T>(
            T t, string nm
        ) where T:IShow {
            // Console.Write(nm);
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
        create string of count spaces, used to offset output 
        */
        public static string Indent(int count) {
            StringBuilder sb = new StringBuilder();
            sb.Append(' ', count);
            return sb.ToString();
        }
        /*-------------------------------------------------
        Truncate string to length of N, but only if
        its length is greater than N
        */
        public static string truncate(int N, string bigStr) {
            if(bigStr.Length <= N) {
                return bigStr;
            }
            StringBuilder sb =  new StringBuilder();
            sb.Append(bigStr);
            sb.Length = N;  // move back pointer to desired length
            return sb.ToString();
        }
        /*------------------------------------------------- 
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
        public static void Println(String s) {
            Console.WriteLine(s);
        }
        /*-------------------------------------------------
          Emphasize text with borders
        */
        public static void ShowNote(string s, int n=60, string suffix = "") {
            string line = new string('-', n);
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
        /*-----------------------------------------------------------
          All methods below this are not part of this presentation
          - may use advanced code techniques
          - may be experimental
          - may simply be alternate designs that could be
            useful in other applications
        -----------------------------------------------------------*/
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
      sb.Append("[ ");
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
      sb.Append(" ]\n");
      return sb.ToString();
    }
    /*-----------------------------------------------------
      Direct implementation of enumerating associative
      collection.  This can also be done with
      ToStringRepIEnumerable<Dict,KVPair>(dict).
    -----------------------------------------------------*/
    public static string ToStringRepAssocCont<Dict,Key,Value>(Dict assoc) 
      where Dict:IDictionary<Key,Value>
    {
      StringBuilder sb = new StringBuilder();
      sb.Append("{ ");
      bool first = true;
      foreach(var item in assoc) {
        if(first) {
          var sf = String.Format("{{{0}, {1}}}", item.Key, item.Value);
          sb.Append(sf);
          first = false;
        }
        else {
          sb.AppendFormat(", {{{0}, {1}}}", item.Key, item.Value);
        }
      }
      sb.Append(" }\n");
      return sb.ToString();
    }
    /*-- move this to functions --*/
    static void DemoPassValAndRef() {
      Display.ShowNote("Pass by value", 60, "\n");
      double d = 3.1415927;
      Pass_by_value<double>(d, "d");
      TestForNullValue(d, "d");

      List<int> li = new List<int>{ 1, 2, 3, 2, 1 };
      Pass_by_value<List<int>>(li, "li");
      TestForNullValue(li, "li");
    }
    public static string GetTypeString<T>(T t, String nm, String suffix = "")
    {
      /*-- t! asserts that t is not null --*/
      Type tt = t!.GetType();
      string typeInfo = String.Format("{0}: Type: {1}\n", nm, tt.Name);
      int size = GetManagedSize(tt);
      string instanceInfo = String.Format("value: {0}\nsize: {1}{2}", t, size, suffix);
      return typeInfo + instanceInfo;
    }
    static unsafe void Pass_by_value<T>(T? t, string nm) {
      string ts = GetTypeString(t, nm);
      Console.WriteLine(ts);
      /*
        Suppresses warning about taking address of managed type.
        The pointer is used only to show the address of ptr
        as part of analysis of copy operations.
      */
      #pragma warning disable 8500
      string addrd = ToStringAddress<T>(&t);
      #pragma warning restore 8500
      Console.WriteLine("{0}: {1}", nm, addrd);
      t = default(T);
      /*
        caller sees this change if and only if T is a reference type
        in which case t is null.
      */
    }
    #pragma warning disable 8500
    public static unsafe string ToStringAddress<T>(T* ptr) {
      if(ptr == null) {
        return "";
      }
      IntPtr addr = (IntPtr)ptr;
      string addrstr = string.Format("address: 0x" + addr.ToString("x"));
      return addrstr;
    }
    #pragma warning restore 8500
    /*-- extract address of reference instance in managed heap --*/
    public static string ToStringAddressFromHandle<T>(T t) {
      string addrstr = "for handle\n";
      try {
        GCHandle handle = GCHandle.Alloc(t, GCHandleType.Pinned);
        IntPtr address = handle.AddrOfPinnedObject();
        addrstr = "address: " + String.Format("0x" + address.ToString("x"));
        handle.Free();
        return addrstr + "\n";
      }
      catch {
        Console.WriteLine("GCHandle exception thrown");
      }
      return addrstr;
    }
    static void TestForNullValue<T>(T? t, string nm) {
      if(t == null) {
        Console.WriteLine(nm + " is null");
      }
      else {
        Console.WriteLine(nm + " is {0}", t);
      }
    }
    static void DemoPrimitives() {
      Display.ShowNote(
        "Examples of creation and display of Primitive Types",
        60
      );
      short s = 123;
      Display.ShowTypeScalar(s, "s", "\n");
      long l = 12345;
      Display.ShowTypeScalar(l, "l", "\n");
      float f = 3.1415927f;
      Display.ShowTypeScalar(f, "f", "\n");
      double d = 3.1415927;
      Display.ShowTypeScalar(d, "d", "\n");
      int[] arr = new int[]{ 4, 3, 2, 1, 0, -1};
      Display.ShowTypeScalar(arr, "arr");
    }
  /*-------------------------------------------------
        Build string of comma separated values from 
        Enumerable collection
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
        /*-------------------------------------------------
        Returns value of T for IEnumerable<T> at runtime.  
        Needed for some functions that operate on generic 
        collections.
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
        /*----------------------------------------------------------------
            This method uses advanced relection 

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

