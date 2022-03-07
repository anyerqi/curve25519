#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::*;

mod curve;
mod utils;
pub mod error;

use error::Result;


pub use {
    curve::{KeyPair as InterKeyPair, PrivateKey, PublicKey},
    error::CurveError,
};

#[napi(object)]
pub struct KeyPair {
    pub pub_key: Buffer,
    pub priv_key: Buffer,
}

#[napi]
pub fn generate_key_pair() -> Result<KeyPair> {
    let mut rng = rand::rngs::OsRng;
    let keypair = InterKeyPair::generate(&mut rng);
    let private_key = keypair.private_key;
    let pub_key = private_key.public_key()?;
    Ok(KeyPair{
      priv_key: private_key.serialize().into(),
      pub_key: pub_key.serialize().as_ref().into()
    })
}

#[napi]
pub fn calculate_signature(priv_key: Buffer, message: Buffer) -> Result<Buffer> {
  
  let priv_key_vec: Vec<u8> = priv_key.into();
  let msg_vec: Vec<u8> = message.into();
  let priv_key_obj = PrivateKey::deserialize(&priv_key_vec[..])?;
  let mut rng = rand::rngs::OsRng;
  Ok(priv_key_obj.calculate_signature(&msg_vec[..], &mut rng)?.into_vec().into())
}

#[napi]
pub fn verify_signature(
  pub_key: Buffer,
  message: Buffer,
  signature: Buffer
) -> Result<bool> {
  let pub_key_vec: Vec<u8> = pub_key.into();
  let msg_vec: Vec<u8> = message.into();
  let signature_vec: Vec<u8> = signature.into();
  let pub_key_obj = PublicKey::deserialize(&pub_key_vec[..])?;
  let verifyed = pub_key_obj.verify_signature(&msg_vec[..], &signature_vec[..])?;
  Ok(!verifyed)
}

#[napi]
pub fn create_key_pair(priv_key: Buffer) -> Result<KeyPair> {
  let priv_key_vec = priv_key.into();
  validate_priv_key(&priv_key_vec)?;
  //clamp_private_key(&mut priv_key_vec);
  let priv_key = PrivateKey::deserialize(&priv_key_vec[..])?;
  let pub_key = priv_key.public_key()?;
  Ok(KeyPair {
    priv_key: priv_key.serialize().into(),
    pub_key: pub_key.serialize().as_ref().into()
  })

}

#[napi]
pub fn calculate_agreement(pub_key: Buffer, priv_key: Buffer) -> Result<Buffer> {
  let priv_key_vec: Vec<u8> = priv_key.into();
  let pub_key_vec = pub_key.into();
  let prefix_pub_key = prefix_public_key(&pub_key_vec)?;
  let pub_key_obj = PublicKey::deserialize(&prefix_pub_key[..])?;
  let priv_key_obj = PrivateKey::deserialize(&priv_key_vec[..])?;
  let agree = priv_key_obj.calculate_agreement(&pub_key_obj)?;
  let shared_secret: Vec<u8> = agree.into_vec();
  Ok(shared_secret.into())
}

#[napi]
pub fn validate_pub_key_format(pub_key: Buffer) -> Result<Buffer> {
  let mut pub_key_vec: Vec<u8> = pub_key.into();
  _validate_pubkey_format(&mut pub_key_vec)?;
  Ok(pub_key_vec.into())
}


fn validate_priv_key(priv_key: &Vec<u8>) -> Result<()> {
  if priv_key.len() != 32 {
    return Err(Error::new(Status::InvalidArg, String::from("privkey is invalid")));
  }
  Ok(())
}

fn _validate_pubkey_format(pub_key: &mut Vec<u8>) -> Result<()> {
  if (pub_key.len() != 33 || pub_key[0] != 5) && pub_key.len() != 32 {
    return Err(Error::new(Status::InvalidArg, String::from("invalid public key")));
  }
  if pub_key.len() == 33 {
    pub_key.remove(0);
  }
  Ok(())
}

fn prefix_public_key(pub_key: &Vec<u8>) -> Result<Vec<u8>> {
  let mut copy = pub_key.clone();
  _validate_pubkey_format(&mut copy)?;
  copy.insert(0, 0x05u8);
  Ok(copy)
}

// fn clamp_private_key(private_key: &mut Vec<u8>) {
//   private_key[0] &= 248;
//   private_key[31] &= 127;
//   private_key[31] |= 64;
// }

#[napi]
pub fn hello() -> String {
  String::from("hello world!")
}

