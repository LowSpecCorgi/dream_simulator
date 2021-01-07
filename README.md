## Dream simulator
This program tries to mimic the ender pearl luck a youtuber called "[Dream](https://www.youtube.com/user/DreamTraps)" managed to get in his highly controversial 1.16 speedrun by running billions of simulations to try and match the luck gained in that speedrun.

You can see a demo of this in action [here](https://www.youtube.com/watch?v=sF_YLrItRW0&t=745s&ab_channel=Basilicous)

## Dream pearl luck achieved in 49 billion attempts!
![Image](https://hypixel.net/attachments/1609918695809-png.2240318/)

## Installation instructions

### Windows
For windows you can simply go the [releases tab](https://github.com/LowSpecCorgi/dream_simulator/releases/tag/v1.0.0) and download the `.exe` file, then run it.

### Other platforms
Since I haven't built binaries for other platforms, you must build it youself, however since this uses a lanuage called `Rust`, which has excellent tools for building programs, which require minimal effort to use.
* Install a tool called `rustup`, [which can be found at this link](https://rustup.rs/)
* Enter the commands:
```bash
cd dream_simulator
cargo build --release
```
* To actually run the simulation, run the command:
```bash
cargo run --release
```
