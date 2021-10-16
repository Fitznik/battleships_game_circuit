mod coursegame_circuit;
mod setup_circuit;
mod board_tools;
mod ship_tools; 

use coursegame_circuit::*;
use setup_circuit::*;
use board_tools::*;

use franklin_crypto::bellman::{
    pairing::{
        bn256::Bn256,
    
    },

}; 
use franklin_crypto::bellman::pairing::ff::{PrimeField, ScalarEngine};


use rand::thread_rng;

use franklin_crypto::bellman::groth16::{
    create_random_proof,
    generate_random_parameters,
    prepare_verifying_key, 
    verify_proof  };

fn main() {
    let rng = &mut thread_rng();
    let params_cours_cir = {
        let c = BaseCircuite::<Bn256>{
            board: [None; 100],
            commit: None,
            salt: None,
            pos_x: None,
            pos_y: None,
            claimed_value: None,
        
        };
        generate_random_parameters(c, rng).unwrap()
    };
    let pvk = prepare_verifying_key(&params_cours_cir.vk);

   let board = [Some(true), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(true), Some(false), 
                Some(false), Some(false), Some(true), Some(true), Some(true), Some(true), Some(false), Some(false), Some(true), Some(false), 
                Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(true), Some(false), 
                Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), 
                Some(false), Some(false), Some(false), Some(false), Some(true), Some(true), Some(false), Some(true), Some(false), Some(false), 
                Some(false), Some(true), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), 
                Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(true), Some(false), Some(false), Some(true), 
                Some(false), Some(false), Some(true), Some(true), Some(true), Some(false), Some(true), Some(false), Some(false), Some(true), 
                Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), 
                Some(false), Some(true), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false)];
    // hash: 0x17620e965c34449801592aa56a7c05b7539c5bcb49370af3c252ed94e49ac972
    // hash in dec: 10576447207914496356066582761873345551809146087656932304357887380116557449586
    // salt: 1515151515151515
    let salt = Some(<Bn256 as  ScalarEngine>::Fr::from_str("1515151515151515").unwrap());
    let commit = Some(<Bn256 as  ScalarEngine>::Fr::from_str("10576447207914496356066582761873345551809146087656932304357887380116557449586").unwrap());
    let pos_y = Some(<Bn256 as  ScalarEngine>::Fr::from_str("0").unwrap());
    let pos_x = Some(<Bn256 as  ScalarEngine>::Fr::from_str("0").unwrap());
    let claimed_value = Some(true);

    let c = BaseCircuite::<Bn256>{
        board: board,
        commit: commit,
        salt: salt,
        pos_x: pos_x,
        pos_y: pos_y,
        claimed_value: claimed_value,
    };
    let proof = create_random_proof(c, &params_cours_cir, rng).unwrap();
    println!("{}", verify_proof(&pvk, &proof, &[]).unwrap());

    let params_setap_cir = {
        let c = SetupCircuite::<Bn256>{
            one_deck_ship: [[None; 2]; 4],
            two_deck_ship: [([None; 2], None); 3],
            three_deck_ship: [([None; 2], None); 2],
            four_deck_ship: [([None; 2], None); 1],
            commit: None,
            board: [None; 100],
            salt: None,
        
        };
        generate_random_parameters(c, rng).unwrap()
    };
    let pvk = prepare_verifying_key(&params_setap_cir.vk);
    let board = [Some(true), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(true), Some(false), 
                Some(false), Some(false), Some(true), Some(true), Some(true), Some(true), Some(false), Some(false), Some(true), Some(false), 
                Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(true), Some(false), 
                Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), 
                Some(false), Some(false), Some(false), Some(false), Some(true), Some(true), Some(false), Some(true), Some(false), Some(false), 
                Some(false), Some(true), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), 
                Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(true), Some(false), Some(false), Some(true), 
                Some(false), Some(false), Some(true), Some(true), Some(true), Some(false), Some(true), Some(false), Some(false), Some(true), 
                Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), 
                Some(false), Some(true), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false), Some(false)];

    let salt = Some(<Bn256 as  ScalarEngine>::Fr::from_str("1515151515151515").unwrap());
    let commit = Some(<Bn256 as  ScalarEngine>::Fr::from_str("10576447207914496356066582761873345551809146087656932304357887380116557449586").unwrap());

    let one_deck_ship = [[Some(<Bn256 as  ScalarEngine>::Fr::from_str("0").unwrap()), Some(<Bn256 as  ScalarEngine>::Fr::from_str("0").unwrap())],
                         [Some(<Bn256 as  ScalarEngine>::Fr::from_str("7").unwrap()), Some(<Bn256 as  ScalarEngine>::Fr::from_str("40").unwrap())],
                         [Some(<Bn256 as  ScalarEngine>::Fr::from_str("1").unwrap()), Some(<Bn256 as  ScalarEngine>::Fr::from_str("50").unwrap())],
                         [Some(<Bn256 as  ScalarEngine>::Fr::from_str("1").unwrap()), Some(<Bn256 as  ScalarEngine>::Fr::from_str("90").unwrap())]];
    let two_deck_ship = [([Some(<Bn256 as  ScalarEngine>::Fr::from_str("4").unwrap()), Some(<Bn256 as  ScalarEngine>::Fr::from_str("40").unwrap())], Some(true)),
                        ([Some(<Bn256 as  ScalarEngine>::Fr::from_str("6").unwrap()), Some(<Bn256 as  ScalarEngine>::Fr::from_str("60").unwrap())], Some(false)),
                        ([Some(<Bn256 as  ScalarEngine>::Fr::from_str("9").unwrap()), Some(<Bn256 as  ScalarEngine>::Fr::from_str("60").unwrap())], Some(false))];
    let three_deck_ship =[([Some(<Bn256 as  ScalarEngine>::Fr::from_str("8").unwrap()), Some(<Bn256 as  ScalarEngine>::Fr::from_str("0").unwrap())], Some(false)),
                          ([Some(<Bn256 as  ScalarEngine>::Fr::from_str("2").unwrap()), Some(<Bn256 as  ScalarEngine>::Fr::from_str("70").unwrap())], Some(true))];
    let four_deck_ship = [([Some(<Bn256 as  ScalarEngine>::Fr::from_str("2").unwrap()), Some(<Bn256 as  ScalarEngine>::Fr::from_str("10").unwrap())], Some(true))];

    let a = SetupCircuite::<Bn256>{
        one_deck_ship: one_deck_ship,
        two_deck_ship: two_deck_ship,
        three_deck_ship: three_deck_ship,
        four_deck_ship: four_deck_ship,
        commit: commit,
        board: board,
        salt: salt,
    };
    let proof = create_random_proof(a, &params_setap_cir, rng).unwrap();
    println!("{}", verify_proof(&pvk, &proof, &[]).unwrap());


    





