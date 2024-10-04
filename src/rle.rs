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

    fn line_part_str(count: usize, cell: bool) -> String {
        if count == 0 {
            String::new()
        } else if count == 1 {
            if cell { "o".to_string() } else { "b".to_string() }
        } else {
            format!("{}{}", count, if cell { "o" } else { "b" })
        }
    }

    fn encode_rle_line(line: &[bool]) -> String {
        if line.is_empty() {
            return String::new();
        }

        let mut rle_line = String::new();
        let mut last_cell = line[0];
        let mut count = 0;

        for &cell in line {
            if cell == last_cell {
                count += 1;
            } else {
                rle_line.push_str(&Self::line_part_str(count, last_cell));
                count = 1;
                last_cell = cell;
            }
        }

        if last_cell {
            rle_line.push_str(&Self::line_part_str(count, last_cell));
        }
        rle_line
    }

    fn process_empty_lines(all_lines: &mut Vec<String>) {
        let mut count_empty_lines = 0;
        let mut i = 0;

        while i < all_lines.len() {
            if all_lines[i].is_empty() {
                all_lines.remove(i);
                count_empty_lines += 1;
            } else {
                if count_empty_lines != 0 && i > 0 {
                    all_lines[i -1] += &(count_empty_lines+1).to_string();
                    count_empty_lines = 0;
                }
                i += 1;
            }
        }
    }

    // ---------

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
        let re = Regex::new(r"^((\s?[0-9]*[bo]\s?)+\s?[0-9]?\s?[$]?)+\s?!$").unwrap();
        let data = match file_content.lines().nth(1) {
            Some(l) => if re.is_match(l) { l } else { return Err("Content line not found or incorrect. (Help: all content must be on one line)") },
            None => return Err("Content line not found"),
        }.to_string();

        Ok(RLE { comments, x, y, rule, data })
    }

    pub fn from_cell_map(c: &CellMap, comments: Option<&Vec<String>>) -> RLE {
        let c = c.auto_crop();
        let mut all_lines: Vec<String> = c.actual_generation.iter()
            .map(|line| Self::encode_rle_line(line))
            .collect();

        Self::process_empty_lines(&mut all_lines);

        RLE {
            comments: if let Some(c) = comments { c.clone() } else { Vec::<String>::new() },
            x: c.w,
            y: c.h,
            rule: Some(String::from("B3/S23")),
            data: all_lines.join("$") + "!",
        }
    }

    pub fn to_cell_map(&self) -> Result<CellMap, &'static str> {
        // TODO :
        // - Refactor
        // - Split the code into functions

        let mut map = vec![vec![false; self.x as usize]; self.y as usize];

        let mut cleaned_data = self.data.replace(" ", "");
        cleaned_data.pop();

        let all_lines: Vec<&str> = cleaned_data.split("$").collect();
        let mut all_lines_parsed: Vec<Vec<(u32, bool)>> = Vec::new();

        for line in all_lines {
            let mut line_parsed: Vec<(u32, bool)> = Vec::new();
            let mut number_construct = String::new();
            for c in line.chars() {
                if c.is_ascii_digit() {
                    number_construct.push(c);
                } else if c == 'b' || c == 'o' {
                    let factor: u32 = if !number_construct.is_empty() {
                        if let Ok(n) = number_construct.parse() {
                            n
                        } else {
                            return Err("Error while parsing RLE file: Number parsing")
                        }
                    } else {
                        1
                    };

                    line_parsed.push((factor, if c == 'o' { true } else { false }));
                    number_construct = String::new();
                } else {
                    return Err("Error while parsing RLE file: Unknown character")
                }
            }

            let is_last_line_empty = line_parsed.is_empty();
            if !is_last_line_empty { all_lines_parsed.push(line_parsed) }

            if !number_construct.is_empty() {
                if let Ok(n) = number_construct.parse::<u32>() {
                    let count_empty_lines = if is_last_line_empty { n } else { n-1 };
                    for _ in 0..count_empty_lines {
                        all_lines_parsed.push(vec![(self.x, false)]);
                    }
                } else {
                    return Err("Error while parsing RLE file: Number parsing")
                }
            }
        }

        // Add lines parsed to final map
        for i in 0..map.len() {
            for j in 0..map[i].len() {
                if all_lines_parsed[i].is_empty() {
                    break
                }

                map[i][j] = all_lines_parsed[i][0].1;

                if all_lines_parsed[i][0].0 > 1 {
                    all_lines_parsed[i][0].0 -= 1;
                } else {
                    all_lines_parsed[i].remove(0);
                }
            }
        }

        Ok(CellMap::new(map)?)
    }

    pub fn export(&self) -> String {
        // Add comments
        let mut content = String::from("#C Generated by ALife\n");
        content += &self.comments.join("\n");
        // Add x, y and rule
        if let Some(r) = &self.rule {
            content += &format!("\nx = {}, y = {}, rule = {}\n", self.x, self.y, r);
        } else {
            content += &format!("\nx = {}, y = {}\n", self.x, self.y);
        }
        // Add data
        content += &self.data;
        content
    }

    pub fn file_to_cell_map(file_content: String) -> Result<CellMap, &'static str> {
        let rle = Self::parse(file_content)?;
        Ok(rle.to_cell_map()?)
    }

    pub fn cell_map_to_file(c: &CellMap, comments: Option<&Vec<String>>) -> String {
        Self::from_cell_map(c, comments).export()
    }
}


