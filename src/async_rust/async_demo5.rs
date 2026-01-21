use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

async fn fake_count_lines_async() {
    count_lines_sync();
}
fn count_lines_sync() -> i32 {
    let mut count = 0;
    if let Ok(lines) = read_lines("shakespeare.txt") {
        lines.for_each(|line| {
            if let Ok(line) = line
                && !line.trim().is_empty()
            {
                count += 1;
            }
        });
    }
    count
}
fn read_lines<T>(filename: T) -> anyhow::Result<io::Lines<io::BufReader<File>>>
where
    T: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

async fn real_count_lines_async(filename: String) -> anyhow::Result<usize> {
    use tokio::fs::File;
    use tokio::io::AsyncBufReadExt;
    use tokio::io::BufReader;

    let now = std::time::Instant::now();
    let mut count = 0;
    let file = File::open(filename).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    while let Some(line) = lines.next_line().await? {
        if !line.trim().is_empty() {
            count += 1;
        }
    }

    println!(
        "Real version: read {} lines in {:.3} seconds",
        count,
        now.elapsed().as_secs_f32()
    );
    Ok(count)
}

#[cfg(test)]
mod tests {

    use std::time::Instant;

    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_fake_read_lines() {
        let now = Instant::now();
        let line_count = count_lines_sync();
        tokio::join!(fake_count_lines_async(), fake_count_lines_async());
        println!(
            "Sync version: read {} lines in {:.3} seconds",
            line_count,
            now.elapsed().as_secs_f32()
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_real_read_lines() {
        let now = Instant::now();
        let _ = tokio::join!(
            real_count_lines_async(String::from("shakespeare.txt")),
            real_count_lines_async(String::from("shakespeare.txt"))
        );
        println!(
            "Real version: in {:.3} seconds",
            now.elapsed().as_secs_f32()
        );
    }
}
