#![cfg(test)]

mod utils;

use crate::utils::{tear_down, test_path, tmp_path};
use bts::args::{NewArgs, RegisterArgs};
use bts::{new, register};
use std::path::PathBuf;

mod new {
    use super::*;
    use dir_assert::assert_paths;
    use test_case::test_case;

    #[test_case(false, "file_example",        "file_example",               1; "single file is copied to destination")]
    #[test_case(false, "multi_file_example",  "multi_file_example",         1; "multiple files are copied to destination")]
    #[test_case(false, "wiki/cinematography", "deep_copy_example",          1; "files are copied from sub folders")]
    #[test_case(true,  "wiki",                "deep_copy_w_parent_example", 2; "files are copied preserving directory path")]
    #[test_case(false, "literature",          "max_depth_limit_example",    1; "files are copied up to selected depth")]
    fn acceptance(with_parent: bool, template_name: &str, target_name: &str, max_depth: u8) {
        let config_location = test_path("new").join(target_name).join(".bts");
        let target_path = tmp_path().join(target_name);

        let new_args = NewArgs {
            with_parent,
            template_name: template_name.into(),
            target_path: Some(target_path.clone()),
            max_depth,
        };

        new(new_args, config_location).expect("new");

        let actual_path = PathBuf::from("tests")
            .join("data")
            .join("expected")
            .join("new")
            .join(target_name);

        assert_paths!(actual_path, target_path.clone());

        tear_down(target_path);
    }
}

mod register {
    use super::*;
    use dir_assert::assert_paths;
    use test_case::test_case;

    #[test_case(false, "islands",               "single_file",     "easter_island.txt", 1; "can create template out of single file")]
    #[test_case(false, "ships",                 "whole_directory", "",                 1; "can create template out of multiple files")]
    #[test_case(false, "programming_languages", "overwrite",       "rust,c_plus_plus",  1; "overwrites previous template")]
    #[test_case(true,  "animals/domestic",      "append",          "cats.txt,dogs",     1; "appends to existing template")]
    fn acceptance(
        append: bool,
        template_name: &str,
        target_name: &str,
        partials: &str,
        max_depth: u8,
    ) {
        let target_path = test_path("register").join(target_name);
        let config_location = tmp_path().join(target_name);

        std::fs::create_dir_all(config_location.join("templates")).expect("create config dir");

        for partial in partials.split(',') {
            let partial_path = target_path.join(partial);

            let register_args = RegisterArgs {
                append,
                template_name: template_name.into(),
                target_path: partial_path.clone(),
                max_depth,
            };

            register(register_args, config_location.clone()).expect("register");
        }

        let actual_path = PathBuf::from("tests")
            .join("data")
            .join("expected")
            .join("register")
            .join(target_name);

        assert_paths!(actual_path, config_location.clone());

        tear_down(config_location);
    }
}
