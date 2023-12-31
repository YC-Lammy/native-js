use std::marker::PhantomData;

use crate::context::GeneratorDesc;
use crate::mir::MIR;
use crate::types::*;
use crate::util::*;

pub(crate) struct SSA<'ctx>{
    pub id: ValueID,
    pub ty: Type<'ctx>
}

pub(crate) struct BlockDesc<'ctx> {
    pub(crate) id: BlockID,
    pub(crate) inst: Vec<MIR<'ctx>>,
}

impl<'ctx> BlockDesc<'ctx> {
    pub fn new(id: BlockID, params: &[Type<'ctx>]) -> Self {
        let mut param_values = Vec::new();
        let mut values = Vec::new();

        for i in 0..params.len() {
            let id = ValueID::new();

            param_values.push(id);
            values.push((id, params[i].clone()));
        }

        Self {
            id,
            inst: Vec::new(),
        }
    }
}

pub struct Function<'ctx> {
    pub(crate) params: Vec<Type<'ctx>>,
    pub(crate) return_: Type<'ctx>,
    pub(crate) is_async: bool,
    pub(crate) is_generator: Option<GeneratorDesc<'ctx>>,
    pub(crate) map_ssa_func: Vec<(FunctionID<'ctx>, ValueID)>,
    pub(crate) blocks: Vec<BlockDesc<'ctx>>,
    pub(crate) stackslots: Vec<Type<'ctx>>,
    pub(crate) _mark: PhantomData<&'ctx ()>,
}

impl<'ctx> Function<'ctx> {
    
}
