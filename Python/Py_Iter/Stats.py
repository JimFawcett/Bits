#----------------------------------------------------------
#   Py_Generic::Stats.py
#   Stats[T]
#     Collection of T items with simple statistics.
#----------------------------------------------------------

import AnalysisIter
from typing import TypeVar, Generic, Iterator

# Generic point class with N coordinates
# - Supports indexing and iteration

T = TypeVar('T', bound=int | float)

Anal = AnalysisIter

class Stats(Generic[T]):
  def __init__(self, l:list[T]) -> None:
    self.items = l

  def len(self) -> int:
    return self.items.len()

  def max(self) -> T | None:
    if not self.items:
      return None
    max:T = self.items[0]
    for item in self.items:
      if max < item:
        max = item
    return max
  
  def min(self) -> T | None:
    if not self.items:
      return None
    min:T = self.items[0]
    for item in self.items:
      if min > item:
        min = item
    return min
  
  def sum(self) -> T:
    sum:T = 0
    for item in self.items:
      sum += item
    return sum
  
  def avg(self) -> T | None:
     if not self.items:
       return None
     num = self.sum()
     den = len(self.items)
     return num/den
     
  def __iter__(self) -> Iterator:                 # p.iter()
      return self.items.__iter__()
