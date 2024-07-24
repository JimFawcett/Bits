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
import Stats

T = TypeVar('T')

# short-hand identifiers
Anal = AnalysisIter
Points = PointsIter

# generator doesn't need to load entire collection
#------------------------------------------------
def generator(coll: Iterable[T]) -> Generator:
  for item in coll:
    yield item

#------------------------------------------------
# executing basic iterations over range
#------------------------------------------------
def execute_basic_iterations():
    Anal.showNote("  basic iteration over ranges", "\n")

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

    # idiomatic iteration over range using for loop
    Anal.showOp("idiomatic iteration over range(1,6)")
    print("  ", end='')
    for i in range(1,6):
      print("{} ".format(i), end='')
    print()

    # direct use of generator
    Anal.showOp("using iterator returned by generator")
    iter = generator(range(1,5))
    print("  ", end='')
    try:
      while True:
        print("{} ".format(next(iter)), end='')
    except StopIteration:
      pass
    print()

    # iteration using generator
    Anal.showOp("idiomatic iteration over range(1,5) using generator")
    print("  ", end='')
    for item in generator(range(1,5)):
      print("{} ".format(item), end='')
    print("\n")

# demo iterations over standard collections
#------------------------------------------------

# for seq iter returns seq item
def forloop_seq_iteration(coll):
    print(type(coll))
    iter = coll.__iter__()
    print("  {}".format(next(iter)), end='')
    for item in iter:
      print(", {}".format(item), end='')
    print()

# for map iter returns map key
def forloop_map_iteration(coll):
    print(type(coll))
    iter = coll.__iter__()
    key = next(iter)
    print('  {', key, ': ', coll[key], '}', end='')
    for key in iter:
      print(', {', key, ': ', coll[key], '}', end='')
    print()

#------------------------------------------------
# executing iteration over std collections
# -----------------------------------------------

def iteration_over_std_collections():
  Anal.showNote("  iteration over std collections", "\n")
  
  Anal.showOp("list[float]")
  l:list[float] = [1, 2.25, 3.5, 2.75, 1]
  forloop_seq_iteration(l)
  print()

  Anal.showOp("string")
  s:str = "a string"
  forloop_seq_iteration(s)
  print()

  Anal.showOp("dictionary[str, int]")
  d:dict[str, int] = {
    "zero": 0, "one": 1, "two":2
  }
  d["three"] = 3
  
  forloop_map_iteration(d)
  print()
  
#------------------------------------------------
# executing iterations over user-defined types
#------------------------------------------------

def iteration_over_user_defined_types():
  Anal.showNote("  iteration over user-defined collections", "\n")

  Anal.showOp("Stats[float]")
  l:list[float] = [1.0, 2.25, 3.5, 4.75, 5.0]
  s:Stats.Stats[float] = Stats.Stats[float](l)
  print(l)
  print("  max: {}".format(s.max()))
  print("  min: {}".format(s.min()))
  print("  sum: {}".format(s.sum()))
  print("  avg: {}".format(s.avg()))
  print()

  Anal.showOp("Point[float]")
  p:Points.Point[float] = Points.Point[float](0)
  p.coors = l  # all Python members are public
  # alternate initialization
  # p.append(1.0)
  # p.append(0.5)
  # p.append(0.0)
  # p.append(-0.5)
  # p.append(-1.0)
  p.show("p")
  print()
  
# demo formatting iterations over std collections
#------------------------------------------------

# forloop extracts iterator from enumerable
#------------------------------------------------
def forloopFormatted(enum: Sequence, nm:str, left:int, width:int):
  print("  ", nm, type(enum))
  print("  ", end='')
  first:bool = True
  for item in enum:
    if first:
      print("  {}".format(item), end='')
      first = False
    else:
      print(", {}".format(item), end='')
  print()

# forloopFormattedFolded uses AnalysisIter function fold
#------------------------------------------------
def forloopFormattedFolded(enum:Sequence, nm:str, left, width):
  print(nm)
  s = Anal.fold(enum, left, width)  # uses for loop
  print(s)

# forloopAssocFolded uses fold function implemented for
# associative containers
#------------------------------------------------
def forloopAssocFolded(enum:Mapping, nm:str, left, width):
  print(nm)
  s = Anal.foldAssoc(enum, left, width) # uses for loop
  print(s)

#------------------------------------------------
# demo executing formatted iteration over 
# std collections
#------------------------------------------------

def iteration_using_formatting_functions():

  Anal.showNote("  iteration_using_formatting_functions")
  print()

  Anal.showOp("forloopFormatted(list[int])")
  l:list[int] = [1, 2, 3, 2, 1]
  forloopFormatted(l, "l", 2, 4)
  print()

  Anal.showOp("forloopFormatted(str)")
  s:str = "a string"
  forloopFormatted(s, "str", 2, 4)
  print()

  Anal.showOp("Anal.showTypeEnum(range(1,6), ...)")
  Anal.showTypeEnum(range(1,6), "range(1,6)", 2, 7)
  print()

  Anal.showOp("forloopFormattedFolded(list)")
  l:list[float] = [1.0, 2.5, -3.5, 2.0, 1.5, 0.5]
  forloopFormattedFolded(l, "  list[float]", 4, 4)
  print()

  Anal.showOp("forloopAssocFolded(dict)")
  d:dict[int, str] = {
    1:"one", 2:"two", 3:"three", 4:"four", 5:"five"
  }
  forloopAssocFolded(d, "  dictionary[int, str]", 4, 4)
  print()

#------------------------------------------------
# executing formatting iterations over user-defined 
# types
# -----------------------------------------------
def iteration_formatted_user_defined_types():
  Anal.showNote("  formatted iteration over user-defined type", "\n")

  Anal.showOp("p.show('p', 0, 7)")
  p = Points.Point[float](10)
  p[1] = 1
  p[3] = 3.5
  p[9] = 42
  p.show("p", 2, 7)
  print()

  Anal.showOp(
    "forloopFormattedFolded(p, 'p : Points...', 4, 7)"
  )
  forloopFormattedFolded(p, "  p : Points.Point[float](10)", 4, 7)
    
#------------------------------------------------
# function implementing all demonstrations
#------------------------------------------------
def execute() : 
    print()
    Anal.showNote("  Demonstrate Python Iterators", "\n")

    execute_basic_iterations()
    iteration_over_std_collections()
    iteration_over_user_defined_types()
    iteration_using_formatting_functions()
    iteration_formatted_user_defined_types()

    print("\n\nThat's all folks!\n")

execute()

