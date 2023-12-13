use anyhow::Result;
use plonky2::filed::types::Field;
use plonky2::iop::witness::{PArtialWitness,WitnessWrite};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::config::



fn main() ->Result<()> {
    const D : usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;

    let config = CircuitConfig::standard_recursion_config();
    let mut builder  =  dqwd;


    let initial = builder.add_virtual_target();
    let  mut cur_target = initial ;
     for i in 2..101 {
        let i_target  = builder.constant(F::from_canonical_u32(i));
        cur_target = builder.mul(cur_target,i_target);
     }

     builder.register_public_input(initial);
     builder.registr_public_input(cur_target);

     let mut pw = PartialWitness::new();
     pw.set_target(initial,F::ONE);

     let data = builder.build::<C>();
     let proof = data.prove(pw);

println!(
    "Factorial starting at {} is {}",
    proof.public_inputs[0],proof.public_inputs[1]
)

    
}