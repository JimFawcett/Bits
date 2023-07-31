/*-------------------------------------------------------------------
  Stats.cs
  - provides definitions for user-defined class Stats<T>
*/

using System.Collections;
using System.Collections.Generic;   // IEnumerable<T>, List<T>, ...
using Analysis;

namespace Stats {
  /*----------------------------------------------------------------------
    Stats<T> is a generic demo type 
     - holds any finite number of generic values
     - computes several statistics on its values
  */
  public class Stats<T> 
    where T: IComparable, IComparable<T>, IEquatable<T>, IConvertible
  {
    /*--------------------------------------------------------------------
      Constructs a point with N coordinates each with default value
    */
    public List<T> items { get; set; }
    public Stats(List<T> l) {
      items = l;
    }
    public void check() {
      if (items == null) {
        throw new SystemException("stats is null");
      }
      else if (items.Count < 1)
        throw new SystemException("stats is empty");
    }
    public int size() {
      check();
      return items.Count; 
    }
    public T max() {
      check();
      T max = items[0];
      foreach(T item in items) {
        if(max.CompareTo(item) < 0)
          max = item;
      }
      return max;
    }
    public T min() {
      check();
      T min = items[0];
      foreach(T item in items) {
        if(min.CompareTo(item) > 0)
          min = item;
      }
      return min;
    }
    public T? sum() {
      check();
      dynamic? sum = default(T);
      if(sum != null) {
        foreach(T item in items) {
          sum += item;
        }
        return sum;
      }
      return default(T);
    }
    public T avg() {
      check();
      dynamic? sum = this.sum();
      return sum/items.Count;
    }
  }
}
