use franklin_crypto::bellman::{
    pairing::{
        Engine,
    
    },
    PrimeField,
};
 use franklin_crypto::circuit::{
    boolean::Boolean,
    num::AllocatedNum,

};

use franklin_crypto::bellman::{SynthesisError, ConstraintSystem};

use franklin_crypto::circuit::rescue::rescue_hash;
use franklin_crypto::rescue::RescueEngine;



pub fn hash_board<E: RescueEngine, CS: ConstraintSystem<E>>(
    cs: &mut CS,
    board_in_alloc_num: AllocatedNum<E>,
    salt: AllocatedNum<E>, 
    rescue_params: &<E as RescueEngine>::Params,
) -> Result<AllocatedNum<E>, SynthesisError> {
    let result = rescue_hash(cs, &[board_in_alloc_num, salt], rescue_params)?;

    Ok(AllocatedNum::from(result[0].clone()))
}


pub struct Board{
    pub square: Vec<Boolean>
}

pub fn conditionally_select_boolean<E:Engine, CS: ConstraintSystem<E>>(
    mut cs: CS,
    a: &Boolean,
    b: &Boolean,
    condition: &Boolean
)-> Result<Boolean, SynthesisError>{
    // (condition and a) xor ((not condition) and b)
    let part1 = Boolean::and(cs.namespace(|| " ..."), &a, &condition)?;
    let part2 = Boolean::and(cs.namespace(|| " ..."), &b, &condition.not())?;
    let result = Boolean::xor(cs.namespace(|| " ..."), &part1, &part2)?;
    Ok(result)


}

impl Board{
     pub fn board_into_alloc_num<E:Engine, CS: ConstraintSystem<E>>(&self, cs: &mut CS) -> Result<AllocatedNum<E>, SynthesisError> {
        
        let board_in_alloc_num = AllocatedNum::pack_bits_to_element(cs.namespace(|| "board into num"), &self.square[..])?;

        Ok(board_in_alloc_num)
    } 

    pub fn board_pos<E:Engine, CS: ConstraintSystem<E>>(&self, cs: &mut CS, pos: &AllocatedNum<E>)->Result<Boolean, SynthesisError>{
        let mut result = Boolean::Constant(false);
        for i in 0..100 {
            let i_allocated = AllocatedNum::<E>::alloc(cs.namespace(|| "..."), || Ok(E::Fr::from_str(&i.to_string()).unwrap()))?;
            i_allocated.assert_number(
            cs.namespace(|| "assert i is a constant"),
             &E::Fr::from_str(&i.to_string()).unwrap(),
            )?;
            let flag = AllocatedNum::equals(cs.namespace(|| "for flag"), &i_allocated , &pos)?;
            result = conditionally_select_boolean(cs.namespace(|| " "),  &self.square[i], &result, &Boolean::Is(flag))?;
         }
        Ok(result)
    }
}
