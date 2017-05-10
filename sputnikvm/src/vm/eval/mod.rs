use utils::bigint::M256;
use utils::gas::Gas;
use super::commit::{AccountState, BlockhashState};
use super::errors::{RequireError, MachineError, CommitError};
use super::{Stack, Context, BlockHeader, Patch, PC, Storage, Memory, AccountCommitment};

pub mod cost;
pub mod run;

/// A VM state without PC.
pub struct State<M, S> {
    pub memory: M,
    pub stack: Stack,

    pub context: Context,
    pub block: BlockHeader,
    pub patch: Patch,

    pub out: Vec<u8>,

    pub memory_gas: Gas,
    pub used_gas: Gas,
    pub refunded_gas: Gas,

    pub account_state: AccountState<S>,
    pub blockhash_state: BlockhashState,
}

impl<M: Memory + Default, S: Storage + Default + Clone> State<M, S> {
    pub fn derive(&self, context: Context) -> Self {
        State {
            memory: M::default(),
            stack: Stack::default(),

            context: context,
            block: self.block.clone(),
            patch: self.patch.clone(),

            out: Vec::new(),

            memory_gas: Gas::zero(),
            used_gas: Gas::zero(),
            refunded_gas: Gas::zero(),

            account_state: self.account_state.clone(),
            blockhash_state: self.blockhash_state.clone()
        }
    }
}

/// A VM state with PC.
pub struct Machine<M, S> {
    state: State<M, S>,
    pc: PC,
    status: Status,
}

#[derive(Debug, Clone)]
pub enum Status {
    Running,
    ExitedOk,
    ExitedError(MachineError),
    InvokeCall(Context, (M256, M256)),
}

impl<M: Memory + Default, S: Storage + Default + Clone> Machine<M, S> {
    #[allow(unused_variables)]
    pub fn new(context: Context, block: BlockHeader, patch: Patch) -> Self {
        unimplemented!()
    }

    pub fn derive(&self, context: Context) -> Self {
        Machine {
            pc: PC::new(context.code.as_slice()),
            status: Status::Running,
            state: self.state.derive(context),
        }
    }

    pub fn commit_account(&mut self, commitment: AccountCommitment<S>) -> Result<(), CommitError> {
        self.state.account_state.commit(commitment)
    }

    pub fn commit_blockhash(&mut self, number: M256, hash: M256) -> Result<(), CommitError> {
        self.state.blockhash_state.commit(number, hash)
    }

    #[allow(unused_variables)]
    pub fn apply_sub(&mut self, sub: Machine<M, S>) {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn invoke_sub(&mut self, context: Context, from: M256, len: M256) {
        unimplemented!()
    }

    pub fn step(&mut self) -> Result<(), RequireError> {
        unimplemented!()
    }

    pub fn status(&self) -> &Status {
        &self.status
    }
}