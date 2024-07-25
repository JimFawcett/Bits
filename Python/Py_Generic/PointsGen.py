#----------------------------------------------------------
#   Py_Generic::PointsGen.py
#   Point[T]
#     Point in N dimensional hyperspace with type T
#     coordinates.
#----------------------------------------------------------

import datetime
import time
import AnalysisGen
from typing import TypeVar, Generic

# Generic point class with N coordinates
#   Python doesn't need generics to support types
#   defined at translation time, but they were
#   introduced to enable translation time type 
#   error detection.

T = TypeVar('T', bound=int | float)

class Point(Generic[T]):
    
    # supports constructor notation
    def __init__(self, n: int) -> None:         # p = Point<double>(8)
        self.coors:list[T] = []
        for i in range(n):
            self.coors.append(0)
        self._dt: datetime = datetime.datetime.now()
    
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

    @property
    def dt(self):
        return self._dt
    
    @dt.getter
    def dt(self):
        return self._dt
    
    def now(self):
        _dt = datetime.datetime.now()

    # show named value of Point[T] instance
    def show(self, name, left = 2, width = 7) :
        print(AnalysisGen.indent(left), name, ' ', sep='')
        print(AnalysisGen.indent(left), "{", sep='')
        print(AnalysisGen.fold(self.coors, left+3, width), sep='')
        print(AnalysisGen.indent(left + 2), self.dt)
        print(AnalysisGen.indent(left), "}", sep = '')
