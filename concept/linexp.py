import numpy as np
import matplotlib.pyplot as plt

class LinExp():
    def __init__(self):
        self.s = 0
        self._update_parameters()

    def _update_parameters(self):
        self.ya = lambda x: x*(4*self.s - 1)/(2*self.s + 1) + 1
        self.yb = lambda x: x*(- 1)/(2*self.s + 1) + 1 + self.s
        self.yc = lambda x: x*(-2*self.s - 1) + (1 + self.s)*(2*self.s + 1)
        self.yd = lambda x: x*(-2*self.s - 1)/(1 - 4*self.s) + (1 + 2*self.s)/(1 - 4*self.s)
    
        self.xa_lim = self.s/2 + .25
        self.xb_lim = self.s + .5
        self.xc_lim = self.s + .75
        self.xd_lim = 1

    def set_s(self, s):
        self.s = s
        self._update_parameters()


    def y(self, x):
        idx_a = np.where((0 <= x) & (x < self.xa_lim))[0]
        idx_b = np.where((self.xa_lim <= x) & (x < self.xb_lim))[0]
        idx_c = np.where((self.xb_lim <= x) & (x < self.xc_lim))[0]
        idx_d = np.where((self.xc_lim <= x) & (x < self.xd_lim))[0]
        # print(idx_a, idx_b, idx_c, idx_d)
        y = np.zeros(len(x))
        y[idx_a] = self.ya(x[idx_a])
        y[idx_b] = self.yb(x[idx_b])
        y[idx_c] = self.yc(x[idx_c])
        y[idx_d] = self.yd(x[idx_d])
        return y

if __name__ == '__main__':
    linexp = LinExp()
    x = np.linspace(0, 1, 10)

    for s in np.linspace(0, -0.5, 10, endpoint=False):
        linexp.set_s(s)
        plt.plot(x, linexp.y(x), 'x', alpha=.5)
        plt.xlim([0, 1])
        plt.ylim([0, 1])
        plt.grid(True)
    plt.show()





