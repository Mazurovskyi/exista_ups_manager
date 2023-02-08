
use exista_lib::modbus;
use std:: {io, process};
use std::{result::Result, error::Error, env};



fn main() -> Result<(), Box<dyn Error>>{

    let args: Vec<String> = env::args().collect();

    let path = args.get(1).ok_or("Arguments error! Path to port")?;
    let duration = args.get(2).ok_or("Arguments error! Port timeout (millisec)")?;
    let timeout:u64 = duration.parse()?;


    let port = match modbus::config(path, timeout){
        Ok(port)=>port,
        Err(err)=>{println!("ERROR CONFIG MODBUS: {err}"); return Err(err)}
    };


    if let Err(err) = modbus::run(port){
        println!("ERROR RUNNING MODBUS: {err}");
        return Err(err)
    };
    
    Ok(())
}

