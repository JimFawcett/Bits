import sys
from ctypes import c_int, addressof

# Python/Py_Data::Py_DataAnalysis.py
# 

nl = "\n"

# displays type, value, and size of apex object
# - does not account for sizes of decendent objects
def showType(t, nm, suffix = "") :
    print(nm, type(t))
    print("value: ", t, ', size: ', sys.getsizeof(t), suffix)

# id is heap address
def showIdent(t, nm, suffix = "") :
    print(nm, ": ", hex(id(t)), suffix, sep='')

# evaluates heap address
def showAddress(t, nm, suffix = "") :
    print(nm, "address: ", hex(id(t)), suffix, sep='')

# show text encased in upper and lower lines
def showNote(text, suffix = "") :
    print("----------------------------------------")
    print(" ", text)
    print("----------------------------------------", suffix)

# show text enclosed in -- delimiters on same line
def showOp(text, suffix = ""):
    print("--", text, "--", suffix)