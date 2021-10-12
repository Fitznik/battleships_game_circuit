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

use crate::board_tools::Board;
use crate::ship_tools::Ship;
use crate::hash_board;
use crate::ship_tools::check_amount_ship;

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
    one_deck_ship: [Option<E::Fr>; 4],
    two_deck_ship: [(Option<E::Fr>, Option<bool>); 3],
    three_deck_ship: [(Option<E::Fr>, Option<bool>); 2],
    four_deck_ship: [(Option<E::Fr>, Option<bool>); 1],
    commit: Option<E::Fr>,
    board: [Option<bool>; 100],
    salt: Option<E::Fr>,
    pos: Option<E::Fr>,

}

impl Circuit<Bn256> for SetupCircuite<Bn256>{
    fn synthesize<CS: ConstraintSystem<Bn256>>(self, cs: &mut CS) -> Result<(), SynthesisError>{
        let rescue_params = &Bn256RescueParams::new_checked_2_into_1();
        let commit = AllocatedNum::alloc(cs.namespace(|| "commit"), || self.commit.grab())?;
        let salt = AllocatedNum::alloc(cs.namespace(|| "salt"), || self.salt.grab())?;
        let pos = AllocatedNum::alloc(cs.namespace(|| "pos"), || self.pos.grab())?;

        let one_deck_ship: Vec<Ship<Bn256, 1>> = self.one_deck_ship
        .iter()
        .enumerate()
        .map(|(i, n)| {
                Ship::<Bn256, 1>{
                    head_pos: AllocatedNum::alloc(cs.namespace(|| format!("input {}", i)), || n.grab()).unwrap(),
                    gorizontal: Boolean::Constant(false)

            
        }})
        .collect();
        

        let two_deck_ship: Vec<(Ship::<Bn256, 2>)> = self.two_deck_ship
        .iter()
        .enumerate()
        .map(|(i, (a, b))| {
                Ship::<Bn256, 2>{
                    head_pos: AllocatedNum::alloc(cs.namespace(|| format!("input {}", i)), || a.grab()).unwrap(), 
                    gorizontal:Boolean::from(AllocatedBit::alloc(cs.namespace(|| format!("input {}", i)), *b).unwrap())
        }})
        .collect();

        let three_deck_ship:  Vec<(Ship::<Bn256, 3>)>= self.three_deck_ship
        .iter()
        .enumerate()
        .map(|(i, (a, b))| {
            Ship::<Bn256, 3>{
                head_pos: AllocatedNum::alloc(cs.namespace(|| format!("input {}", i)), || a.grab()).unwrap(), 
                gorizontal:Boolean::from(AllocatedBit::alloc(cs.namespace(|| format!("input {}", i)), *b).unwrap())
    }})
    .collect();

        let four_deck_ship:  Vec<(Ship::<Bn256, 4>)> = self.four_deck_ship
        .iter()
        .enumerate()
        .map(|(i, (a, b))| {
            Ship::<Bn256, 4>{
                head_pos: AllocatedNum::alloc(cs.namespace(|| format!("input {}", i)), || a.grab()).unwrap(), 
                gorizontal:Boolean::from(AllocatedBit::alloc(cs.namespace(|| format!("input {}", i)), *b).unwrap())
    }})
    .collect();


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

        let board_in_alloc_num = board.board_into_alloc_num(cs)?;
        let commit_2 = hash_board(cs, board_in_alloc_num, salt, rescue_params)?;
        let check_commit = AllocatedNum::equals(cs.namespace(|| "check commit"), &commit, &commit_2)?;
        let nor_check_commit = AllocatedBit::nor(cs.namespace(|| "nor result"), &check_commit, &check_commit)?;
        AllocatedBit::alloc_conditionally(
            cs.namespace(|| "check commmit"),
            Some(true), 
            &nor_check_commit
        
        )?;
 
        let mut check_ships = Boolean::Constant(true);
        for i in 0..4{
            let check_ship = one_deck_ship[i].check_ship(cs, &board)?;
            check_ships = Boolean::and(cs.namespace(|| " "), &check_ship, &check_ship)?;
        }
        let nor_check_ship = AllocatedBit::nor(cs.namespace(|| "nor result"), &Boolean::get_variable(&check_ships).unwrap(), &Boolean::get_variable(&check_ships).unwrap())?;
        AllocatedBit::alloc_conditionally(
            cs.namespace(|| " "),
            Some(true), 
            &nor_check_ship
        
        )?;

        let mut check_ships = Boolean::Constant(true);
        for i in 0..3{
            let check_ship = two_deck_ship[i].check_ship(cs, &board)?;
            check_ships = Boolean::and(cs.namespace(|| " "), &check_ship, &check_ship)?;
        }
        let nor_check_ship = AllocatedBit::nor(cs.namespace(|| "nor result"), &Boolean::get_variable(&check_ships).unwrap(), &Boolean::get_variable(&check_ships).unwrap())?;
        AllocatedBit::alloc_conditionally(
            cs.namespace(|| " "),
            Some(true), 
            &nor_check_ship
        
        )?;
        let mut check_ships = Boolean::Constant(true);
        for i in 0..2{
            let check_ship = three_deck_ship[i].check_ship(cs, &board)?;
            check_ships = Boolean::and(cs.namespace(|| " "), &check_ship, &check_ship)?;
        }
        let nor_check_ship = AllocatedBit::nor(cs.namespace(|| "nor result"), &Boolean::get_variable(&check_ships).unwrap(), &Boolean::get_variable(&check_ships).unwrap())?;
        AllocatedBit::alloc_conditionally(
            cs.namespace(|| " "),
            Some(true), 
            &nor_check_ship
        
        )?;
        let mut check_ships = Boolean::Constant(true);
        for i in 0..1{
            let check_ship = four_deck_ship[i].check_ship(cs, &board)?;
            check_ships = Boolean::and(cs.namespace(|| " "), &check_ship, &check_ship)?;
        }
        let nor_check_ship = AllocatedBit::nor(cs.namespace(|| "nor result"), &Boolean::get_variable(&check_ships).unwrap(), &Boolean::get_variable(&check_ships).unwrap())?;
        AllocatedBit::alloc_conditionally(
            cs.namespace(|| " "),
            Some(true), 
            &nor_check_ship
        
        )?;
        
        let mut check_space = Boolean::Constant(true);
        for i in 0..4{
            check_space = one_deck_ship[i].check_ship_space(cs, &board)?;
            check_space = Boolean::and(cs.namespace(|| " "), &check_space, &check_space)?;
        }
        let nor_check_space = AllocatedBit::nor(cs.namespace(|| "nor result"), &Boolean::get_variable(&check_space).unwrap(), &Boolean::get_variable(&check_space).unwrap())?;
        AllocatedBit::alloc_conditionally(
            cs.namespace(|| " "),
            Some(true), 
            &nor_check_space
        
        )?;

        let mut check_space = Boolean::Constant(true);
        for i in 0..3{
            check_space = two_deck_ship[i].check_ship_space(cs, &board)?;
            check_space = Boolean::and(cs.namespace(|| " "), &check_space, &check_space)?;
        }
        let nor_check_space = AllocatedBit::nor(cs.namespace(|| "nor result"), &Boolean::get_variable(&check_space).unwrap(), &Boolean::get_variable(&check_space).unwrap())?;
        AllocatedBit::alloc_conditionally(
            cs.namespace(|| " "),
            Some(true), 
            &nor_check_space
        
        )?;

        let mut check_space = Boolean::Constant(true);
        for i in 0..2{
            check_space = three_deck_ship[i].check_ship_space(cs, &board)?;
            check_space = Boolean::and(cs.namespace(|| " "), &check_space, &check_space)?;
        }
        let nor_check_space = AllocatedBit::nor(cs.namespace(|| "nor result"), &Boolean::get_variable(&check_space).unwrap(), &Boolean::get_variable(&check_space).unwrap())?;
        AllocatedBit::alloc_conditionally(
            cs.namespace(|| " "),
            Some(true), 
            &nor_check_space
        
        )?;

        let mut check_space = Boolean::Constant(true);
        for i in 0..1{
            check_space = four_deck_ship[i].check_ship_space(cs, &board)?;
            check_space = Boolean::and(cs.namespace(|| " "), &check_space, &check_space)?;
        }
        let nor_check_space = AllocatedBit::nor(cs.namespace(|| "nor result"), &Boolean::get_variable(&check_space).unwrap(), &Boolean::get_variable(&check_space).unwrap())?;
        AllocatedBit::alloc_conditionally(
            cs.namespace(|| " "),
            Some(true), 
            &nor_check_space
        
        )?;

        let check_amount =check_amount_ship(cs, &board, &pos)?;
        let nor_check_amount = AllocatedBit::nor(cs.namespace(|| "nor result"), &Boolean::get_variable(&check_amount).unwrap(), &Boolean::get_variable(&check_amount).unwrap())?;
        AllocatedBit::alloc_conditionally(
            cs.namespace(|| "check amount"),
            Some(true), 
            &nor_check_amount
            
        )?;

        Ok(())
    }
}
