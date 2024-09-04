# ppm_convert

Command line converter that converts ppm ascii image files (i.e. P3 type) to either .png or .jpg/.jpeg.

# How to build

You must have rust installed on your machine.

Grab the repo:
```
git clone https://github.com/cainmartin/ppm_convert.git
```

Build a release / debug version as required
```
cd ppm_convert
cargo build --release
```


# How to use

This assumes you are in the directory with the executable, or it is in your system path.

Run the command to convert the image files
```
ppm_convert --input input.ppm --output output.png
```

Other commands
```
ppm_convert --help
```

# Other information

This was a quick project to solve a problem I had when working on my raytracer. Because that 
project worked on .ppm files, and not all platforms provided good tools for viewing .ppm files.
So this tool is there to solve a specific problem - however, if you have ascii PPM files that you
need to convert, then this might just be a solution.

## To Do

I want to add in the future support for the P6 binary version of the file, it also requires some
additional testing and code tidy up. 