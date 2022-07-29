use std::{env, thread};
use std::process::Command;

fn worker() -> std::io::Result<()> {
    let path = env::current_exe()?;
    let my = path.as_path().display().to_string();
    let relative_path = my.strip_suffix("/loader").unwrap();

    let java_path = "/Home/bin/java".to_string();
    // java环境路径，可以修改；
    let jar_name = "/usb.jar".to_string(); 
    // jar包名字可以修改
    
    let jar_path = relative_path;
    let jar_invoke_path = jar_path.to_owned()+&jar_name;

    let para = " -jar ".to_string();


    let invoke_path = relative_path.to_owned() + &java_path + &para + &jar_invoke_path;


    println!("java path is {}",invoke_path);
    println!("jar path is {}",jar_invoke_path);



    println!("Server has been Startup.please access http://localhost:8081/home");
    let output = Command::new("sh")
            .arg("-c")
            .arg(invoke_path)
            .output()
            .expect("failed to execute process");

    let outputs = output.stdout;
    println!("{:?}",outputs);

    Ok(())
}


fn main() {
    println!("program boot loader.");
    thread::spawn(move || {
        worker(); //忽略Result处理；
    }).join().unwrap();
}


