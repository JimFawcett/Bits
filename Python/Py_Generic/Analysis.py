# Analysis.py

import sys
import collections
from typing import TypeVar
# from typing import TypeVar, Generic, Sized, Iterable, Collection, Union, Any
from collections.abc import Sequence

T = TypeVar('T')
# Coll = Union[Iterable, Sized] = 'Coll'

# Python requires definition before use ordering
#  - no link phase to find definitions
import copy
nl = "\n"

# show name, type, value, and size of a Python instance
def showType(t:T, nm:str, suffix = "") :
    print(nm, type(t), "dynamic")
    print("value: ", t, ', size: ', sys.getsizeof(t), suffix)

# generate indent string with n spaces
def indent(n:int):
    tmpStr = ""
    for i in range(n):
        tmpStr += ' '
    return tmpStr

# fold indexable into rows of width elements indented by 
# left spaces
def fold(enum: Sequence, left:int, width:int) -> str:
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
    # topStr = indent(left) + nm + type(enum) + "dynamic"
    print(indent(left),nm, ' ', type(enum), ' ', "dynamic", sep='')
    print(indent(left), "{", sep='')
    print(fold(enum, left+2, width))
    print(indent(left), "}", sep = '')
    print(indent(left), "size: ", sys.getsizeof(enum), suffix, sep='')

# same as showType except uses class method to show value
def showTypeShowable(t, nm, suffix = ""):
    print(type(t), "dynamic")
    t.show(nm)

# show Python id, unique for each instance
def showIdent(t, name, suffix = "") :
    print(name, '"{}"'.format(t), id(t), suffix)

# show emphasized note
def showNote(text, suffix = "", n: int = 50) :
    tmpStr = ""
    for i in range(n):
      tmpStr += '-'
    print(tmpStr)
    print(text)
    print(tmpStr, suffix)

# show delineated string to announce a program operation
def showOp(text):
    print("--- {} ---".format(text))

