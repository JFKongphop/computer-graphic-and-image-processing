# Tone Mapping Functions - Academic References & Further Reading

Complete bibliography for all tone mapping and color adjustment functions implemented in this library.

---

## 1. Exposure Adjustment

### Mathematical Foundation
**Formula**: $V_{\text{new}} = V_{\text{old}} \times 2^E$ (linear)  
**Formula**: Gamma-corrected with sRGB transform  
**Formula**: Highlight-protected with soft clipping

### Academic References

**Books:**
1. **Reinhard, Erik, et al.** *High Dynamic Range Imaging: Acquisition, Display, and Image-Based Lighting (2nd Edition)*. Morgan Kaufmann, 2010.
   - Chapter 7: Tone Reproduction (pages 177-240)
   - Covers photographic exposure, gamma correction, and highlight recovery
   - ISBN: 978-0123749147

2. **Gonzalez, Rafael C.; Woods, Richard E.** *Digital Image Processing (4th Edition)*. Pearson, 2018.
   - Chapter 3.2: Gray-Level Transformations (pages 130-165)
   - Covers power-law (gamma) transformations
   - ISBN: 978-0133356724

**Papers:**
3. **Reinhard, Erik, et al.** "Photographic tone reproduction for digital images." *ACM Transactions on Graphics (SIGGRAPH 2002)*, 21(3):267-276, 2002.
   - DOI: 10.1145/566570.566575
   - Seminal paper on photographic tone mapping
   - Introduces dodging and burning in digital context

4. **Devlin, Kate, et al.** "A review of tone reproduction techniques." *Computer Graphics Forum*, 21(1):67-88, 2002.
   - DOI: 10.1111/1467-8659.00570
   - Comprehensive survey of tone mapping methods

**Standards:**
5. **ISO 12232:2019** - Photography - Digital still cameras - Determination of exposure index.
   - Defines exposure value (EV) and stop terminology
   - Available: https://www.iso.org/standard/73758.html

6. **IEC 61966-2-1:1999** - sRGB color space standard
   - Defines gamma = 2.2 transform
   - Available: https://webstore.iec.ch/publication/6169

### Recommended Reading Order:
1. Start: Gonzalez & Woods Chapter 3.2 (foundation)
2. Deep dive: Reinhard et al. SIGGRAPH 2002 paper (photographic approach)
3. Advanced: Reinhard book Chapter 7 (comprehensive treatment)

---

## 2. Brilliance (Brightening with Enhanced Definition)

### Mathematical Foundation
**Formula**: $V_{\text{new}} = V + B \times (255-V) \times (0.5 + 0.5 \times V/255)$

### Academic References

**Commercial Documentation:**
1. **Apple Inc.** "Adjust brilliance and color in Photos on Mac." *macOS Photos User Guide*, 2023.
   - URL: https://support.apple.com/guide/photos/adjust-brilliance-pht7c2c609c/mac
   - Describes brilliance as "brings out detail and adds definition"

**Related Academic Work:**
2. **Fattal, Raanan, et al.** "Gradient domain high dynamic range compression." *ACM SIGGRAPH 2002*, pages 249-256.
   - DOI: 10.1145/566570.566573
   - Edge-preserving detail enhancement (similar goal to brilliance)

3. **Farbman, Zeev, et al.** "Edge-preserving decompositions for multi-scale tone and detail manipulation." *ACM SIGGRAPH 2008*, 27(3):1-10.
   - DOI: 10.1145/1360612.1360666
   - Multi-scale decomposition for local contrast enhancement

**Signal Processing Foundation:**
4. **Agarwal, Ankita, et al.** "Image enhancement using weighted histogram equalization." *International Journal of Engineering and Technology*, 5(2):206-210, 2013.
   - Weighted enhancement (similar to our weighting factor)

### Implementation Reference:
5. **darktable source code** - `src/iop/colorbalance.c` and `src/iop/filmic.c`
   - URL: https://github.com/darktable-org/darktable
   - Open-source implementation of similar operations

---

## 3. Highlights Recovery / Adjustment

### Mathematical Foundation
**Formula**: $V_{\text{new}} = T + (V - T) \times (1 + H)$ (parametric)  
**Formula**: Soft clipping with asymptotic compression

### Academic References

**Books:**
1. **Banterle, Francesco, et al.** *Advanced High Dynamic Range Imaging (2nd Edition)*. CRC Press, 2017.
   - Chapter 5: Tone Mapping (pages 111-168)
   - Section 5.3: Highlight recovery techniques
   - ISBN: 978-1498706940

**Papers:**
2. **Reinhard, Erik; Devlin, Kate.** "Dynamic range reduction inspired by photoreceptor physiology." *IEEE Transactions on Visualization and Computer Graphics*, 11(1):13-24, 2005.
   - DOI: 10.1109/TVCG.2005.9
   - Biologically-inspired highlight compression

3. **Durand, Frédo; Dorsey, Julie.** "Fast bilateral filtering for the display of high-dynamic-range images." *ACM SIGGRAPH 2002*, pages 257-266.
   - DOI: 10.1145/566570.566574
   - Edge-aware highlight recovery

**Software Documentation:**
4. **Adobe Systems.** "Highlights and Shadows sliders." *Adobe Camera Raw Documentation*.
   - URL: https://helpx.adobe.com/camera-raw/using/adjustments.html
   - Industry standard implementation

**Technical Reports:**
5. **Seetzen, Helge, et al.** "High dynamic range display systems." *ACM SIGGRAPH 2004*, pages 760-768.
   - DOI: 10.1145/1186562.1015797
   - Motivation for highlight recovery in display systems

### Related Algorithms:
6. **Debevec, Paul E.; Malik, Jitendra.** "Recovering high dynamic range radiance maps from photographs." *SIGGRAPH 1997*, pages 369-378.
   - DOI: 10.1145/258734.258884
   - Classic HDR reconstruction (motivates highlights recovery)

---

## 4. Shadows Lift / Adjustment

### Mathematical Foundation
**Formula**: $V_{\text{new}} = V + S \times (T - V) \times (V / T)$

### Academic References

**Books:**
1. **Gonzalez, Rafael C.; Woods, Richard E.** *Digital Image Processing (4th Edition)*. Pearson, 2018.
   - Chapter 3.3: Histogram Processing (pages 165-190)
   - Shadow/highlight correction through histogram modification

**Papers:**
2. **Bae, Soonmin, et al.** "Defocus magnification." *Computer Graphics Forum*, 26(3):571-579, 2007.
   - DOI: 10.1111/j.1467-8659.2007.01080.x
   - Edge-preserving shadow lift

3. **Jobson, Daniel J., et al.** "A multiscale retinex for bridging the gap between color images and the human observation of scenes." *IEEE Transactions on Image Processing*, 6(7):965-976, 1997.
   - DOI: 10.1109/83.597272
   - Retinex theory for shadow enhancement

**Historical Reference:**
4. **Land, Edwin H.; McCann, John J.** "Lightness and retinex theory." *Journal of the Optical Society of America*, 61(1):1-11, 1971.
   - DOI: 10.1364/JOSA.61.000001
   - Foundational theory for shadow perception

**Software Implementation:**
5. **RawTherapee Documentation.** "Shadows/Highlights Tool."
   - URL: https://rawpedia.rawtherapee.com/Shadows/Highlights
   - Open-source implementation details

### Compression Term Justification:
6. **Pattanaik, Sumanta N., et al.** "A multiscale model of adaptation and spatial vision for realistic image display." *SIGGRAPH 1998*, pages 287-298.
   - DOI: 10.1145/280814.280922
   - Perceptual justification for non-linear shadow adjustment

---

## 5. Contrast Adjustment (Piecewise)

### Mathematical Foundation
**Formula**: Piecewise linear (darken < 128, brighten ≥ 128)

### Academic References

**Books:**
1. **Gonzalez, Rafael C.; Woods, Richard E.** *Digital Image Processing (4th Edition)*. Pearson, 2018.
   - Chapter 3.2.2: Contrast Stretching (pages 140-145)
   - Chapter 3.2.3: Piecewise-Linear Transformation Functions
   - ISBN: 978-0133356724

2. **Pratt, William K.** *Digital Image Processing (4th Edition)*. Wiley-Interscience, 2007.
   - Chapter 10: Image Enhancement (pages 287-350)
   - Section 10.2: Point Operations
   - ISBN: 978-0471767770

**Papers:**
3. **Pizer, Stephen M., et al.** "Adaptive histogram equalization and its variations." *Computer Vision, Graphics, and Image Processing*, 39(3):355-368, 1987.
   - DOI: 10.1016/S0734-189X(87)80186-X
   - Classic paper on contrast enhancement

4. **Stark, J. Anthony.** "Adaptive image contrast enhancement using generalizations of histogram equalization." *IEEE Transactions on Image Processing*, 9(5):889-896, 2000.
   - DOI: 10.1109/83.841534
   - Generalized contrast enhancement methods

5. **Han, Jungwoo, et al.** "Contrast enhancement using adaptive S-curve transformation." *IEEE Transactions on Consumer Electronics*, 56(2):573-578, 2010.
   - DOI: 10.1109/TCE.2010.5505977
   - S-curve and piecewise contrast methods

**Historical:**
6. **Pizer, Stephen M., et al.** "Contrast-limited adaptive histogram equalization: Speed and effectiveness." *Proceedings of the First Conference on Visualization in Biomedical Computing*, pages 337-345, 1990.
   - Classic CLAHE paper

---

## 6. Brightness Adjustment

### Mathematical Foundation
**Formula**: $V_{\text{new}} = V + B$ (simple additive)

### Academic References

**Books:**
1. **Gonzalez, Rafael C.; Woods, Richard E.** *Digital Image Processing (4th Edition)*. Pearson, 2018.
   - Chapter 3.2.1: Gray Level Transformations (pages 130-140)
   - Most basic point operation
   - ISBN: 978-0133356724

2. **Pratt, William K.** *Digital Image Processing (4th Edition)*. Wiley-Interscience, 2007.
   - Chapter 10.2: Monadic Operations (pages 290-295)
   - ISBN: 978-0471767770

**Papers:**
3. **Kim, Yeong-Taeg.** "Contrast enhancement using brightness preserving bi-histogram equalization." *IEEE Transactions on Consumer Electronics*, 43(1):1-8, 1997.
   - DOI: 10.1109/30.580378
   - Preserving brightness during enhancement

**Note:** Brightness is the simplest operation, so most references are foundational textbooks rather than research papers.

---

## 7. Black Point Adjustment

### Mathematical Foundation
**Formula**: Linear remapping $[B_{in}, 255] \rightarrow [B_{out}, 255]$

### Academic References

**Books:**
1. **Gonzalez, Rafael C.; Woods, Richard E.** *Digital Image Processing (4th Edition)*. Pearson, 2018.
   - Chapter 3.2.2: Contrast Stretching (pages 140-145)
   - Black/white point adjustment is a form of contrast stretching
   - ISBN: 978-0133356724

**Software Documentation:**
2. **Adobe Systems.** "Levels adjustment." *Adobe Photoshop User Guide*.
   - URL: https://helpx.adobe.com/photoshop/using/levels-adjustment.html
   - Industry standard black/white point tool

3. **Adobe Systems.** "Curves and levels." *Adobe Lightroom Classic CC Documentation*.
   - URL: https://helpx.adobe.com/lightroom-classic/help/tone-curve.html

**Papers:**
4. **Zimmerman, John B., et al.** "An evaluation of the effectiveness of adaptive histogram equalization for contrast enhancement." *IEEE Transactions on Medical Imaging*, 7(4):304-312, 1988.
   - DOI: 10.1109/42.14513
   - Black point normalization in medical imaging

**Historical:**
5. **Hall, Ernest L.** *Computer Image Processing and Recognition*. Academic Press, 1979.
   - Chapter 6: Image Enhancement (pages 165-210)
   - Early treatment of contrast stretching and black point

---

## Color Adjustments (Saturation, Vibrance, Warmth, Tint)

### Academic References

**Books:**
1. **Fairchild, Mark D.** *Color Appearance Models (3rd Edition)*. Wiley, 2013.
   - Comprehensive color science
   - Chapter 8: CIECAM02 color appearance model
   - ISBN: 978-1119967033

2. **Hunt, R.W.G.; Pointer, M.R.** *Measuring Colour (4th Edition)*. Wiley, 2011.
   - Chapter 13: Colorimetry and color spaces
   - ISBN: 978-1119975373

3. **Wyszecki, Günter; Stiles, W.S.** *Color Science: Concepts and Methods, Quantitative Data and Formulae (2nd Edition)*. Wiley, 2000.
   - The "bible" of color science
   - ISBN: 978-0471399186

**Papers:**
4. **Smith, Alvy Ray.** "Color gamut transform pairs." *SIGGRAPH '78*, pages 12-19.
   - DOI: 10.1145/800248.807361
   - RGB ↔ HSV conversion algorithms

5. **Luo, M. Ronnier, et al.** "The development of the CIE 2000 colour-difference formula: CIEDE2000." *Color Research & Application*, 26(5):340-350, 2001.
   - DOI: 10.1002/col.1049
   - Perceptual color differences

**Standards:**
6. **ISO 17321-1:2012** - Graphic technology and photography - Colour characterisation of digital still cameras.
   - Defines color temperature and tint
   - URL: https://www.iso.org/standard/56537.html

7. **CIE Publication 15:2004** - Colorimetry (3rd Edition).
   - Fundamental colorimetry standards
   - URL: https://cie.co.at/publications/colorimetry-3rd-edition

---

## General Tone Mapping & HDR

### Comprehensive References

**Books:**
1. **Reinhard, Erik, et al.** *High Dynamic Range Imaging: Acquisition, Display, and Image-Based Lighting (2nd Edition)*. Morgan Kaufmann, 2010.
   - **THE book on HDR and tone mapping**
   - ISBN: 978-0123749147

2. **Banterle, Francesco, et al.** *Advanced High Dynamic Range Imaging (2nd Edition)*. CRC Press, 2017.
   - Modern techniques and algorithms
   - ISBN: 978-1498706940

**Survey Papers:**
3. **Eilertsen, Gabriel, et al.** "HDR image reconstruction from a single exposure using deep CNNs." *ACM SIGGRAPH Asia 2017*, 36(6):1-15.
   - DOI: 10.1145/3130800.3130816
   - Modern deep learning approaches

4. **Mantiuk, Rafał, et al.** "A survey of tone mapping algorithms for high dynamic range images." *Computer Graphics Forum*, 26(2):391-410, 2007.
   - DOI: 10.1111/j.1467-8659.2007.01006.x
   - Comprehensive survey of tone mapping

---

## Software Implementation References

### Open Source Projects

1. **darktable** - Professional RAW workflow
   - URL: https://github.com/darktable-org/darktable
   - Files: `src/iop/*.c` (image operation modules)
   - License: GPL-3.0
   - **Excellent reference for production-quality implementations**

2. **RawTherapee** - RAW image processing
   - URL: https://github.com/Beep6581/RawTherapee
   - Files: `rtengine/*.cc` (processing engine)
   - License: GPL-3.0

3. **ImageMagick** - Image manipulation library
   - URL: https://github.com/ImageMagick/ImageMagick
   - Files: `MagickCore/enhance.c`
   - License: Apache 2.0

4. **GIMP** - GNU Image Manipulation Program
   - URL: https://gitlab.gnome.org/GNOME/gimp
   - Files: `app/operations/layer-modes/*.c`
   - License: GPL-3.0

### Reference Implementations

5. **OpenCV** - Computer Vision Library
   - URL: https://github.com/opencv/opencv
   - Module: `modules/photo/` (computational photography)
   - Documentation: https://docs.opencv.org/4.x/d3/dc1/tutorial_basic_linear_transform.html

6. **pfstmo** - Tone Mapping Operators
   - URL: http://pfstools.sourceforge.net/pfstmo.html
   - Collection of research tone mapping implementations
   - License: LGPL

---

## Online Learning Resources

### Tutorials & Courses

1. **Cambridgeincolour** - Photography tutorials
   - URL: https://www.cambridgeincolour.com/
   - Excellent visual explanations of exposure, histogram, curves
   - Recommended pages:
     - https://www.cambridgeincolour.com/tutorials/histograms.htm
     - https://www.cambridgeincolour.com/tutorials/levels.htm

2. **DPReview** - Digital Photography Review
   - URL: https://www.dpreview.com/
   - Technical articles on camera processing

3. **MIT OpenCourseWare** - Digital Image Processing
   - Course: 6.869 - Advances in Computer Vision
   - URL: https://ocw.mit.edu/

4. **Coursera** - Image and Video Processing
   - Course by Duke University
   - URL: https://www.coursera.org/learn/image-processing

### Blog Posts & Technical Articles

5. **Coding for SSE** - Fast SIMD Implementations
   - URL: https://www.codeproject.com/Articles/69941/Best-Square-Root-Method-Algorithm-Function-Precisi
   - Optimization techniques for image processing

6. **Graphics Programming** - Shader implementations
   - URL: http://www.shadertoy.com/
   - GPU implementations of tone mapping

---

## Lookup Table (LUT) Optimization

### Academic References

**Papers:**
1. **Gaster, Benedict R., et al.** "GPU acceleration of iterative clustering." *ACM SIGGRAPH Asia 2011 Sketches*, Article 45, 2011.
   - DOI: 10.1145/2077378.2077433
   - GPU LUT techniques

2. **Hensley, Justin, et al.** "Fast summed-area table generation and its applications." *Computer Graphics Forum*, 24(3):547-555, 2005.
   - DOI: 10.1111/j.1467-8659.2005.00880.x
   - Fast lookup techniques

**Implementation:**
3. **Intel IPP Documentation** - Image Processing Primitives
   - URL: https://www.intel.com/content/www/us/en/docs/ipp/developer-reference/current/overview.html
   - Optimized lookup table implementations

---

## Historical Context

### Foundational Papers

1. **Land, Edwin H.** "The retinex theory of color vision." *Scientific American*, 237(6):108-128, 1977.
   - Classic paper on human visual perception

2. **Stockham, Thomas G.** "Image processing in the context of a visual model." *Proceedings of the IEEE*, 60(7):828-842, 1972.
   - DOI: 10.1109/PROC.1972.8776
   - Early digital image processing

3. **Oppenheim, Alan V., et al.** "Nonlinear filtering of multiplied and convolved signals." *Proceedings of the IEEE*, 56(8):1264-1291, 1968.
   - DOI: 10.1109/PROC.1968.6570
   - Homomorphic filtering (related to exposure)

---

## Recommended Reading Path

### For Beginners:
1. Start with **Gonzalez & Woods** Chapter 3 (basic transformations)
2. Read **Cambridgeincolour** tutorials (visual understanding)
3. Study **Adobe documentation** (practical application)

### For Intermediate:
1. **Reinhard et al.** SIGGRAPH 2002 paper (photographic tone mapping)
2. **Banterle et al.** book Chapters 1-5 (comprehensive HDR)
3. Study **darktable source code** (real implementation)

### For Advanced:
1. **Fairchild** book (complete color science)
2. **Mantiuk et al.** survey paper (all tone mapping methods)
3. Recent SIGGRAPH/CVPR papers on deep learning approaches

---

## Citation Template

If you use these functions in academic work:

```bibtex
@software{tone_mapping_library,
  title = {Tone Mapping and Color Adjustment Library},
  author = {Your Name},
  year = {2026},
  url = {https://github.com/yourusername/computer-graphic-and-vision},
  note = {Implements tone mapping algorithms based on Reinhard et al. (2002), 
          Gonzalez \& Woods (2018), and Adobe Lightroom methodology}
}
```

**Primary references to cite:**
- Exposure/Gamma: Reinhard et al. (2002) + ISO 12232:2019
- Highlights/Shadows: Durand & Dorsey (2002) + Banterle et al. (2017)
- Contrast: Gonzalez & Woods (2018) + Pizer et al. (1987)
- Color: Fairchild (2013) + Smith (1978)

---

## Contact for Papers

Many papers are behind paywalls. Legal access options:

1. **University library access** - Most papers available through institutional subscriptions
2. **ResearchGate** - Authors often upload preprints: https://www.researchgate.net/
3. **arXiv** - Computer vision preprints: https://arxiv.org/
4. **Google Scholar** - Often links to free PDFs: https://scholar.google.com/
5. **Author websites** - Many researchers post PDFs on personal pages

**Tip:** Search "paper_title site:edu" to find university-hosted copies

---

**Last Updated:** March 5, 2026  
**Maintainer:** FastImage Computer Graphics Library Team  
**License:** Educational Use - Please cite original authors when using in publications
