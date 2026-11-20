# Zasto's Changelog

---

### 0.2.1

**Bug fixing**
- Correcting most of unsafe and not working features on Windows

**Notes**
I always was testing on Linux but never tried it on Windows, turns out to be pretty glitchy! I fixed some issues such as:
- Path being seprated at the ":" but not realizing Windows' file system writes paths as "C**:**/Users/..." which could litteraly say "delete my whole disk"
- Root path being set as "/", which works on Linux but on Windows has to be "C:/"


---

### 0.2.0

**Features**
- 733% faster scanner by using jwalk
- New logos and randomization added to fancy module

**Notes**
The older scanner was scanning recursively each directory and ignoring, now were using the `jwalk` crate, which divide the scan to multiple thread.
For reference, the older scanner took 22s to scan my disk (about 1.6M files), it now takes 3s, so it's about 7.33 times faster.
Now we can really feel like this tool doesnt need patience to help, I'm so happy to have made this way much faster.

---

### 0.1.0

**BIG UPDATE !!!**
- Rust migration

**Notes**
Some features are still not implemented but most of basic features are in there

---

### 1.2  - 05/10/26

**Features**
- JSON Outputting option (`--json`)
- Force option (`--force`)
- Yes option (`--yes`)
- Nuitka Compiling instruction in README.md

**Bug fixing**
- False positive warning while `--scan` used

### 1.1  - 27/09/26

**Features**
- TOML Config file
- More colorful prints
- New AI response format

**Bug fixing**
- api key reading error due to whitespace chars

---

### 1.0

**Features**
- Base of Zasto
