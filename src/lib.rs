use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// JSON-Format zum Austausch von .gbx-Dateien zwischen Server / Client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfFile {
    /// Ob diese Datei digitalisiert wurde (hat zugehörige PDF-Datei) oder nicht
    #[serde(default)]
    pub digitalisiert: bool,
    /// hOCR Layout der digitalisierten Datei,
    #[serde(skip_serializing_if = "HocrLayout::is_empty")]
    #[serde(default)]
    pub hocr: HocrLayout,
    /// Benutzerdefinierte Anpassungen an das Seitenlayout (SeitenTyp, etc.), indexiert nach Seitenzahl
    #[serde(default)]
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub anpassungen_seite: BTreeMap<String, AnpassungSeite>,
    /// Analysiertes / bearbeitetes Grundbuchblatt
    pub analysiert: Grundbuch,
}

/// Digitalisiertes Layout der erkannten Buchstaben auf den Seiten, indexiert nach Seitenzahl
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HocrLayout {
    /// hOCR-Layout der individuellen PDF-Seiten, indexiert nach Seitenzahl
    #[serde(default)]
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub seiten: BTreeMap<String, HocrSeite>,
}

impl HocrLayout {
    fn is_empty(&self) -> bool {
        self.seiten.is_empty()
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct HocrSeite {
    /// Breite der PDF-Seite in Millimeter
    pub breite_mm: f32,
    /// Höhe der PDF-Seite in Millimeter
    pub hoehe_mm: f32,
    /// Automatisch digitalisierte hOCR-Ausgabe der erkannten Texte auf der Seite
    pub parsed: ParsedHocr,
    /// Rote Linien auf der PDF-Seite
    #[serde(default)]
    pub rote_linien: Vec<Linie>,
}

/// Definition für eine rote Linie mit n Punkten auf der PDF-Seite
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Linie {
    /// Punkte der Linie auf der Seite
    pub punkte: Vec<Punkt>,
}

/// Generelle Punkt-Definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Punkt {
    /// X-Koordinate in Millimeter vom oberen Rand
    pub x: f32,
    /// Y-Koordinate in Millimeter vom linken Rand
    pub y: f32,
}

/// hOCR Ausgabe
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ParsedHocr {
    /// Bildkoordinaten in Pixeln
    pub bounds: Rect,
    /// Content-Areas (carea) im hOCR XML (Koordinaten in Pixeln)
    pub careas: Vec<HocrArea>,
}

/// hOCR-carea
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HocrArea {
    /// Koordinaten der carea relativ zur oberen linken Ecke, Angaben in Pixeln
    pub bounds: Rect,
    /// paragraph-Nodes der hOCR-Datei
    pub paragraphs: Vec<HocrParagraph>,
}

/// Absatz innerhalb einer Seite
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HocrParagraph {
    /// Koordinaten des Absatzes in Pixeln von der oberen linken Ecke im Bild
    pub bounds: Rect,
    /// Zeilen innerhalb des Absatzes
    pub lines: Vec<HocrLine>,
}

/// Zeile im hOCR-Absatz
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HocrLine {
    /// Koordinaten der Zeile in Pixeln von der oberen linken Ecke im Bild
    pub bounds: Rect,
    /// Worte innerhalb dieser Zeile
    pub words: Vec<HocrWord>,
}

/// hOCR-erkanntes Wort im Bild
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct HocrWord {
    /// Koordinaten des Worts in Pixeln von der oberen linken Ecke im Bild
    pub bounds: Rect,
    /// Wahrscheinlichkeit des Worts, richtig erkannt zu sein
    pub confidence: f32,
    /// Erkannter Text
    pub text: String,
}

/// Benutzerdefinierte Anpassungen der Seite
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnpassungSeite {
    /// Überschreibt den automatisch erkannten SeitenTyp
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klassifikation_neu: Option<SeitenTyp>,
    /// Überschreibt die Dimensionen der automatisch erkannten Spalten (indexiert nach Spalten-ID)
    #[serde(default)]
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub spalten: BTreeMap<String, Rect>,
    /// Manuell eingefügte Zeilen
    #[serde(default)]
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub zeilen: BTreeMap<String, f32>,
    /// Automatisch eingefügte Zeilen
    #[serde(default)]
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub zeilen_auto: BTreeMap<String, f32>,
}

/// Generelle Struktur für ein Rechteck (üblicherweise Koordinaten in Millimeter von oberer linker Ecke)
#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Rect {
    /// Minimum-X-Koordinate des Rechtecks
    pub min_x: f32,
    /// Minimum-Y-Koordinate des Rechtecks
    pub min_y: f32,
    /// Maximum-X-Koordinate des Rechtecks
    pub max_x: f32,
    /// Maximum-Y-Koordinate des Rechtecks
    pub max_y: f32,
}

