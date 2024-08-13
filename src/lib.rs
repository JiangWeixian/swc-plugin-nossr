use swc_core::ecma::{
    ast::Program,
    visit::{as_folder, FoldWith},
};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};
mod no_ssr;
use no_ssr::NoSSRVisitor;

/// An example plugin function with macro support.
/// `plugin_transform` macro interop pointers into deserialized structs, as well
/// as returning ptr back to host.
///
/// It is possible to opt out from macro by writing transform fn manually
/// if plugin need to handle low-level ptr directly via
/// `__transform_plugin_process_impl(
///     ast_ptr: *const u8, ast_ptr_len: i32,
///     unresolved_mark: u32, should_enable_comments_proxy: i32) ->
///     i32 /*  0 for success, fail otherwise.
///             Note this is only for internal pointer interop result,
///             not actual transform result */`
///
/// This requires manual handling of serialization / deserialization from ptrs.
/// Refer swc_plugin_macro to see how does it work internally.
#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(NoSSRVisitor))
}
#[cfg(test)]
mod test {
    use swc_core::ecma::parser::Syntax;
    use swc_core::ecma::{parser::EsSyntax, transforms::testing::test_inline, visit::as_folder};
    test_inline!(
        Syntax::Es(EsSyntax {
            jsx: true,
            ..Default::default()
        }),
        |_| as_folder(super::NoSSRVisitor),
        basic,
        // Input codes
        r#"<NoSSR><C /></NoSSR>"#,
        // Output codes after transformed with plugin
        r#"<NoSSR></NoSSR>"#
    );

    test_inline!(
        Syntax::Es(EsSyntax {
            jsx: true,
            ..Default::default()
        }),
        |_| as_folder(super::NoSSRVisitor),
        child,
        // Input codes
        r#"<ComponentA><NoSSR><C /></NoSSR></ComponentA>"#,
        // Output codes after transformed with plugin
        r#"<ComponentA><NoSSR></NoSSR></ComponentA>"#
    );
}
