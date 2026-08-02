// ABOUTME: The 3270 screen model at the field level: positioned fields carrying attribute
// ABOUTME: bytes, plus a renderer that lays them onto a fixed 24x80 Model 2 grid.

/// A Model 2 screen, the size every 3270 emulator defaults to.
pub const SCREEN_ROWS: usize = 24;
pub const SCREEN_COLUMNS: usize = 80;

/// What a field's attribute byte says about it.
///
/// A real 3270 attribute byte packs protection, numeric-only entry, intensity, and
/// non-display into one byte ahead of each field. This models the distinctions a Natural
/// programmer actually works with. The 3270 data stream itself (SBA and SF orders, 12-bit
/// addressing, EBCDIC) is deliberately out of scope: it is weeks of work that is invisible
/// from the language. See research/08-mainframe-emulators-3270.md.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Attribute {
    /// Literal text the operator cannot type into.
    Protected,
    /// An ordinary entry field.
    Unprotected,
    /// An entry field that accepts digits only.
    Numeric,
    /// An entry field shown brighter, conventionally for errors or emphasis.
    Intensified,
    /// An entry field whose contents are not displayed, for a password or PIN.
    Hidden,
}

impl Attribute {
    /// True when the operator may type into a field carrying this attribute.
    pub fn is_input(self) -> bool {
        !matches!(self, Attribute::Protected)
    }
}

/// One positioned field on a screen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScreenField {
    /// One-based row, as a Natural map defines it.
    pub row: usize,
    /// One-based column.
    pub column: usize,
    /// The text currently occupying the field: a label, or an entry field's value.
    pub text: String,
    /// How many columns the field occupies.
    pub width: usize,
    pub attribute: Attribute,
    /// The program field an entry field writes into, if any.
    pub bound_to: Option<String>,
    /// The modified data tag. A real 3270 sets this when the operator changes a field, and
    /// Read Modified returns only tagged fields. It is carried here so the concept is
    /// available to Tier 2 lessons.
    pub modified: bool,
}

/// A screen waiting to be filled in.
///
/// This is what a map read suspends on, exactly as a line-mode INPUT suspends on a prompt.
/// Modeling it as data rather than as terminal escape codes keeps the browser free to
/// render it however it likes, and keeps a lesson checker able to inspect it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Screen {
    pub name: String,
    pub rows: usize,
    pub columns: usize,
    pub fields: Vec<ScreenField>,
    /// A message line, used by REINPUT to say why the last attempt was rejected.
    pub message: Option<String>,
}

impl Screen {
    pub fn blank(name: &str) -> Self {
        Self {
            name: name.to_string(),
            rows: SCREEN_ROWS,
            columns: SCREEN_COLUMNS,
            fields: Vec::new(),
            message: None,
        }
    }

    /// The entry fields, in the order the operator would tab through them.
    pub fn input_fields(&self) -> impl Iterator<Item = &ScreenField> {
        self.fields.iter().filter(|f| f.attribute.is_input())
    }

    /// Lays the fields onto a fixed grid, one string per row, each exactly 80 columns.
    ///
    /// Rows are padded rather than trimmed, because a screen is a panel of a fixed size:
    /// a terminal drawing it needs every row to be full width.
    pub fn render(&self) -> Vec<String> {
        let mut grid = vec![vec![' '; self.columns]; self.rows];

        for field in &self.fields {
            if field.row == 0 || field.row > self.rows {
                continue;
            }
            // A hidden field occupies its space but never shows what it holds.
            let shown = if field.attribute == Attribute::Hidden {
                String::new()
            } else {
                field.text.clone()
            };
            let row = &mut grid[field.row - 1];
            for (offset, ch) in shown.chars().enumerate() {
                let column = field.column + offset;
                if column == 0 || column > self.columns {
                    continue;
                }
                row[column - 1] = ch;
            }
        }

        let mut lines: Vec<String> = grid
            .into_iter()
            .map(|row| row.into_iter().collect())
            .collect();

        // The message line sits on the last row, which is where a 3270 application
        // conventionally puts it.
        if let Some(message) = &self.message
            && let Some(last) = lines.last_mut()
        {
            let mut row: Vec<char> = vec![' '; self.columns];
            for (i, ch) in message.chars().take(self.columns).enumerate() {
                row[i] = ch;
            }
            *last = row.into_iter().collect();
        }
        lines
    }
}
