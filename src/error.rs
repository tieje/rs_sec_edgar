//! This module houses the error type for EDGAR.

use std::io;
use thiserror::Error;

#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum EDGARError {
    #[error("Regex Err")]
    RegexErr {
        #[from]
        source: regex::Error,
    },
    #[error("IO Error")]
    IOError {
        #[from]
        source: io::Error,
    },
    #[error("Reqwest Error")]
    ReqwestError {
        #[from]
        source: reqwest::Error,
    },
    #[error("Atom Syndication Error")]
    AtomSyndicationError {
        #[from]
        source: atom_syndication::Error,
    },
    #[error("Url Parse Error")]
    UrlParseError {
        #[from]
        source: url::ParseError,
    },
    #[error("Serde XML Error")]
    SerdeXMLError {
        #[from]
        source: serde_xml_rs::Error,
    },
    #[error("CIK Not Found")]
    CIKNotFound,
    #[error("Filing Content Value Not Found")]
    FilingTypeNotFound,
    #[error("Filing Content Not Found")]
    FilingContentNotFound,
    #[error("Filing Content Value Not Found")]
    FilingContentValueNotFound,
    #[error("User Agent Env Var Missing")]
    UserAgentEnvVarMissing,
    #[error("Getting Feed Failed")]
    GettingFeedFailed,
    #[error("Owner Option Not Found")]
    OwnerOptionNotFound,
}
