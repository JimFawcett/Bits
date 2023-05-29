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

import copy
import Analysis
import Points

def execute() : 
    print(" Demonstrate Python Generics")
    print("----------------------------", "\n")

    Analysis.showNote(   
        "  Python introduced generics and type hints\n"\
        "  to check for type errors using mypy, a static\n"\
        "  type checking tool."
    )
    print()

    Analysis.showNote("Type Hints", "\n")

    d1: float = 3.1415927
    Analysis.showType(d1, "d1")
    
    s1: str = "a string"
    Analysis.showType(s1, "s1")

    Analysis.showOp("s2 = s1")
    s2: str = s1
    Analysis.showIdent(s1, "s1")
    Analysis.showIdent(s2, "s2")
    #print("s2 = {}".format(s2))

    Analysis.showOp("s2 += \" and more\"")
    s2 += " and more"
    Analysis.showIdent(s2, "s2")
    Analysis.showIdent(s1, "s1")
    # print("s2 = {}".format(s2))
    # print("s1 = {}".format(s1))
    print()

    Analysis.showNote(   
        "Assignment, in Python, assigns references not\n"\
        "values.  So s1 and s2 share same heap instance\n"\
        "But strings are immutable. So when a change is\n"\
        "made to one, that creates a new instance without\n"\
        "changing the original."
    )
    print()

    Analysis.showNote("Generics", "\n")

    l1: list[str] = ["you", "me", "them", "us"]
    Analysis.showType(l1, "l1", Analysis.nl)

    print("l1 = ", l1)
    
    Analysis.showOp("l2 = l1")
    l2 = l1
    
    Analysis.showOp('l2.append("everyone")')
    l2.append("everyone")
    print("l2 = ", l2)
    print("l1 = ", l1)
    
    Analysis.showOp('l2[1] = "myself"')
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

    Analysis.showNote("user defined type","\n")
    
    Analysis.showOp("p1 = Points.PointN[float](8)")
    p1: Points.PointN = Points.PointN[float](10)
    # p1.append("3")
    p1[1] = 1.5
    p1[3] = -2.0
    index = p1.len() - 1
    p1[index] = 42
    Analysis.showTypeEnum(p1, "p1", 0, 5)

    Analysis.showOp("Analysis.showTypeShowable(p1, \"p1\", nl)")
    Analysis.showTypeShowable(p1, "p1", Analysis.nl)
    print("len(p1) = {}".format(len(p1)))
    print("p1.len() = {}".format(p1.len()))
    print()

    p1[0] = 2
    p1[1] = -3.5
    p1[2] = -42
    Analysis.showOp("p1.show('p1')")
    p1.show("p1")
    # Analysis.showTypeEnum(p1, "p1", 0, 7)

    Analysis.showOp("p2 = p1")
    p2 = p1
    p2.show("p2")

    Analysis.showOp("p2[1] = 13")
    p2[1] = 13
    p2.show("p2")
    p1.show("p1")
    Analysis.showNote(
        "Reference assigned, not value.\nSo change"\
        " in p2 changed source p1."
    )
    print()

    Analysis.showOp("p3 = copy.deepcopy(p2)")
    p3 = copy.deepcopy(p2)
    p3.show("p3");
    p2.show("p2")

    Analysis.showOp("p3[2] = 12")
    p3[2] = 12
    p3.show("p3")
    p2.show("p2")
    Analysis.showNote(
        "p3[2] reference assigned, not value. But no\n"\
        "change in p2 since p3 is deep clone of p2."
    )
    print()

    # reference behavior - new child object
    Analysis.showOp("t5 = (1, 2, 3)")
    t5 = (1, 2, 3)
    Analysis.showIdent(t5, "t5")

    Analysis.showOp("t6 = [1, t5, \"weird\"]")
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