// --------
// Tests
// --------

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
24bo$22bobo$12b2o6b2o12b2o$11bo3bo4b2o12b2o$2o8bo5bo3b2o$2o8bo3bob2o4bobo$ 10b o 5b o 7b o $11bo3bo$12b2o !");

        match RLE::parse(content) {
            Ok(result) => {
                assert_eq!(result.comments, Vec::<String>::new());
                assert_eq!(result.x, 36);
                assert_eq!(result.y, 9);
                assert_eq!(result.rule, None);
                assert_eq!(result.data, "24bo$22bobo$12b2o6b2o12b2o$11bo3bo4b2o12b2o$2o8bo5bo3b2o$2o8bo3bob2o4bobo$ 10b o 5b o 7b o $11bo3bo$12b2o !".to_string())
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

    // Test RLE.export

    #[test]
    fn test_export_1() {
        let rle = RLE {
            comments: vec!["#C Game of Life".to_string()],
            x: 36,
            y: 9,
            rule: Some("B3/S23".to_string()),
            data: "24bo$22bobo$12b2o6b2o12b2o$11bo3bo4b2o12b2o$2o8bo5bo3b2o$2o8bo3bob2o4bobo$10bo5bo7bo$11bo3bo$12b2o!".to_string()
        };

        let result = rle.export();

        assert_eq!(
            result,
            "#C Generated by ALife\n#C Game of Life\nx = 36, y = 9, rule = B3/S23\n24bo$22bobo$12b2o6b2o12b2o$11bo3bo4b2o12b2o$2o8bo5bo3b2o$2o8bo3bob2o4bobo$10bo5bo7bo$11bo3bo$12b2o!"
        )
    }

    #[test]
    fn test_export_2() {
        let rle = RLE {
            comments: vec![
                "#N Gosper glider gun".to_string(),
                "#C This was the first gun discovered.".to_string(),
                "#C As its name suggests, it was discovered by Bill Gosper.".to_string()
            ],
            x: 36,
            y: 9,
            rule: Some("B3/S23".to_string()),
            data: "24bo$22bobo$12b2o6b2o12b2o$11bo3bo4b2o12b2o$2o8bo5bo3b2o$2o8bo3bob2o4bobo$10bo5bo7bo$11bo3bo$12b2o!".to_string()
        };

        let result = rle.export();

        assert_eq!(
            result,
            "#C Generated by ALife\n#N Gosper glider gun\n#C This was the first gun discovered.\n#C As its name suggests, it was discovered by Bill Gosper.\nx = 36, y = 9, rule = B3/S23\n24bo$22bobo$12b2o6b2o12b2o$11bo3bo4b2o12b2o$2o8bo5bo3b2o$2o8bo3bob2o4bobo$10bo5bo7bo$11bo3bo$12b2o!"
        )
    }

    // Test RLE::from_cell_map

    #[test]
    fn test_from_cell_map_1() {
        let c = CellMap::new(vec![
            vec![false, true, false],
            vec![false, false, true],
            vec![true, true, true]
        ]).unwrap();

        let rle = RLE::from_cell_map(&c, Some(&vec!["#C Glider".to_string()]));

        assert_eq!(rle.comments, vec!["#C Glider".to_string()]);
        assert_eq!(rle.x, 3);
        assert_eq!(rle.y, 3);
        assert_eq!(rle.rule, Some(String::from("B3/S23")));
        assert_eq!(rle.data, String::from("bo$2bo$3o!"));
    }

    #[test]
    fn test_from_cell_map_2() {
        let c = CellMap::new(vec![
            vec![false, false, false],
            vec![false, true, false],
            vec![false, false, true],
            vec![true, true, true],
            vec![false, false, false],
        ]).unwrap();

        let rle = RLE::from_cell_map(&c, Some(&vec!["#C Glider".to_string()]));

        assert_eq!(rle.comments, vec!["#C Glider".to_string()]);
        assert_eq!(rle.x, 3);
        assert_eq!(rle.y, 3);
        assert_eq!(rle.rule, Some(String::from("B3/S23")));
        assert_eq!(rle.data, String::from("bo$2bo$3o!"));
    }

    #[test]
    fn test_from_cell_map_3() {
        let c = CellMap::new(vec![
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, false, true, false, false],
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, false, false, true, false],
            vec![false, false, false, false, false],
            vec![false, true, true, true, false],
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
        ]).unwrap();

        let rle = RLE::from_cell_map(&c, Some(&vec!["#C Glider".to_string()]));

        assert_eq!(rle.comments, vec!["#C Glider".to_string()]);
        assert_eq!(rle.x, 3);
        assert_eq!(rle.y, 6);
        assert_eq!(rle.rule, Some(String::from("B3/S23")));
        assert_eq!(rle.data, String::from("bo3$2bo2$3o!"));
    }

    // Test RLE::cell_map_to_file

    #[test]
    fn test_cell_map_to_file() {
        let c = CellMap::new(vec![
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, false, true, false, false],
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, false, false, true, false],
            vec![false, false, false, false, false],
            vec![false, true, true, true, false],
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
        ]).unwrap();

        let file_content = String::from("#C Generated by ALife\n#C Stretched glider\nx = 3, y = 6, rule = B3/S23\nbo3$2bo2$3o!");

        assert_eq!(RLE::cell_map_to_file(&c, Some(&vec![String::from("#C Stretched glider")])), file_content);
    }

    // Test RLE.to_cell_map

    #[test]
    fn test_to_cell_map_1() {
        let rle = RLE {
            comments: vec![],
            x: 3,
            y: 6,
            rule: None,
            data: String::from("b o 2 $ 2b o 3 $3o!")
        };

        let cell_map = CellMap::new(vec![
            vec![false, true, false],
            vec![false, false, false],
            vec![false, false, true],
            vec![false, false, false],
            vec![false, false, false],
            vec![true, true, true],
        ]).unwrap();

        let result = rle.to_cell_map().unwrap();

        assert_eq!(result.w, cell_map.w);
        assert_eq!(result.h, cell_map.h);
        assert_eq!(result.actual_generation, cell_map.actual_generation);
    }

    #[test]
    fn test_to_cell_map_2() {
        let rle = RLE {
            comments: vec![],
            x: 11,
            y: 14,
            rule: None,
            data: String::from("b o 2 $10bo$10$3o!")
        };

        let false_vector = vec![false; 11];
        let mut map = vec![false_vector; 14];
        map[0][1] = true;
        map[2][10] = true;
        map[13][0] = true;
        map[13][1] = true;
        map[13][2] = true;
        let cell_map = CellMap::new(map).unwrap();

        let result = rle.to_cell_map().unwrap();

        assert_eq!(result.w, cell_map.w);
        assert_eq!(result.h, cell_map.h);
        assert_eq!(result.actual_generation, cell_map.actual_generation);
    }

    // Test RLE::file_to_cell_map

    // TODO
}