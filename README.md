# hostfxr-sys

[![CI](https://github.com/OpenByteDev/hostfxr-sys/actions/workflows/ci.yml/badge.svg)](https://github.com/OpenByteDev/hostfxr-sys/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/hostfxr-sys.svg)](https://crates.io/crates/hostfxr-sys)
[![Documentation](https://docs.rs/hostfxr-sys/badge.svg)](https://docs.rs/hostfxr-sys)
[![dependency status](https://deps.rs/repo/github/openbytedev/hostfxr-sys/status.svg)](https://deps.rs/repo/github/openbytedev/hostfxr-sys)
[![MIT](https://img.shields.io/crates/l/hostfxr-sys.svg)](https://github.com/OpenByteDev/hostfxr-sys/blob/master/LICENSE)

<!-- cargo-sync-readme start -->

FFI bindings for [hostfxr](https://github.com/dotnet/runtime/blob/main/docs/design/features/host-components.md#components-of-the-hosting).

# Related crates
- [nethost-sys](https://crates.io/crates/nethost-sys) - bindings for the nethost library.
- [coreclr-hosting-shared](https://crates.io/crates/coreclr-hosting-shared) - shared bindings between this crate and [nethost-sys](https://crates.io/crates/nethost-sys).
- [netcorehost](https://crates.io/crates/netcorehost) - rusty wrapper over the hostfxr and hostfxr libraries.

## Additional Information
- [Hosting layer APIs](https://github.com/dotnet/core-setup/blob/master/Documentation/design-docs/hosting-layer-apis.md)
- [Native hosting](https://github.com/dotnet/core-setup/blob/master/Documentation/design-docs/native-hosting.md#runtime-properties)
- [Write a custom .NET Core host to control the .NET runtime from your native code](https://docs.microsoft.com/en-us/dotnet/core/tutorials/netcore-hosting)

## License
Licensed under the MIT license ([LICENSE](https://github.com/OpenByteDev/hostfxr-sys/blob/master/LICENSE) or http://opensource.org/licenses/MIT)

<!-- cargo-sync-readme end -->
