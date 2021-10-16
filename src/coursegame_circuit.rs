use franklin_crypto::bellman::{
    pairing::{
        bn256::Bn256,
        Engine,
    
    },

};
 use franklin_crypto::circuit::{
    boolean::{AllocatedBit, Boolean},
    num::AllocatedNum,

};

use franklin_crypto::bellman::{Circuit,SynthesisError, ConstraintSystem};

use franklin_crypto::rescue::{bn256::Bn256RescueParams}; 

use crate::board_tools::*;

trait OptionExt<T> {
    fn grab(&self) -> Result<T, SynthesisError>;
}

impl<T: Copy> OptionExt<T> for Option<T> {
    fn grab(&self) -> Result<T, SynthesisError> {
        self.ok_or(SynthesisError::AssignmentMissing)
    }
}

#[derive(Clone, Debug)]
pub struct BaseCircuite<E: Engine> {
    pub board: [Option<bool>; 100],
    pub commit: Option<E::Fr>,
    pub salt: Option<E::Fr>,
    pub pos_x: Option<E::Fr>,
    pub pos_y: Option<E::Fr>,
    pub claimed_value: Option<bool>,

}

impl Circuit<Bn256> for BaseCircuite<Bn256>{
    fn synthesize<CS: ConstraintSystem<Bn256>>(self, cs: &mut CS) -> Result<(), SynthesisError>{
        let rescue_params = &Bn256RescueParams::new_checked_2_into_1();

        let commit = AllocatedNum::alloc(cs.namespace(|| "commit"), || self.commit.grab())?;
        let salt = AllocatedNum::alloc(cs.namespace(|| "salt"), || self.salt.grab())?;
        let pos_x = AllocatedNum::alloc(cs.namespace(|| "pos_x"), || self.pos_x.grab())?;
        let pos_y = AllocatedNum::alloc(cs.namespace(|| "pos_y"), || self.pos_y.grab())?;
        let claimed_value = AllocatedBit::alloc(cs.namespace(|| "check posi"), self.claimed_value).unwrap();
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



        let board_in_alloc_num = board.board_into_alloc_num(cs, "567890")?;
        let commit_2 = hash_board(cs, board_in_alloc_num, salt, rescue_params)?;
        let check_commit = AllocatedNum::equals(cs.namespace(|| "check commit"), &commit, &commit_2)?;
        let nor_check_commit = AllocatedBit::nor(cs.namespace(|| "nor result"), &check_commit, &check_commit)?;
        AllocatedBit::alloc_conditionally(
            cs.namespace(|| "check commmit is true"),
            Some(true), 
            &nor_check_commit
            
        )?;

        let pos_board = board.board_pos(cs, &pos_x, &pos_y, "100")?;
        let check_claimed_value = AllocatedBit::xor(cs.namespace(|| "check commit 2"), &pos_board.get_variable().unwrap(), &claimed_value)?;
        // let nor_check_claimed_value = AllocatedBit::nor(cs.namespace(|| "nor result for claimed value"), &check_claimed_value, &check_claimed_value)?;
        AllocatedBit::alloc_conditionally(
            cs.namespace(|| "check commit 2 is true"),
            Some(true), 
            &check_claimed_value
            
        )?;
        


        Ok(())
    }
}

