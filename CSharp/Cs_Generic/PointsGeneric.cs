/*-------------------------------------------------------------------
  Points.cs
  - provides definitions for user-defined class PointN<T>
*/

using System.Collections;
using System.Collections.Generic;   // IEnumerable<T>, List<T>
using Analysis;

namespace Points {
  /*----------------------------------------------------------------------
    Point<T> represents a point in N-dimensional space 
     - C# generics do not accept integers as generic parameters
       so we cannot declare P<T,N> where N is an integer like
       we did for C++ and Rust
     - Point holds any finite number of generic coordinates
     - coordinates are held in a List<T>
     - implements IEnumerable<T> so it can be indexed and iterated
     - it is a reference type because it is implemented with a class.
       its List<T> is also a reference type
  */
  public class Point<T> : IEnumerable<T>, Analysis.IShow
  {
    /*--------------------------------------------------------------------
      Constructs an empty point
    */
    public Point() {
      coor = new List<T>();
      // _N initializes to 0
    }
    /*--------------------------------------------------------------------
      Constructs a point with N coordinates each with default value
    */
    public Point(int N) {
      coor = new List<T>();
      for(int i = 0; i<N; ++i) {
        T? test = default(T);
        if(test != null) {
          coor.Add(test);
        }
      }
      _N = coor.Count;
    }
    /* translates IShow::show() for needs of Point class */
    public void Show(string name, int _width = 5, int _left = 2) {
      int SaveWidth = Width;
      int SaveLeft = Left;
      Width = _width;
      Left = _left;
      PrintSelf(name);
      Width = SaveWidth;
      Left = SaveLeft;
    }
    /*
      Displays structure and state of N-dimensional point.
      - state is a set of rows of coordinate data
      - property Width specifies number of elements in each row
      - property Left specifies offset of row from terminal Left edge
    */
    public void PrintSelf(string name) {
      Console.Write(Display.Indent(Left));
      Console.Write("{0} {{\n{1}", name, Display.Indent(Left + 2));
      for(int i=0; i<coor.Count; ++i) {
        Console.Write("{0}", coor[i]);
        if(i < coor.Count - 1) {
          Display.print(", ");
        }
        if((i+1) % Width == 0 && i != coor.Count - 1) {
          Console.Write("\n");
          Console.Write(Display.Indent(Left + Indent));
        }
      }
      Console.Write("\n{0}", Display.Indent(Left));
      Console.WriteLine("}");
      String callname = "  " + dt;
      Console.WriteLine(callname);
    }
    /* The three functions below support indexing and iterating */
    public T this[int index] {
      get { return coor[index]; }
      set { coor.Insert(index, value); }
    }
    public IEnumerator<T> GetEnumerator() {
      return coor.GetEnumerator();
    }
    IEnumerator IEnumerable.GetEnumerator() {
      return this.GetEnumerator();
    }
    /* supports inializer list */
    public void Add(T t) {
      coor.Add(t);
      _N = coor.Count;
    }
    public void AddRange(IEnumerable<T> list) {
      coor.AddRange(list);
      _N = coor.Count;
    }
    private int _N = 0;
    public int Count {
      get { return _N; }
    }
    public List<T> coor { get; set; }
    public DateTime dt { get; set; } = DateTime.Now; 
    public int Length { get { return coor.Count; } }
    public int Width { get; set; } = 5;   // default row size
    public int Left { get; set; } = 2;    // default offset
    public int Indent { get; set; } = 2;
    /* initializers */
    public Point<T> width(int w) {
      this.Width = w;
      return this;
    } 
    public Point<T> left(int l) {
      this.Left = l;
      return this;
    } 
    public Point<T> indent(int i) {
      this.Indent = i;
      return this;
    } 
  }
  /* experiment */
  public struct Dim { public static int N { get; set; } = 0; }
  public class PointN<T, N> : IEnumerable<T>
    where T : new() 
  {
    private int _N = 0;
    public int Count { 
      get { return _N; } 
      private set {
        _N = value;
      }
    }
    public List<T> coor { get; private set; }
    public PointN(int N) {
      coor = new List<T>();
      foreach (var i in Enumerable.Range(0, N)) {
        coor.Add(new T());
      }
      Count = coor.Count;
    }
    public void SetRange(IEnumerable<T> rng) {
      List<T> coll = rng.ToList();
      foreach (var i in Enumerable.Range(0, Count)) {
        if (i < coll.Count()) {
          coor[i] = coll[i];
        }
      }
    }
    /* The three functions below support indexing and iterating */
    public T this[int index] {
      get { return coor[index]; }
      set { coor.Insert(index, value); }
    }
    public IEnumerator<T> GetEnumerator() {
      return coor.GetEnumerator();
    }
    IEnumerator IEnumerable.GetEnumerator() {
      return this.GetEnumerator();
    }
  }
}