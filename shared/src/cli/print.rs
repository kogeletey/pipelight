use pipeline::types::Pipeline;
use std::error::Error;
/// Pretty print logs from json log file
pub fn pretty() -> Result<(), Box<dyn Error>> {
    let pipelines = Pipeline::get()?;
    for pipeline in pipelines {
        println!("{}", pipeline);
    }
    Ok(())
}

// /// Print json log file
// pub fn json() -> Result<(), Box<dyn Error>> {
//     let pipelines = Pipeline::get_logged()?;
//     for pipeline in pipelines {
//         let pipeline_json = serde_json::to_string::<Pipeline>(&pipeline)?;
//         println!("{}", pipeline_json);
//     }
//     Ok(())
// }
