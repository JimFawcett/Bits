#----------------------------------------------------------
# Py_Generic.py
#
# Python type hints and generics
#   - Demonstrates generic and type hint syntax
#   - Shows how to build user-defined generic type
#   - Type hints are ignored by the Python interpreter
#   - To check for type errors use tool mypy, e.g.:
#
#         mypy Py_Generic.py --check-untyped-defs
#
#     mypy requires installation using pip
#----------------------------------------------------------
# Python std types
#  list, tuple, range, dict, set,  
import copy
import AnalysisGen
import PointsGen
# from array import array

# short-hand identifiers
Anal = AnalysisGen
Points = PointsGen

#-- demonstrate std collection types with generic hints

def demo_std_generics():
  Anal.showNote("  demo std generics")
  print()

  Anal.showOp("list[int]")
  l1: list[int] = [1, 2, 3, 2, 1]
  print(type(l1))
  print(l1)
  print()

  Anal.showOp("list[str]")
  l2: list[str] = ["you", "me", "them", "us"]
  print(type(l2))
  print(l2)
  print()

  Anal.showOp("tuple(int, float, str)")
  t: tuple[int, float, str] = (42, -0.5, "forty two")
  print(type(t))
  print(t)
  print()

  Anal.showOp("dict[str, int]")
  d: dict[str, int] = {
      "zero":0, "one":1, "two":2, "three":3
  }
  print(type(d))
  print(d)
  print()

#-- user defined Demo type --#

from typing import TypeVar, Generic
T = TypeVar('T', bound=int | float)

class Demo(Generic[T]):
    
  # supports constructor notation
  def __init__(self, n: int) -> None:
      self.datum:T = n
  
  def getvalue(self) -> T:
      return self.datum

  def setvalue(self, val:T):
      self.datum = val

  # show named value of PointN[T] instance
  def show(self, name, left = 0) :
      print(AnalysisGen.indent(left), sep='')
      print(name, ": ", "Demo { ", self.datum, " }", sep='')
      # print(AnalysisGen.indent(left), name, ": ", self.datum, sep='')
      print()

#-- demonstrate user-defined generic types --#

def demo_user_defined_generics():
    Anal.showNote("  demo user-defined generics")
    print()

    Anal.showOp("Demo[int]")
    d: Demo[int] = Demo(42)
    print(type(d))
    d.show("d", 2)

    Anal.showOp("p1 = Points.PointN[float](5)")
    p1: Points.PointN = Points.PointN[float](5)
    p1[1] = 1.5
    p1[3] = -2.0
    index = p1.len() - 1
    p1[index] = 42
    print(type(p1))
    p1.show("p1")
    print()

def demo_generic_functions():

  Anal.showNote("demo generic functions", "\n")

  Anal.showOp("Anal.showType(list[int], name)")
  l:list[int] = [1, 2, 3, 2, 1]
  Anal.showType(l, "l")
  print()

  Anal.showOp("showTypeEnum(list[int], name)")
  Anal.showTypeEnum(l, "l", left = 2, width = 7, suffix = "")
  print() 

#-- demonstration starts here --#

def execute() : 
    Anal.showNote("  Demonstrate Python Generics")

    Anal.showNote(   
        "  Python introduced generics and type hints\n"\
        "  to check for type errors using mypy, a static\n"\
        "  type checking tool."
    )
    print()

    demo_std_generics()
    demo_user_defined_generics()
    demo_generic_functions()

    print("\nThat's all folks!\n")

execute()

