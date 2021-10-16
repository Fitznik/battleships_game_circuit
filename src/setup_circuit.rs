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
pub struct SetupCircuite<E: Engine> {
    pub one_deck_ship: [[Option<E::Fr>; 2]; 4],
    pub two_deck_ship: [([Option<E::Fr>; 2], Option<bool>); 3],
    pub three_deck_ship: [([Option<E::Fr>; 2], Option<bool>); 2],
    pub four_deck_ship: [([Option<E::Fr>; 2], Option<bool>); 1],
    pub commit: Option<E::Fr>,
    pub board: [Option<bool>; 100],
    pub salt: Option<E::Fr>,

}

impl Circuit<Bn256> for SetupCircuite<Bn256>{
    fn synthesize<CS: ConstraintSystem<Bn256>>(self, cs: &mut CS) -> Result<(), SynthesisError>{
        let rescue_params = &Bn256RescueParams::new_checked_2_into_1();
        let commit = AllocatedNum::alloc(cs.namespace(|| "commit"), || self.commit.grab())?;
        let salt = AllocatedNum::alloc(cs.namespace(|| "salt"), || self.salt.grab())?;

        let one_deck_ship: Vec<Ship<Bn256, 1>> = self.one_deck_ship
        .iter()
        .enumerate()
        .map(|(i, [x, y])| {
                Ship::<Bn256, 1>{
                    head_pos_x: AllocatedNum::alloc(cs.namespace(|| format!("input {}", i+4528311)), || x.grab()).unwrap(),
                    head_pos_y: AllocatedNum::alloc(cs.namespace(|| format!("input {}", i+784539)), || y.grab()).unwrap(),
                    gorizontal: Boolean::Constant(false)

            
        }})
        .collect();
        

        let two_deck_ship: Vec<Ship::<Bn256, 2>> = self.two_deck_ship
        .iter()
        .enumerate()
        .map(|(i, ([x, y], b))| {
                Ship::<Bn256, 2>{
                    head_pos_x: AllocatedNum::alloc(cs.namespace(|| format!("input {}", i+12345621)), || x.grab()).unwrap(),
                    head_pos_y: AllocatedNum::alloc(cs.namespace(|| format!("input {}", i+9999999)), || y.grab()).unwrap(),
                    gorizontal:Boolean::from(AllocatedBit::alloc(cs.namespace(|| format!("input {}", i+121200)), *b).unwrap())
        }})
        .collect();

        let three_deck_ship:  Vec<Ship::<Bn256, 3>>= self.three_deck_ship
        .iter()
        .enumerate()
        .map(|(i, ([x, y], b))| {
            Ship::<Bn256, 3>{
                head_pos_x: AllocatedNum::alloc(cs.namespace(|| format!("input {}", i+15151515)), || x.grab()).unwrap(),
                head_pos_y: AllocatedNum::alloc(cs.namespace(|| format!("input {}", i+591231)), || y.grab()).unwrap(),
                gorizontal:Boolean::from(AllocatedBit::alloc(cs.namespace(|| format!("input {}", i+309821)), *b).unwrap())
    }})
    .collect();

        let four_deck_ship:  Vec<Ship::<Bn256, 4>> = self.four_deck_ship
        .iter()
        .enumerate()
        .map(|(i, ([x, y], b))| {
            Ship::<Bn256, 4>{
                head_pos_x: AllocatedNum::alloc(cs.namespace(|| format!("input {}", i+1000001)), || x.grab()).unwrap(),
                head_pos_y: AllocatedNum::alloc(cs.namespace(|| format!("input {}", i+1200023)), || y.grab()).unwrap(), 
                gorizontal:Boolean::from(AllocatedBit::alloc(cs.namespace(|| format!("input {}", i+123774)), *b).unwrap())
    }})
    .collect();


        let board: Vec<Boolean> = self.board
        .iter()
        .enumerate()
        .map(|(i, b)| {
            Boolean::from(
                AllocatedBit::alloc(cs.namespace(|| format!("input {}", i+56789)), *b).unwrap()
            )
        })
        .collect();
        let board = Board{
            square: board
        };

        let board_in_alloc_num = board.board_into_alloc_num(cs, "0001")?;
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
            check_ships = Boolean::and(cs.namespace(|| format!("input {}", i+709090)), &check_ship, &check_ship)?;
        }
        let nor_check_ship = AllocatedBit::nor(cs.namespace(|| "nor result is true ship-1"), &Boolean::get_variable(&check_ships).unwrap(), &Boolean::get_variable(&check_ships).unwrap())?;
        AllocatedBit::alloc_conditionally(
            cs.namespace(|| "check ship-1 is true  "),
            Some(true), 
            &nor_check_ship
        
        )?;

