use franklin_crypto::bellman::{
    pairing::{
        bn256::{Bn256, Fr},
        Engine,
    },
};
 use franklin_crypto::circuit::{
    boolean::{AllocatedBit, Boolean},
    num::{AllocatedNum, Num},
    // linear_combination::*, 
    // bigint,
};

use franklin_crypto::bellman::{Circuit,SynthesisError, ConstraintSystem};

use franklin_crypto::circuit::rescue::rescue_hash;

use franklin_crypto::rescue::{bn256::Bn256RescueParams}; 
use franklin_crypto::rescue::RescueEngine;

use rescue_poseidon::RescueParams;
use rescue_poseidon::{generic_hash, CircuitGenericSponge, DomainStrategy, GenericSponge};
use num_bigint::BigUint;

#[derive(Clone, Debug)]
struct BaseCircuite<E: Engine> {
    Board: [Option<bool>; 100],
    commit: Option<E::Fr>,
    salt: Option<E::Fr>,
    pos: Option<E::Fr>,

}

impl<E: Engine> Circuit<E> for BaseCircuite<E>{
    fn synthesize<CS: ConstraintSystem<E>>(self, cs: &mut CS) -> Result<(), SynthesisError>{
        Ok(())
    }
}

fn hash_board<E: RescueEngine, CS: ConstraintSystem<E>>(
    cs: &mut CS,
     board_in_num: Num<E>,
     salt: Num<E>, 
     ) -> Result<Num<E>, SynthesisError> {
        let params = Bn256RescueParams::new_checked_2_into_1();
        let result =rescue_hash(cs, &[Num::<E>::get_variable(&board_in_num)], &params)?;
        const RATE: usize = 1;
        const WIDTH: usize = 2;

     Ok(Num::Variable(result[0]))
}

fn board_plus_salt(){
    todo!();
}

struct Board{
    pub square: [Boolean; 100]
}

impl Board{
    fn board_into_num<E:Engine, CS: ConstraintSystem<E>>(self, cs: &mut CS) -> Result<Num<E>, SynthesisError> {

        todo!();

        // let boolean_into_num= Num::<E>::conditionally_select(cs, &board.square[i], &Num::one(), &Num::zero());
        // let mut lc = LinearCombination::<E>::zero();
        // let base = Num::<E>::Constant(bigint::biguint_to_fe(BigUint::from(2 as u64)));
        // let mut coeff = Num::one();
        // let use_this_byte = true;

        // for (boolean_into_num, use_this_byte) in buf.iter().rev(){
        //     let value = boolean_into_num.inner.mul(cs, &coeff)?;
        //     let term = Num::mask(cs, &value, &use_this_byte)?;
        //     lc.add_assign_number_with_coeff(&term, E::Fr::one());

        //     let new_coeff = coeff.mul(cs, &base);
        //     coeff = Num::conditionally_select(cs,&use_this_byte, &new_coeff, &coeff)?;
        // }
        // let board_in_num = lc.into_num(cs)?;
        // Ok(())
 } 
}
