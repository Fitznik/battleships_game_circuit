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
    let part1 = Boolean::and(cs.namespace(|| "part1"), &a, &condition)?;
    let part2 = Boolean::and(cs.namespace(|| "part2"), &b, &condition.not())?;
    let result = Boolean::xor(cs.namespace(|| "result for cond.select bolean"), &part1, &part2)?;
    Ok(result)


}

impl Board{
     pub fn board_into_alloc_num<E:Engine, CS: ConstraintSystem<E>>(&self, cs: &mut CS, id: &str) -> Result<AllocatedNum<E>, SynthesisError> {
        
        let board_in_alloc_num = AllocatedNum::pack_bits_to_element(cs.namespace(|| format!("{} input {}", id, 32)), &self.square[..])?;

        Ok(board_in_alloc_num)
    } 


    pub fn board_pos<E:Engine, CS: ConstraintSystem<E>>(&self, cs: &mut CS, pos_x: &AllocatedNum<E>, pos_y: &AllocatedNum<E>, id: &str)->Result<Boolean, SynthesisError>{
        let mut result = Boolean::Constant(false);
        let array = [0, 10, 20, 40, 50, 60, 70, 80, 90];
        for i in array{
            let i_allocated = AllocatedNum::<E>::alloc(cs.namespace(|| format!("{} input {}", id, i+9001)), || Ok(E::Fr::from_str(&i.to_string()).unwrap()))?;
            i_allocated.assert_number(
            cs.namespace(|| format!("{} input {}", id, i+10000)),
            &E::Fr::from_str(&i.to_string()).unwrap(),
            )?;
            for j in 0..10{
                let j_allocated = AllocatedNum::<E>::alloc(cs.namespace(|| format!("{} input {}", id, j+i+120000)), || Ok(E::Fr::from_str(&j.to_string()).unwrap()))?;
                j_allocated.assert_number(
                cs.namespace(|| format!("{} input {}", id, i+j+7000)),
                &E::Fr::from_str(&j.to_string()).unwrap(),
                )?;
                let flag_x = AllocatedNum::equals(cs.namespace(|| format!("{} input {}", id, i+j+1000)), &j_allocated, &pos_x)?;
                let flag_y = AllocatedNum::equals(cs.namespace(|| format!("{} input {}", id, i+j+2000)), &i_allocated, &pos_y)?;
                let combo_flag = Boolean::and(cs.namespace(|| format!("{} input {}", id, i+j+3000)), &Boolean::from(flag_x), &Boolean::from(flag_y))?;
                result = conditionally_select_boolean(cs.namespace(|| format!("{} input {}", id, i+j+4000)),  &self.square[i+j], &result, &combo_flag)?;
            }
         }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {

    use franklin_crypto::bellman::{ConstraintSystem};
    use franklin_crypto::bellman::pairing::bls12_381::{Bls12, Fr};
    use franklin_crypto::bellman::pairing::ff::{Field, PrimeField, PrimeFieldDecodingError, PrimeFieldRepr, ScalarEngine, SqrtField};
    use super::{
        AllocatedBit,
        Boolean,
    };
    use super::*;
    use franklin_crypto::circuit::test::TestConstraintSystem;
    use franklin_crypto::bellman::{
        pairing::{
            bn256::Bn256,
            Engine
        
        },
    
    };

    #[test]
    fn board_pos(){
        let mut cs = TestConstraintSystem::<Bn256>::new();
        let board = [Some(true), Some(false), Some(false), Some(false), Some(false), Some(true), Some(false), Some(false), Some(true), Some(false), 
        Some(false), Some(false), Some(false), Some(false), Some(false), Some(true), Some(false), Some(false), Some(false), Some(false), 
        Some(false), Some(false), Some(false), Some(false), Some(false), Some(true), Some(false), Some(false), Some(false), Some(true), 
        Some(false), Some(false), Some(false), Some(false), Some(false), Some(true), Some(false), Some(true), Some(false), Some(true), 
        Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(true), Some(false), Some(false), 
        Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(true), Some(false), Some(false), 
        Some(false), Some(true), Some(false), Some(true), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), 
        Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), 
        Some(true), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), 
        Some(true), Some(false), Some(true), Some(true), Some(false), Some(false), Some(false), Some(true), Some(true), Some(true)];
        let board: Vec<Boolean> = board
            .iter()
            .enumerate()
            .map(|(i, b)| {
                Boolean::from(
                    AllocatedBit::alloc(cs.namespace(|| format!("input {}", i)), *b).unwrap()
                    )
            })
            .collect();
        let mut board = Board{
            square: board,
        };
        

        let ex_1 = AllocatedNum::alloc(cs.namespace(|| "cs for ex_1"), || Ok(<Bn256 as  ScalarEngine>::Fr::from_str("0").unwrap())).unwrap();
        let ex_2 = AllocatedNum::alloc(cs.namespace(|| "cs for ex_2 "), || Ok(<Bn256 as ScalarEngine>::Fr::from_str("0").unwrap())).unwrap();
        let test_el = board.board_pos(&mut cs, &ex_1, &ex_2, "1").unwrap();
        let test_el = board.board_pos(&mut cs, &ex_1, &ex_2, "2").unwrap();
        assert_eq!(test_el.get_value().unwrap(), true);

    }
}