use tokio::process::Command;
use serde::Serialize;
use std::path::PathBuf;
use open_xlsxwriter_sys::Workbook;
use open_xlsxwriter_sys::Worksheet;
use open_xlsxwriter_sys::XlsxError;

/// Represents an Excel Workbook
#[derive(Serialize)]
pub struct ExcelWorkbook {
    pub path: PathBuf,
    pub sheets: Vec<ExcelSheet>,
}

/// Represents an Excel Worksheet
#[derive(Serialize)]
pub struct ExcelSheet {
    pub name: String,
    pub rows: Vec<Vec<String>>,
}

impl ExcelWorkbook {
    /// Creates a new Excel workbook
    pub fn new(path: PathBuf) -> Self {
        ExcelWorkbook {
            path,
            sheets: vec![],
        }
    }

    /// Adds a new worksheet to the workbook
    pub fn add_sheet(&mut self, sheet: ExcelSheet) {
        self.sheets.push(sheet);
    }

    /// Saves the workbook to a file
    pub async fn save(&self) -> Result<(), XlsxError> {
        let mut workbook = Workbook::new();
        
        for sheet in &self.sheets {
            let mut worksheet = workbook.add_worksheet(&sheet.name);
            for row in &sheet.rows {
                for (index, value) in row.iter().enumerate() {
                    worksheet.write_string(0, index as u32, value)?;
                }
            }
        }
        
        workbook.save(&self.path).map_err(|e| e.into())
    }
}

#[tokio::main]
async fn main() -> Result<(), XlsxError> {
    let mut workbook = ExcelWorkbook::new(PathBuf::from("example.xlsx"));
    let sheet1 = ExcelSheet {
        name: "Sheet1".to_string(),
        rows: vec![
            vec!["Header1".to_string(), "Header2".to_string()],
            vec!["Data1".to_string(), "Data2".to_string()],
        ],
    };
    workbook.add_sheet(sheet1);
    workbook.save().await?;
    println!("Excel file generated successfully!");
    Ok(())
}
