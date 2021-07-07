//! nfe - Crate para acesso aos dados da Nota Fiscal Eletrônica

mod ide;
mod emit;
mod dest;
mod nfe;
mod nfe_base;

pub use crate::{
    nfe_base::*,
    nfe::*,
    emit::*,
    dest::*,
    ide::*
};

#[cfg(test)]
mod tests;
