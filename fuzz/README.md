# Fuzzing crabvpx

Standard [`cargo-fuzz`](https://github.com/rust-fuzz/cargo-fuzz) targets, also
wired for [OSS-Fuzz](https://github.com/google/oss-fuzz).

## Targets

| Target | Input | Notes |
| --- | --- | --- |
| `decode_frame` | raw bytes treated as a single VP8 frame | Most direct; no seed corpus needed. Deeply exercises key-frame parsing. |
| `decode_ivf` | an IVF byte stream (multi-frame) | Exercises inter-frame / reference paths. Seed with `../test_data/*.ivf`. |

The invariant under test: **decoding arbitrary or hostile input must never panic
or trigger undefined behavior** — only return an error.

## Running locally

```sh
cargo install cargo-fuzz          # one-time; requires a nightly toolchain
cargo fuzz run decode_frame       # fuzz the raw-frame decoder

# Seed the IVF target with the valid vectors, then fuzz outward:
mkdir -p fuzz/corpus/decode_ivf
cp test_data/*.ivf fuzz/corpus/decode_ivf/
cargo fuzz run decode_ivf fuzz/corpus/decode_ivf
```

Reproduce a crash artifact:

```sh
cargo fuzz run decode_frame fuzz/artifacts/decode_frame/crash-<hash>
```

## OSS-Fuzz integration

`oss-fuzz/` holds drop-in copies of the files OSS-Fuzz expects under
`projects/crabvpx/` in the [oss-fuzz repo](https://github.com/google/oss-fuzz):

- `project.yaml` — metadata, engines, sanitizers
- `Dockerfile` — clones this repo into the build image
- `build.sh` — runs `cargo fuzz build` and stages the targets + IVF seed corpus

To onboard, copy `oss-fuzz/*` into `oss-fuzz/projects/crabvpx/` and open a PR
there.
