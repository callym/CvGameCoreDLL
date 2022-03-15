# Rustified Civ 4 DLL

## Instructions

* Follow [the guide](https://forums.civfanatics.com/threads/compiling-the-dll-on-linux.658833/) to set up compiling the DLL on Linux/WINE
* Change any settings in `CvGameCoreDLL/compile_settings.sh`
* Change any settings in `config.ron`
* Run `cargo xtask [build/run]`

## Limitations

* This is still super early stages, so at the moment basically nothing is implemented or works
* Because Civ 4 uses such an old compiler (from ~2003), we can't compile with a modern MSVC toolchain, like the one Rust wants to use. This means we build the Rust library as a `#[no_std]` library (`std` wants to link to the MSVC runtime, then we have two versions of it linked that conflict!)
* Because the compiler is so old, I don't expect `bindgen` would really work too well, so we might have to write the `extern "C"` blocks by hand - but I've not investigated this yet
* Also because of this, I've had to write a simple compatibility layer for the `cstdint` types - I'm not sure how correct this is, but am hoping it's close enough!
* The `compile.sh`/`compile_settings.sh` scripts should probably be re-written in Rust or something? At the moment this'll only work on Linux!
* `cargo xtask run` will only work with a Steam install of Civ 4 at the moment
* There's probably a huge amount more other problems waiting to be discovered!

## Tips

If you want to run Civ4 windowed on a high-res display, you can use [gamescope](https://github.com/Plagman/gamescope) - I've been running it with my Steam command set to:

`INTEL_DEBUG=norbc VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/intel_icd.i686.json:/usr/share/vulkan/icd.d/intel_icd.x86_64.json gamescope -w 1440 -h 960 -W 2880 -H 1920 -n -r 60 %command%`

## Symbols

To get a list of the (mostly mangled) symbols - you can run this:

`objdump -x 'CvGameCoreDLL/Output/CvGameCoreDLL.dll' > dump.txt`

Then "just" `ctrl+f` to get the actual function name you're looking for.
