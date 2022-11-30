# Solaris
A **clean-room** Rust implementation of SolTrack (arXiv:2209.01557v1).

Equations were tested by paper authors to be valid between **2017** and **2116**.

# Motivation
1. Learning a new language
2. Implementing the equations of SolTrack under a more widely-known license
3. Having fun!

# Validation
## Accuracy
- [ ] What the paper authors did:
  - [ ] VSOP87 comparison: Position of sun seen from Arnhem, Netherlands near sunset/sunrise time
- [ ] Further VSOP87 comparison: Random lat/long/time, scale up # timepoints proportionally based on size of sunset/sunrise time slice

## Performance
- TBD:
  - Need to be able to lock a CPU core to 2.67 GHz for a fair comparison against the paper-- likely possible just haven't done it before


# FAQ
## Why "Solaris"?
While yes, this name overloads the [operating system of the same name](https://en.wikipedia.org/wiki/Oracle_Solaris), I chose Solaris as a reference to a well-known sci-fi novel. Predicting the position of three suns was done by *[Trisolaris](https://www.litcharts.com/lit/the-three-body-problem/terms/trisolaris)*, thus, the prediction of the position of just one sun must be done by none other than *Solaris* (it only makes sense!).
