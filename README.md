# ESP Isolator
Reads the statics from a ESP file and packages the ESP, meshes, and textures into a single zip. I use this for submitting files to the Tamriel Rebuilt website.

# Installation
Download the file from the [releases](https://github.com/nicholas477/esp-isolator/releases) page. Unzip it anywhere. I put it next to my ESPs for ease of use.

# How to use
On Windows, drag and drop your ESP file onto the program, and it will create a zip file next to your ESP.

There's also a command line interface:

# Command line options
```
Usage: esp-isolator.exe [OPTIONS] <FILE>

Arguments:
  <FILE>  ESP file to isolate meshes and textures from

Options:
  -o, --output <OUTPUT>  (Optional) Output file path. If not specified, the zip file will be created in the same directory as the input ESP file
  -h, --help             Print help
  -V, --version          Print version
  ```
