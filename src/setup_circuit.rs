use franklin_crypto::bellman::{
    pairing::{
        bn256::{Bn256, Fr},
        Engine,
    
    },
};

use franklin_crypto::circuit::{
    boolean::{AllocatedBit, Boolean},
    num::{AllocatedNum, Num},

};
use franklin_crypto::bellman::{Circuit,SynthesisError, ConstraintSystem};

use franklin_crypto::rescue::{bn256::Bn256RescueParams}; 


trait OptionExt<T> {
    fn grab(&self) -> Result<T, SynthesisError>;
}

impl<T: Copy> OptionExt<T> for Option<T> {
    fn grab(&self) -> Result<T, SynthesisError> {
        self.ok_or(SynthesisError::AssignmentMissing)
    }
}




#[derive(Clone, Debug)]
struct SetupCircuite<E: Engine> {
    ship: Option<bool>,
    hash: Option<E::Fr>,

}

impl Circuit<Bn256> for SetupCircuite<Bn256>{
    fn synthesize<CS: ConstraintSystem<Bn256>>(self, cs: &mut CS) -> Result<(), SynthesisError>{
        todo!();
    }
}