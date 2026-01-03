# Mathematical References for FastImage Graphics Library

This document lists all 14 core mathematical concepts used in the FastImage system with their academic and technical references.

---

## 1. Data Conversion (Matrix to Buffer)

**Formula:** Total bytes = width × height × 3

**References:**
- OpenCV Documentation: [Mat Class Reference](https://docs.opencv.org/master/d3/d63/classcv_1_1Mat.html)
- OpenCV Documentation: [Basic Structures - Mat](https://docs.opencv.org/4.x/d6/d6d/tutorial_mat_the_basic_image_container.html)

---

## 2. Pixel Indexing (2D to 1D Conversion)

**Formula:** idx = (y × width + x) × 3

**References:**
- Hughes, John F., et al. *Computer Graphics: Principles and Practice (3rd Edition)*. Addison-Wesley, 2013. Chapter 3: Raster Images.
- Marschner, Steve; Shirley, Peter. *Fundamentals of Computer Graphics (5th Edition)*. CRC Press, 2021. Chapter 3: Raster Images.

---

## 3. Alpha Blending (Compositing)

**Formula:** C_new = C_old + (C_new - C_old) × α

**References:**
- **Porter, Thomas; Tom Duff (1984). "Compositing Digital Images". *SIGGRAPH '84 Proceedings*. ACM. pp. 253–259. DOI: [10.1145/800031.808606](https://dl.acm.org/doi/epdf/10.1145/800031.808606)**
- Wikipedia: [Alpha compositing](https://en.wikipedia.org/wiki/Alpha_compositing)
- OpenGL Specification: [Blending](https://registry.khronos.org/OpenGL/specs/gl/glspec46.core.pdf) (Chapter 17)

---

## 4. Circle Equation

**Formula:** x² + y² ≤ r²

**References:**
- Wikipedia: [Circle - Equations](https://en.wikipedia.org/wiki/Circle#Equations)
- Weisstein, Eric W. "Circle." *MathWorld--A Wolfram Web Resource*. [https://mathworld.wolfram.com/Circle.html](https://mathworld.wolfram.com/Circle.html)
- Marschner & Shirley. *Fundamentals of Computer Graphics*. Chapter 2: Miscellaneous Math.

---

## 5. Steepness Test (Line Classification)

**Formula:** steep = |Δy| > |Δx|

**References:**
- Bresenham, J. E. (1965). "Algorithm for computer control of a digital plotter". *IBM Systems Journal*, 4(1): 25–30. DOI: [10.1147/sj.41.0025](https://doi.org/10.1147/sj.41.0025)
- Wikipedia: [Bresenham's line algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm)

---

## 6. Gradient/Slope Calculation

**Formula:** gradient = Δy / Δx

**References:**
- Wikipedia: [Slope](https://en.wikipedia.org/wiki/Slope)
- Weisstein, Eric W. "Slope." *MathWorld--A Wolfram Web Resource*. [https://mathworld.wolfram.com/Slope.html](https://mathworld.wolfram.com/Slope.html)
- Any basic algebra textbook (fundamental concept)

---

## 7. Number Decomposition (Integer & Fractional Parts)

**Formula:** x = ⌊x⌋ + frac(x)

**References:**
- Wikipedia: [Floor and ceiling functions](https://en.wikipedia.org/wiki/Floor_and_ceiling_functions)
- Wikipedia: [Fractional part](https://en.wikipedia.org/wiki/Fractional_part)
- Weisstein, Eric W. "Floor Function." *MathWorld*. [https://mathworld.wolfram.com/FloorFunction.html](https://mathworld.wolfram.com/FloorFunction.html)

---

## 8. Wu's Anti-Aliasing Algorithm

**Formula:** α_lower = 1 - frac(y), α_upper = frac(y)

**References:**
- **Wu, Xiaolin (1991). "An efficient antialiasing technique". *Computer Graphics*, 25(4): 143–152. DOI: [10.1145/127719.122734](https://doi.org/10.1145/127719.122734)**
- Wikipedia: [Xiaolin Wu's line algorithm](https://en.wikipedia.org/wiki/Xiaolin_Wu%27s_line_algorithm)
- Hughes et al. *Computer Graphics: Principles and Practice*. Chapter 8: The Graphics Pipeline.

---

## 9. Vector Length (Euclidean Distance)

**Formula:** length = √(dx² + dy²)

**References:**
- Wikipedia: [Euclidean distance](https://en.wikipedia.org/wiki/Euclidean_distance)
- Wikipedia: [Pythagorean theorem](https://en.wikipedia.org/wiki/Pythagorean_theorem)
- Marschner & Shirley. *Fundamentals of Computer Graphics*. Chapter 2: Miscellaneous Math - Vectors.

---

## 10. Perpendicular Unit Vector (Normalization & Rotation)

**Formula:** p⊥ = (-dy/length, dx/length)

**References:**
- Wikipedia: [Rotation matrix - 2D rotation](https://en.wikipedia.org/wiki/Rotation_matrix#In_two_dimensions)
- Wikipedia: [Unit vector](https://en.wikipedia.org/wiki/Unit_vector)
- Marschner & Shirley. *Fundamentals of Computer Graphics*. Chapter 2: Miscellaneous Math - 2D Transformations.
- Weisstein, Eric W. "Perpendicular." *MathWorld*. [https://mathworld.wolfram.com/Perpendicular.html](https://mathworld.wolfram.com/Perpendicular.html)

---

## 11. Parallel Line Offset (Thick Line Drawing)

**Formula:** offset = i - (thickness/2) + 0.5

**References:**
- Hughes et al. *Computer Graphics: Principles and Practice*. Chapter 8: Line Drawing.
- Akenine-Möller, Tomas; Haines, Eric; Hoffman, Naty. *Real-Time Rendering (4th Edition)*. CRC Press, 2018. Chapter 23: Graphics Hardware.

---

## 12. Image Scaling (Aspect Ratio Preservation)

**Formula:** scale = min(max_dim / max_side, 1.0)

**References:**
- Wikipedia: [Image scaling](https://en.wikipedia.org/wiki/Image_scaling)
- Wikipedia: [Aspect ratio (image)](https://en.wikipedia.org/wiki/Aspect_ratio_(image))
- OpenCV Documentation: [Geometric Image Transformations](https://docs.opencv.org/4.x/da/d54/group__imgproc__transform.html)
- Wikipedia: [Lanczos resampling](https://en.wikipedia.org/wiki/Lanczos_resampling)

---

## 13. Min-Max Normalization (Feature Scaling)

**Formula:** normalized = (value - min) / (max - min)

**References:**
- Wikipedia: [Feature scaling - Rescaling (min-max normalization)](https://en.wikipedia.org/wiki/Feature_scaling#Rescaling_(min-max_normalization))
- Han, Jiawei; Kamber, Micheline; Pei, Jian. *Data Mining: Concepts and Techniques (3rd Edition)*. Morgan Kaufmann, 2011. Chapter 3: Data Preprocessing.

---

## 14. Pixel Coordinate Mapping (GPS to Image)

**Formula:** x_pixel = (offset_x + n_x × scale) × image_width

**References:**
- Wikipedia: [Geographic coordinate conversion](https://en.wikipedia.org/wiki/Geographic_coordinate_conversion)
- Garmin: [FIT SDK Documentation](https://developer.garmin.com/fit/protocol/)
- Hughes et al. *Computer Graphics: Principles and Practice*. Chapter 7: Viewing - Viewport Transformation.
- Wikipedia: [Linear map](https://en.wikipedia.org/wiki/Linear_map)

---

## Additional General References

### Books
1. **Hughes, John F., et al.** *Computer Graphics: Principles and Practice (3rd Edition)*. Addison-Wesley Professional, 2013. ISBN: 978-0321399526
2. **Marschner, Steve; Shirley, Peter.** *Fundamentals of Computer Graphics (5th Edition)*. CRC Press, 2021. ISBN: 978-0367505035
3. **Akenine-Möller, Tomas; Haines, Eric; Hoffman, Naty.** *Real-Time Rendering (4th Edition)*. CRC Press, 2018. ISBN: 978-1138627000

### Technical Standards
- OpenGL 4.6 Core Profile Specification: [https://registry.khronos.org/OpenGL/specs/gl/glspec46.core.pdf](https://registry.khronos.org/OpenGL/specs/gl/glspec46.core.pdf)
- FIT Protocol: [https://developer.garmin.com/fit/protocol/](https://developer.garmin.com/fit/protocol/)

### Online Documentation
- OpenCV Documentation: [https://docs.opencv.org/](https://docs.opencv.org/)
- MathWorld (Wolfram): [https://mathworld.wolfram.com/](https://mathworld.wolfram.com/)
- Wikipedia Mathematics Portal: [https://en.wikipedia.org/wiki/Portal:Mathematics](https://en.wikipedia.org/wiki/Portal:Mathematics)

---

## Key Papers (Full Citations)

### Porter-Duff Compositing (Alpha Blending)
```
Porter, T., & Duff, T. (1984). Compositing digital images. 
In Proceedings of the 11th annual conference on Computer graphics 
and interactive techniques (SIGGRAPH '84) (pp. 253-259). 
Association for Computing Machinery, New York, NY, USA.
DOI: https://doi.org/10.1145/800031.808606
```

### Xiaolin Wu's Line Algorithm
```
Wu, X. (1991). An efficient antialiasing technique. 
In Proceedings of the 18th annual conference on Computer graphics 
and interactive techniques (SIGGRAPH '91) (pp. 143-152). 
Association for Computing Machinery, New York, NY, USA.
DOI: https://doi.org/10.1145/127719.122734
```

### Bresenham's Line Algorithm
```
Bresenham, J. E. (1965). Algorithm for computer control 
of a digital plotter. IBM Systems Journal, 4(1), 25-30.
DOI: https://doi.org/10.1147/sj.41.0025
```

---

## Citation Format

When citing this work, please reference:

**For academic papers:**
> FastImage: Mathematical Foundations of 2D Computer Graphics for GPS Visualization. 
> Educational Documentation, 2026.

**For each specific algorithm:**
- Alpha Blending: Porter & Duff (1984)
- Anti-aliased Lines: Wu (1991)
- Circle Drawing: Standard circle equation
- GPS Normalization: Min-max feature scaling

---

**Document Version:** 1.0  
**Last Updated:** January 2, 2026  
**License:** Educational Use
