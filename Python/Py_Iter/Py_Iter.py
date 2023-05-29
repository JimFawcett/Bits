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

# import copy
# import collections
from typing import TypeVar, Iterable, Tuple, Generator

from collections.abc import Sequence, Mapping
import AnalysisIter
import PointsIter

T = TypeVar('T')

# short-hand identifiers
Anal = AnalysisIter
Points = PointsIter

# generator 
def generator(coll: Iterable[T]) -> Generator:
  for item in coll:
    yield item

# forloop extracts iterator from enumerable
#------------------------------------------------
def forloopEnum(enum: Sequence, nm:str, left:int, width:int):
  print(nm)
  print("  ", end='')
  for item in enum:
    print("{} ".format(item), end='')
  print()

# forloopEnumFolded uses AnalysisIter function fold
#------------------------------------------------
def forloopEnumFolded(enum:Sequence, nm:str, left, width):
  print(nm)
  s = Anal.fold(enum, left, width)
  print(s)

# forloopAssocFolded uses fold function implemented for
# associative containers
#------------------------------------------------
def forloopAssocFolded(enum:Mapping, nm:str, left, width):
  print(nm)
  s = Anal.foldAssoc(enum, left, width)
  print(s)

# function implementing all demonstrations
#------------------------------------------------
def execute() : 
    print()
    print(" Demonstrate Python Iterators")
    print("------------------------------", "\n")

    Anal.showNote("  basic iteration", "\n")

    # idiomatic iteration over range using for loop
    Anal.showOp("range(1,6) is iterable")
    print("  ", end='')
    for i in range(1,6):
      print("{} ".format(i), end='')
    print()

    # iteration using iterator directly
    Anal.showOp("extracting iterator from range(0,10)")
    iter = range(0,10).__iter__()
    print("  ", end='')
    try:
      while True:
        print("{} ".format(next(iter)), end='')
    except StopIteration:
      pass
    print()

    Anal.showOp("using generator to walk through range(1,5)")
    print("  ", end='')
    for item in generator(range(1,5)):
      print("{} ".format(item), end='')
    print()

    Anal.showOp("using iterator returned by generator")
    iter = generator(range(1,5))
    print("  ", end='')
    try:
      while True:
        print("{} ".format(next(iter)), end='')
    except StopIteration:
      pass
    print("\n")

    Anal.showNote("  iteration using AnalysisIter functions", "\n")
    
    Anal.showOp("Anal.showTypeEnum(range(1,6), ...)")
    Anal.showTypeEnum(range(1,6), "range(1,6)", 2, 7)

    Anal.showOp("forloopEnum(str)")
    s:str = "a string"
    forloopEnum(s, "str", 2, 4)

    Anal.showOp("forloopEnumFolded(list)")
    l:list[float] = [1.0, 2.5, -3.5, 2.0, 1.5, 0.5]
    forloopEnumFolded(l, "list[float]", 2, 4)

    Anal.showOp("forloopAssocFolded(dict)")
    d:dict[int, str] = {
      1:"one", 2:"two", 3:"three", 4:"four", 5:"five"
    }
    forloopAssocFolded(d, "dictionary[int, str]", 2, 4)
    print()

    Anal.showNote("iteration over user-defined type", "\n")

    Anal.showOp("p.show('p', 0, 7)")
    p = Points.PointN[float](10)
    p[1] = 1
    p[3] = 3.5
    p[9] = 42
    p.show("p", 0, 7)
    print()

    Anal.showOp(
      "forloopEnumFolded(p, 'p : Points...', 2, 7)"
    )
    forloopEnumFolded(p, "p : Points.PointN[float](10)", 2, 7)
      
    print("\n\nThat's all folks!\n")

execute()

