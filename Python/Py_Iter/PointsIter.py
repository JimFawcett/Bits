#----------------------------------------------------------
#   Py_Generic::PointsGen.py
#   Point[T]
#     Point in N dimensional hyperspace with type T
#     coordinates.
#----------------------------------------------------------

import AnalysisIter
from typing import TypeVar, Generic, Iterator

# Generic point class with N coordinates
# - Supports indexing and iteration

T = TypeVar('T', bound=int | float)

Anal = AnalysisIter

class Point(Generic[T]):
    
    # supports constructor notation
    def __init__(self, n: int) -> None:         # p = Point<double>(8)
        self.coors:list[T] = []
        for i in range(n):
            self.coors.append(0)    # there doesn't seem to be a default
                                    # value for T, e.g., T::default()
    def append(self, t: T):
        self.coors.append(t)

    def len(self) -> int:                       # p.len()
        return len(self.coors)

    def __len__(self) -> int:                   # len(p)
        return len(self.coors)

    def __getitem__(self, key: int) -> T:       # value = p[1]
        return self.coors[key]

    def __setitem__(self, key: int, val: T):    # p[1] = value
        self.coors[key] = val

    def iter(self) -> Iterator:                 # p.iter()
        return self.coors.__iter__()
    
    # Point does not have to define an iterator. 
    # It simply uses coors iterator.
    
    # show named value of Point[T] instance
    def show(self, name, left = 0, width = 7) :
        print(Anal.indent(left), name, ' {', sep='')
        print(Anal.fold(self.coors, left+2, width))
        print(Anal.indent(left), "}", sep = '')
