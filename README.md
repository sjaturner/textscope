# This is textscope, a minimal text based scope trace for streams of data where each line contains an epoch and value

This is really just an excuse to write some Rust.

Use case: 

* You are monitoring a changing value in a stream of text.
* You are not great at visualising numbers as graphs, particularly when the readings arrive fast.
* You are logged on to a remote machine and you are too lazy to set up X window, Octave or Gnuplot.
* You just want a quick look at the data and you are not at all fussy about Axis lables, etc.

Expected input format:


