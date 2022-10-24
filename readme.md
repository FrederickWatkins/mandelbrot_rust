# mandelbrot_rust
## Premise
Mandelbrot rust is a program I've created to learn the basics of rust. It has a simple premise: to provide an API that can be queried to determine if a number is part of the Mandelbrot set. It's designed to be used to render the Mandelbrot set, which is why it contains functions for mapping a screen to the complex plane.
## Design
All of the library source code is in mandelbrot_functions.rs. The runtime configuration is stored in a struct called MandelbrotCalculator. I chose to store it in a struct instead of using static variables because it allows the API to be used to create multiple instances of mandelbrot calculator with different configurations within the same program.