/// Seitentyp der Seite im Grundbuch-PDF, jeder SeitenTyp hat andere Spalten / ein anderes Formular
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SeitenTyp {
    #[serde(rename = "bv-horz")]
    BestandsverzeichnisHorz,
    #[serde(rename = "bv-horz-zu-und-abschreibungen")]
    BestandsverzeichnisHorzZuUndAbschreibungen,
    #[serde(rename = "bv-vert")]
    BestandsverzeichnisVert,
    #[serde(rename = "bv-vert-typ2")]
    BestandsverzeichnisVertTyp2,
    #[serde(rename = "bv-vert-zu-und-abschreibungen")]
    BestandsverzeichnisVertZuUndAbschreibungen,
    #[serde(rename = "bv-vert-zu-und-abschreibungen-alt")]
    BestandsverzeichnisVertZuUndAbschreibungenAlt,

    #[serde(rename = "abt1-horz")]
    Abt1Horz,
    #[serde(rename = "abt1-vert")]
    Abt1Vert,
    #[serde(rename = "abt1-vert-typ2")]
    Abt1VertTyp2,

    #[serde(rename = "abt2-horz-veraenderungen")]
    Abt2HorzVeraenderungen,
    #[serde(rename = "abt2-horz")]
    Abt2Horz,
    #[serde(rename = "abt2-vert-veraenderungen")]
    Abt2VertVeraenderungen,
    #[serde(rename = "abt2-vert")]
    Abt2Vert,
    #[serde(rename = "abt2-vert-typ2")]
    Abt2VertTyp2,

    #[serde(rename = "abt3-horz-veraenderungen-loeschungen")]
    Abt3HorzVeraenderungenLoeschungen,
    #[serde(rename = "abt3-vert-veraenderungen-loeschungen")]
    Abt3VertVeraenderungenLoeschungen,
    #[serde(rename = "abt3-horz")]
    Abt3Horz,
    #[serde(rename = "abt3-vert-veraenderungen")]
    Abt3VertVeraenderungen,
    #[serde(rename = "abt3-vert-loeschungen")]
    Abt3VertLoeschungen,
    #[serde(rename = "abt3-vert")]
    Abt3Vert,
}

/// Analysiertes Grundbuch mit manuellen Änderungen
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Grundbuch {
    /// Titelblatt des Grundbuchs
    pub titelblatt: Titelblatt,
    /// Bestandsverzeichnis (Eigentum / Flurstücke)
    #[serde(default)]
    #[serde(skip_serializing_if = "Bestandsverzeichnis::is_empty")]
    pub bestandsverzeichnis: Bestandsverzeichnis,
    /// Abteilung 1 (Eigentümer)
    #[serde(default)]
    #[serde(skip_serializing_if = "Abteilung1::is_empty")]
    pub abt1: Abteilung1,
    /// Abteilung 2 (Rechte)
    #[serde(default)]
    #[serde(skip_serializing_if = "Abteilung2::is_empty")]
    pub abt2: Abteilung2,
    /// Abteilung 3 (Belastungen)
    #[serde(default)]
    #[serde(skip_serializing_if = "Abteilung3::is_empty")]
    pub abt3: Abteilung3,
}

