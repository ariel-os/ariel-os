/// Registers a function to run during initial system startup, so it can control configuration.
///
/// Hook functions are provided with a parameter, which the function is allowed to mutate.
/// Hook functions are guaranteed to run before tasks defined with [`#[ariel_os::task]`](macro@crate::task)
/// and threads defined with [`#[ariel_os::thread]`](macro@crate::thread) are started.
///
/// **Important**: for a hook to be taken into account, the associated Cargo feature needs to be
/// enabled on the `ariel-os` dependency (see table below).
///
/// | Hook name     | Parameter type           | Cargo feature to enable |
/// | ------------- | ------------------------ | ----------------------- |
/// | `usb_builder` | `ariel_os::usb::UsbBuilder` | `usb-builder-hook`      |
///
///
/// # Example
///
/// ```
/// #[ariel_os::hook(usb_builder)]
/// fn usb_builder(builder: &mut ariel_os::usb::UsbBuilder) {
///     // Your hook here
/// }
/// ```
///
/// # Panics
///
/// This macro panics when the `ariel-os` crate cannot be found as a dependency of the crate where
/// this macro is used.
#[proc_macro_attribute]
pub fn hook(args: TokenStream, item: TokenStream) -> TokenStream {
    #[allow(clippy::wildcard_imports)]
    use hook_macro::*;

    use quote::{format_ident, quote};

    let mut attrs = HookAttributes::default();
    let hook_attr_parser = syn::meta::parser(|meta| attrs.parse(&meta));
    syn::parse_macro_input!(args with hook_attr_parser);

    let hook_fn = syn::parse_macro_input!(item as syn::ItemFn);
    assert!(hook_fn.sig.asyncness.is_none(), "hook functions cannot be async");
    assert!(hook_fn.sig.output == syn::ReturnType::Default, "hook functions must not have a non-default return type");

    let hook_fn_name = &hook_fn.sig.ident;

    let ariel_os_crate = utils::ariel_os_crate();

    let (fn_name, builder_type) = match attrs.kind {
        Some(HookKind::UsbBuilder) => (
            format_ident!("__ariel_os_usb_builder_hook"),
            quote!{ #ariel_os_crate::usb::UsbBuilder },
        ),
        None => {
            panic!("a hook must be specified");
        }
    };

    // Place the provided function into a function whose type signature we enforce.
    // This is important as that function will be called unsafely via FFI.
    let expanded = quote! {
        #[no_mangle]
        fn #fn_name(builder: &mut #builder_type) {
            #hook_fn

            #hook_fn_name(builder);
        }
    };

    TokenStream::from(expanded)
}

mod hook_macro {
    #[derive(Default)]
    pub struct HookAttributes {
        pub kind: Option<HookKind>,
    }

    impl HookAttributes {
        /// Parses macro attributes.
        ///
        /// # Errors
        ///
        /// Returns an error when an unsupported parameter is found.
        pub fn parse(&mut self, meta: &syn::meta::ParseNestedMeta) -> syn::Result<()> {
            use enum_iterator::all;

            for (hook_name, kind) in all::<HookKind>().map(|c| (c.as_name(), c)) {
                if meta.path.is_ident(hook_name) {
                    self.check_only_one_kind(hook_name);
                    self.kind = Some(kind);
                    return Ok(());
                }
            }

            let supported_params = all::<HookKind>()
                .map(|c| format!("`{}`", c.as_name()))
                .collect::<Vec<_>>()
                .join(", ");
            Err(meta.error(format!(
                "unsupported parameter ({supported_params} are supported)",
            )))
        }

        /// Checks that the macro is used for only one kind of hook.
        ///
        /// # Panics
        ///
        /// Panics if multiple kinds are found.
        fn check_only_one_kind(&self, param: &str) {
            assert!(
                self.kind.is_none(),
                "only one hook is supported at a time, use a separate constant for `{param}` configuration",
            );
        }
    }

    #[derive(Debug, enum_iterator::Sequence)]
    pub enum HookKind {
        UsbBuilder,
    }

    impl HookKind {
        pub fn as_name(&self) -> &'static str {
            match self {
                Self::UsbBuilder => "usb_builder",
            }
        }
    }
}
