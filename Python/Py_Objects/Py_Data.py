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

nl = "\n"

def showType(t, nm, suffix = "") :
    print(nm, type(t))
    print("value: ", t, ', size: ', sys.getsizeof(t), suffix)

def showIdent(t, n, suffix = "") :
    print(n, t, id(t), suffix)

def showNote(text, suffix = "") :
    print("-------------------------")
    print(text)
    print("-------------------------", suffix)

def execute() : 
    print(" Demonstrate Python Types")
    print("--------------------------")
    # types
    print("\n  -- instances of all types reside in managed heap --\n")
    t1 = 42
    showType(t1, "t1", nl)
    t1 = 3.14159         #dynamic type t1 redefined
    showType(t1, "t1", nl)
    t1 = [1, 2, 3, 4]    #dynamic type t1 redefined
    showType(t1, "t1", nl)

    # reference behavior - change of object
    print("-- t2 = t1 --")
    t2 = t1
    showIdent(t1,"t1")
    showIdent(t2,"t2", nl)

    print("\n-- t2 = 3.33333 : change of object --")
    t2 = 3.33333
    showIdent(t2,"t2")
    showType(t2,"t2", nl)

    # reference behavior - modify object
    print("-- t2 += -1.0 : modify object --")
    t2 += -1.0
    showIdent(t2,"t2")
    showType(t2, "t2", nl)

    # reference behavior
    print("-- t3 = [1, \"one\"] --")
    t3 = [1, "one"]
    showIdent(t3, "t3")
    showType(t3, "t3", nl)

    print("-- t4 = t3 --")
    t4 = t3
    showIdent(t3, "t3")
    showType(t3,"t3")
    showIdent(t4, "t4")
    showType(t4,"t4", nl)
    
    # reference behavior - no copy on write
    print("-- t4.append('b') : no copy on write --")
    t4.append('b')
    showIdent(t3, "t3")
    showType(t3,"t3")
    showIdent(t4, "t4")
    showType(t4,"t4")
    showNote("t3 modified - same heap object, no copy on write", nl)

    # reference behavior - new child object
    print("-- t5 = (1, 2, 3) --")
    t5 = (1, 2, 3)
    showIdent(t5, "t5")
    print("-- t6 = [1, t5, \"weird\"] --")
    t6 = [1, t5, "weird"]
    showIdent(t6, "t6")
    showType(t6, "t6")
    print("-- t5 = 1 + 1j : new object --")
    t5 = 1 + 1j
    showIdent(t5, "t5")
    showIdent(t6, "t6")
    showNote("new object for t5, t6 not affected", nl)
    
    # reference behavior - string copy on write
    print("-- t7 = \"a string\" --")
    t7 = "a string"
    showIdent(t7, "t7")
    showType(t7, "t7")
    print("-- t8 = t7 --")
    t8 = t7
    showIdent(t8, "t8")
    print("-- t8 += \" and another string\" : copy on write")
    t8 += " and another string"
    showIdent(t7, "t7")
    showIdent(t8, "t8")
    showType(t8, "t8")
    showNote("t7 not modified : copy on write", nl)

    # reference behavior - iterate over children
    print("-- iterate over t6 children --")
    for i in t6:
        showIdent(i, "elem")

    print("\n-- iterate over t6 methods --")
    print(dir(t6))
      
    print("\nThat's all folks!\n")

execute()

