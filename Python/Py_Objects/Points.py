import Analysis
import datetime
import time

# point class with three coordinates
class Point4D:
    x = 0.0;
    y = 0.0;
    z = 0.0;
    t = datetime.datetime.now();

    # supports constructor notation
    def __init__(self) -> None:
        pass
    
    # show named value of Point4D instance
    def show(self, name) :
        print("{} {{".format(name))
        print("  x:{}, y:{}, z:{}".format(self.x, self.y, self.z));
        print("  t:{}".format(self.t));
        print("}");

