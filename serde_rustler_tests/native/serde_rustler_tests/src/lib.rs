//! Library implementing tests to be called from ExUnit.
//!
//! See `run_ser_test` and `run_de_test` for details about how to use `serde_rustler::Serializer` and `serde_rustler::Deserializer`.

extern crate rustler;

mod json;
mod test;
mod types;

use crate::types::Animal;
use rustler::{types::tuple, Encoder, Env, NifResult, Term};
use serde_rustler::{atoms, from_term, to_term, Deserializer, Error, Serializer};

rustler::init!("Elixir.SerdeRustlerTests");

/// Implements the README example.
#[inline]
#[rustler::nif]
pub fn readme<'a>(env: Env<'a>, arg: Term) -> NifResult<Term<'a>> {
    let animal: Animal = from_term(arg)?;
    // println!("\n deserialized animal from README example: {:?}", animal);
    to_term(env, animal).map_err(|err| err.into())
}

/// Deserializes anything from an Elixir term and subsequently serializes the result back into an Elixir term, returning it.
#[inline]
fn internal_transcode<'a>(env: Env<'a>, arg: Term) -> NifResult<Term<'a>> {
    tag_tuple(env, || {
        let de = Deserializer::from(arg);
        let ser = Serializer::from(env);
        serde_transcode::transcode(de, ser)
    })
}

#[inline]
#[rustler::nif]
pub fn transcode<'a>(env: Env<'a>, arg: Term) -> NifResult<Term<'a>> {
    internal_transcode(env, arg)
}

#[inline]
#[rustler::nif(schedule = "DirtyCpu")]
pub fn transcode_dirty<'a>(env: Env<'a>, arg: Term) -> NifResult<Term<'a>> {
    internal_transcode(env, arg)
}

fn tag_tuple<'a, F>(env: Env<'a>, func: F) -> NifResult<Term<'a>>
where
    F: FnOnce() -> Result<Term<'a>, Error>,
{
    match func() {
        Ok(term) => Ok(ok_tuple(env, term)),
        Err(reason) => {
            let reason_term = reason.to_string().encode(env);
            Ok(error_tuple(env, reason_term))
        }
    }
}

fn ok_tuple<'a>(env: Env<'a>, term: Term<'a>) -> Term<'a> {
    let ok_atom_term = atoms::ok().encode(env);
    tuple::make_tuple(env, &[ok_atom_term, term])
}

fn error_tuple<'a>(env: Env<'a>, reason_term: Term<'a>) -> Term<'a> {
    let err_atom_term = atoms::error().encode(env);
    tuple::make_tuple(env, &[err_atom_term, reason_term])
}
