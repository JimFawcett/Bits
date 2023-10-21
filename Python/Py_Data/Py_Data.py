import sys
# Python/Py_Data::Py_Data.py
# 
# Python Dynamic Data Types
#   int, float, complex
#   bytes, bytearray, memoryview
#   list, tuple, range
#   dict, set, frozenset
#   bool
#   str
#   NoneType

# Python requires definition before use ordering
#  - no link phase to find definitions

# nl = "\n"

# def showType(t, nm, suffix = "") :
#     print(nm, type(t))
#     print("value: ", t, ', size: ', sys.getsizeof(t), suffix)

# def showIdent(t, n, suffix = "") :
#     print(n, t, id(t), suffix)

# def showNote(text, suffix = "") :
#     print("-------------------------")
#     print(text)
#     print("-------------------------", suffix)

import Py_DataAnalysis
import copy
# from ctypes import c_int, addressof

Anal = Py_DataAnalysis
nl = Anal.nl

def demoScalars() :
    
  Anal.showNote("Initialize Scalars", nl)

  # NoneType
  Anal.showOp("n = None")
  n = None
  Anal.showType(n, "n", nl)

  # boolean
  Anal.showOp("bl = true")
  bl = True
  Anal.showType(bl, "bl")
  Anal.showOp("bl2 = bool()")
  bl2 = bool()
  Anal.showType(bl2, "bl2", nl)

  # integer
  Anal.showOp("i = 42")
  i = 42
  Anal.showType(i, "i")
  Anal.showOp("i2 = -3000000000000000")
  i2 = -3000000000000000
  Anal.showType(i2, "i2")
  Anal.showOp("i3 = int()")
  i3 = int()
  Anal.showType(i3, "i3", nl)
  
  # float
  Anal.showOp("f = 3.1415927")
  f = 3.1415927
  Anal.showType(f, "f")
  Anal.showOp("f2 = 3.33333e5")
  f2 = 3.33333e5
  Anal.showType(f2, "f2")
  Anal.showOp("f3 = 3.33333e55")
  f3 = 3.33333e55
  Anal.showType(f3, "f3")
  Anal.showOp("f4 = float()")
  f4 = float()
  Anal.showType(f4, "f4", nl)
  
  # complex
  Anal.showOp("c = 1.5 + 3.0j")
  c = 1.5 + 3.0j
  Anal.showType(c, "c")
  Anal.showType(c.real, "c.real")
  Anal.showType(c.imag, "c.imag")
  Anal.showOp("c1 = complex()")
  c1 = complex()
  Anal.showType(c1, "c1", nl)
  
def demoAggr() :
  Anal.showNote("Initialize Aggregates", nl)

  # bytes
  Anal.showOp("byts = b\"python\"")
  byts = b"python"
  Anal.showType(byts, "byts")
  byts1 = byts[0]
  # byts[1] = 2  # error bytes are immutable
  Anal.showType(byts1, "byts[0]")
  bytstr = byts.decode('UTF-8')
  Anal.showType(bytstr, "byts.decode(UTF-8)")
  Anal.showType(bytstr[0], "byts.decode(UTF-8)[0]", nl)
  
  # bytearray
  Anal.showOp("ba = bytearray(4)")
  ba = bytearray(4)
  ba[0] = 1
  ba[1] = 0xee
  ba[2] = 0
  ba[3] = 1
  Anal.showType(ba, "ba", nl)

  # str
  Anal.showOp("s = \"hello python\"")
  s = "hello python"
  Anal.showType(s, "s")
  Anal.showType("", "empty str")
  Anal.showType("h", "1 char str")
  Anal.showType("he", "2 char str")
  Anal.showType(s[0], "s[0]", nl)

  # range
  Anal.showOp("r = range(6)")
  r = range(6)
  Anal.showType(r, "range(6)")
  for n in r :
    print(n, " ", end="")
  # 0 1 2 3 4 5
  print()
  r2 = range(-1, 10, 2)
  Anal.showType(r2, "range(-1, 10, 2)")
  for n in r2 :
    print(n, " ", end="")
  print(nl)
  # -1 1 3 5 7 9

  # tuple
  Anal.showOp("t = (42, 3.1415927, 'z')")
  t = (42, 3.1415927, 'z')
  Anal.showType(t, "t")
  Anal.showType(t[1], "t[1]")
  telem2 = t[1]
  Anal.showType(telem2, "t[1]")
  # t[0] = 84  # invalid, tuples are immutable
  
  # list
  Anal.showOp("l = [1, 2, 3, 2, 1]")
  l = [1, 2, 3, 2, 1]
  Anal.showType(l, "l")
  Anal.showOp("l.append(\"weird\")")
  l.append("weird")
  Anal.showType(l, "l")
  Anal.showOp("l0 = l[0]")
  l0 = l[0]
  Anal.showType(l0, "l0", nl)
  
  # dict
  Anal.showOp("d = { \"zero\": 0, \"one\": 1, \"two\":2 }")
  d = { "zero": 0, "one": 1, "two":2 }
  Anal.showType(d, "d")
  Anal.showOp("d[\"three\"] = 3")
  d["three"] = 3
  Anal.showType(d, "d")
  Anal.showOp("d2 = d[\"two\"]")
  d2 = d["two"]
  Anal.showType(d2, "d2", nl)
  
