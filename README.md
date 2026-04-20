# Void

Space station decompression survival. Maintain a failing station. Hull breaches decompress rooms. Seal bulkheads, manage oxygen, repair in zero-G. One bad patch and everyone breathes vacuum.

## Why This Exists?

Most survival games happen on planets. Void traps you in a machine that's falling apart. Your enemy isn't nature — it's physics. Pressure differentials, gas composition, power cascades. Every room is a puzzle, every breach is an emergency, and the station doesn't care if you live.

## Features

- Atmosphere simulation (O2, N2, CO2, pressure, temperature per room)
- 3 decompression types (rapid, slow, explosive)
- 10 station room types with power consumers
- Power grid with reactor, solar, and battery systems
- Shutdown cascades when demand exceeds supply
- Zero-G movement with thruster packs and magnetic boots
- EVA operations for external hull repairs
- Cascade event chains (breach -> decompression -> power failure)
- 5 hostile and 5 passive station creatures

## Quick Start

```bash
cargo build
cargo run
```

Built on the [Lattice](https://github.com/sudokatie/lattice) survival engine.

## License

MIT

---

*The air is leaving. Seal the bulkhead. Now.*
