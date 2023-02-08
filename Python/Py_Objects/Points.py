import Analysis

# point class with three coordinates
class Point1:
    x = 0;
    y = 0;
    z = 0;

    # supports constructor notation
    def __init__(self) -> None:
        pass
    
    # show named value of Point1 instance
    def show(self, name) :
        print("{} {{ ".format(name), end='')
        print(" x:{}, y:{}, z:{}".format(self.x, self.y, self.z), end='');
        print(" }");

# point class with three coordinates
class Point2:
    coors = []

    # supports constructor notation
    def __init__(self, n) -> None:
        self.coors = list()
        for i in range(n):
            self.coors.append(0)
    
    def append(self, t):
        self.coors.append(t)

    def __len__(self):
        return len(self.coors)

    def __getitem__(self, key):
        return self.coors[key]

    def __setitem__(self, key, val):
        self.coors[key] = val

    # show named value of Point2 instance
    def show(self, name) :
        print("{} {{ ".format(name), end='')
        print(" coors:{}".format(self.coors), end='');
        print(" }");

