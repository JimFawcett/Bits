/*-------------------------------------------------------------------
  Points.cs
  - provides definitions for user-defined class BasicPoint<T>
*/

using System.Collections;
using System.Collections.Generic;   // IEnumerable<T>, List<T>
using Analysis;

namespace Points {

  /*----------------------------------------------------------------------
    BasicPoint<T>
     - holds any finite number of generic coordinates
     - coordinates are held in a List<T>
     - implements IEnumerable<T> so it can be indexed and iterated
     - does not implement ICollection<T> or IList<T> to avoid making
       demonstration too complicated for learning example
     - it is a reference type because it is implemented with a class.
       its List<T> is also a reference type
  */
  public class BasicPoint<T> : IEnumerable<T>, Analysis.IShow {
    /*--------------------------------------------------------------------
      Constructs a BasicPoint with N coordinates each with default value
    */
    public BasicPoint(int N) {
      coor = new List<T>();
      for(int i = 0; i<N; ++i) {
        T? test = default(T);
        if(test != null) {
          coor.Add(test);
        }
      }
    }
    /*--------------------------------------------------------------------
      Supports building BasicPoint by Adding elements after construction
    */
    public BasicPoint() {
      coor = new List<T>();
    }
    /*-----------------------------------------------------
      Add element to back of coordinate list
      - supports using list initializer, e.g.
        var p = new BasicPoint<int> { 1, 2. 3 }
    */
    public void Add(T t) {
      coor.Add(t);
    }
    /* translates IShow::show() for needs of BasicPoint class */
    public void Show(string name) {
      PrintSelf(name);
    }
    /*
      Displays structure and state of N-dimensional BasicPoint.
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
    }
    /* The functions below support indexing and iterating */
    public T this[int index] {
      get { return coor[index]; }
      set { coor.Insert(index, value); }
    }
    /*-- returns coor enumerator --*/
    public IEnumerator<T> GetEnumerator() {
      return coor.GetEnumerator();
    }
    /*-- returns BasicPoint<T> enumerator --*/
    IEnumerator IEnumerable.GetEnumerator() {
      return this.GetEnumerator();
    }
    /*-- returns BasicPoint<T> iterator --*/
    public IEnumerable<T> iter() {
      foreach (var item in coor) {
        yield return item;
      }
    }
    public List<T> coor { get; set; }
    public int Length { get { return coor.Count; } }
    public int Width { get; set; } = 5;   // default row size
    public int Left { get; set; } = 2;    // default offset
    public int Indent { get; set; } = 2;
    /* initializers */
    public BasicPoint<T> width(int w) {
      this.Width = w;
      return this;
    } 
    public BasicPoint<T> left(int l) {
      this.Left = l;
      return this;
    } 
    public BasicPoint<T> indent(int i) {
      this.Indent = i;
      return this;
    } 
  }
  /*-------------------------------------------------------
    Point<T>
    - same as BasicPoint<T> with additions:
      - implements ICollection<T> and IList<T> interfaces
      - single inheritance of BasicPoint<T>'s implementation
      - Deriving from ICollecton<T> and IList<T> supports using
        generic functions for analysis and display.
  */
  public class Point<T> : 
    BasicPoint<T>, ICollection<T>, IList<T> 
  {
    public Point(int N) : base(N) { }
    public Point() : base() {}
    public void Clear() { 
      base.coor.Clear(); 
    }
    public bool Contains(T item) { 
      return base.coor.Contains(item); 
    }
    public void CopyTo(T[] array, int i) {
      base.coor.CopyTo(array, i);
    }
    public bool Remove(T item) {
      return base.coor.Remove(item);
    }
    public int Count { 
      get { return base.coor.Count; } 
    }
    public bool IsReadOnly {
      get { return false; }
    }
    public int IndexOf(T item) {
      return base.coor.IndexOf(item);
    }
    public void Insert(int i, T item) {
      base.coor.Insert(i, item);
    }
    public void RemoveAt(int index) {
      base.coor.RemoveAt(index);
    }
  }
}