        let mut check_ships = Boolean::Constant(true);
        for i in 0..3{
            let check_ship = two_deck_ship[i].check_ship(cs, &board)?;
            check_ships = Boolean::and(cs.namespace(|| format!("input {}", i+129000)), &check_ship, &check_ship)?;
        }
        let nor_check_ship = AllocatedBit::nor(cs.namespace(|| "nor result is true ship-2"), &Boolean::get_variable(&check_ships).unwrap(), &Boolean::get_variable(&check_ships).unwrap())?;
        AllocatedBit::alloc_conditionally(
            cs.namespace(|| "check ship-2 is true "),
            Some(true), 
            &nor_check_ship
        
        )?;
        let mut check_ships = Boolean::Constant(true);
        for i in 0..2{
            let check_ship = three_deck_ship[i].check_ship(cs, &board)?;
            check_ships = Boolean::and(cs.namespace(|| format!("input {}", i+1000002)), &check_ship, &check_ship)?;
        }
        let nor_check_ship = AllocatedBit::nor(cs.namespace(|| "nor result is true ship-3"), &Boolean::get_variable(&check_ships).unwrap(), &Boolean::get_variable(&check_ships).unwrap())?;
        AllocatedBit::alloc_conditionally(
            cs.namespace(|| " check ship-3 is true "),
            Some(true), 
            &nor_check_ship
        
        )?;
        let mut check_ships = Boolean::Constant(true);
        for i in 0..1{
            let check_ship = four_deck_ship[i].check_ship(cs, &board)?;
            check_ships = Boolean::and(cs.namespace(|| format!("input {}", i+202020)), &check_ship, &check_ship)?;
        }
        let nor_check_ship = AllocatedBit::nor(cs.namespace(|| "nor result is true ship-4"), &Boolean::get_variable(&check_ships).unwrap(), &Boolean::get_variable(&check_ships).unwrap())?;
        AllocatedBit::alloc_conditionally(
            cs.namespace(|| "check ship-4 is true "),
            Some(true), 
            &nor_check_ship
        
        )?;
        
        let mut check_space = Boolean::Constant(true);
        for i in 0..4{
            check_space = one_deck_ship[i].check_ship_space(cs, &board)?;
            check_space = Boolean::and(cs.namespace(|| format!("input {}", i+98981)), &check_space, &check_space)?;
        }
        let nor_check_space = AllocatedBit::nor(cs.namespace(|| "nor result space ship-1"), &Boolean::get_variable(&check_space).unwrap(), &Boolean::get_variable(&check_space).unwrap())?;
        AllocatedBit::alloc_conditionally(
            cs.namespace(|| " check is true ship-4 space"),
            Some(true), 
            &nor_check_space
        
        )?;

        let mut check_space = Boolean::Constant(true);
        for i in 0..3{
            check_space = two_deck_ship[i].check_ship_space(cs, &board)?;
            check_space = Boolean::and(cs.namespace(|| format!("input {}", i+10465)), &check_space, &check_space)?;
        }
        let nor_check_space = AllocatedBit::nor(cs.namespace(|| format!("input {}", 3547)), &Boolean::get_variable(&check_space).unwrap(), &Boolean::get_variable(&check_space).unwrap())?;
        AllocatedBit::alloc_conditionally(
            cs.namespace(|| format!("input {}", 1114)),
            Some(true), 
            &nor_check_space
        
        )?;

        let mut check_space = Boolean::Constant(true);
        for i in 0..2{
            check_space = three_deck_ship[i].check_ship_space(cs, &board)?;
            check_space = Boolean::and(cs.namespace(|| format!("input {}", i+12465)), &check_space, &check_space)?;
        }
        let nor_check_space = AllocatedBit::nor(cs.namespace(|| format!("input {}", 559)), &Boolean::get_variable(&check_space).unwrap(), &Boolean::get_variable(&check_space).unwrap())?;
        AllocatedBit::alloc_conditionally(
            cs.namespace(|| format!("input {}", 555)),
            Some(true), 
            &nor_check_space
        
        )?;

        let mut check_space = Boolean::Constant(true);
        for i in 0..1{
            check_space = four_deck_ship[i].check_ship_space(cs, &board)?;
            check_space = Boolean::and(cs.namespace(|| "check space for ship-4"), &check_space, &check_space)?;
        }
        let nor_check_space = AllocatedBit::nor(cs.namespace(|| "nor result ship-4"), &Boolean::get_variable(&check_space).unwrap(), &Boolean::get_variable(&check_space).unwrap())?;
        AllocatedBit::alloc_conditionally(
            cs.namespace(|| "check is ship-4 is true "),
            Some(true), 
            &nor_check_space
        
        )?;

        let check_amount =check_amount_ship(cs, &board)?;
        let nor_check_amount = AllocatedBit::nor(cs.namespace(|| "nor result amount"), &Boolean::get_variable(&check_amount).unwrap(), &Boolean::get_variable(&check_amount).unwrap())?;
        AllocatedBit::alloc_conditionally(
            cs.namespace(|| "check amount"),
            Some(true), 
            &nor_check_amount
            
        )?;

        Ok(())
    }
}
