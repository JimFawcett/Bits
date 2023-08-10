import sys
# Python/Py_Data::Py_DataAnalysis.py
# 

nl = "\n"

def showType(t, nm, suffix = "") :
    print(nm, type(t))
    print("value: ", t, ', size: ', sys.getsizeof(t), suffix)

def showIdent(t, nm, suffix = "") :
    print(nm, t, ":", id(t), suffix, sep=' ')

def showNote(text, suffix = "") :
    print("----------------------------------------")
    print(" ", text)
    print("----------------------------------------", suffix)

def showOp(text):
    print("--", text, "--")