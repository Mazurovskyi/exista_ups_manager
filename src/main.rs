
use exista_lib::{modbus, app};
use std:: {io, process};
use std::{result::Result, error::Error, env};



fn main() -> Result<(), Box<dyn Error>>{

    let args: Vec<String> = env::args().collect();
    
    let path = args.get(1).unwrap_or_default();

    //configure the application
    let app_entities = match app::config(path){
        Ok(app_entities) => app_entities,
        Err(err) => return Err(format!("CONFIGURATION ERROR: {err}").into())
    };

    //run the application
    if let Err(err) = app::run(app_entities){
        return Err(format!("RUN ERROR: {err}").into())
    }
    
    Ok(())
}

