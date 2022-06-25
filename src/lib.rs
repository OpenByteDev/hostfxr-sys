#![warn(clippy::pedantic, clippy::cargo, unsafe_op_in_unsafe_fn)]
#![allow(
    clippy::missing_safety_doc,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::multiple_crate_versions,
    clippy::doc_markdown,
    non_camel_case_types,
    dead_code
)]
#![cfg_attr(all(feature = "doc-cfg", nightly), feature(doc_cfg))]

/*!
FFI bindings for [hostfxr](https://github.com/dotnet/runtime/blob/main/docs/design/features/host-components.md#components-of-the-hosting).

## Related crates
- [nethost-sys](https://crates.io/crates/nethost-sys) - bindings for the nethost library.
- [coreclr-hosting-shared](https://crates.io/crates/coreclr-hosting-shared) - shared bindings between this crate and [nethost-sys](https://crates.io/crates/nethost-sys).
- [netcorehost](https://crates.io/crates/netcorehost) - rusty wrapper over the nethost and hostfxr libraries.

## Additional Information
- [Hosting layer APIs](https://github.com/dotnet/core-setup/blob/master/Documentation/design-docs/hosting-layer-apis.md)
- [Native hosting](https://github.com/dotnet/core-setup/blob/master/Documentation/design-docs/native-hosting.md#runtime-properties)
- [Write a custom .NET Core host to control the .NET runtime from your native code](https://docs.microsoft.com/en-us/dotnet/core/tutorials/netcore-hosting)

## License
Licensed under the MIT license ([LICENSE](https://github.com/OpenByteDev/hostfxr-sys/blob/master/LICENSE) or <http://opensource.org/licenses/MIT>)
*/

#[macro_use]
extern crate dlopen_derive;

pub use dlopen;

use core::{ffi::c_void, mem};
use coreclr_hosting_shared::{char_t, size_t};

/// Signifies that the target method is marked with the [`UnmanagedCallersOnlyAttribute`].
/// This means that the name alone can identify the target method.
///
/// [`UnmanagedCallersOnlyAttribute`]: https://docs.microsoft.com/en-us/dotnet/api/system.runtime.interopservices.unmanagedcallersonlyattribute
pub const UNMANAGED_CALLERS_ONLY_METHOD: *const char_t = usize::MAX as *const _;

/// Seperator char used to sepeate a list of paths in string.
#[cfg(windows)]
/// Seperator char used to sepeate a list of paths in string.
pub const PATH_LIST_SEPARATOR: char_t = b';' as char_t;
#[cfg(not(windows))]
pub const PATH_LIST_SEPARATOR: char_t = b':' as char_t;

#[repr(i32)]
pub enum hostfxr_delegate_type {
    hdt_com_activation = 0,
    hdt_load_in_memory_assembly = 1,
    hdt_winrt_activation = 2,
    hdt_com_register = 3,
    hdt_com_unregister = 4,
    hdt_load_assembly_and_get_function_pointer = 5,
    hdt_get_function_pointer = 6,
}

/// Error reporting function signature.
pub type hostfxr_error_writer_fn = extern "C" fn(message: *const char_t);

#[allow(non_upper_case_globals)]
/// Flag constants for `hostfxr_resolve_sdk2`.
pub mod hostfxr_resolve_sdk2_flags_t {
    pub const none: i32 = 0x0;
    pub const disallow_prerelease: i32 = 0x1;
}

#[repr(i32)]
pub enum hostfxr_resolve_sdk2_result_key_t {
    resolved_sdk_dir = 0,
    global_json_path = 1,
}

/// Result callback signature for `hostfxr_resolve_sdk2`.
pub type hostfxr_resolve_sdk2_result_fn =
    extern "C" fn(key: hostfxr_resolve_sdk2_result_key_t, value: *const char_t);

/// Result callback signature for `hostfxr_get_available_sdks`.
pub type hostfxr_get_available_sdks_result_fn =
    extern "C" fn(sdk_count: i32, sdk_dirs: *const *const char_t);

/// Handle to a hostfxr context.
pub type hostfxr_handle = *const c_void;

/// Signature of delegate returned by [`hostfxr_get_runtime_delegate`] for type [`hdt_load_assembly_and_get_function_pointer`]
///
/// # Arguments
///  * `assembly_path`:
///    Fully qualified path to assembly
///  * `type_name`:
///     Assembly qualified type name
///  * `method_name`:
///     Public static method name compatible with delegateType
///  * `delegate_type_name`:
///     Assembly qualified delegate type name or [`null`](core::ptr::null()), or [`UNMANAGED_CALLERS_ONLY_METHOD`] if the method is marked with the [`UnmanagedCallersOnlyAttribute`].
///  * `load_context`:
///     Extensibility parameter (currently unused and must be 0)
///  * `reserved`:
///     Extensibility parameter (currently unused and must be 0)
///  * `delegate`:
///     Pointer where to store the function pointer result
///
/// [`hostfxr_get_runtime_delegate`]: struct.HostfxrLib.html#method.hostfxr_get_runtime_delegate
/// [`hdt_load_assembly_and_get_function_pointer`]: hostfxr_delegate_type::`hdt_load_assembly_and_get_function_pointer
/// [`UNMANAGED_CALLERS_ONLY_METHOD`]: UNMANAGED_CALLERS_ONLY_METHOD
/// [`UnmanagedCallersOnlyAttribute`]: https://docs.microsoft.com/en-us/dotnet/api/system.runtime.interopservices.unmanagedcallersonlyattribute
pub type load_assembly_and_get_function_pointer_fn = unsafe extern "system" fn(
    assembly_path: *const char_t,
    type_name: *const char_t,
    method_name: *const char_t,
    delegate_type_name: *const char_t,
    reserved: *const c_void,
    /*out*/ delegate: *mut *const c_void,
) -> i32;

