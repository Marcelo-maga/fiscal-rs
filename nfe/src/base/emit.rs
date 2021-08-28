//! Emitente da NF-e

use super::endereco::*;
use super::Error;
use serde::Deserialize;
use std::str::FromStr;

/// Emitente da NF-e
#[derive(Debug, Deserialize, PartialEq)]
pub struct Emitente {
    #[serde(rename = "CNPJ")]
    pub cnpj: String,
    #[serde(rename = "xNome")]
    pub razao_social: String,
    #[serde(rename = "xFant")]
    pub nome_fantasia: Option<String>,
    #[serde(rename = "IE")]
    pub ie: String,
    #[serde(rename = "IEST")]
    pub iest: Option<u32>,
    #[serde(rename = "enderEmit")]
    pub endereco: Endereco,
}

impl FromStr for Emitente {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s).map_err(|e| e.into())
    }
}
