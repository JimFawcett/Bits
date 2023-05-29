#----------------------------------------------------------
# PointN[T]
#   Point in N dimensional hyperspace with type T
#   coordinates.
#----------------------------------------------------------

import Analysis
from typing import TypeVar, Generic

# Generic point class with N coordinates
#   Python doesn't need generics to support types
#   defined at translation time, but they were
#   introduced to enable translation time type 
#   error detection.

T = TypeVar('T', bound=int | float)

class PointN(Generic[T]):
    
    # supports constructor notation
    def __init__(self, n: int) -> None:         # p = PointN<double>(8)
        self.coors:list[T] = []
        for i in range(n):
            self.coors.append(0)
    
    def append(self, t: T):
        self.coors.append(t)

    def len(self) -> int:
        return len(self.coors)

    def __len__(self) -> int:                   # len(p)
        return len(self.coors)

    def __getitem__(self, key: int) -> T:       # value = p[1]
        return self.coors[key]

    def __setitem__(self, key: int, val: T):    # p[1] = value
        self.coors[key] = val

    # show named value of PointN[T] instance
    def show(self, name, left = 0, width = 7) :
        print(Analysis.indent(left), name, ' ', sep='')
        print(Analysis.indent(left), "{", sep='')
        print(Analysis.fold(self.coors, left+2, width))
        print(Analysis.indent(left), "}", sep = '')
