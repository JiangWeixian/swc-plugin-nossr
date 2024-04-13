use swc_core::ecma::visit::VisitMut;
use swc_core::ecma::ast::*;

pub struct NoSSRVisitor;

impl VisitMut for NoSSRVisitor {
    // Implement necessary visit_mut_* methods for actual custom transform.
    // A comprehensive list of possible visitor methods can be found here:
    // https://rustdoc.swc.rs/swc_ecma_visit/trait.VisitMut.html
    fn visit_mut_jsx_element(&mut self, n: &mut JSXElement) {
      let tag = &n.opening.name;
      let tag_name = match tag {
        JSXElementName::Ident(ident) => ident.sym.to_string(),
        _ => "".to_string(),
      };
      let no_ssr_tag_name = String::from("NoSSR");
      if tag_name != no_ssr_tag_name {
        return;
      }
      // let null_element = JSXExprContainer {
      //   span: DUMMY_SP,
      //   expr: JSXExpr::Lit(Lit::Null(Null { span: DUMMY_SP })),
      // };
      n.children = vec![];
    }
}
