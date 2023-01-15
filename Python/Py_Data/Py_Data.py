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

def showType(t, n) :
    print(n, type(t))
    print("value: ", t, ', size: ', sys.getsizeof(t), "\n")

def showIdent(t, n) :
    print(n, t, id(t))

def execute() : 
    print(" Demonstrate Python Types")
    print("--------------------------")
    # types
    print("\n  -- instances of all types reside in managed heap --\n")
    t1 = 42
    showType(t1, "t1")
    t1 = 3.14159         #dynamic type t1 redefined
    showType(t1, "t1")
    t1 = [1, 2, 3, 4]    #dynamic type t1 redefined
    showType(t1, "t1")

    # reference behavior - copy on write
    print("-- t2 = t1 --")
    t2 = t1
    showIdent(t1,"t1")
    showIdent(t2,"t2")
    print("\n-- t2 = 3.33333 : copy on write --")
    t2 = 3.33333
    showIdent(t1,"t1")
    showIdent(t2,"t2")

    # reference behavior - copy on write
    print("\n-- t2 += -1.0 : copy on write --")
    t2 += -1.0
    showIdent(t1,"t1")
    showType(t1, "t1")
    showIdent(t2,"t2")
    showType(t2, "t2")

    # reference behavior - copy on write
    print("-- t3 = [1, \"one\"] --")
    t3 = [1, "one"]
    showIdent(t3, "t3")
    print("-- t4 = t3 --")
    t4 = t3
    showIdent(t3, "t3")
    showType(t3,"t3")
    showIdent(t4, "t4")
    showType(t4,"t4")
    
    # reference behavior - no copy on write
    print("-- t3.append('b') : no copy on write --")
    t3.append('b')
    showIdent(t3, "t3")
    showType(t3,"t3")
    showIdent(t4, "t4")
    showType(t4,"t4")

    # reference behavior - child copy on write
    print("-- t5 = (1, 2, 3) --")
    t5 = (1, 2, 3)
    showIdent(t5, "t5")
    print("-- t6 = [1, t5, \"weird\"] --")
    t6 = [1, t5, "weird"]
    showIdent(t6, "t6")
    print("-- t5 = 1 + 1j : copy on write --")
    t5 = 1 + 1j
    showIdent(t5, "t5")
    showIdent(t6, "t6")
    
    # reference behavior - iterate over children
    print("\n-- iterate over t6 --")
    for i in t6:
        showIdent(i, "elem")
    # print(dir(t6))
      
    print("\nThat's all folks!\n")
execute()

