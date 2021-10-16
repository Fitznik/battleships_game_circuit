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

use crate::board_tools::Board;

pub struct Ship<E:Engine, const N: usize>{
    pub head_pos_x: AllocatedNum<E>,
    pub head_pos_y: AllocatedNum<E>,
    pub gorizontal: Boolean,
    
    
}
impl<E:Engine, const N: usize> Ship<E, N>{
    pub fn check_ship<CS: ConstraintSystem<E>>(
        &self, 
        cs: &mut CS,
        board: &Board, 
    )-> Result<Boolean, SynthesisError>{
        let ten = ten(cs)?;
        let one = AllocatedNum::one::<CS>();

        let mut check_ship = board.board_pos(cs, &self.head_pos_x, &self.head_pos_y, "101")?;
        let head_pos_x_add =  self.head_pos_x.add(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), 66)), &one)?;
        let head_pos_y_add =  self.head_pos_y.add(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), 99)), &ten)?;

        let mut pos_x = AllocatedNum::conditionally_select(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), 908677000)), &head_pos_x_add , &self.head_pos_x, &self.gorizontal)?;
        let mut pos_y = AllocatedNum::conditionally_select(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), 45430530)), &self.head_pos_y, &head_pos_y_add, &self.gorizontal)?;
        
        for i in 0..N-1{
            let check = board.board_pos(cs, &pos_x, &pos_y, &format!("ratata input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), i+49800)[..])?;
            check_ship = Boolean::and(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), i+5000)), &check_ship, &check)?;


            let pos_x_add =  pos_x.add(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), i+1000)), &one)?;
            let pos_y_add =  pos_y.add(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), i+2000)), &ten)?;
            pos_x = AllocatedNum::conditionally_select(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), i+3000)), &pos_x_add , &pos_x, &self.gorizontal)?;
            pos_y = AllocatedNum::conditionally_select(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), i+4000)), &pos_y, &pos_y_add, &self.gorizontal)?;
        }
        


        Ok(check_ship)
        
    }

    pub fn check_ship_space<CS: ConstraintSystem<E>>(
        &self, 
        cs: &mut CS,
        board: &Board,
    )-> Result<Boolean, SynthesisError>{

        let ten = ten(cs)?;
        let one = AllocatedNum::one::<CS>();

        let extrem_left_pos_x = self.head_pos_x.sub(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), 20)), &one)?;
        let extrem_left_pos_y = self.head_pos_y.sub(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), 21)), &ten)?;
        let mut check_ship_space = board.board_pos(cs, &extrem_left_pos_x, &extrem_left_pos_y, "167")?.not();

        let extrem_left_pos_x_add =  extrem_left_pos_x.add(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), 22)), &one)?;
        let extrem_left_pos_y_add =  extrem_left_pos_y.add(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), 23)), &ten)?;


        let mut space_pos_x = AllocatedNum::conditionally_select(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), 24)), &extrem_left_pos_x_add, &extrem_left_pos_x, &self.gorizontal)?;
        let mut space_pos_y = AllocatedNum::conditionally_select(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), 25)), &extrem_left_pos_y, &extrem_left_pos_y_add, &self.gorizontal)?;
        let mut check = board.board_pos(cs, &space_pos_x, &space_pos_y, &format!("hnhnfsfshhnn input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), 49800)[..])?.not();
        check_ship_space = Boolean::and(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), 26)), &check_ship_space , &check)?;

        for i in 0..N{
            let pos_x_add =  space_pos_x.add(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), i+6000)), &one)?;
            let pos_y_add =  space_pos_y.add(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), i+7000)), &ten)?;

            space_pos_x = AllocatedNum::conditionally_select(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), i+8000)), &pos_x_add, &space_pos_x, &self.gorizontal)?;
            space_pos_y = AllocatedNum::conditionally_select(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), i+9000)), &space_pos_y, &pos_y_add, &self.gorizontal)?;
            check = board.board_pos(cs, &space_pos_x, &space_pos_y, &format!("nmn input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), i+49800)[..])?.not();
            check_ship_space = Boolean::and(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), i+10000)), &check_ship_space , &check)?;

        }

        let mut other_ship_dir_x = one.add(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), 30)), &space_pos_x)?;
        let mut other_ship_dir_y = ten.add(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), 31)), &space_pos_y)?;

        space_pos_x = AllocatedNum::conditionally_select(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), 32)), &space_pos_x, &other_ship_dir_x, &self.gorizontal)?;
        space_pos_y = AllocatedNum::conditionally_select(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), 33)), &other_ship_dir_y, &space_pos_y, &self.gorizontal)?;

        check = board.board_pos(cs, &space_pos_x, &space_pos_y, "2781")?.not();
        check_ship_space  = Boolean::and(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), 34)), &check_ship_space, &check)?;

        other_ship_dir_x = one.add(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), 35)), &space_pos_x)?;
        other_ship_dir_y = ten.add(cs.namespace(|| " "), &space_pos_y)?;

        space_pos_x = AllocatedNum::conditionally_select(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), 36)), &space_pos_x, &other_ship_dir_x, &self.gorizontal)?;
        space_pos_y = AllocatedNum::conditionally_select(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), 37)), &other_ship_dir_y, &space_pos_y, &self.gorizontal)?;

        check = board.board_pos(cs, &space_pos_x, &space_pos_y, "1234")?.not();
        check_ship_space  = Boolean::and(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), 38)), &check_ship_space, &check)?;


        for i in 0..N+1{
            let pos_x_add =  space_pos_x.sub(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), i+11000)), &one)?;
            let pos_y_add =  space_pos_y.sub(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), i+12000)), &ten)?;

            space_pos_x = AllocatedNum::conditionally_select(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), i+13000)), &pos_x_add, &space_pos_x, &self.gorizontal)?;
            space_pos_y = AllocatedNum::conditionally_select(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), i+14000)), &space_pos_y, &pos_y_add, &self.gorizontal)?;
            check = board.board_pos(cs, &space_pos_x, &space_pos_y, &format!("pxpxp input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), i+49800)[..])?.not();
            check_ship_space = Boolean::and(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), i+15000)), &check_ship_space , &check)?;

        }

        other_ship_dir_x = space_pos_x.sub(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), 50)), &one)?;
        other_ship_dir_y = space_pos_y.sub(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), 52)), &ten)?;

        space_pos_x = AllocatedNum::conditionally_select(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), 53)), &space_pos_x, &other_ship_dir_x, &self.gorizontal)?;
        space_pos_y = AllocatedNum::conditionally_select(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), 54)), &other_ship_dir_y, &space_pos_y, &self.gorizontal)?;

        check = board.board_pos(cs, &space_pos_x, &space_pos_y, "1237")?.not();
        check_ship_space  = Boolean::and(cs.namespace(|| format!("input {:?} {:?} {}", self.head_pos_x.get_value(), self.head_pos_y.get_value(), 55)), &check_ship_space, &check)?;




        Ok(check_ship_space)
    }



}

