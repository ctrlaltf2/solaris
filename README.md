<div align="center">
  <img src="https://raw.githubusercontent.com/ctrlaltf2/solaris/main/assets/banner.png?token=GHSAT0AAAAAAB3UX2JQT2ZPVMILBNAD3TXOY4IFNBQ" alt="Solaris banner logo" width="400"/>
</div>

<br>

Solaris is a **clean-room** Rust implementation of SolTrack ([arXiv:2209.01557v1](https://arxiv.org/abs/2209.01557)). SolTrack is a modern and optimized set of equations to predict the position of the Sun at any time and location on Earth. The equations are simple enough to run efficently on embedded systems, including but not limited to microcontrollers or PLCs. Equations were verified to have high accuracy between **2017** and **2116**. This library is a clean-room implementation of the equations presented in the paper; clean-room implementation is done in order to license this as MIT without any relicensing shenanigans.

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
