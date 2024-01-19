import sys
import copy
import AnalysisObj
import PointsObj

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

# define alias shortcut
anal = AnalysisObj

# # Python requires definition before use ordering

#-- Demonstrate primitive and library types ---------------
def demolibtypes() :
    anal.showNote("  primitive and library types","\n")

    # type of d1 is inferred as float from RHS
    d1 = 3.1415927
    print("d1 = ", d1)
    
    # repr(s) wraps string s in quotes
    s1 = "a string"
    print("s1 = ", repr(s1))

    anal.showOp("s2 = s1")
    s2 = s1
    anal.showIdent(s1, "s1")
    anal.showIdent(s2, "s2")

    #print("s2 = {}".format(s2))

    anal.showOp("s2 += \" and more\"")
    s2 += " and more"
    anal.showIdent(s2, "s2")
    anal.showIdent(s1, "s1")
    print()
    anal.showNote(   
        "  Assignment, in Python, assigns references not\n"\
        "  values.  So s1 and s2 share same heap instance\n"\
        "  But strings are immutable. So when a change is\n"\
        "  made to one, that creates a new changed instance\n"\
        "  without changing the original."
    )
    print()

    l1 = ["you", "me", "them", "us"]
    anal.showValueEnum(l1, "l1")

    print("\nl1 = ", l1, "\n")
    
    anal.showOp("l2 = l1")
    l2 = l1
    
    anal.showOp('l2.append("everyone")')
    l2.append("everyone")
    print("\nl2 = ", l2)
    print("l1 = ", l1, "\n")
    
    anal.showOp('l2[1] = "myself"')
    l2[1] = "myself"
    print("\nl2 = ", l2)
    print("l1 = ", l1)
    print()

    anal.showNote(   
        "  Changes to target of assignment affect source\n"\
        "  except for immutable strings."\
        "  \"caveat emptor\""
    )
    print()

#-- demonstrate user-define type --------------------------
def demouserdeftype() :
    anal.showNote("  user defined type","\n")
    
    anal.showOp("p1a = Point4D()")
    p1a = PointsObj.Point4D()
    
    anal.showOp("AnalysisObj.showTypeShowable(p1a, \"p1a\", nl)")
    
    # function defined in AnalysisObj.py
    anal.showTypeShowable(p1a, "p1a", "\n")
    p1a.x = 2
    p1a.y = -3.5
    p1a.z = -42

    print()
    # method defined in Point4D
    p1a.show("p1a")

    anal.showOp("p1b = p1a")
    p1b = p1a
    p1b.show("p1b")

    anal.showOp("p1b.y = 13")
    p1b.y = 13
    p1b.show("p1b")
    p1a.show("p1a")
    anal.showNote(
        "  Reference assigned, not value.  So change\n"\
        "  in P1b changed source p1a."
    )
    print()

    # copy.deepcopy(p) copies entire object graph of p
    anal.showOp("p1c = copy.deepcopy(p1b)")
    p1c = copy.deepcopy(p1b)
    p1c.show("p1c");
    p1b.show("p1b")

    anal.showOp("p1c.z = 12")
    p1c.z = 12
    p1c.show("p1c")
    p1b.show("p1b")
    anal.showNote(
        "  p1c.z reference assigned, not value. But no\n"\
        "  change in p1b since p1c is deep clone of p1b."
    )
    print()

# -- illustrate reference behavior ------------------------ 
def demorefbehavior() :
    # reference behavior - new child object
    anal.showOp("t5 = (1, 2, 3)")
    t5 = (1, 2, 3)
    anal.showIdent(t5, "t5")

    anal.showOp("t6 = [1, t5, \"weird\"]")
    t6 = [1, t5, "weird"]
    anal.showIdent(t6, "t6")
    anal.showType(t6, "t6")
    print("-- t5 = 1 + 1j : new object --")

    #new object for t5
    t5 = 1 + 1j
    anal.showIdent(t5, "t5")

    # t6 still refers to old t5 object
    anal.showIdent(t6, "t6")
    print()
    anal.showNote(
        "  new object for t5, t6 not affected", "\n"
    )
    
    # # reference behavior - iterate over children
    print("-- iterate over t6 children --")
    for i in t6:
        anal.showIdent(i, "elem")

    # uncommenting the two statements below shows all the user
    # and system defined methods
    
    # print("\n-- iterate over t6 methods --")
    # print(dir(t6))
  
# -- Demonstration starts here ----------------------------
def execute() : 
    print(" Demonstrate Python Objects")
    print("----------------------------")
    print()

    anal.showNote(   
        "  All Python types are reference-based\n"\
        "  with values in the managed heap. That\n"\
        "  has consequences we explore in this demo."
    )
    print()

    demolibtypes()
    demouserdeftype()
    demorefbehavior()

    print("\nThat's all folks!\n")

execute()