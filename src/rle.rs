use std::error::Error;
use crate::cell_map::CellMap;
use regex::Regex;

#[derive(Debug)]
pub struct RLE {
    comments: Vec<String>,
    x: u32,
    y: u32,
    rule: Option<String>,
    data: String,
}

impl RLE {
    fn remove_and_collect_comments(content: &str) -> (String, Vec<String>) {
        let mut cleaned_lines = Vec::new();
        let mut removed_lines = Vec::new();

        for line in content.lines() {
            if line.trim_start().starts_with('#') {
                removed_lines.push(line.to_string());
            } else {
                cleaned_lines.push(line);
            }
        }

        (cleaned_lines.join("\n"), removed_lines)
    }

    fn parse_config_string(input: &str) -> Result<(u32, u32, Option<String>), &'static str> {
        let re = Regex::new(r"^x\s*=\s*([0-9]+)\s*,\s*y\s*=\s*([0-9]+)\s*(,\s*rule\s*=\s*(\S+))?$").unwrap();

        if let Some(captures) = re.captures(input) {
            // Extract values
            let x: u32 = captures[1].parse().unwrap();
            let y: u32 = captures[2].parse().unwrap();
            let rule: Option<String> = match captures.get(4) {
                Some(v) => Some(v.as_str().to_string()),
                None => None,
            };

            Ok((x, y, rule))
        } else {
            Err("Header line not found or incorrect")
        }
    }

    pub fn parse(mut file_content: String) -> Result<RLE, &'static str> {
        // Store comments lines
        let (cleaned_content, comments) = Self::remove_and_collect_comments(&file_content);
        file_content = cleaned_content;
        // Extract information
        let (x, y, rule): (u32, u32, Option<String>) =  match file_content.lines().nth(0) {
            Some(desc) => Self::parse_config_string(desc.trim()),
            None => return Err("Header line not found"),
        }?;
        // Verify and extract data
        let re = Regex::new(r"^(([0-9]*[bo])+[$]?)+!$").unwrap();
        let data = match file_content.lines().nth(1) {
            Some(l) => if re.is_match(l) { l } else { return Err("Content line not found or incorrect. (Help: all content must be on one line)") },
            None => return Err("Content line not found"),
        }.to_string();

        Ok(RLE { comments, x, y, rule, data })
    }

    fn from_cell_map(c: &CellMap) -> Result<RLE, &'static str> {
        panic!("TODO: Not implemented");
    }

    pub fn to_cell_map(&self) -> CellMap {
        panic!("TODO: Not implemented");
    }

    fn export(&self) -> String {
        panic!("TODO: Not implemented");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test RLE::remove_and_collect_comments
    #[test]
    fn test_remove_and_collect_comments() {
        let input = "# Comment 1\nLine 1\n# Comment 2\nLine 2\n  # Comment indented\nLine 3";
        let (cleaned, comments) = RLE::remove_and_collect_comments(input);

        assert_eq!(cleaned, "Line 1\nLine 2\nLine 3");
        assert_eq!(comments, vec![
            "# Comment 1",
            "# Comment 2",
            "  # Comment indented"
        ]);
    }

    // Test RLE::parse
    #[test]
    fn test_parse_1() -> Result<(), &'static str> {
        let content = String::from("#N Gosper glider gun
#C This was the first gun discovered.
#C As its name suggests, it was discovered by Bill Gosper.
x = 36, y = 9, rule = B3/S23
24bo$22bobo$12b2o6b2o12b2o$11bo3bo4b2o12b2o$2o8bo5bo3b2o$2o8bo3bob2o4bobo$10bo5bo7bo$11bo3bo$12b2o!");

        match RLE::parse(content) {
            Ok(result) => {
                assert_eq!(result.comments, vec!["#N Gosper glider gun", "#C This was the first gun discovered.", "#C As its name suggests, it was discovered by Bill Gosper."]);
                assert_eq!(result.x, 36);
                assert_eq!(result.y, 9);
                assert_eq!(result.rule, Some("B3/S23".to_string()));
                assert_eq!(result.data, "24bo$22bobo$12b2o6b2o12b2o$11bo3bo4b2o12b2o$2o8bo5bo3b2o$2o8bo3bob2o4bobo$10bo5bo7bo$11bo3bo$12b2o!".to_string())
            },
            Err(error) => panic!("{}", error),
        }
        Ok(())
    }

    #[test]
    fn test_parse_2() -> Result<(), &'static str> {
        let content = String::from("x = 36, y = 9
24bo$22bobo$12b2o6b2o12b2o$11bo3bo4b2o12b2o$2o8bo5bo3b2o$2o8bo3bob2o4bobo$10bo5bo7bo$11bo3bo$12b2o!");

        match RLE::parse(content) {
            Ok(result) => {
                assert_eq!(result.comments, Vec::<String>::new());
                assert_eq!(result.x, 36);
                assert_eq!(result.y, 9);
                assert_eq!(result.rule, None);
                assert_eq!(result.data, "24bo$22bobo$12b2o6b2o12b2o$11bo3bo4b2o12b2o$2o8bo5bo3b2o$2o8bo3bob2o4bobo$10bo5bo7bo$11bo3bo$12b2o!".to_string())
            },
            Err(error) => panic!("{}", error),
        }
        Ok(())
    }

    #[test]
    fn test_parse_3() -> Result<(), &'static str> {
        let content = String::from("x = 36, y = 9
24bo$22bobo$12b2o6b2o12b2o$11bo3ao4b2o12b2o$2o8bo5bo3b2o$2o8bo3bob2o4bobo$10bo5bo7bo$11bo3bo$12b2o!");

        match RLE::parse(content) {
            Ok(result) => panic!("The result should not be Ok"),
            Err(error) => assert_eq!(error, "Content line not found or incorrect. (Help: all content must be on one line)"),
        }
        Ok(())
    }
}