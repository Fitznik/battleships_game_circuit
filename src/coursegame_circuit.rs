use std::convert::TryInto;
use franklin_crypto::bellman::{
    pairing::{
        bn256::Bn256,
        Engine,
    
    },
    PrimeField,
};
 use franklin_crypto::circuit::{
    boolean::{AllocatedBit, Boolean},
    num::AllocatedNum,

};

use franklin_crypto::bellman::{Circuit,SynthesisError, ConstraintSystem};

use franklin_crypto::circuit::rescue::rescue_hash;

use franklin_crypto::rescue::{bn256::Bn256RescueParams}; 
use franklin_crypto::rescue::RescueEngine;

use rescue_poseidon::RescueParams;
use rescue_poseidon::{generic_hash, CircuitGenericSponge, DomainStrategy, GenericSponge};


use franklin_crypto::plonk::circuit::bigint::bigint::{biguint_to_fe};

trait OptionExt<T> {
    fn grab(&self) -> Result<T, SynthesisError>;
}

impl<T: Copy> OptionExt<T> for Option<T> {
    fn grab(&self) -> Result<T, SynthesisError> {
        self.ok_or(SynthesisError::AssignmentMissing)
    }
}

#[derive(Clone, Debug)]
struct BaseCircuite<E: Engine> {
    board: [Option<bool>; 100],
    commit: Option<E::Fr>,
    salt: Option<E::Fr>,
    pos: Option<E::Fr>,
    board_pos_for_check: Option<bool>,

}

impl Circuit<Bn256> for BaseCircuite<Bn256>{
    fn synthesize<CS: ConstraintSystem<Bn256>>(self, cs: &mut CS) -> Result<(), SynthesisError>{
        let rescue_params = &Bn256RescueParams::new_checked_2_into_1();

        let commit = AllocatedNum::alloc(cs.namespace(|| "commit"), || self.commit.grab())?;
        let salt = AllocatedNum::alloc(cs.namespace(|| "salt"), || self.salt.grab())?;
        let pos = AllocatedNum::alloc(cs.namespace(|| "pos"), || self.pos.grab())?;
        let board_pos_for_check = AllocatedBit::alloc(cs.namespace(|| "check posi"), self.board_pos_for_check).unwrap();
        let board: Vec<Boolean> = self.board
            .iter()
            .enumerate()
            .map(|(i, b)| {
                Boolean::from(
                    AllocatedBit::alloc(cs.namespace(|| format!("input {}", i)), *b).unwrap()
                )
            })
            .collect();


        let board = Board{
            square: board
        };


        let board_in_alloc_num = board.board_into_alloc_num(cs)?;
        let commit_2 = hash_board(cs, board_in_alloc_num, salt, rescue_params)?;
        let check_commit = AllocatedNum::equals(cs.namespace(|| "check commit"), &commit, &commit_2)?;
        let nor_check_commit = AllocatedBit::nor(cs.namespace(|| "nor result"), &check_commit, &check_commit)?;
        AllocatedBit::alloc_conditionally(
            cs.namespace(|| "check commmit"),
            Some(true), 
            &nor_check_commit
            
        )?;

        let mut pos_board = board.board_pos(cs, pos)?;


        Ok(())
    }
}

fn hash_board<E: RescueEngine, CS: ConstraintSystem<E>>(
    cs: &mut CS,
    board_in_alloc_num: AllocatedNum<E>,
    salt: AllocatedNum<E>, 
    rescue_params: &<E as RescueEngine>::Params,
) -> Result<AllocatedNum<E>, SynthesisError> {
    let result = rescue_hash(cs, &[board_in_alloc_num, salt], rescue_params)?;

    Ok(AllocatedNum::from(result[0].clone()))
}


struct Board{
    pub square: Vec<Boolean>
}

fn conditionally_select_boolean<E:Engine, CS: ConstraintSystem<E>>(
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
    fn board_into_alloc_num<E:Engine, CS: ConstraintSystem<E>>(&self, cs: &mut CS) -> Result<AllocatedNum<E>, SynthesisError> {
        
        let board_in_alloc_num = AllocatedNum::pack_bits_to_element(cs.namespace(|| "board into num"), &self.square[..])?;

        // let sub_cs = cs.namespace(|| "zero constant");
        // let alloc_num_zero = AllocatedNum::<E>::zero(sub_cs)?;
        // let sub_cs = cs.namespace(|| "zero constant");
        // let mut board_in_alloc_num = AllocatedNum::<E>::zero(sub_cs)?;

        // for i in 0..100 {
        //     let sub_cs = cs.namespace(|| "zero constant");
        //     let boolean_into_alloc_num = AllocatedNum::<E>::conditionally_select( sub_cs, &AllocatedNum::one::<CS>(), &alloc_num_zero, &self.square[i])?;
        //     let sub_cs = cs.namespace(|| "zero constant");
        //     board_in_alloc_num = AllocatedNum::<E>::mul(&boolean_into_alloc_num, sub_cs, &biguint_to_fe(BigUint::from((1<<8)as u64)))?;


        // }

        Ok(board_in_alloc_num)
    } 

    fn board_pos<E:Engine, CS: ConstraintSystem<E>>(&self, cs: &mut CS, pos: AllocatedNum<E>)->Result<Boolean, SynthesisError>{
        let mut result = Boolean::Constant(false);
        for i in 0..100 {
            let i_allocated = AllocatedNum::<E>::alloc(cs.namespace(|| "..."), || Ok(E::Fr::from_str(&i.to_string()).unwrap()))?;
            i_allocated.assert_number(
            cs.namespace(|| "assert i is a constant"),
             &E::Fr::from_str(&i.to_string()).unwrap(),
            )?;
            let flag = AllocatedNum::equals(cs.namespace(|| "for flag"), &i_allocated , &pos)?;
            result = conditionally_select_boolean(cs.namespace(|| " "), &Boolean::Is(flag), &self.square[i], &result)?;
         }
        Ok(result)
    }
}
