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

Anal = Py_DataAnalysis
nl = Anal.nl

def execute() : 
    Anal.showNote("Demonstrate Python Types")
    # types
    print("\n  -- instances of all types reside in managed heap --\n")
    t1 = 42
    Anal.showType(t1, "t1", nl)
    t1 = 3.14159         #dynamic type t1 redefined
    Anal.showType(t1, "t1", nl)
    t1 = [1, 2, 3, 4]    #dynamic type t1 redefined
    Anal.showType(t1, "t1", nl)

    # reference behavior - change of object
    Anal.showOp("t2 = t1")
    t2 = t1
    Anal.showIdent(t1,"t1")
    Anal.showIdent(t2,"t2", nl)

    Anal.showOp("t2 = 3.33333 : change of object")
    t2 = 3.33333
    Anal.showIdent(t2,"t2")
    Anal.showType(t2,"t2", nl)

    # reference behavior - modify object
    Anal.showOp("t2 += -1.0 : modify object")
    t2 += -1.0
    Anal.showIdent(t2,"t2")
    Anal.showType(t2, "t2", nl)

    # reference behavior
    Anal.showOp("t3 = [1, \"one\"]")
    t3 = [1, "one"]
    Anal.showIdent(t3, "t3")
    Anal.showType(t3, "t3", nl)

    Anal.showOp("t4 = t3")
    t4 = t3
    Anal.showIdent(t3, "t3")
    Anal.showType(t3,"t3")
    Anal.showIdent(t4, "t4")
    Anal.showType(t4,"t4")
    Anal.showNote(
      """t3 and t4 are two distinct handles\n"""
      """  pointing to same instance""", nl
    )
    
    # reference behavior - no copy on write
    Anal.showOp("t4.append('b') : no copy on write")
    t4.append('b')
    Anal.showIdent(t3, "t3")
    Anal.showType(t3,"t3")
    Anal.showIdent(t4, "t4")
    Anal.showType(t4,"t4")
    Anal.showNote(
      "t3 modified - same heap object,\n  no copy on write", nl
    )

    # reference behavior - new child object
    Anal.showOp("t5 = (1, 2, 3)")
    t5 = (1, 2, 3)
    Anal.showIdent(t5, "t5")
    Anal.showOp("t6 = [1, t5, \"weird\"]")
    t6 = [1, t5, "weird"]
    Anal.showIdent(t6, "t6")
    Anal.showType(t6, "t6")
    Anal.showOp("t5 = 1 + 1j : new object")
    t5 = 1 + 1j
    Anal.showIdent(t5, "t5")
    Anal.showIdent(t6, "t6")
    Anal.showNote("new object for t5, t6 not affected", nl)
    
    # reference behavior - string copy on write
    Anal.showOp("t7 = \"a string\"")
    t7 = "a string"
    Anal.showIdent(t7, "t7")
    Anal.showType(t7, "t7")
    Anal.showOp("t8 = t7")
    t8 = t7
    Anal.showIdent(t8, "t8")
    Anal.showOp("t8 += \" and another string\" : copy on write")
    t8 += " and another string"
    Anal.showIdent(t7, "t7")
    Anal.showIdent(t8, "t8")
    Anal.showType(t8, "t8")
    Anal.showNote("t7 not modified : str copy on write", nl)

    # deepcopy
    Anal.showOp("t10 = copy.deepcopy(t9)")
    t9 = list[1, 2, 3]
    Anal.showIdent(t9, "t9")
    t10 = copy.deepcopy(t9);
    Anal.showIdent(t10, "t10")
    Anal.showNote(
      "two distinct handles each with\n  a unique instance",
      nl
    )

    # reference behavior - iterate over children
    Anal.showOp("iterate over t6 children")
    for i in t6:
        Anal.showIdent(i, "elem")
    print();

    Anal.showOp("iterate over t6 methods")
    print(dir(t6))
      
    print("\nThat's all folks!\n")

execute()

