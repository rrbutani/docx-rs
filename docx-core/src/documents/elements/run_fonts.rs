use serde::{Deserialize, Serialize};

use crate::documents::BuildXML;
use crate::xml_builder::*;

/*
  17.3.2.26 rFonts (Run Fonts)
  This element specifies the fonts which shall be used to display the text contents of this run.
  Within a single run, there can be up to four types of font slot which shall each be allowed to use a unique font:
  - ASCII (i.e., the first 128 Unicode code points)
  - High ANSI
  - Complex Script
*/
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RunFonts {
    pub ascii: Option<String>,
    pub hi_ansi: Option<String>,
    pub east_asia: Option<String>,
    pub cs: Option<String>,
}

impl RunFonts {
    pub fn new() -> RunFonts {
        Default::default()
    }

    pub fn ascii(mut self, f: impl Into<String>) -> Self {
        self.ascii = Some(f.into());
        self
    }

    pub fn hi_ansi(mut self, f: impl Into<String>) -> Self {
        self.hi_ansi = Some(f.into());
        self
    }

    pub fn east_asia(mut self, f: impl Into<String>) -> Self {
        self.east_asia = Some(f.into());
        self
    }

    pub fn cs(mut self, f: impl Into<String>) -> Self {
        self.cs = Some(f.into());
        self
    }
}

impl BuildXML for RunFonts {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.run_fonts(
            self.ascii.as_ref(),
            self.hi_ansi.as_ref(),
            self.cs.as_ref(),
            self.east_asia.as_ref(),
        )
        .build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_run_fonts_east_asia_build() {
        let c = RunFonts::new().east_asia("Hiragino");
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:rFonts w:eastAsia="Hiragino" />"#
        );
    }

    #[test]
    fn test_run_fonts_ascii_build() {
        let c = RunFonts::new().ascii("Arial");
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:rFonts w:ascii="Arial" />"#
        );
    }

    #[test]
    fn test_run_fonts_hi_ansi_build() {
        let c = RunFonts::new().hi_ansi("Arial");
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:rFonts w:hAnsi="Arial" />"#
        );
    }

    #[test]
    fn test_run_fonts_cs_build() {
        let c = RunFonts::new().cs("Arial");
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:rFonts w:cs="Arial" />"#
        );
    }
}