def demoCopyAndMod() :
  Anal.showNote("Copy and Modify scalar", nl)

  Anal.showOp("t1 = 42")
  t1 = 42  # t1 is handle pointing to value 42 in heap
  Anal.showType(t1, "t1")
  Anal.showOp("t2 = t1")
  t2 = t1
  Anal.showType(t2, "t2")
  Anal.showAddress(t1,"t1: ")
  Anal.showAddress(t2, "t2: ")
  Anal.showNote(
     "After copy construction t1 and t2 have\n  same address," +
     " e.g., two handles pointing\n  to the same heap integer object."
  )
  Anal.showOp("t1 = 0  # new object")
  t1 = 0  # change of heap object
  Anal.showType(t1, "t1")
  Anal.showAddress(t1,"t1: ")
  Anal.showType(t2, "t2")
  Anal.showAddress(t2,"t2: ")
  Anal.showNote(
     "After setting new value for t1,\n  t1 and t2 have unique addresses,\n" +
     "  e.g., two handles pointing to different\n  heap integer objects."
     , nl
  )
  
  Anal.showNote("copy and modify aggregate", nl)

  Anal.showOp("t3 = [1, 2, 3, 2, 1]")
  t3 = [1, 2, 3, 2, 1]  # t3 is handle pointing to list in heap
  Anal.showType(t3, "t3")
  Anal.showOp("t4 = t3")
  t4 = t3
  Anal.showType(t4, "t4")
  Anal.showAddress(t3,"t3: ")
  Anal.showAddress(t4, "t4: ")
  Anal.showNote(
     "After copy construction t3 and t4 have\n  same address," +
     " e.g., two handles pointing\n  to the same heap integer object."
  )
  Anal.showOp("t3.append(0)  # modify object")
  t3.append(0)  # changes value, doesn't create new object
  Anal.showType(t3, "t3")
  Anal.showAddress(t3,"t3: ")
  Anal.showType(t4, "t4")
  Anal.showAddress(t4,"t4: ")
  Anal.showNote(
    "After appending new value for t3,\n  t3 and t4 still have same value and\n" +
    "  address, e.g., two handles pointing\n  to same heap integer object. No copy\n" +
    "  on write."
    , nl
  )

  Anal.showOp("t5 = \"Hello Python\"")
  t5 = "Hello Python"  # handle pointing to string in heap
  Anal.showType(t5, "t5")
  Anal.showOp("t6 = t5  # copy construction")
  t6 = t5
  Anal.showType(t6, "t6")
  Anal.showAddress(t5,"t5: ")
  Anal.showAddress(t6, "t6: ")
  Anal.showNote(
     "After copy construction t5 and t6 have\n  same address," +
     " e.g., two handles pointing\n  to the same heap string object."
  )
  Anal.showOp("t6 = t6.replace(\"P\", \"p\")  # copy on write")
  t6 = t6.replace("P", "p")  # copy on write creates new t6 object
  Anal.showType(t6, "t6")
  Anal.showType(t5, "t5")
  Anal.showAddress(t6,"t6: ")
  Anal.showAddress(t5,"t5: ")
  Anal.showNote(
    "After modifying value for t6,\n  t5 and t6 have different values\n" +
    "  and addresses, e.g., string has\n  copy on write."
    , nl
  )