/// Titelblatt des Grundbuchs
#[derive(Debug, Default, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Titelblatt {
    /// Amtsgericht
    pub amtsgericht: String,
    /// Grundbuch von ...
    pub grundbuch_von: String,
    /// Blatt ...
    pub blatt: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bestandsverzeichnis {
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub eintraege: Vec<BvEintrag>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub zuschreibungen: Vec<BvZuschreibung>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub abschreibungen: Vec<BvAbschreibung>,
}

impl Bestandsverzeichnis {
    pub fn is_empty(&self) -> bool {
        self.eintraege.is_empty()
            && self.zuschreibungen.is_empty()
            && self.abschreibungen.is_empty()
    }
}

/// Eintrag im Bestandsverzeichnis
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BvEintrag {
    /// Flurstück
    Flurstueck(BvEintragFlurstueck),
    /// Herrschvermerk / grundstücksgleiches Recht
    Recht(BvEintragRecht),
}

/// Eintrag für ein grundstücksgleiches Recht
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct BvEintragRecht {
    pub lfd_nr: usize,
    #[serde(default)]
    #[serde(skip_serializing_if = "StringOrLines::is_empty")]
    pub zu_nr: StringOrLines,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bisherige_lfd_nr: Option<usize>,
    #[serde(default)]
    #[serde(skip_serializing_if = "StringOrLines::is_empty")]
    pub text: StringOrLines,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatisch_geroetet: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manuell_geroetet: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_in_pdf: Option<PositionInPdf>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct BvEintragFlurstueck {
    pub lfd_nr: usize,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bisherige_lfd_nr: Option<usize>,
    pub flur: usize,
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub flurstueck: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gemarkung: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bezeichnung: Option<StringOrLines>,
    #[serde(default)]
    #[serde(skip_serializing_if = "FlurstueckGroesse::ist_leer")]
    pub groesse: FlurstueckGroesse,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatisch_geroetet: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manuell_geroetet: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_in_pdf: Option<PositionInPdf>,
}

/// Größe des Flurstücks in m2
#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Serialize, Deserialize)]
#[serde(tag = "typ", content = "wert")]
pub enum FlurstueckGroesse {
    #[serde(rename = "m")]
    Metrisch { m2: Option<u64> },
    #[serde(rename = "ha")]
    Hektar {
        ha: Option<u64>,
        a: Option<u64>,
        m2: Option<u64>,
    },
}

impl Default for FlurstueckGroesse {
    fn default() -> Self {
        FlurstueckGroesse::Metrisch { m2: None }
    }
}

impl FlurstueckGroesse {
    pub fn ist_leer(&self) -> bool {
        match self {
            FlurstueckGroesse::Metrisch { m2 } => m2.is_none(),
            FlurstueckGroesse::Hektar { ha, a, m2 } => m2.is_none() && ha.is_none() && a.is_none(),
        }
    }
    pub fn get_m2(&self) -> u64 {
        match self {
            FlurstueckGroesse::Metrisch { m2 } => m2.unwrap_or(0),
            FlurstueckGroesse::Hektar { ha, a, m2 } => {
                ha.unwrap_or(0) * 100_000 + a.unwrap_or(0) * 100 + m2.unwrap_or(0)
            }
        }
    }

    pub fn get_ha_string(&self) -> String {
        let m2_string = format!("{}", self.get_m2());
        let mut m2_string_chars: Vec<char> = m2_string.chars().collect();
        for _ in 0..4 {
            m2_string_chars.pop();
        }
        m2_string_chars.iter().collect()
    }

    pub fn get_a_string(&self) -> String {
        let m2_string = format!("{}", self.get_m2());
        let mut m2_string_chars: Vec<char> = m2_string.chars().collect();
        m2_string_chars.reverse();
        for _ in 0..(m2_string_chars.len().saturating_sub(4)) {
            m2_string_chars.pop();
        }
        m2_string_chars.reverse();
        for _ in 0..2 {
            m2_string_chars.pop();
        }
        m2_string_chars.iter().collect()
    }

    pub fn get_m2_string(&self) -> String {
        let m2_string = format!("{}", self.get_m2());
        let mut m2_string_chars: Vec<char> = m2_string.chars().collect();
        m2_string_chars.reverse();
        for _ in 0..(m2_string_chars.len().saturating_sub(2)) {
            m2_string_chars.pop();
        }
        m2_string_chars.reverse();
        let fi: String = m2_string_chars.iter().collect();
        if fi.is_empty() {
            format!("0")
        } else {
            fi
        }
    }
}

/// Position eines Textblocks im PDF
#[derive(Debug, Default, Clone, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct PositionInPdf {
    /// Seite, auf der der Text gefunden wurde
    pub seite: String,
    /// Koordinaten in Millimeter
    pub rect: Rect,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct BvZuschreibung {
    #[serde(default)]
    #[serde(skip_serializing_if = "StringOrLines::is_empty")]
    pub bv_nr: StringOrLines,
    #[serde(default)]
    #[serde(skip_serializing_if = "StringOrLines::is_empty")]
    pub text: StringOrLines,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatisch_geroetet: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manuell_geroetet: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_in_pdf: Option<PositionInPdf>,
}

impl BvZuschreibung {
    pub fn ist_geroetet(&self) -> bool {
        self.manuell_geroetet
            .or(self.automatisch_geroetet.clone())
            .unwrap_or(false)
    }
    pub fn ist_leer(&self) -> bool {
        self.bv_nr.is_empty() && self.text.is_empty()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct BvAbschreibung {
    #[serde(default)]
    #[serde(skip_serializing_if = "StringOrLines::is_empty")]
    pub bv_nr: StringOrLines,
    #[serde(default)]
    #[serde(skip_serializing_if = "StringOrLines::is_empty")]
    pub text: StringOrLines,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatisch_geroetet: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manuell_geroetet: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_in_pdf: Option<PositionInPdf>,
}

impl BvAbschreibung {
    pub fn ist_geroetet(&self) -> bool {
        self.manuell_geroetet
            .or(self.automatisch_geroetet.clone())
            .unwrap_or(false)
    }

    pub fn ist_leer(&self) -> bool {
        self.bv_nr.is_empty() && self.text.is_empty()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Abteilung1 {
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub eintraege: Vec<Abt1Eintrag>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub grundlagen_eintragungen: Vec<Abt1GrundEintragung>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub veraenderungen: Vec<Abt1Veraenderung>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loeschungen: Vec<Abt1Loeschung>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[repr(C)]
pub enum Abt1Eintrag {
    V1(Abt1EintragV1),
    V2(Abt1EintragV2),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Abt1EintragV2 {
    // lfd. Nr. der Eintragung
    pub lfd_nr: usize,
    // Rechtstext
    #[serde(default)]
    #[serde(skip_serializing_if = "StringOrLines::is_empty")]
    pub eigentuemer: StringOrLines,
    // Used to distinguish from Abt1EintragV1
    pub version: usize,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatisch_geroetet: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manuell_geroetet: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_in_pdf: Option<PositionInPdf>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Abt1EintragV1 {
    // lfd. Nr. der Eintragung
    pub lfd_nr: usize,
    // Rechtstext
    #[serde(default)]
    #[serde(skip_serializing_if = "StringOrLines::is_empty")]
    pub eigentuemer: StringOrLines,
    // lfd. Nr der betroffenen Grundstücke im Bestandsverzeichnis
    #[serde(default)]
    #[serde(skip_serializing_if = "StringOrLines::is_empty")]
    pub bv_nr: StringOrLines,
    // Vec<BvNr>,
    #[serde(default)]
    #[serde(skip_serializing_if = "StringOrLines::is_empty")]
    pub grundlage_der_eintragung: StringOrLines,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatisch_geroetet: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manuell_geroetet: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_in_pdf: Option<PositionInPdf>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Abt1GrundEintragung {
    // lfd. Nr. der Eintragung
    #[serde(default)]
    #[serde(skip_serializing_if = "StringOrLines::is_empty")]
    pub bv_nr: StringOrLines,
    // Grundlage der Eintragung
    #[serde(default)]
    #[serde(skip_serializing_if = "StringOrLines::is_empty")]
    pub text: StringOrLines,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatisch_geroetet: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manuell_geroetet: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_in_pdf: Option<PositionInPdf>,
}

/// String mit Option für mehreren Zeilen, zur Vermeidung von Problemen mit Zeilenumbrüchen
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrLines {
    SingleLine(String),
    MultiLine(Vec<String>),
}

impl StringOrLines {
    pub fn is_empty(&self) -> bool {
        match self {
            StringOrLines::SingleLine(s) => s.is_empty(),
            StringOrLines::MultiLine(ml) => ml.is_empty(),
        }
    }

    pub fn text(&self) -> String {
        self.lines().join("\r\n")
    }

    pub fn text_clean(&self) -> String {
        unhyphenate(&self.lines().join("\r\n"))
    }

    pub fn lines(&self) -> Vec<String> {
        match self {
            StringOrLines::SingleLine(s) => s.lines().map(|s| s.to_string()).collect(),
            StringOrLines::MultiLine(ml) => ml.clone(),
        }
    }
}

lazy_static::lazy_static! {
    static ref REGEX_UNHYPHENATE: regex::Regex = {
        regex::RegexBuilder::new("(.*)-\\s([a-züäö])(.*)")
                .multi_line(true)
                .case_insensitive(false)
                .build().unwrap()
    };
}

impl Abt1EintragV1 {
    pub fn ist_geroetet(&self) -> bool {
        self.manuell_geroetet
            .or(self.automatisch_geroetet.clone())
            .unwrap_or(false)
    }
}

impl BvEintrag {
    pub fn ist_geroetet(&self) -> bool {
        match self {
            BvEintrag::Flurstueck(flst) => flst
                .manuell_geroetet
                .unwrap_or(flst.automatisch_geroetet.unwrap_or(false)),
            BvEintrag::Recht(recht) => recht
                .manuell_geroetet
                .unwrap_or(recht.automatisch_geroetet.unwrap_or(false)),
        }
    }
}

impl Abt1GrundEintragung {
    pub fn ist_geroetet(&self) -> bool {
        self.manuell_geroetet
            .or(self.automatisch_geroetet.clone())
            .unwrap_or(false)
    }
}

impl BvZuschreibung {
    pub fn ist_geroetet(&self) -> bool {
        self.manuell_geroetet
            .or(self.automatisch_geroetet.clone())
            .unwrap_or(false)
    }
    pub fn ist_leer(&self) -> bool {
        self.bv_nr.is_empty() && self.text.is_empty()
    }
}

impl BvAbschreibung {
    pub fn ist_geroetet(&self) -> bool {
        self.manuell_geroetet
            .or(self.automatisch_geroetet.clone())
            .unwrap_or(false)
    }

    pub fn ist_leer(&self) -> bool {
        self.bv_nr.is_empty() && self.text.is_empty()
    }
}

impl Abt1EintragV2 {
    pub fn ist_geroetet(&self) -> bool {
        self.manuell_geroetet
            .or(self.automatisch_geroetet.clone())
            .unwrap_or(false)
    }
}

impl Abt1Eintrag {
    pub fn get_lfd_nr(&self) -> usize {
        match self {
            Abt1Eintrag::V1(v1) => v1.lfd_nr,
            Abt1Eintrag::V2(v2) => v2.lfd_nr,
        }
    }

    pub fn get_eigentuemer(&self) -> String {
        match self {
            Abt1Eintrag::V1(v1) => v1.eigentuemer.text(),
            Abt1Eintrag::V2(v2) => v2.eigentuemer.text(),
        }
    }

    pub fn ist_geroetet(&self) -> bool {
        match self {
            Abt1Eintrag::V1(v1) => v1.ist_geroetet(),
            Abt1Eintrag::V2(v2) => v2.ist_geroetet(),
        }
    }
}

impl Abt2Eintrag {
    pub fn ist_geroetet(&self) -> bool {
        self.manuell_geroetet
            .or(self.automatisch_geroetet.clone())
            .unwrap_or(false)
    }
}

impl Abt3Eintrag {
    pub fn ist_geroetet(&self) -> bool {
        self.manuell_geroetet
            .or(self.automatisch_geroetet.clone())
            .unwrap_or(false)
    }
}

fn unhyphenate(text: &str) -> String {
    let und_saetze = text
        .lines()
        .map(|s| s.split("- und ").map(|s| s.to_string()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut text_sauber = String::new();

    for l in und_saetze.into_iter() {
        let und_len = l.len();
        for (index, mut s) in l.into_iter().enumerate() {
            while REGEX_UNHYPHENATE.is_match(&s) {
                s = REGEX_UNHYPHENATE.replace_all(&s, "$1$2$3").to_string();
            }
            text_sauber.push_str(&s);
            if index + 1 != und_len {
                text_sauber.push_str("- und ");
            }
        }
    }

    text_sauber
}

impl Default for StringOrLines {
    fn default() -> Self {
        String::new().into()
    }
}

impl From<String> for StringOrLines {
    fn from(s: String) -> StringOrLines {
        StringOrLines::MultiLine(s.lines().map(|s| s.to_string()).collect())
    }
}

impl From<StringOrLines> for String {
    fn from(s: StringOrLines) -> String {
        match s {
            StringOrLines::SingleLine(s) => s,
            StringOrLines::MultiLine(ml) => ml.join("\r\n"),
        }
    }
}

impl Abteilung1 {
    pub fn is_empty(&self) -> bool {
        self.eintraege.is_empty()
            && self.grundlagen_eintragungen.is_empty()
            && self.veraenderungen.is_empty()
            && self.loeschungen.is_empty()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Abt1Veraenderung {
    #[serde(default)]
    #[serde(skip_serializing_if = "StringOrLines::is_empty")]
    pub lfd_nr: StringOrLines,
    #[serde(default)]
    #[serde(skip_serializing_if = "StringOrLines::is_empty")]
    pub text: StringOrLines,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatisch_geroetet: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manuell_geroetet: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_in_pdf: Option<PositionInPdf>,
}

impl Abt1Veraenderung {
    pub fn ist_geroetet(&self) -> bool {
        self.manuell_geroetet
            .or(self.automatisch_geroetet.clone())
            .unwrap_or(false)
    }
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Abt1Loeschung {
    #[serde(default)]
    #[serde(skip_serializing_if = "StringOrLines::is_empty")]
    pub lfd_nr: StringOrLines,
    #[serde(default)]
    #[serde(skip_serializing_if = "StringOrLines::is_empty")]
    pub text: StringOrLines,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatisch_geroetet: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manuell_geroetet: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_in_pdf: Option<PositionInPdf>,
}

impl Abt1Loeschung {
    pub fn ist_geroetet(&self) -> bool {
        self.manuell_geroetet
            .or(self.automatisch_geroetet.clone())
            .unwrap_or(false)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Abteilung2 {
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub eintraege: Vec<Abt2Eintrag>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub veraenderungen: Vec<Abt2Veraenderung>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loeschungen: Vec<Abt2Loeschung>,
}

impl Abteilung2 {
    pub fn is_empty(&self) -> bool {
        self.eintraege.is_empty() && self.veraenderungen.is_empty() && self.loeschungen.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Abt2Eintrag {
    // lfd. Nr. der Eintragung
    pub lfd_nr: usize,
    // lfd. Nr der betroffenen Grundstücke im Bestandsverzeichnis
    #[serde(default)]
    #[serde(skip_serializing_if = "StringOrLines::is_empty")]
    pub bv_nr: StringOrLines,
    // Rechtstext
    #[serde(default)]
    #[serde(skip_serializing_if = "StringOrLines::is_empty")]
    pub text: StringOrLines,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatisch_geroetet: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manuell_geroetet: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_in_pdf: Option<PositionInPdf>,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Abt2Veraenderung {
    #[serde(default)]
    #[serde(skip_serializing_if = "StringOrLines::is_empty")]
    pub lfd_nr: StringOrLines,
    #[serde(default)]
    #[serde(skip_serializing_if = "StringOrLines::is_empty")]
    pub text: StringOrLines,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatisch_geroetet: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manuell_geroetet: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_in_pdf: Option<PositionInPdf>,
}

impl Abt2Veraenderung {
    pub fn ist_geroetet(&self) -> bool {
        self.manuell_geroetet
            .or(self.automatisch_geroetet.clone())
            .unwrap_or(false)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Abt2Loeschung {
    #[serde(default)]
    #[serde(skip_serializing_if = "StringOrLines::is_empty")]
    pub lfd_nr: StringOrLines,
    #[serde(default)]
    #[serde(skip_serializing_if = "StringOrLines::is_empty")]
    pub text: StringOrLines,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatisch_geroetet: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manuell_geroetet: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_in_pdf: Option<PositionInPdf>,
}

impl Abt2Loeschung {
    pub fn ist_geroetet(&self) -> bool {
        self.manuell_geroetet
            .or(self.automatisch_geroetet.clone())
            .unwrap_or(false)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Abteilung3 {
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub eintraege: Vec<Abt3Eintrag>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub veraenderungen: Vec<Abt3Veraenderung>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loeschungen: Vec<Abt3Loeschung>,
}

impl Abteilung3 {
    pub fn is_empty(&self) -> bool {
        self.eintraege.is_empty() && self.veraenderungen.is_empty() && self.loeschungen.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Abt3Eintrag {
    // lfd. Nr. der Eintragung
    pub lfd_nr: usize,
    // lfd. Nr der betroffenen Grundstücke im Bestandsverzeichnis
    #[serde(default)]
    #[serde(skip_serializing_if = "StringOrLines::is_empty")]
    pub bv_nr: StringOrLines,
    // Betrag (EUR / DM)
    #[serde(default)]
    #[serde(skip_serializing_if = "StringOrLines::is_empty")]
    pub betrag: StringOrLines,
    /// Rechtstext
    #[serde(default)]
    #[serde(skip_serializing_if = "StringOrLines::is_empty")]
    pub text: StringOrLines,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatisch_geroetet: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manuell_geroetet: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_in_pdf: Option<PositionInPdf>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Abt3Veraenderung {
    #[serde(default)]
    #[serde(skip_serializing_if = "StringOrLines::is_empty")]
    pub lfd_nr: StringOrLines,
    #[serde(default)]
    #[serde(skip_serializing_if = "StringOrLines::is_empty")]
    pub betrag: StringOrLines,
    #[serde(default)]
    #[serde(skip_serializing_if = "StringOrLines::is_empty")]
    pub text: StringOrLines,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatisch_geroetet: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manuell_geroetet: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_in_pdf: Option<PositionInPdf>,
}

impl Abt3Veraenderung {
    pub fn ist_geroetet(&self) -> bool {
        self.manuell_geroetet
            .or(self.automatisch_geroetet.clone())
            .unwrap_or(false)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Abt3Loeschung {
    #[serde(default)]
    #[serde(skip_serializing_if = "StringOrLines::is_empty")]
    pub lfd_nr: StringOrLines,
    #[serde(default)]
    #[serde(skip_serializing_if = "StringOrLines::is_empty")]
    pub betrag: StringOrLines,
    #[serde(default)]
    #[serde(skip_serializing_if = "StringOrLines::is_empty")]
    pub text: StringOrLines,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatisch_geroetet: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manuell_geroetet: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_in_pdf: Option<PositionInPdf>,
}

impl Abt3Loeschung {
    pub fn ist_geroetet(&self) -> bool {
        self.manuell_geroetet
            .or(self.automatisch_geroetet.clone())
            .unwrap_or(false)
    }
}
