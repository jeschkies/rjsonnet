//! Static checking for jsonnet.

mod check;
mod flow;
mod scope;
mod suggestion;
mod unify;

pub mod error;
pub mod st;

use jsonnet_expr::{Expr, ExprArena, ExprData, Id};
use jsonnet_ty::{LocalStore, Ty};

/// Performs the checks.
#[must_use]
pub fn get(mut st: st::St<'_>, ar: &ExprArena, expr: Expr) -> (st::Statics, LocalStore) {
  st.scope.define(Id::std, Ty::STD, jsonnet_expr::def::Def::Std);
  st.scope.define(Id::std_unutterable, Ty::STD, jsonnet_expr::def::Def::Std);
  //print_expr(ar, expr, 0);
  check::get(&mut st, ar, expr);
  // these can never be marked as unused
  _ = st.scope.undefine(Id::std);
  _ = st.scope.undefine(Id::std_unutterable);
  st.finish()
}

fn print_expr(ar: &ExprArena, expr: Expr, indent: usize) {
  if let Some(r) = expr {
    let r = &ar[r];
    match r {
      ExprData::Prim(prim) => println!("{}get: Prim({prim:?})", "  ".repeat(indent)),
      ExprData::Object { asserts, fields } => {
        println!("{:indent$}Object {{", "  ".repeat(indent));
        println!("{:indent$}  asserts: {asserts:?}", "  ".repeat(indent));
        println!("{:indent$}  fields: {fields:?}", "  ".repeat(indent));
        println!("{:indent$}}}", "  ".repeat(indent));
      }
      ExprData::ObjectComp { name, vis, body, id, ary } => println!(
        "get: ObjectComp {{ name: {name:?}, vis: {vis:?}, body: {body:?}, id: {id:?}, ary: {ary:?} }}"
      ),
      ExprData::Array(elems) => println!("get: Array({elems:?})"),
      ExprData::Subscript { on, idx } => {
        println!("{:indent$}Subscript {{", "  ".repeat(indent));
        println!("{:indent$}  on: ", "  ".repeat(indent));
        print_expr(ar, *on, indent + 1);
        println!("{:indent$}  idx: ", "  ".repeat(indent));
        print_expr(ar, *idx, indent + 1);
        println!("{:indent$}}}", "  ".repeat(indent));
      }
      ExprData::Call { func, positional, named } => {
        println!("{:indent$}Call {{", "  ".repeat(indent));
        println!("{:indent$}func: ", "  ".repeat(indent));
        print_expr(ar, *func, indent + 1);
        println!("{:indent$}positional: ", "  ".repeat(indent));
        for expr in positional {
          print_expr(ar, *expr, indent + 1);
        }
        println!("{:indent$}named: ", "  ".repeat(indent));
        for (id, expr) in named {
          println!("{:indent$}  {id:?}: ", "  ".repeat(indent));
          print_expr(ar, *expr, indent + 1);
        }
        println!("{:indent$}}}", "  ".repeat(indent));
      }
      ExprData::Id(id) => println!("{:indent$}Id({id:?})", "  ".repeat(indent)),
      ExprData::Local { binds, body } => {
        println!("{:indent$}Local {{", "  ".repeat(indent));
        for (id, expr) in binds {
          println!("{:indent$}  {id:?}: ", "  ".repeat(indent));
          print_expr(ar, *expr, indent + 1);
        }
        println!("{:indent$}body: ", "  ".repeat(indent));
        print_expr(ar, *body, indent + 2);
        println!("{:indent$}}}", "  ".repeat(indent));
      }
      ExprData::If { cond, yes, no } => {
        println!("{:indent$}If {{", "  ".repeat(indent));
        println!("{:indent$}  cond: ", "  ".repeat(indent));
        print_expr(ar, *cond, indent + 2);
        println!("{:indent$}  yes: ", "  ".repeat(indent));
        print_expr(ar, *yes, indent + 2);
        println!("{:indent$}  no: ", "  ".repeat(indent));
        print_expr(ar, *no, indent + 2);
        println!("{:indent$}}}", "  ".repeat(indent));
      }
      ExprData::BinOp { lhs, op, rhs } => {
        println!("{:indent$}BinOp {{", "  ".repeat(indent));
        println!("{:indent$}  lhs: ", "  ".repeat(indent));
        print_expr(ar, *lhs, indent + 1);
        println!("{:indent$}  op: {op:?}", "  ".repeat(indent));
        println!("{:indent$}  rhs: ", "  ".repeat(indent));
        print_expr(ar, *rhs, indent + 1);
        println!("{:indent$}}}", "  ".repeat(indent));
      }
      ExprData::UnOp { op, inner } => println!("{:indent$}UnOp {{ op: {op:?}, inner: {inner:?} }}", "  ".repeat(indent)),
      ExprData::Fn { params, body } => {
        println!("{:indent$}Fn {{", "  ".repeat(indent));
        println!("{:indent$}  params: {params:?}", "  ".repeat(indent));
        println!("{:indent$}  body: ", "  ".repeat(indent));
        print_expr(ar, *body, indent + 2);
        println!("{:indent$}}}", "  ".repeat(indent));
      }
      ExprData::Error(expr) => {
        println!("{:indent$}Error {{", "  ".repeat(indent));
        println!("{:indent$}  expr: ", "  ".repeat(indent));
        print_expr(ar, *expr, indent + 1);
        println!("{:indent$}}}", "  ".repeat(indent));
      }
      ExprData::Import { kind, path } => {
        println!("{:indent$}Import {{ kind: {kind:?}, path: {path:?} }}", "  ".repeat(indent));
      }
      ExprData::SubstOuter(expr) => println!("get: SubstOuter({expr:?})"),
    }
  }
}
