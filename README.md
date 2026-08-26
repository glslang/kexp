# win-kexp

`win-kexp` is a Rust 2024 library of Windows kernel exploitation primitives: shellcode
(token stealing, ACL editing, `cmd.exe` spawning), process injection, ROP chain construction
and gadget search, driver IOCTL plumbing, and kernel pool spraying. It targets Windows
**x86_64** and **ARM64**.

The debugger-side half of this crate — the DbgEng bindings and the kernel pool and user-mode
heap walkers built on them — moved to [`dbgscope`](https://github.com/glslang/dbgscope). What
remains here is what runs on the *target* rather than in the debugger. Nothing here depends on
`dbgscope`; the two are unrelated crates.

## Safety Scope

This is for exploit research on isolated Windows test systems you own or have explicit
permission to test. Do not run the payloads anywhere else.

## Modules

| Module | Purpose |
|---|---|
| `shellcode` | Payload loaders and hardcoded fallback byte arrays. |
| `process` | Process discovery, remote allocation, and `CreateRemoteThread` injection. |
| `rop` | ROP chain macros, PE executable-section parsing, and gadget search. |
| `win32k` | Device handles, `IOCTL`/`CTL_CODE` helpers, allocation, driver base lookup. |
| `spray` | `AnonymousPipe`, for shaping the kernel pool from user mode. |
| `util` | Pause, debug break, and byte formatting. |

## Requirements

- Windows x86_64 or Windows ARM64, MSVC build tools, Rust stable.
- Optional assembler: `ml64` (x86_64) or `armasm64` (ARM64).

## Shellcode Pipeline

Assembly sources live in `src/asm/`. On Windows, `build.rs` detects the target assembler and
compiles them into COFF objects in `OUT_DIR`, from which `goblin` extracts the executable
section bytes. With no assembler found, `build.rs` sets the `shellcode_fallback` feature and
hardcoded byte arrays are returned instead.

`test_shellcodes_match_fallback` asserts the two paths produce identical bytes — **this is the
contract to maintain**. Change an `.asm` file and you must update the matching fallback array
in `src/shellcode.rs`.

ARM64 shellcode is version-specific: `build.rs` passes `WINDOWS_VERSION` through to `armasm64`.
Supported values are `23H2` and `24H2` (default `24H2`); an invalid one forces the fallback.
See [`TOKEN_STEALING_ARM64.md`](./TOKEN_STEALING_ARM64.md).

```bash
WINDOWS_VERSION=23H2 cargo build --target aarch64-pc-windows-msvc
```

## License

MIT — see [LICENSE](LICENSE).
