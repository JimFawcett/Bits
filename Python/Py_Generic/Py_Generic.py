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

#-- user defined Hello type --#

from typing import TypeVar, Generic
T = TypeVar('T', bound=int | float)

class Hello(Generic[T]):
    
  # supports constructor notation
  def __init__(self, t: T = 0) -> None:
      self._datum:T = t
  
  # property supports public member syntax while
  # keeping _datum encapsulated, e.g., not in
  # public interface

  @property
  def value(self):
     return self._datum
  
  @value.setter
  def value(self, t:T):
      self._datum = t

  @value.getter
  def value(self):
      return self._datum

  # show named value of Point[T] instance. 
  # Uses default value for left.
  
  def show(self, name, left = 0) :
      print(AnalysisGen.indent(left), sep='', end='')
      print(name, ": ", "Hello { ", self._datum, " }", sep='')

#-- demonstrate user-defined generic types --#

def demo_user_defined_generics():
    Anal.showNote("  demo user-defined generics")
    print()

    Anal.showOp("Hello(42)")
    h1: Hello[int] = Hello(42)
    print(" ", type(h1))
    h1.show("h1", 2)
    print()

    Anal.showOp("Hello() uses default argument")
    h2: Hello[float] = Hello()
    h2.show("h2", 2)
    print()
    Anal.showOp("h2.value = 3.1415927")
    h2.value = 3.1415927
    h2.show("h2", 2)
    print()

    Anal.showOp("p1 = Points.Point[float](5)")
    p1: Points.Point = Points.Point[float](5)
    p1[1] = 1.5
    p1[3] = -2.0
    index = p1.len() - 1
    p1[index] = 42
    print(" ", type(p1))
    p1.show("p1", 2)
    print()

def demo_generic_functions():

  Anal.showNote("  demo generic functions", "\n")

  Anal.showOp("Anal.showType(list[int], name)")
  l1:list[int] = [1, 2, 3, 2, 1, 0, -1, -2, -1, 0]
  Anal.showType(l1, "l1")
  print()

  Anal.showOp("showTypeEnum(list[int], name)")
  Anal.showTypeEnum(l1, "l1", left = 2, width = 7, suffix = "")
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

