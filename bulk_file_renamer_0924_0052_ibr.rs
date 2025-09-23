use std::fs;
use std::io;
use std::path::Path;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::io::Error;
use tokio::io::Result;
use tokio::process;
use tokio::task;

/// 批量文件重命名工具
#[derive(Debug, Clone)]
struct BulkRenamer {
    base_path: String,
    extension: String,
    start_index: usize,
}

impl BulkRenamer {
    /// 创建一个新的批量重命名器
    pub fn new(base_path: &str, extension: &str, start_index: usize) -> Self {
        BulkRenamer {
            base_path: base_path.to_string(),
            extension: extension.to_string(),
            start_index,
        }
    }

    /// 执行批量重命名操作
    pub async fn rename_files(&self) -> Result<()> {
        let mut index = self.start_index;
        let entries = fs::read_dir(&self.base_path).await?;
        let mut futures = Vec::new();

        for entry in entries {
            let entry = entry.await?;
            let path = entry.path();
            if path.is_file() {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let new_name = format!("{}_{}.{}", file_name, index, self.extension);
                let new_path = path.with_file_name(new_name);
                futures.push(fs::rename(&path, new_path));
                index += 1;
            }
        }

        // 等待所有重命名操作完成
        let results = futures::future::join_all(futures).await;

        for result in results {
            result?;
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let base_path = "./"; // 需要重命名的文件所在目录
    let extension = "txt"; // 重命名后文件的扩展名
    let start_index = 1; // 重命名的起始索引

    let renamer = BulkRenamer::new(base_path, extension, start_index);
    renamer.rename_files().await?;

    println!("文件重命名完成");

    Ok(())
}
