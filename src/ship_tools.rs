use franklin_crypto::bellman::{
    pairing::{
        Engine,
    
    },
};

use franklin_crypto::circuit::{
    boolean::Boolean,
    num::AllocatedNum,

};
use franklin_crypto::bellman::{SynthesisError, ConstraintSystem};

use rand::thread_rng;

use crate::board_tools::Board;
use crate::hash_board;

pub struct Ship<E:Engine, const N: usize>{
    pub head_pos: AllocatedNum<E>,
    pub gorizontal: Boolean,
    
    
}
impl<E:Engine, const N: usize> Ship<E, N>{
    pub fn check_ship<CS: ConstraintSystem<E>>(
        &self, 
        cs: &mut CS,
        board: &Board, 
    )-> Result<Boolean, SynthesisError>{
        let ten = ten(cs)?;
        let mut check_ship = board.board_pos(cs, &self.head_pos)?;
        let mut pos = self.head_pos.mul(cs.namespace(|| " "), &AllocatedNum::one::<CS>())?;
        let ship_dir = AllocatedNum::conditionally_select(cs.namespace(|| " "), &AllocatedNum::one::<CS>(), &ten, &self.gorizontal)?;
        for i in 0..N-1{
            pos = pos.add(cs.namespace(|| " "), &ship_dir)?;
            let check = board.board_pos(cs, &pos)?;
            check_ship = Boolean::and(cs.namespace(|| " "), &check_ship, &check)?;
        }

        Ok(check_ship)
        
    }

    pub fn check_ship_space<CS: ConstraintSystem<E>>(
        &self, 
        cs: &mut CS,
        board: &Board,
    )-> Result<Boolean, SynthesisError>{
        let ten = ten(cs)?;
        let elev = ten.add(cs.namespace(|| " "), &AllocatedNum::one::<CS>())?;
        let extrem_left_pos = elev.sub(cs.namespace(|| " "), &self.head_pos)?;
        let mut check_ship_space = board.board_pos(cs, &extrem_left_pos)?;
        let ship_dir = AllocatedNum::conditionally_select(cs.namespace(|| " "), &AllocatedNum::one::<CS>(), &ten, &self.gorizontal)?;
        let mut pos = extrem_left_pos.mul(cs.namespace(|| " "), &AllocatedNum::one::<CS>())?;

        for i in 0..N+1{
            pos = pos.add(cs.namespace(|| " "), &ship_dir)?;
            let check = board.board_pos(cs, &pos)?;
            check_ship_space = Boolean::and(cs.namespace(|| " "), &check_ship_space , &check)?;
        }

        let other_ship_dir = elev.sub(cs.namespace(|| " "), &ship_dir)?;
        pos = pos.add(cs.namespace(|| " "), &other_ship_dir)?;
        let check = board.board_pos(cs, &pos)?;
        check_ship_space  = Boolean::and(cs.namespace(|| " "), &check_ship_space, &check)?;
        pos = pos.add(cs.namespace(|| " "), &other_ship_dir)?;
        let check = board.board_pos(cs, &pos)?;
        check_ship_space  = Boolean::and(cs.namespace(|| " "), &check_ship_space, &check)?;

        for i in 0..N+1{
            pos = pos.sub(cs.namespace(|| " "), &ship_dir)?;
            let check = board.board_pos(cs, &pos)?;
            check_ship_space  = Boolean::and(cs.namespace(|| " "), &check_ship_space, &check)?;
        }

        pos = pos.sub(cs.namespace(|| " "), &other_ship_dir)?;
        let check = board.board_pos(cs, &pos)?;
        check_ship_space  = Boolean::and(cs.namespace(|| " "), &check_ship_space , &check)?;

        Ok(check_ship_space)
    }



}

pub fn check_amount_ship<E:Engine, CS: ConstraintSystem<E>>(
    cs: &mut CS,
    board: &Board, 
    pos: &AllocatedNum<E>
    )->Result<Boolean, SynthesisError>{

    let alloc_num_zero = AllocatedNum::<E>::zero(cs.namespace(|| "zero constant"))?;
    let mut amount = AllocatedNum::<E>::zero(cs.namespace(|| "zero constant"))?;

    for i in 0..100 {
        let bit = AllocatedNum::<E>::conditionally_select( 
            cs.namespace(|| "zero constant"), 
            &AllocatedNum::one::<CS>(), 
            &alloc_num_zero, 
            &board.square[i])?;
        amount = amount.add( cs.namespace(|| "zero constant"), &bit)?;
    }
    let ten = ten(cs)?;
    let twenty = ten.add(cs.namespace(|| " "), &ten)?;
    let check = AllocatedNum::<E>::equals(cs.namespace(|| " "), &twenty, &amount)?;

    Ok(Boolean::from(check))

}


pub fn ten<E:Engine, CS: ConstraintSystem<E>>(cs: &mut CS)-> Result<AllocatedNum<E>, SynthesisError>{
    let mut new_value = AllocatedNum::one::<CS>();

    for i in 0..9{
        new_value = new_value.add(cs.namespace(|| " "), &AllocatedNum::one::<CS>())?;
        
    }
    Ok(new_value)
}