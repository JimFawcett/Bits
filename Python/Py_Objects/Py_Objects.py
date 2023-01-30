import sys
# Python/Py_Objects::Py_Objects.py
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
import copy
nl = "\n"

# show name, type, value, and size of a Python instance
def showType(t, nm, suffix = "") :
    print(nm, type(t), "dynamic")
    print("value: ", t, ', size: ', sys.getsizeof(t), suffix)

# same as showType except uses class method to show value
def showTypeShowable(t, nm, suffix = ""):
    print(nm, type(t), "dynamic")
    t.show(nm)

# show Python id, unique for each instance
def showIdent(t, n, suffix = "") :
    print(n, t, id(t), suffix)

# show emphasized note
def showNote(text, suffix = "") :
    print("-------------------------")
    print(text)
    print("-------------------------", suffix)

# show delineated string to announce a program operation
def showOp(text):
    print("--- {} ---".format(text))

# point class with three coordinates
class Point1:
    x = 0;
    y = 0;
    z = 0;

    # supports constructor notation
    def __init__(self) -> None:
        pass
    
    # show named value of Point1 instance
    def show(self, name) :
        print("{} {{ ".format(name), end='')
        print(" x:{}, y:{}, z:{}".format(self.x, self.y, self.z), end='');
        print(" }");

# Demonstration starts here
def execute() : 
    print(" Demonstrate Python Objects")
    print("----------------------------")

    showNote(   
        "All Python types are reference-based\n"\
        "with values in the managed heap. That\n"\
        "has consequences we explore in this demo."
    )

    d1 = 3.1415927;
    showType(d1, "d1");
    
    s1 = "a string"
    showType(s1, "s1")

    showOp("s2 = s1")
    s2 = s1
    showIdent(s1, "s1")
    showIdent(s2, "s2")
    #print("s2 = {}".format(s2))
    showOp("s2 += \" and more\"")
    s2 += " and more"
    showIdent(s2, "s2")
    showIdent(s1, "s1")
    # print("s2 = {}".format(s2))
    # print("s1 = {}".format(s1))
    showNote(   
        "Assignment, in Python, assigns references not\n"\
        "values.  So s1 and s2 share same heap instance\n"\
        "But strings are immutable. So when a change is\n"\
        "made to one, that creates a new instance without\n"\
        "changing the original."
    )
    print()

    l1 = ["you", "me", "them", "us"]
    showType(l1, "l1", nl)

    print("l1 = ", l1)
    showOp("l2 = l1")
    l2 = l1
    showOp('l2.append("everyone")')
    l2.append("everyone")
    print("l2 = ", l2)
    print("l1 = ", l1)
    showOp('l2[1] = "myself"')
    l2[1] = "myself"
    print("l2 = ", l2)
    print("l1 = ", l1)
    showNote(   
        "Assignment, in Python, assignes references\n"\
        "not values.  So changes to target affect the\n"\
        "source.  \"caveat emptor\""
    )
    print()

    showOp("p1a = Point1()")
    p1a = Point1()
    showOp("showTypeShowable(p1a, \"p1a\", nl)")
    showTypeShowable(p1a, "p1a", nl)
    p1a.x = 2
    p1a.y = -3.5
    p1a.z = -42
    p1a.show("p1a")

    showOp("p1b = p1a")
    p1b = p1a
    p1b.show("p1b")

    showOp("p1b.y = 13")
    p1b.y = 13
    p1b.show("p1b")
    p1a.show("p1a")
    showNote(
        "Reference assigned, not value.\nSo change"\
        " in P1b changed source p1a."
    )
    print()

    showOp("p1c = copy.deepcopy(p1b)")
    p1c = copy.deepcopy(p1b)
    p1c.show("p1c");
    p1b.show("p1b")
    showOp("p1c.z = 12")
    p1c.z = 12
    p1c.show("p1c")
    p1b.show("p1b")
    showNote(
        "p1c.z reference assigned, not value. But no\n"\
        "change in p1b since p1c is deep clone of p1b."
    )
    print()

    # # reference behavior - new child object
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
    
    # # reference behavior - iterate over children
    print("-- iterate over t6 children --")
    for i in t6:
        showIdent(i, "elem")

    print("\n-- iterate over t6 methods --")
    print(dir(t6))
      
    print("\nThat's all folks!\n")

execute()

