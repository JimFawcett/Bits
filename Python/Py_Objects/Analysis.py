# Analysis.py

import sys

# Python requires definition before use ordering
#  - no link phase to find definitions
import copy
nl = "\n"

# show name, type, value, and size of a Python instance
def showType(t, nm, suffix = "") :
    print(nm, type(t), "dynamic")
    print("value: ", t, ', size: ', sys.getsizeof(t), suffix)

# generate indent string with n spaces
def indent(n):
    tmpStr = ""
    for i in range(n):
        tmpStr += ' '
    return tmpStr

# fold indexable into rows of width elements indented by 
# left spaces
def fold(enum, left, width):
    tmpStr = indent(left)
    for i in range(len(enum)):
        tmpStr += str(enum[i]) + ", "
        if(((i + 1) % width) == 0 and i != 0):
            tmpStr += "\n" + indent(left)
    rIndex = tmpStr.rindex(',')
    tmpStr = tmpStr[:rIndex]
    return tmpStr

# show name, type, value, and size of a Python instance
def showTypeEnum(enum, nm, left = 2, width = 7, suffix = "") :
    print(nm, type(enum), "dynamic")
    print(fold(enum, left, width))
    print("size:", sys.getsizeof(enum), suffix)

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