//               for genarate hash
    // use franklin_crypto::bellman::{ConstraintSystem};
    // use franklin_crypto::bellman::pairing::bls12_381::{Bls12, Fr};
    // use franklin_crypto::bellman::pairing::ff::{Field, PrimeField, PrimeFieldDecodingError, PrimeFieldRepr, ScalarEngine, SqrtField};
    // use franklin_crypto::circuit::test::TestConstraintSystem;
    // use franklin_crypto::bellman::{
    //     pairing::{
    //         bn256::Bn256,
    //         Engine
        
    //     },
    
    // };
    // let mut cs = &mut TestConstraintSystem::<Bn256>::new();         
    // let rescue_params = &Bn256RescueParams::new_checked_2_into_1();
    // let salt = <Bn256 as  ScalarEngine>::Fr::from_str("1515151515151515").unwrap();
    // let salt = AllocatedNum::alloc(cs.namespace(|| "salt"), || Ok(salt)).unwrap();
    // let board: Vec<Boolean> = board
    // .iter()
    // .enumerate()
    // .map(|(i, b)| {
    //     Boolean::from(
    //         AllocatedBit::alloc(cs.namespace(|| format!("input {}", i)), *b).unwrap()
    //     )
    // })
    // .collect();


    // let board = Board{
    //     square: board
    // };

    // let board_in_alloc_num = board.board_into_alloc_num(cs, "567890").unwrap();
    // let commit_2 = hash_board(cs, board_in_alloc_num, salt, rescue_params).unwrap();

    // dbg!(commit_2.get_value().unwrap());
}