/// Signature of delegate returned by [`hostfxr_get_runtime_delegate`] for type [`hdt_get_function_pointer`]
///
/// # Arguments
///  * `type_name`:
///     Assembly qualified type name
///  * `method_name`:
///     Public static method name compatible with delegateType
///  * `delegate_type_name`:
///     Assembly qualified delegate type name or [`null`](core::ptr::null()), or [`UNMANAGED_CALLERS_ONLY_METHOD`] if the method is marked with the [`UnmanagedCallersOnlyAttribute`].
///  * `load_context`:
///     Extensibility parameter (currently unused and must be 0)
///  * `reserved`:
///     Extensibility parameter (currently unused and must be 0)
///  * `delegate`:
///     Pointer where to store the function pointer result
///
/// [`hdt_get_function_pointer`]: hostfxr_delegate_type::hdt_get_function_pointer
/// [`hostfxr_get_runtime_delegate`]: struct.HostfxrLib.html#method.hostfxr_get_runtime_delegate
/// [`UNMANAGED_CALLERS_ONLY_METHOD`]: crate::UNMANAGED_CALLERS_ONLY_METHOD
/// [`UnmanagedCallersOnlyAttribute`]: https://docs.microsoft.com/en-us/dotnet/api/system.runtime.interopservices.unmanagedcallersonlyattribute
pub type get_function_pointer_fn = unsafe extern "system" fn(
    type_name: *const char_t,
    method_name: *const char_t,
    delegate_type_name: *const char_t,
    load_context: *const c_void,
    reserved: *const c_void,
    /*out*/ delegate: *mut *const c_void,
) -> i32;

/// Signature of delegate returned by [`load_assembly_and_get_function_pointer_fn`] when `delegate_type_name == null` (default)
pub type component_entry_point_fn = unsafe extern "system" fn(*const c_void, size_t) -> i32;

/// A structure that stores parameters which are common to all forms of initialization.
#[repr(C)]
pub struct hostfxr_initialize_parameters {
    /// The size of the structure.
    /// This is used for versioning.
    /// Should be set to `mem::size_of::<hostfxr_initialize_parameters>()`.
    pub size: size_t,
    /// Path to the native host (typically the `.exe`).
    /// This value is not used for anything by the hosting components.
    /// It's just passed to the CoreCLR as the path to the executable.
    /// It can point to a file which is not executable itself, if such file doesn't exist
    /// (for example in COM activation scenarios this points to the `comhost.dll`).
    /// This is used by PAL (Platform Abstraction Layer) to initialize internal command line structures, process name and so on.
    pub host_path: *const char_t,
    /// Path to the root of the .NET Core installation in use.
    /// This typically points to the install location from which the hostfxr has been loaded.
    /// For example on Windows this would typically point to `C:\Program Files\dotnet`.
    /// The path is used to search for shared frameworks and potentially SDKs.
    pub dotnet_root: *const char_t,
}

impl hostfxr_initialize_parameters {
    /// Creates a new instance of [`hostfxr_initialize_parameters`] with the given `host_path`.
    /// The `size` field is set accordingly to the size of the struct and `dotnet_root` to [`core::ptr::null()`].
    #[must_use]
    pub fn with_host_path(host_path: *const char_t) -> hostfxr_initialize_parameters {
        hostfxr_initialize_parameters {
            size: mem::size_of::<hostfxr_initialize_parameters>(),
            host_path,
            dotnet_root: core::ptr::null(),
        }
    }
    /// Creates a new instance of [`hostfxr_initialize_parameters`] with the given `dotnet_root`.
    /// The `size` field is set accordingly to the size of the struct and `host_path` to [`core::ptr::null()`].
    #[must_use]
    pub fn with_dotnet_root(dotnet_root: *const char_t) -> hostfxr_initialize_parameters {
        hostfxr_initialize_parameters {
            size: mem::size_of::<hostfxr_initialize_parameters>(),
            host_path: core::ptr::null(),
            dotnet_root,
        }
    }
}

/*
TODO: why does this not work?
macro_rules! derive_apis {
    (
        $( #[$struct_attrs:meta] )*
        $visibility:vis struct $name:ident {
        $(
            $( #[$field_attrs:meta] )*
            $field:ident : $field_type:ty,
        )*
    }) => {
        $visibility mod symbor {
            #[allow(unused_imports)]
            use super::*;
            use dlopen::symbor::{Symbol, SymBorApi};

            $( #[$struct_attrs] )*
            #[derive(SymBorApi)]
            $visibility struct $name <'a> {
                $(
                    $( #[$field_attrs] )*
                    $field : Symbol<'a, $field_type>
                ),*
            }
        }

        $visibility mod wrapper {
            #[allow(unused_imports)]
            use super::*;
            use dlopen::wrapper::WrapperApi;

            $( #[$struct_attrs] )*
            #[derive(WrapperApi)]
            $visibility struct $name {
                $(
                    $( #[$field_attrs] )*
                    $field : $field_type
                ),*
            }
        }
    }
}

derive_apis! {
    pub struct Hostfxr {
        ...
    }
}
*/

/// [`dlopen::symbor`] abstraction for the hostfxr library.
pub mod symbor;

/// [`dlopen::wrapper`] abstraction for the  hostfxr library.
pub mod wrapper;
