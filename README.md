<div align="center">
  <img src="https://github.com/ctrlaltf2/solaris/blob/main/assets/banner.png?raw=true" width="600"/>
</div>

<br>

<div align="center">
  Solaris is a Rust library to efficiently predict the position of the Sun at any time and any location on Earth between the years 2017 and 2116. 
</div>

# Paper
Solaris is a *clean-room* Rust implementation of SolTrack ([arXiv:2209.01557v1](https://arxiv.org/abs/2209.01557)). The equations are simple enough to run efficently on embedded systems. The equations were verified by the authors to have high accuracy between 2017 and 2116. This library is an implementation of the paper based _solely on the_ equations presented in the arXiv preprint.

# Validation
The library implementation will be validated using the same methods the paper presents as well as some extra validation. [VSOP87](https://www.caglow.com/info/compute/vsop87) will be used as ground-truth in all tests.

  ## Accuracy
- [ ] What the paper authors did:
  - [ ] VSOP87 comparison: Position of sun seen from Arnhem, Netherlands near sunset/sunrise time
- [ ] Further VSOP87 comparison: 
  - [ ] Random lat/long/time: scale up # timepoints proportionally based on Sun's apparent motion

  ## Performance
- TBD:
  - Need to be able to lock a CPU core to 2.67 GHz for a fair comparison against the paper-- likely possible with Linux

# Further Steps
I intend to take the paper a bit further once I implement my own version in Rust. Most of the paper's optimizations center around dropping higher order terms from standard equations and measuring their effects on the system, effectively finding a local minima where accuracy vs. speed that was good enough to publish. I want to take this a bit further, and allow the number of higher order terms included to be configured at compile-time and on top of that, find other configurations that may compare to their paper in terms of accuracy vs. speed. Each configuration would be defined as the numbers of higher order terms to keep for each of the many sub-equations that make up the mathematical formulas.
  
# Disclaimer
  In addition to the disclaimers noted in the license, the author(s) of this are in no way associated with the authors of the SolTrack paper and make no claim to be.


# FAQ
  ## Why "Solaris"?
Predicting the position of three suns was done by *[Trisolaris](https://www.litcharts.com/lit/the-three-body-problem/terms/trisolaris)*, thus, the prediction of the position of just one sun must be done by none other than *Solaris* (it only makes sense!).

# Citations
```bibtex
@misc{https://doi.org/10.48550/arxiv.2209.01557,
  doi       = {10.48550/ARXIV.2209.01557},
  url       = {https://arxiv.org/abs/2209.01557}, 
  author    = {van der Sluys, Marc and van Kan, Paul},
  keywords  = {Instrumentation and Methods for Astrophysics (astro-ph.IM), Computational Physics (physics.comp-ph), FOS: Physical sciences, FOS: Physical sciences},
  title     = {SolTrack: a free, fast and accurate routine to compute the position of the Sun},
  publisher = {arXiv},
  year      = {2022},
  copyright = {Creative Commons Attribution Non Commercial Share Alike 4.0 International}
}
```
