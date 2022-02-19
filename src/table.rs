use pad::PadStr;
use std::cmp::max;

pub type Table = Vec<Row>;
pub type Row = Vec<Column>;
pub type Column = String;

pub fn to_markdown(data: &[Row]) -> String {
    let lengths = data.iter().fold(vec![1; data[0].len()], |lens, row| {
        row.iter()
            .zip(lens)
            .map(|(s, len)| max(s.len(), len))
            .collect()
    });
    let rows = data
        .iter()
        .map(|row| {
            row.iter()
                .zip(&lengths)
                .map(|(s, len)| s.pad_to_width(*len))
                .collect::<Vec<_>>()
                .join(" | ")
        })
        .collect::<Vec<_>>();
    let separator = lengths
        .iter()
        .map(|len| "-".repeat(*len))
        .collect::<Vec<_>>()
        .join("-|-");
    [
        format!("| {} |", rows[0]),
        format!("|-{}-|", separator),
        format!("| {} |", rows[1..].join(" |\n| ")),
    ]
    .join("\n")
}
