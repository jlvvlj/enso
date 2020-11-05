//! Macro definitions in Enso.

use crate::prelude::*;
use crate::macros::literal::Literal;
use itertools::Itertools;


// ==================
// === Definition ===
// ==================

/// A macro definition.
///
/// A macro definition consists of a name, which identifies the macro to users, and a list of
/// [sections](`Section`). The sections are the most important portion of the macro definition, as
/// they define the literal portions of the token stream on which the macro will match.
#[derive(Clone,Debug,Default,Eq,PartialEq)]
pub struct Definition {
    /// The name of the macro definition.
    ///
    /// This is used both in error messages and for the purposes of nesting a macro within another
    /// macro.
    name : String,
    /// The sections that make up the macro.
    sections : Vec<Section>
}

impl Definition {
    /// Constructor.
    pub fn new(name:impl Str, sections:Vec<Section>) -> Self {
        let name = name.into();
        Self{name,sections}
    }

    /// Get the name of the macro.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Get the sections that make up this macro definition.
    pub fn sections(&self) -> &[Section] {
        self.sections.as_slice()
    }

    /// Get the path for the definition.
    ///
    /// The definition's path consists of the headers of each of the sections that make it up, and
    /// describes the literals that must be matched for the macro to match.
    pub fn path(&self) -> Vec<Literal> {
        self.sections.iter().map(|s| s.symbol.clone()).collect_vec()
    }
}



// ===============
// === Section ===
// ===============

/// A section in a macro, representing both a literal section header to match against, and the
/// tokens that the section contains.
///
/// The literal is the _most_ important portion of a section, as they are constants that allow the
/// macro resolver to divide up the input token stream based on these constants.
#[derive(Clone,Debug,Eq,PartialEq)]
pub struct Section {
    /// The literal that begins the section.
    symbol : Literal,
    // TODO [AA] Pattern (later)
}

impl Section {
    /// Constructor.
    pub fn new(symbol:Literal) -> Self {
        Self{ symbol }
    }

    /// Get a reference to the literal that heads the section.
    pub fn literal(&self) -> &Literal {
        &self.symbol
    }
}


// === Macros ===

// TODO [AA] Macro for section definition.
