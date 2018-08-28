# Pattern Detector

## Overview

An algorithm to detect if a sequence contains a repeating pattern.

For example, the sequence:

```
0, 0, 1, 0, 2,
0, 0, 1, 0, 2,
0, 0, 1, 0, 2,
```

Contains three copies of the pattern `[0, 0, 1, 0, 2]`.

The algorithm takes three arguments as inputs:

* the sequence to test
* the minimum sequence length to consider
* the maximum sequence length to consider

## Context

This code resulted from a programming problem I made up for myself while listening to some complex drumming. I was interested in the computational complexity of finding repeated sequences in a simple discrete-time problem.

This algorithm fits broadly into the realms of data compression and string algorithms. Thinking about standing waves in physics helped unlock some optimizations.

My apologies; the description below is far from complete. If this algorithm interests you, please reach out -- this feedback will give me extra incentive to elaborate on the details below.

## Algorithm

This algorithm uses dynamic programming.

### Data Structures

There are two supporting internal data structures:

* A vector - serves as a one-dimensional table to track the status of each possible pattern length.
* A queue - orders the optional second phase of the algorithm.

### Phases

The algorithm has three phases:

* Phase 1
	* Loop repeatedly over each largest untested pattern length
	* If no pattern matches, returns `None`
	* If a pattern matches, add factors of the current length to the queue in descending order, then switch to phase 2

* Phase 2 (Optional)
	* Loop repeatedly over entries in the queue
	* If a pattern matches, clear queue and add factors of the current length to the queue in descending order

* Wrap-up
	* Returns the shortest pattern length that matched

### Foundations

The correctness of the algorithm relies on two mathematical claims. If patterns are checked from largest to smallest, if pattern of length `k` is detected, then:

1. it is *necessary* to check pattern lengths that are factors of `k`.

2. it is *sufficient* to only check pattern lengths that are factors of `k`.

### Future Work

Try out a similar algorithm that traverses pattern lengths in increasing (instead of decreasing) order.