def execute() : 
    Anal.showNote("Demonstrate Python Types")
    print("\n-- instances of all types reside in managed heap --\n")

    demoScalars()
    demoAggr()
    demoCopyAndMod()

    # # types
    # t1 = True
    # Anal.showType(t1, "t1", nl)
    # t1 = 42              #dynamic type t1 redefined
    # Anal.showType(t1, "t1", nl)
    # t1 = 3.14159         #dynamic type t1 redefined
    # Anal.showType(t1, "t1", nl)
    # t1 = 1.5 + 3.3j      #dynamic type t1 redefined
    # Anal.showType(t1, "t1", nl)
    # t1 = [1, 2, 3, 4]    #dynamic type t1 redefined
    # Anal.showType(t1, "t1", nl)

    # # reference behavior - change of object
    # Anal.showOp("t2 = t1")
    # t2 = t1
    # Anal.showType(t1,"t1")
    # #Anal.showAddress(t1, "t1")
    # Anal.showType(t2, "t2")
    # #Anal.showAddress(t2, "t2")
    # Anal.showIdent(t1,"t1")
    # Anal.showIdent(t2,"t2", nl)

    # Anal.showOp("t2 = 3.33333 : change of object")
    # t2 = 3.33333
    # Anal.showIdent(t2,"t2")
    # Anal.showType(t2,"t2", nl)

    # # reference behavior - modify object
    # Anal.showOp("t2 += -1.0 : modify object")
    # t2 += -1.0
    # Anal.showIdent(t2,"t2")
    # Anal.showType(t2, "t2", nl)

    # # reference behavior
    # Anal.showOp("t3 = [1, \"one\"]")
    # t3 = [1, "one"]
    # Anal.showIdent(t3, "t3")
    # Anal.showType(t3, "t3", nl)

    # Anal.showOp("t4 = t3")
    # t4 = t3
    # Anal.showIdent(t3, "t3")
    # Anal.showType(t3,"t3")
    # Anal.showIdent(t4, "t4")
    # Anal.showType(t4,"t4")
    # Anal.showNote(
    #   """t3 and t4 are two distinct handles\n"""
    #   """  pointing to same instance""", nl
    # )
    
    # # reference behavior - no copy on write
    # Anal.showOp("t4.append('b') : no copy on write")
    # t4.append('b')
    # Anal.showIdent(t3, "t3")
    # Anal.showType(t3,"t3")
    # Anal.showIdent(t4, "t4")
    # Anal.showType(t4,"t4")
    # Anal.showNote(
    #   "t3 modified - same heap object,\n  no copy on write", nl
    # )

    # # reference behavior - new child object
    # Anal.showOp("t5 = (1, 2, 3)")
    # t5 = (1, 2, 3)
    # Anal.showIdent(t5, "t5")
    # Anal.showOp("t6 = [1, t5, \"weird\"]")
    # t6 = [1, t5, "weird"]
    # Anal.showIdent(t6, "t6")
    # Anal.showType(t6, "t6")
    # Anal.showOp("t5 = 1 + 1j : new object")
    # t5 = 1 + 1j
    # Anal.showIdent(t5, "t5")
    # Anal.showIdent(t6, "t6")
    # Anal.showNote("new object for t5, t6 not affected", nl)
    
    # # reference behavior - string copy on write
    # Anal.showOp("t7 = \"a string\"")
    # t7 = "a string"
    # Anal.showIdent(t7, "t7")
    # Anal.showType(t7, "t7")
    # Anal.showOp("t8 = t7")
    # t8 = t7
    # Anal.showIdent(t8, "t8")
    # Anal.showOp("t8 += \" and another string\" : copy on write")
    # t8 += " and another string"
    # Anal.showIdent(t7, "t7")
    # Anal.showIdent(t8, "t8")
    # Anal.showType(t8, "t8")
    # Anal.showNote("t7 not modified : str copy on write", nl)

    # # deepcopy
    # Anal.showOp("t10 = copy.deepcopy(t9)")
    # t9 = list[1, 2, 3]
    # Anal.showIdent(t9, "t9")
    # t10 = copy.deepcopy(t9);
    # Anal.showIdent(t10, "t10")
    # Anal.showNote(
    #   "two distinct handles each with\n  a unique instance",
    #   nl
    # )

    # # reference behavior - iterate over children
    # Anal.showOp("iterate over t6 children")
    # for i in t6:
    #     Anal.showType(i, "elem")
    #     Anal.showIdent(i, "elem")
    # print();

    # # Anal.showOp("iterate over t6 methods")
    # # print(dir(t6))
      
    print("That's all folks!\n")

execute()

