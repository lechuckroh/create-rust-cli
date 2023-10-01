use std::collections::HashMap;
use exif_rename::{create_vars_from_create_date, create_vars_from_filename, extend_vars, format_filename, Vars};

#[test]
fn test_create_vars_from_create_date() {
    let inputs = vec!["2023:09:08 18:56:54", "2023:09:08 18:56:54+09:00"];
    for input in inputs {
        let vars = create_vars_from_create_date(input);
        let expected = vec![
            ("D", "08"),
            ("H", "18"),
            ("M", "56"),
            ("S", "54"),
            ("Y", "2023"),
            ("h", "06"),
            ("m", "09"),
            ("t", "185654"),
            ("y", "23"),
        ];
        for (k, v) in expected {
            assert_eq!(v, vars.get(k).map_or("", |v| v.as_str()));
        }
    }
}

#[test]
fn test_create_vars_from_filename() {
    // filename, {f}, {r}, {e}
    let tests = vec![
        ("IMG_1234.JPG", "IMG_", "1234", "JPG"),
        ("3Z1A2113.CR3", "3Z1A", "2113", "CR3"),
        ("DSC03165.ARW", "DSC", "03165", "ARW"),
    ];
    for (filename, f, r, e) in tests {
        let vars = create_vars_from_filename(filename);
        assert_eq!(f, vars.get("f").map_or("", |v| v.as_str()));
        assert_eq!(r, vars.get("r").map_or("", |v| v.as_str()));
        assert_eq!(e, vars.get("e").map_or("", |v| v.as_str()));
    }
}

#[test]
fn test_format_filename() {
    let mut exif_vars = HashMap::new();
    exif_vars.insert("CreateDate".to_string(), "2023:09:08 18:56:54".to_string());
    exif_vars.insert("FileName".to_string(), "IMG_9876.JPG".to_string());
    exif_vars.insert("Model".to_string(), "iPhone 14".to_string());

    let pattern = "{y}{m}{D}_{t}_{T2}_{r}.{e}";
    let expected = "230908_185654_iPhone 14_9876.JPG";

    let mut vars: Vars = extend_vars(&exif_vars);
    vars.extend(exif_vars);
    let actual = format_filename(pattern, vars);

    assert_eq!(expected, actual)
}
