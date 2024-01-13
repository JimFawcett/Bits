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
    - That affects the way assignments work - see demo in Program.cs
  */
  public class Point4D : Analysis.IShow  // reference type with value type members
  { 
    public Point4D() {
      /* 
        doesn't need to do anything because its properties have
        default values
      */
    }
    public Point4D(double xarg, double yarg, double zarg) {
      x = xarg;
      y = yarg;
      z = zarg;
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
}