<p align=center>
<img src="logo.png" alt="logo" width="256" height="256"></p>


<h1 align="center">Generative</h1>

<div align="center">
  
  [![GitHub license](https://img.shields.io/github/license/gp-97/generative?style=for-the-badge&logo=apache)](https://github.com/gp-97/generative/blob/master/LICENSE)
  [![Status](https://img.shields.io/badge/status-active-success?style=for-the-badge&logo=statuspal)]()
  [![GitHub issues](https://img.shields.io/github/issues/gp-97/generative?style=for-the-badge&logo=github)](https://github.com/gp-97/generative/issues)
  [![GitHub Pull Requests](https://img.shields.io/github/issues-pr/gp-97/generative?style=for-the-badge&logo=github-actions)](https://github.com/gp-97/generative/pulls)
  ![GitHub Workflow Status](https://img.shields.io/github/workflow/status/gp-97/generative/Continuous%20Integration?logo=github&style=for-the-badge)

</div>

---

**Generative** (__WIP__) is 2D generational arts creation library written in Rust.
Currently it is in nascent stage and is somewhat unstable.

## Example Outputs
<details>
<summary>Outputs</summary>

#### Perlin Loop
![Perlin Loop](examples/outputs/perlin_loop.png)
#### Watercolored Cirles
![Watercolored Circles](examples/outputs/watercolor_circles.png)
#### Mandelbrot
![Mandelbrot](examples/outputs/mandelbrot.png)
#### Generating sinusoidal wave image
|Original|Wave Image|
|:------:|:--------:|
|![Original](examples/inputs/animal.jpg)|![Generated](examples/outputs/animal_wave.png)|

</details>

## Current features:
- [ ] 2D Shape + Transformations (Translation, Rotation, Shearing in X and Y) + Anti-Aliasing
  - [x] Lines
  - [x] Line from multiple points
  - [x] Squares
  - [x] Rectangles
  - [x] Polygons
  - [x] Circles
  - [ ] Ellipse
- [x] Curves + Transformations (Translation, Rotation, Shearing in X and Y) + Anti-Aliasing
  - [x] Catmull-Rom Spline (Uniform + Centripetal + Chordal)
  - [x] n-degree Bezier curve
## TODO:
- A lot of things
