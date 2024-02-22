// #[cfg(test)]
// mod open {
//     use robusta_dxf::open::open_from_path;

//     use anyhow::Result;
//     // use robusta_dxf::wrapper::RFile;
//     use std::fs::File;
//     use std::io::prelude::*;
//     use std::io::BufReader;

//     // #[test]
//     // fn minimal_open() {
//     //     assert!(open_from_path("tests/resources/minimal-2013.dxf".into()).is_ok());
//     //     assert!(open_from_path("tests/resources/minimal-2018.dxf".into()).is_ok());
//     // }

//     #[test]
//     fn minimal_open1() -> Result<()> {
//         let f = File::open("tests/resources/minimal-2018.dxf")?;
//         let mut reader = BufReader::new(f);

//         let mut line = String::new();
//         let len = reader.read_line(&mut line)?;
//         println!("First line is {len} bytes long");
//         Ok(())
//     }

//     #[test]
//     fn parse_lines() {
//         let d = open_from_path("tests/resources/lines-2018.dxf".into());
//         let a = RFile::from(&d);
//         assert!(!a.entities.is_empty());
//     }

//     #[test]
//     fn parse_arcs() {
//         let d = open_from_path("tests/resources/arcs-2018.dxf".into());
//         let a = RFile::from(&d);
//         assert!(!a.entities.is_empty());
//     }

//     #[test]
//     fn parse_minimal() {
//         let d = open_from_path("tests/resources/minimal-2018.dxf".into());
//         let a = RFile::from(&d);
//         assert!(!a.entities.is_empty());
//     }
// }
