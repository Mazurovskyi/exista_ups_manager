
use exista_lib::{modbus, app};
use std:: {io, process};
use std::{result::Result, error::Error, env};



fn main() -> Result<(), Box<dyn Error>>{

    //let _args: Vec<String> = env::args().collect();
    //let _path = args.get(1).unwrap_or_default();

    //configure the application
    let app_entities = match app::config(){
        Ok(app_entities) => app_entities,
        Err(err) => return Err(format!("CONFIGURATION ERROR: {err}").into())
    };

    //run the application
    if let Err(err) = app::run(app_entities){
        return Err(format!("RUN ERROR: {err}").into())
    }
    
    Ok(())
}

