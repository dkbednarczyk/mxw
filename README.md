# mxw - Model (O/O-/D/D-) Wireless
Cross platform CLI for configuring Glorious wireless mice.

See the original project and it's README at [korkje/mow](https://github.com/korkje/mow).

This fork uses updated dependencies, adds support for more devices (see below) and fixes panics when changing the polling rate and lift-off distance.

## Supported Devices
- [x] Model O
    - [x] Tested
- [x] Model D
    - [x] Tested
- [x] Model O-
    - [ ] Tested
- [x] Model D-
    - [x] Tested
- [x] Model D2 Pro [https://github.com/dxbednarczyk/mxw/pull/7]
    - [ ] Tested

This project currently only works with these mice, as they are compatible with [Glorious Core v1](https://www.gloriousgaming.com/pages/legacy-software). I don't have a newer device to test with, so if you've got a weekend to waste and no side project to work on, feel free to submit a pull request. [Here](https://github.com/dxbednarczyk/mxw/issues/1#issuecomment-2135500375) may be a good starting point.

## Common issues
- You get an error message like `Error: hidapi error: Failed to open a device with path '/dev/hidraw5': Permission denied`
    - You can bypass this for the current session by running `sudo chmod a+rw /dev/hidraw5`.
    - Use a [HID rule](https://github.com/libusb/hidapi/blob/master/udev/69-hid.rules) for a more permanent fix. All relevant information you would need to fill in is in [`glorious.rs`](https://github.com/dkbednarczyk/mxw/blob/master/src/glorious.rs).

## Installation

### Nix
You can either import this in your flake or just run 

```nix 
nix run github:dxbednarczyk/mxw -- [INSERT_FLAGS_HERE]
```

Example:

```nix 
nix run github:dxbednarczyk/mxw -- config led-effect off
```