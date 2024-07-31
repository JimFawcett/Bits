#----------------------------------------------------------
#   Py_Generic::AnalysisGen.py
#   - Collection of display and analysis functions
#----------------------------------------------------------

import sys
import collections
from collections.abc import Sequence
from typing import TypeVar

T = TypeVar('T')
# Coll = Union[Iterable, Sized] = 'Coll'

# Python requires definition before use ordering
#  - no link phase to find definitions
import copy
nl = "\n"

# show name, type, value, and size of a Python instance
def showType(t:T, nm:str, indnt:int = 2, suffix:str = "") :
    print(indent(indnt), nm, ' ', type(t), " dynamic", sep="")
    print(indent(indnt), "value: ", t, ', size: ', sys.getsizeof(t), suffix, sep="")

# generate indent string with n spaces
def indent(n:int) -> str:
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
def showTypeEnum(enum:Sequence, nm:str, left:int = 2, width:int = 7, suffix:str = "") :
    # topStr = indent(left) + nm + type(enum) + "dynamic"
    print(indent(left),nm, ' ', type(enum), ' ', "dynamic", sep='')
    print(indent(left), "{", sep='')
    print(fold(enum, left+2, width))
    print(indent(left), "}", sep = '')
    print(indent(left), "size: ", sys.getsizeof(enum), suffix, sep='')

# same as showType except uses class method to show value
def showTypeShowable(t:T, nm:str, suffix:str = ""):
    print(type(t), "dynamic")
    t.show(nm)

# show Python id, unique for each instance
def showIdent(t:T, name:str, suffix:str = "") :
    print(name, '"{}"'.format(t), id(t), suffix)

# show emphasized note
def showNote(text:str, suffix:str = "", n: int = 50) :
    tmpStr = ""
    for i in range(n):
      tmpStr += '-'
    print(tmpStr)
    print(text)
    print(tmpStr, suffix)

# show delineated string to announce a program operation
def showOp(text:str):
    print("--- {} ---".format(text))

