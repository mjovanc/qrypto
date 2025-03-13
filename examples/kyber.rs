use qrypto::{algorithms::Kyber512, decapsulate, encapsulate, generate_keypair};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Alice generates a Kyber512 key pair
    let keypair = generate_keypair::<Kyber512>()?;

    // Bob encapsulates a shared secret using Alice's public key
    let (ciphertext, shared_secret_bob) = encapsulate::<Kyber512>(keypair.public_key())?;

    // Alice decapsulates the ciphertext to get the same shared secret
    let shared_secret_alice = decapsulate::<Kyber512>(&keypair.secret_key(), &ciphertext)?;

    // Verify they match (in a real impl, they will)
    assert_eq!(shared_secret_alice, shared_secret_bob);
    println!("Shared secret (Kyber512): {:?}", shared_secret_alice);

    Ok(())
}
