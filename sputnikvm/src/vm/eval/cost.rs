use utils::gas::Gas;

use vm::{Memory, Storage, Instruction};
use vm::errors::EvalError;
use super::State;

#[allow(unused_variables)]
pub fn memory_cost<M: Memory + Default, S: Storage + Default + Clone>(instruction: Instruction, state: &State<M, S>) -> Result<Gas, EvalError> {
    unimplemented!()
}

#[allow(unused_variables)]
pub fn gas_cost<M: Memory + Default, S: Storage + Default + Clone>(instruction: Instruction, state: &State<M, S>) -> Result<Gas, EvalError> {
    unimplemented!()
}

#[allow(unused_variables)]
pub fn gas_stipend<M: Memory + Default, S: Storage + Default + Clone>(instruction: Instruction, state: &State<M, S>) -> Result<Gas, EvalError> {
    unimplemented!()
}

#[allow(unused_variables)]
pub fn gas_refund<M: Memory + Default, S: Storage + Default + Clone>(instruction: Instruction, state: &State<M, S>) -> Result<Gas, EvalError> {
    unimplemented!()
}