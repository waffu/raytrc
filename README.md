# waytracer
waytracer is a Rust port of the C++ raytracer described in Ray Tracing in One Weekend with additional features such as multi-threaded ray calculations, logging and PNG output.

# Usage

To modify and generate the render:

1. Change the render parameters such as ```{dimensions, samples per pixel}``` in 'settings.toml'

2. Change the amount and parameters of 3D instances in 'world.json'

proceed to build and run the project with 

```cargo run --release```

generating the file ```img.{ext}``` in the project directory.

# Preview

![A Render](img.png "Render")

