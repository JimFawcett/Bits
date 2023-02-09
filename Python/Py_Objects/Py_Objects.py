import sys
import copy
import Analysis
import Points

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

# # Python requires definition before use ordering
# #  - no link phase to find definitions
# Demonstration starts here
def execute() : 
    print(" Demonstrate Python Objects")
    print("----------------------------")

    Analysis.showNote(   
        "  All Python types are reference-based\n"\
        "  with values in the managed heap. That\n"\
        "  has consequences we explore in this demo."
    )
    print()

    d1 = 3.1415927
    Analysis.showType(d1, "d1")
    
    s1 = "a string"
    Analysis.showType(s1, "s1")

    #region
    Analysis.showOp("s2 = s1")
    #endregion
    s2 = s1
    Analysis.showIdent(s1, "s1")
    Analysis.showIdent(s2, "s2")
    #print("s2 = {}".format(s2))

    #region
    Analysis.showOp("s2 += \" and more\"")
    #endregion
    s2 += " and more"
    Analysis.showIdent(s2, "s2")
    Analysis.showIdent(s1, "s1")
    # print("s2 = {}".format(s2))
    # print("s1 = {}".format(s1))
    Analysis.showNote(   
        "Assignment, in Python, assigns references not\n"\
        "values.  So s1 and s2 share same heap instance\n"\
        "But strings are immutable. So when a change is\n"\
        "made to one, that creates a new instance without\n"\
        "changing the original."
    )
    print()

    l1 = ["you", "me", "them", "us"]
    Analysis.showType(l1, "l1", Analysis.nl)

    print("l1 = ", l1)
    
    #region
    Analysis.showOp("l2 = l1")
    #endregion
    l2 = l1
    
    #region
    Analysis.showOp('l2.append("everyone")')
    #endregion
    l2.append("everyone")
    print("l2 = ", l2)
    print("l1 = ", l1)
    
    #region
    Analysis.showOp('l2[1] = "myself"')
    #endregion
    l2[1] = "myself"
    print("l2 = ", l2)
    print("l1 = ", l1)
    Analysis.showNote(   
        "Changes to target of assignment affect source\n"\
        "except for immutable strings, as we are required\n"\
        "to create new string instead of modify.\n"\
        "\"caveat emptor\""
    )
    print()

    #region
    Analysis.showOp("p1a = Point1()")
    #endregion
    p1a = Points.Point1()
    
    #region
    Analysis.showOp("Analysis.showTypeShowable(p1a, \"p1a\", nl)")
    #endregion
    Analysis.showTypeShowable(p1a, "p1a", Analysis.nl)
    p1a.x = 2
    p1a.y = -3.5
    p1a.z = -42
    p1a.show("p1a")

    #region
    Analysis.showOp("p1b = p1a")
    #endregion
    p1b = p1a
    p1b.show("p1b")

    #region
    Analysis.showOp("p1b.y = 13")
    #endregion
    p1b.y = 13
    p1b.show("p1b")
    p1a.show("p1a")
    Analysis.showNote(
        "Reference assigned, not value.\nSo change"\
        " in P1b changed source p1a."
    )
    print()

    #region
    Analysis.showOp("p1c = copy.deepcopy(p1b)")
    #endregion
    p1c = copy.deepcopy(p1b)
    p1c.show("p1c");
    p1b.show("p1b")

    #region
    Analysis.showOp("p1c.z = 12")
    #endregion
    p1c.z = 12
    p1c.show("p1c")
    p1b.show("p1b")
    Analysis.showNote(
        "p1c.z reference assigned, not value. But no\n"\
        "change in p1b since p1c is deep clone of p1b."
    )
    print()

    p2 = Points.Point2(3)

    #region
    Analysis.showOp("p2.show(\"p2\")")
    #endregion
    p2.show("p2")
    p2[1] = 1
    p2[2] = 2
    p2.append(3)
    p2.show("p2")
    for i in range(10):
         p2.append(10 - i)

    #region
    Analysis.showOp("p2.show(\"p2\")")
    #endregion
    p2.show("p2")

    #region
    Analysis.showOp("showTypeEnum(p2, \"p2\", 2, 7)")
    #endregion
    Analysis.showTypeEnum(p2,"p2", 2, 7)
    print()

    # # reference behavior - new child object
    #region
    Analysis.showOp("t5 = (1, 2, 3)")
    #endregion
    t5 = (1, 2, 3)
    Analysis.showIdent(t5, "t5")

    #region
    Analysis.showOp("t6 = [1, t5, \"weird\"]")
    #endregion
    t6 = [1, t5, "weird"]
    Analysis.showIdent(t6, "t6")
    Analysis.showType(t6, "t6")
    print("-- t5 = 1 + 1j : new object --")
    t5 = 1 + 1j
    Analysis.showIdent(t5, "t5")
    Analysis.showIdent(t6, "t6")
    Analysis.showNote(
        "new object for t5, t6 not affected", Analysis.nl
    )
    
    # # reference behavior - iterate over children
    print("-- iterate over t6 children --")
    for i in t6:
        Analysis.showIdent(i, "elem")

    # print("\n-- iterate over t6 methods --")
    # print(dir(t6))
      
    print("\nThat's all folks!\n")

execute()

