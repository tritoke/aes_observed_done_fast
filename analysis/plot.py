#!/usr/bin/env python

import matplotlib.pyplot as plt
import pandas as pd

df = pd.read_csv("stats_800.csv", header=None).rename(columns={0: "key_byte", 1: "guess", 2: "correlation"})

flag = ""
fig, axes = plt.subplots(nrows=4, ncols=8, figsize=(40, 10), layout="tight")
for i, ax in enumerate(axes.flat):
    data = df[df.key_byte == i]
    ax.bar(data.guess, data.correlation)
    correct = data.correlation.argmax()
    correlation = data.correlation.max()
    ax.set_facecolor("#eeeeee")
    ax.set_xlabel("guess")
    ax.set_ylabel("correlation")
    bbox_props = dict(boxstyle="square,pad=0.3", fc="w", ec="k", lw=0.72)
    kw = dict(xycoords='data',textcoords="axes fraction", bbox=bbox_props, ha="right", va="top")
    ax.annotate(f"{correct} - {chr(correct)!r}", xy=(correct, correlation), xytext=(1/256 * correct + 0.20, 0.96), **kw)
    flag += chr(correct)

fig.suptitle(flag)
plt.savefig("correlations_800.png")
