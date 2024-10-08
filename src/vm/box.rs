use super::EvalAction;
use crate::Value;
use cairo_lang_sierra::{
    extensions::{
        boxing::BoxConcreteLibfunc,
        core::{CoreLibfunc, CoreType},
        lib_func::SignatureAndTypeConcreteLibfunc,
    },
    program_registry::ProgramRegistry,
};
use smallvec::smallvec;

pub fn eval(
    registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    selector: &BoxConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    match selector {
        BoxConcreteLibfunc::Into(_) => todo!(),
        BoxConcreteLibfunc::Unbox(info) => eval_unbox(registry, info, args),
        BoxConcreteLibfunc::ForwardSnapshot(_) => todo!(),
    }
}

pub fn eval_unbox(
    _registry: &ProgramRegistry<CoreType, CoreLibfunc>,
    _info: &SignatureAndTypeConcreteLibfunc,
    args: Vec<Value>,
) -> EvalAction {
    let [value] = args.try_into().unwrap();

    EvalAction::NormalBranch(0, smallvec![value])
}
