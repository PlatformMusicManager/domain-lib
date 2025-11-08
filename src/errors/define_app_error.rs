#[macro_export]
macro_rules! define_app_error {
    (
        // The first part of the macro now matches the entire enum definition
        $(#[$enum_meta:meta])*
        pub enum $enum_name:ident {
            $($variants:tt)*
        }

        // The second part uses a keyword to introduce the response logic
        response: {
            delegates: [$($delegate_variant:ident),*],
            custom: {
                $($custom_match_arms:tt)*
            }
        }
    ) => {
        // --- The Generated Code ---

        // Generate the enum exactly as it was passed in.
        $(#[$enum_meta])*
        pub enum $enum_name {
            $($variants)*
        }

        // Generate the IntoResponse implementation.
        impl axum::response::IntoResponse for $enum_name {
            fn into_response(self) -> axum::response::Response {
                match self {
                    // Create a match arm for each delegated variant.
                    $(
                        Self::$delegate_variant(inner) => inner.into_response(),
                    )*
                    // Paste in the custom match arms verbatim.
                    $($custom_match_arms)*
                }
            }
        }
    };
}