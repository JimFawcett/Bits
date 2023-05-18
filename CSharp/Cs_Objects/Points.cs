/*-------------------------------------------------------------------
  Points.cs
  - provides definitions for user-defined class Point4D
*/

using System.Collections;
using System.Collections.Generic;   // IEnumerable<T>, List<T>
using Analysis;

namespace Points {
  /*-----------------------------------------------------------------------
    Point4D is a basic point class with three integer coordinates and one
    time coordinate.
    - It is a reference type because it's instances are created
      from a class
    - That affects the way assignments work - see demo near the
      end of main.
  */
  public class Point4D : Analysis.IShow  // reference type with value type members
  { 
    public Point4D() {
      /* 
        doesn't need to do anything because its properties have
        default values
      */
    }
    public double x { get; set; } = 0.0;
    public double y { get; set; } = 0.0;
    public double z { get; set; } = 0.0;
    public DateTime dt { get; set; } = DateTime.Now; 
    public int Length { get; } = 4;
    public int Width { get; set; } = 3;
    public int Left { get; set; } = 2;
    public int Indent { get; set; } = 2;
    /* translates IShow::show to needs of class */
    public void Show(string name) {
      PrintSelf(name);
    }
    /* Display Point4D structure and state */
    public void PrintSelf(string name) {
      string ofstr = Display.Spaces(Left);
      string instr = Display.Spaces(Indent);
      
      Console.WriteLine("{0}{1} {{", ofstr, name);
      Console.WriteLine(
        "{5}x:{1}, y:{2}, z:{3},\n{5}{4}", name, x, y, z, dt, ofstr + instr
      );
      Console.WriteLine("{0}}}\n", ofstr);
    }
  }
  // /*----------------------------------------------------------------------
  //   Point2<T> is a generalization of Point4D 
  //    - holds any finite number of generic coordinates
  //    - coordinates are held in a List<T>
  //    - implements IEnumerable<T> so it can be indexed and iterated
  //    - it is a reference type because it is implemented with a class.
  //      its List<T> is also a reference type
  // */
  // public class Point2<T> : IEnumerable<T> , Analysis.IShow {
  //   /*--------------------------------------------------------------------
  //     Constructs a point with N coordinates each with default value
  //   */
  //   public Point2(int N) {
  //     coor = new List<T>();
  //     for(int i = 0; i<N; ++i) {
  //       T? test = default(T);
  //       if(test != null) {
  //         coor.Add(test);
  //       }
  //     }
  //   }
  //   /* translates IShow::show() for needs of Point2 class */
  //   public void Show(string name) {
  //     PrintSelf(name);
  //   }
  //   /*
  //     Displays structure and state of N-dimensional point.
  //     - state is a set of rows of coordinate data
  //     - property Width specifies number of elements in each row
  //     - property Left specifies offset of row from terminal Left edge
  //   */
  //   public void PrintSelf(string name) {
  //     Console.Write(Display.Indent(Left));
  //     Console.Write("{0} {{\n{1}", name, Display.Indent(Left + 2));
  //     for(int i=0; i<coor.Count; ++i) {
  //       Console.Write("{0}", coor[i]);
  //       if(i < coor.Count - 1) {
  //         Display.print(", ");
  //       }
  //       if((i+1) % Width == 0 && i != coor.Count - 1) {
  //         Console.Write("\n");
  //         Console.Write(Display.Indent(Left + Indent));
  //       }
  //     }
  //     Console.Write("\n{0}", Display.Indent(Left));
  //     Console.WriteLine("}");
  //   }
  //   /* The three functions below support indexing and iterating */
  //   public T this[int index] {
  //     get { return coor[index]; }
  //     set { coor.Insert(index, value); }
  //   }
  //   public IEnumerator<T> GetEnumerator() {
  //     return coor.GetEnumerator();
  //   }
  //   IEnumerator IEnumerable.GetEnumerator() {
  //     return this.GetEnumerator();
  //   }
  //   public List<T> coor { get; set; }
  //   public int Length { get { return coor.Count; } }
  //   public int Width { get; set; } = 5;   // default row size
  //   public int Left { get; set; } = 2;    // default offset
  //   public int Indent { get; set; } = 2;
  //   /* initializers */
  //   public Point2<T> width(int w) {
  //     this.Width = w;
  //     return this;
  //   } 
  //   public Point2<T> left(int l) {
  //     this.Left = l;
  //     return this;
  //   } 
  //   public Point2<T> indent(int i) {
  //     this.Indent = i;
  //     return this;
  //   } 
  // }

}