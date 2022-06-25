use coreclr_hosting_shared::{char_t, size_t};
use dlopen::symbor::{SymBorApi, Symbol};

use crate::{
    hostfxr_delegate_type, hostfxr_error_writer_fn, hostfxr_get_available_sdks_result_fn,
    hostfxr_handle, hostfxr_initialize_parameters,
    hostfxr_resolve_sdk2_result_fn,
};

/// [`dlopen::symbor`] abstraction for the hostfxr library.
#[derive(SymBorApi)]
pub struct Hostfxr<'lib> {
    // ensures that 'lib is used if not other
    #[cfg(not(feature = "netcore1_0"))]
    _dummy: Option<Symbol<'lib, fn()>>,

    /// Run an application.
    ///
    /// # Arguments
    ///  * `argv` - command-line arguments
    ///
    /// This function does not return until the application completes execution.
    /// It will shutdown CoreCLR after the application executes.
    /// If the application is successfully executed, this value will return the exit code of the application. Otherwise, it will return an error code indicating the failure.
    #[cfg(feature = "netcore1_0")]
    #[cfg_attr(all(feature = "doc-cfg", nightly), doc(cfg(feature = "netcore1_0")))]
    pub hostfxr_main:
        Symbol<'lib, unsafe extern "C" fn(argc: i32, argv: *const *const char_t) -> i32>,

    /// Determines the directory location of the SDK accounting for
    /// `global.json` and multi-level lookup policy.
    ///
    /// Invoked via MSBuild SDK resolver to locate SDK props and targets
    /// from an msbuild other than the one bundled by the CLI.
    ///
    /// # Arguments
    ///  * `exe_dir`
    ///      The main directory where SDKs are located in `sdk\[version]`
    ///      sub-folders. Pass the directory of a dotnet executable to
    ///      mimic how that executable would search in its own directory.
    ///      It is also valid to pass nullptr or empty, in which case
    ///      multi-level lookup can still search other locations if
    ///      it has not been disabled by the user's environment.
    ///
    ///  * `working_dir`
    ///      The directory where the search for `global.json` (which can
    ///      control the resolved SDK version) starts and proceeds
    ///      upwards.
    ///
    ///  * `buffer`
    ///      The buffer where the resolved SDK path will be written.
    ///
    ///  * `buffer_size`
    ///      The size of the buffer argument in [`char_t`] units.
    ///
    /// # Return value:
    ///  * `<0` - Invalid argument
    ///  * `0`  - SDK could not be found.
    ///  * `>0` - The number of characters (including null terminator)
    ///        required to store the located SDK.
    ///
    /// If resolution succeeds and the positive return value is less than
    /// or equal to `buffer_size` (i.e. the the buffer is large enough),
    /// then the resolved SDK path is copied to the buffer and null
    /// terminated. Otherwise, no data is written to the buffer.
    #[deprecated(note = "Use `hostfxr_resolve_sdk2` instead.")]
    #[cfg(feature = "netcore2_0")]
    #[cfg_attr(all(feature = "doc-cfg", nightly), doc(cfg(feature = "netcore2_0")))]
    pub hostfxr_resolve_sdk: Symbol<
        'lib,
        unsafe extern "C" fn(
            exe_dir: *const char_t,
            working_dir: *const char_t,
            buffer: *mut char_t,
            buffer_size: i32,
        ) -> i32,
    >,

    /// Run an application.
    ///
    /// # Arguments
    ///  * `argv`
    ///     command-line arguments
    ///  * `host_path`
    ///     path to the host application
    ///  * `dotnet_root`
    ///     path to the .NET Core installation root
    ///  * `app_path`
    ///     path to the application to run
    ///
    /// This function does not return until the application completes execution.
    /// It will shutdown CoreCLR after the application executes.
    /// If the application is successfully executed, this value will return the exit code of the application. Otherwise, it will return an error code indicating the failure.
    #[cfg(feature = "netcore2_1")]
    #[cfg_attr(all(feature = "doc-cfg", nightly), doc(cfg(feature = "netcore2_1")))]
    pub hostfxr_main_startupinfo: Symbol<
        'lib,
        unsafe extern "C" fn(
            argc: i32,
            argv: *const *const char_t,
            host_path: *const char_t,
            dotnet_root: *const char_t,
            app_path: *const char_t,
        ) -> i32,
    >,

    #[cfg(all(feature = "netcore2_1", feature = "undocumented"))]
    #[cfg_attr(
        all(feature = "doc-cfg", nightly),
        doc(cfg(feature = "netcore2_1", feature = "undocumented"))
    )]
    pub hostfxr_main_bundle_startupinfo: Symbol<
        'lib,
        unsafe extern "C" fn(
            argc: i32,
            argv: *const *const char_t,
            host_path: *const char_t,
            dotnet_root: *const char_t,
            app_path: *const char_t,
            bundle_header_offset: i64,
        ) -> i32,
    >,

    /// Determine the directory location of the SDK, accounting for `global.json` and multi-level lookup policy.
    ///
    /// # Arguments
    ///  * `exe_dir` - main directory where SDKs are located in `sdk\[version]` sub-folders.
    ///  * `working_dir` - directory where the search for `global.json` will start and proceed upwards
    ///  * `flags` - flags that influence resolution:
    ///         `disallow_prerelease` - do not allow resolution to return a pre-release SDK version unless a pre-release version was specified via `global.json`
    ///  * `result` - callback invoked to return resolved values.
    ///         The callback may be invoked more than once. Strings passed to the callback are valid only for the duration of the call.
    ///
    /// If resolution succeeds, result will be invoked with [`resolved_sdk_dir`](crate::hostfxr_resolve_sdk2_result_key_t::resolved_sdk_dir) key and the value will hold the path to the resolved SDK directory.
    /// If resolution does not succeed, result will be invoked with [`resolved_sdk_dir`](crate::hostfxr_resolve_sdk2_result_key_t::resolved_sdk_dir) key and the value will be [`ptr::null()`](core::ptr::null).
    ///
    /// If `global.json` is used, result will be invoked with [`global_json_path`](crate::hostfxr_resolve_sdk2_result_key_t::global_json_path) key and the value will hold the path to `global.json`.
    /// If there was no `global.json` found, or the contents of `global.json` did not impact resolution (e.g. no version specified), then result will not be invoked with [`global_json_path`](crate::hostfxr_resolve_sdk2_result_key_t::global_json_path) key.
    #[cfg(feature = "netcore2_1")]
    #[cfg_attr(all(feature = "doc-cfg", nightly), doc(cfg(feature = "netcore2_1")))]
    pub hostfxr_resolve_sdk2: Symbol<
        'lib,
        unsafe extern "C" fn(
            exe_dir: *const char_t,
            working_dir: *const char_t,
            flags: i32,
            result: hostfxr_resolve_sdk2_result_fn,
        ) -> i32,
    >,

    /// Get the list of all available SDKs ordered by ascending version.
    ///
    /// # Arguments
    ///  * `exe_dir` - path to the dotnet executable
    ///  * `result` - callback invoked to return the list of SDKs by their directory paths.
    ///             String array and its elements are valid only for the duration of the call.
    #[cfg(feature = "netcore2_1")]
    #[cfg_attr(all(feature = "doc-cfg", nightly), doc(cfg(feature = "netcore2_1")))]
    pub hostfxr_get_available_sdks: Symbol<
        'lib,
        unsafe extern "C" fn(
            exe_dir: *const char_t,
            result: hostfxr_get_available_sdks_result_fn,
        ) -> i32,
    >,

    /// Get the native search directories of the runtime based upon the specified app.
    ///
    /// # Arguments
    ///  * `argc`,`argv` - command-line arguments
    ///  * `buffer` - buffer to populate with the native search directories (including a null terminator).
    ///  * `buffer_size` - size of `buffer` in [`char_t`] units
    ///  * `required_buffer_size` - if buffer is too small, this will be populated with the minimum required buffer size (including a null terminator). Otherwise, this will be set to 0.
    ///
    /// The native search directories will be a list of paths separated by [`PATH_LIST_SEPARATOR`], which is a semicolon (;) on Windows and a colon (:) otherwise.
    ///
    /// If `buffer_size` is less than the minimum required buffer size, this function will return [`HostApiBufferTooSmall`] and buffer will be unchanged.
    ///
    /// [`HostApiBufferTooSmall`]: coreclr_hosting_shared::StatusCode::HostApiBufferTooSmall
    /// [`PATH_LIST_SEPARATOR`]: crate::PATH_LIST_SEPARATOR
    #[cfg(feature = "netcore2_1")]
    #[cfg_attr(all(feature = "doc-cfg", nightly), doc(cfg(feature = "netcore2_1")))]
    pub hostfxr_get_native_search_directories: Symbol<
        'lib,
        unsafe extern "C" fn(
            argc: i32,
            argv: *const *const char_t,
            buffer: *mut char_t,
            buffer_size: i32,
            required_buffer_size: *mut i32,
        ) -> i32,
    >,

    /// Sets a callback which is to be used to write errors to.
    ///
    /// # Arguments
    ///  * `error_writer`:
    ///     A callback function which will be invoked every time an error is to be reported.
    ///     Or [`null`](core::ptr::null()) to unregister previously registered callback and return to the default behavior.
    ///
    /// # Return value
    /// The previously registered callback (which is now unregistered), or [`null`](core::ptr::null()) if no previous callback
    /// was registered
    ///
    /// # Remarks
    /// The error writer is registered per-thread, so the registration is thread-local. On each thread
    /// only one callback can be registered. Subsequent registrations overwrite the previous ones.
    ///
    /// By default no callback is registered in which case the errors are written to stderr.
    ///
    /// Each call to the error writer is sort of like writing a single line (the EOL character is omitted).
    /// Multiple calls to the error writer may occure for one failure.
    ///
    /// If the hostfxr invokes functions in hostpolicy as part of its operation, the error writer
    /// will be propagated to hostpolicy for the duration of the call. This means that errors from
    /// both hostfxr and hostpolicy will be reporter through the same error writer.
    #[cfg(feature = "netcore3_0")]
    #[cfg_attr(all(feature = "doc-cfg", nightly), doc(cfg(feature = "netcore3_0")))]
    pub hostfxr_set_error_writer: Symbol<
        'lib,
        unsafe extern "C" fn(error_writer: hostfxr_error_writer_fn) -> hostfxr_error_writer_fn,
    >,

    /// Initializes the hosting components for a dotnet command line running an application
    ///
    /// # Arguments
    ///  * `argc`:
    ///     Number of argv arguments
    ///  * `argv`:
    ///     Command-line arguments for running an application (as if through the dotnet executable).
    ///  * `parameters`:
    ///     Optional. Additional parameters for initialization
    ///  * `host_context_handle`:
    ///     On success, this will be populated with an opaque value representing the initialized host context
    ///
    /// # Return value
    ///  * [`Success`]:
    ///     Hosting components were successfully initialized
    ///  * [`HostInvalidState`]:
    ///     Hosting components are already initialized
    ///
    /// # Remarks
    /// This function parses the specified command-line arguments to determine the application to run. It will
    /// then find the corresponding `.runtimeconfig.json` and `.deps.json` with which to resolve frameworks and
    /// dependencies and prepare everything needed to load the runtime.
    ///
    /// This function only supports arguments for running an application. It does not support SDK commands.
    ///
    /// This function does not load the runtime.
    ///
    /// [`Success`]: coreclr_hosting_shared::StatusCode::Success
    /// [`HostInvalidState`]: coreclr_hosting_shared::StatusCode::HostInvalidState
    #[cfg(feature = "netcore3_0")]
    #[cfg_attr(all(feature = "doc-cfg", nightly), doc(cfg(feature = "netcore3_0")))]
    pub hostfxr_initialize_for_dotnet_command_line: Symbol<
        'lib,
        unsafe extern "C" fn(
            argc: i32,
            argv: *const *const char_t,
            parameters: *const hostfxr_initialize_parameters,
            /*out*/ host_context_handle: *mut hostfxr_handle,
        ) -> i32,
    >,

    /// Initializes the hosting components using a `.runtimeconfig.json` file
    ///
    /// # Arguments
    ///  * `runtime_config_path`:
    ///     Path to the `.runtimeconfig.json` file
    ///  * `parameters`:
    ///     Optional. Additional parameters for initialization
    ///  * `host_context_handle`:
    ///     On success, this will be populated with an opaque value representing the initialized host context
    ///
    /// # Return value
    /// * [`Success`]:
    ///      Hosting components were successfully initialized
    /// * [`Success_HostAlreadyInitialized`]:
    ///      Config is compatible with already initialized hosting components
    /// * [`Success_DifferentRuntimeProperties`]:
    ///      Config has runtime properties that differ from already initialized hosting components
    /// * [`CoreHostIncompatibleConfig`]:
    ///      Config is incompatible with already initialized hosting components
    ///
    /// # Remarks
    /// This function will process the `.runtimeconfig.json` to resolve frameworks and prepare everything needed
    /// to load the runtime. It will only process the `.deps.json` from frameworks (not any app/component that
    /// may be next to the `.runtimeconfig.json`).
    ///
    /// This function does not load the runtime.
    ///
    /// If called when the runtime has already been loaded, this function will check if the specified runtime
    /// config is compatible with the existing runtime.
    ///
    /// Both [`Success_HostAlreadyInitialized`] and [`Success_DifferentRuntimeProperties`] codes are considered successful
    /// initializations. In the case of [`Success_DifferentRuntimeProperties`], it is left to the consumer to verify that
    /// the difference in properties is acceptable.
    ///
    /// [`Success`]: coreclr_hosting_shared::StatusCode::Success
    /// [`Success_HostAlreadyInitialized`]: coreclr_hosting_shared::StatusCode::Success_HostAlreadyInitialized
    /// [`Success_DifferentRuntimeProperties`]: coreclr_hosting_shared::StatusCode::Success_DifferentRuntimeProperties
    /// [`CoreHostIncompatibleConfig`]: coreclr_hosting_shared::StatusCode::CoreHostIncompatibleConfig
    #[cfg(feature = "netcore3_0")]
    #[cfg_attr(all(feature = "doc-cfg", nightly), doc(cfg(feature = "netcore3_0")))]
    pub hostfxr_initialize_for_runtime_config: Symbol<
        'lib,
        unsafe extern "C" fn(
            runtime_config_path: *const char_t,
            parameters: *const hostfxr_initialize_parameters,
            /*out*/ host_context_handle: *mut hostfxr_handle,
        ) -> i32,
    >,

    /// Gets the runtime property value for an initialized host context
    ///
    /// # Arguments
    ///  * `host_context_handle`:
    ///     Handle to the initialized host context
    ///  * `name`:
    ///     Runtime property name
    ///  * `value`:
    ///     Out parameter. Pointer to a buffer with the property value.
    ///
    /// # Return value
    /// The error code result.
    ///
    /// # Remarks
    /// The buffer pointed to by value is owned by the host context. The lifetime of the buffer is only
    /// guaranteed until any of the below occur:
    ///  * a 'run' method is called for the host context
    ///  * properties are changed via [`hostfxr_set_runtime_property_value`]
    ///  * the host context is closed via [`hostfxr_close`]
    ///
    /// If `host_context_handle` is [`null`](core::ptr::null()) and an active host context exists, this function will get the
    /// property value for the active host context.
    ///
    /// [`hostfxr_set_runtime_property_value`]: struct.HostfxrLib.html#method.hostfxr_set_runtime_property_value
    /// [`hostfxr_close`]: struct.HostfxrLib.html#method.hostfxr_close
    #[cfg(feature = "netcore3_0")]
    #[cfg_attr(all(feature = "doc-cfg", nightly), doc(cfg(feature = "netcore3_0")))]
    pub hostfxr_get_runtime_property_value: Symbol<
        'lib,
        unsafe extern "C" fn(
            host_context_handle: hostfxr_handle,
            name: *const char_t,
            /*out*/ value: *mut *const char_t,
        ) -> i32,
    >,

    /// Sets the value of a runtime property for an initialized host context
    ///
    /// # Arguments
    ///  * `host_context_handle`:
    ///     Handle to the initialized host context
    ///  * `name`:
    ///     Runtime property name
    ///  * `value`:
    ///     Value to set
    ///
    /// # Return value
    /// The error code result.
    ///
    /// # Remarks
    /// Setting properties is only supported for the first host context, before the runtime has been loaded.
    ///
    /// If the property already exists in the host context, it will be overwritten. If value is [`null`](core::ptr::null()), the
    /// property will be removed.
    #[cfg(feature = "netcore3_0")]
    #[cfg_attr(all(feature = "doc-cfg", nightly), doc(cfg(feature = "netcore3_0")))]
    pub hostfxr_set_runtime_property_value: Symbol<
        'lib,
        unsafe extern "C" fn(
            host_context_handle: hostfxr_handle,
            name: *const char_t,
            value: *const char_t,
        ) -> i32,
    >,

    /// Gets all the runtime properties for an initialized host context
    ///
    /// # Arguments
    ///  * `host_context_handle`:
    ///     Handle to the initialized host context
    ///  * `count`:
    ///     \[in\] Size of the keys and values buffers
    ///     \[out\] Number of properties returned (size of keys/values buffers used). If the input value is too
    ///             small or keys/values is [`null`](core::ptr::null()), this is populated with the number of available properties
    ///  * `keys`:
    ///     \[out\] Array of pointers to buffers with runtime property keys
    ///  * `values`:
    ///     \[out\] Array of pointers to buffers with runtime property values
    ///
    /// # Return value
    /// The error code result.
    ///
    /// # Remarks
    /// The buffers pointed to by keys and values are owned by the host context. The lifetime of the buffers is only
    /// guaranteed until any of the below occur:
    ///  * a 'run' method is called for the host context
    ///  * properties are changed via [`hostfxr_set_runtime_property_value`]
    ///  * the host context is closed via [`hostfxr_close`]
    ///
    /// If host_context_handle is [`null`](core::ptr::null()) and an active host context exists, this function will get the
    /// properties for the active host context.
    ///
    /// [`hostfxr_set_runtime_property_value`]: struct.HostfxrLib.html#hostfxr_set_runtime_property_value
    /// [`hostfxr_close`]: struct.HostfxrLib.html#method.hostfxr_closee
    #[cfg(feature = "netcore3_0")]
    #[cfg_attr(all(feature = "doc-cfg", nightly), doc(cfg(feature = "netcore3_0")))]
    pub hostfxr_get_runtime_properties: Symbol<
        'lib,
        unsafe extern "C" fn(
            host_context_handle: hostfxr_handle,
            /*inout*/ count: *mut size_t,
            /*out*/ keys: *mut *const char_t,
            /*out*/ values: *mut *const char_t,
        ) -> i32,
    >,

    /// Load CoreCLR and run the application for an initialized host context
    ///
    /// # Arguments
    ///  * `host_context_handle`:
    ///     Handle to the initialized host context
    ///
    /// # Return value
    /// If the app was successfully run, the exit code of the application. Otherwise, the error code result.
    ///
    /// # Remarks
    /// The `host_context_handle` must have been initialized using [`hostfxr_initialize_for_dotnet_command_line`].
    ///
    /// This function will not return until the managed application exits.
    ///
    /// [`hostfxr_initialize_for_runtime_config`]: struct.HostfxrLib.html#method.hostfxr_initialize_for_runtime_config
    /// [`hostfxr_initialize_for_dotnet_command_line`]: struct.HostfxrLib.html#method.hostfxr_initialize_for_dotnet_command_line
    #[cfg(feature = "netcore3_0")]
    #[cfg_attr(all(feature = "doc-cfg", nightly), doc(cfg(feature = "netcore3_0")))]
    pub hostfxr_run_app:
        Symbol<'lib, unsafe extern "C" fn(host_context_handle: hostfxr_handle) -> i32>,

    /// Gets a typed delegate from the currently loaded CoreCLR or from a newly created one.
    ///
    /// # Arguments
    ///  * `host_context_handle`:
    ///     Handle to the initialized host context
    ///  * `type`:
    ///     Type of runtime delegate requested
    ///  * `delegate`:
    ///     An out parameter that will be assigned the delegate.
    ///
    /// # Return value
    /// The error code result.
    ///
    /// # Remarks
    /// If the `host_context_handle` was initialized using [`hostfxr_initialize_for_runtime_config`],
    /// then all delegate types are supported.
    /// If the host_context_handle was initialized using [`hostfxr_initialize_for_dotnet_command_line`],
    /// then only the following delegate types are currently supported:
    ///  * [`hdt_load_assembly_and_get_function_pointer`]
    ///  * [`hdt_get_function_pointer`]
    ///
    /// [`hdt_load_assembly_and_get_function_pointer`]: hostfxr_delegate_type::hdt_load_assembly_and_get_function_pointer
    /// [`hdt_get_function_pointer`]: hostfxr_delegate_type::hdt_get_function_pointer
    /// [`hostfxr_initialize_for_runtime_config`]: struct.HostfxrLib.html#method.hostfxr_initialize_for_runtime_config
    /// [`hostfxr_initialize_for_dotnet_command_line`]: struct.HostfxrLib.html#method.hostfxr_initialize_for_dotnet_command_line
    #[cfg(feature = "netcore3_0")]
    #[cfg_attr(all(feature = "doc-cfg", nightly), doc(cfg(feature = "netcore3_0")))]
    pub hostfxr_get_runtime_delegate: Symbol<
        'lib,
        unsafe extern "C" fn(
            host_context_handle: hostfxr_handle,
            r#type: hostfxr_delegate_type,
            /*out*/ delegate: *mut *const (),
        ) -> i32,
    >,

    /// Closes an initialized host context
    ///
    /// # Arguments
    ///  * `host_context_handle`:
    ///     Handle to the initialized host context
    ///
    /// # Return value:
    /// The error code result.
    #[cfg(feature = "netcore3_0")]
    #[cfg_attr(all(feature = "doc-cfg", nightly), doc(cfg(feature = "netcore3_0")))]
    pub hostfxr_close:
        Symbol<'lib, unsafe extern "C" fn(host_context_handle: hostfxr_handle) -> i32>,
}