pub fn check_amount_ship<E:Engine, CS: ConstraintSystem<E>>(
    cs: &mut CS,
    board: &Board, 
    )->Result<Boolean, SynthesisError>{

    let alloc_num_zero = AllocatedNum::<E>::zero(cs.namespace(|| "zero constant"))?;
    let mut amount = AllocatedNum::<E>::zero(cs.namespace(|| "zero constant"))?;

    for i in 0..100 {
        let bit = AllocatedNum::<E>::conditionally_select( 
            cs.namespace(|| format!("input {}", i+15000)), 
            &AllocatedNum::one::<CS>(), 
            &alloc_num_zero, 
            &board.square[i])?;
        amount = amount.add( cs.namespace(|| format!("input {}", i+16000)), &bit)?;
    }
    let ten = ten(cs)?;
    let twenty = ten.add(cs.namespace(|| " twenty "), &ten)?;
    let check = AllocatedNum::<E>::equals(cs.namespace(|| "check if point of ship is twenty "), &twenty, &amount)?;

    Ok(Boolean::from(check))

}


pub fn ten<E:Engine, CS: ConstraintSystem<E>>(cs: &mut CS)-> Result<AllocatedNum<E>, SynthesisError>{
    let mut new_value = AllocatedNum::one::<CS>();

    for i in 0..9{
        new_value = new_value.add(cs.namespace(|| format!("input {}", i+17000)), &AllocatedNum::one::<CS>())?;
        
    }
    Ok(new_value)
}