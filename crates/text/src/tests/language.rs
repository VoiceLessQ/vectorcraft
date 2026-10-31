//! Text is shaped in its language, so a font draws its forms for it (OpenType `locl`, #4940).

use vectorcraft_doc::{CharStyle, TextObject, TextRun};

use super::{db, point, serif};
use crate::*;

fn in_language(language: Option<&str>) -> CharStyle {
    CharStyle { language: language.map(str::to_string), ..serif(20.0) }
}

fn gids(t: &TextObject) -> Vec<u32> {
    layout(db(), t).glyphs.iter().map(|g| g.gid).collect()
}

#[test]
fn turkish_keeps_f_and_dotted_i_apart() {
    let plain = gids(&point("fi", in_language(None)));
    let turkish = gids(&point("fi", in_language(Some("tr"))));
    assert_eq!(plain.len(), 1, "the font's ligature without a language");
    assert_eq!(turkish.len(), 2, "Turkish has no fi ligature: {turkish:?}");
    // The shaper's plans are per language: the same face, shaped again, keeps them apart.
    assert_eq!(gids(&point("fi", in_language(None))), plain);
}

#[test]
fn a_language_change_is_a_shaping_boundary() {
    let mut t = point("fi", in_language(None));
    t.runs.push(TextRun::new("fi", in_language(Some("tr"))));
    let plain = gids(&point("fi", in_language(None)));
    let turkish = gids(&point("fi", in_language(Some("tr"))));
    assert_eq!(gids(&t), [plain, turkish].concat());
}

#[test]
fn a_tag_that_is_no_tag_shapes_without_a_language() {
    let plain = gids(&point("fi", in_language(None)));
    let long = "tr-".repeat(1000);
    for bad in ["", "-", "t", "tr_TR", "tr tr", "türkçe", long.as_str()] {
        assert_eq!(gids(&point("fi", in_language(Some(bad)))), plain, "{bad:.8}");
    }
}
