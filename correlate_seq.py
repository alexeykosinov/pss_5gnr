import matplotlib.pyplot as plt
import numpy as np

def plot_ccor(x, y, plot_name, ax): 
    # Plot cross-correlation between two signals
    N = max(len(x), len(y)) 
    n = min(len(x), len(y)) 

    if N == len(y): 
        lags = np.arange(-N + 1, n) 
    else: 
        lags = np.arange(-n + 1, N) 

    c = np.correlate(x / np.std(x), y / np.std(y), 'full') 

    ax = plt.title(plot_name)
    ax = plt.plot(lags, c / n) 


# Pregenerated from rust
n_id2_0 = np.array([0x9CF6, 0xC19A, 0xC5D6, 0x2044, 0xDE58, 0x8FE0, 0xA51A, 0x42AE], np.int16)
n_id2_1 = np.array([0x0898, 0xCB24, 0xFC3A, 0xA370, 0x55F4, 0xCF68, 0x19A8, 0x5D6C], np.int16)
n_id2_2 = np.array([0x6E3E, 0xBE94, 0xED0A, 0x3538, 0xAD82, 0x898A, 0xB240, 0xC3BC], np.int16)

test = np.random.randint(-9999,9999,(4,16), dtype='int16')

hidden_data_0 = (test[0], n_id2_0, test[1], test[2], test[3])
hidden_data_1 = (test[0], test[1], test[2], n_id2_1, test[3])
hidden_data_2 = (test[0], test[1], n_id2_2, test[2], test[3])

sum_of_0 = np.concatenate(hidden_data_0)
sum_of_1 = np.concatenate(hidden_data_1)
sum_of_2 = np.concatenate(hidden_data_2)

fig = plt.figure()

ax1 = fig.add_subplot(311)
plot_ccor(n_id2_0, sum_of_0, 'N_ID2 = 0', ax1)
ax2 = fig.add_subplot(312)
plot_ccor(n_id2_1, sum_of_1, 'N_ID2 = 1', ax2)
ax3 = fig.add_subplot(313)
plot_ccor(n_id2_2, sum_of_2, 'N_ID2 = 2', ax3)

plt.show() 
