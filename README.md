# iced [![crates.io](https://img.shields.io/crates/v/iced-x86.svg)](https://crates.io/crates/iced-x86) [![NuGet](https://img.shields.io/nuget/v/iced.svg)](https://www.nuget.org/packages/iced/) [![maven](https://img.shields.io/maven-central/v/io.github.icedland.iced/iced-x86)](https://central.sonatype.com/artifact/io.github.icedland.iced/iced-x86/1.21.0) [![pypi](https://img.shields.io/pypi/v/iced-x86.svg)](https://pypi.org/project/iced-x86/) [![GitHub builds](https://github.com/icedland/iced/workflows/GitHub%20CI/badge.svg)](https://github.com/icedland/iced/actions) [![codecov](https://codecov.io/gh/icedland/iced/branch/master/graph/badge.svg)](https://codecov.io/gh/icedland/iced)

<img width="160px" height="160px" src="logo.png" alt="Iced logo">

icier is an unbelievably fast and correct x86 (16/32/64-bit) instruction disassembler and assembler.

- 👍 Supports almost all Intel and AMD instructions (including undocumented ones!).
- 👍 Tested and fuzzed against other disassemblers/assemblers (xed, gas, objdump, masm, dumpbin, nasm, ndisasm).
- 👍 Supports MASM, NASM, GAS, and Intel formatting, with the option of a custom formatter as well.
- 👍 Stupid fast: Decodes >250 MB/s and decode+format >130 MB/s (Rust, [see here](https://github.com/icedland/disas-bench/tree/a865849deacfb6c33ee0e78f3a3ad7f4c82099f5#results)).
- 👍 The encoder can be used to re-encode decoded instructions at any address.
- 👍 API to get instruction info, eg. read/written registers, memory and rflags bits; CPUID feature flag, control flow info, etc.

# License
MIT

# Logo
Modified `processor` by [Creative Stall](https://thenounproject.com/creativestall/) from the Noun Project
