#[macro_export]
macro_rules! in_temp_dir {
    ($block:block) => {
        let tmpdir = tempfile::tempdir().unwrap();
        std::env::set_current_dir(&tmpdir).unwrap();

        $block;
    };
}

#[macro_export]
macro_rules! fixture {
    ($filename:expr) => {
        let root_dir = &std::env::var("CARGO_MANIFEST_DIR").expect("$CARGO_MANIFEST_DIR");
        let mut source_path = std::path::PathBuf::from(root_dir);
        source_path.push("tests/fixtures");
        source_path.push($filename);

        std::fs::copy(source_path, $filename).unwrap();
    };
}

#[macro_export]
macro_rules! assert_files_eq {
    ($filename_a:expr, $filename_b:expr) => {
        use pretty_assertions::assert_eq;
        use std::{fs::File, io::Read};

        let mut contents_a = String::new();
        let mut contents_b = String::new();
        File::open($filename_a)
            .expect(&format!("Could not open \"{}\"", $filename_a))
            .read_to_string(&mut contents_a)
            .unwrap();
        File::open($filename_b)
            .expect(&format!("Could not open \"{}\"", $filename_b))
            .read_to_string(&mut contents_b)
            .unwrap();
        assert_eq!(contents_a, contents_b);
    };
}
