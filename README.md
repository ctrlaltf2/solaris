<div align="center">
  ![Solaris banner logo](assets/banner-logo.png)
</div>

<br>

<div align="center">
  Solaris is a Rust library to efficiently predict the position of the Sun at any time and any location on Earth between the years 2017 and 2116. 
</div>

# Paper
Solaris is a *clean-room* Rust implementation of SolTrack ([arXiv:2209.01557v1](https://arxiv.org/abs/2209.01557)). The equations are simple enough to run efficently on embedded systems. Equations were verified to have high accuracy between 2017 and 2116. This library is an implementation based solely on the equations presented in the arXiv preprint.

# Validation
The library implementation will be validated using the same methods the paper presents as well as some extra validation. [VSOP87](https://www.caglow.com/info/compute/vsop87) will be used as ground-truth in all tests.

## Accuracy
- [ ] What the paper authors did:
  - [ ] VSOP87 comparison: Position of sun seen from Arnhem, Netherlands near sunset/sunrise time
- [ ] Further VSOP87 comparison: Random lat/long/time, scale up # timepoints proportionally based on size of sunset/sunrise time slice

## Performance
- TBD:
  - Need to be able to lock a CPU core to 2.67 GHz for a fair comparison against the paper-- likely possible just haven't done it before


# FAQ
## Why "Solaris"?
Predicting the position of three suns was done by *[Trisolaris](https://www.litcharts.com/lit/the-three-body-problem/terms/trisolaris)*, thus, the prediction of the position of just one sun must be done by none other than *Solaris* (it only makes sense!).
