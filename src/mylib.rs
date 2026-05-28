// MPSC practice 

use std::{sync::mpsc, thread::spawn, time::Duration};

pub fn test_channels(){

    let (transmitter,reciever) = mpsc::channel::<u8>();

    // this hangs out the thread
    // reciever.recv(); 


    // drop(reciever);


    let processor = move || {

        println!("starting processor thread !"); 

        let mut failed_count = 0u8;

        loop {

            println!("attempting to recieve message from reciever");
            let recieve_result = reciever.recv_timeout(Duration::from_millis(1000));

            if recieve_result.is_ok(){
                println!("received message: {}", recieve_result.unwrap());
        
            }else{
                failed_count += 1;

                if(failed_count > 10){
                    println!("Aborting processor thread no work available !");

                    break;
                }
            }
            
        }


    };

    for x in 0..=6{

        let send_result: Result<(), mpsc::SendError<u8>> = transmitter.send(x);

        println!("result was : {}",send_result.is_ok());
    }

    std::thread::spawn(processor).join();

    // if(send_result.is_ok()){
    // }else{
    //     println!("result was not fine ")
    // }

}
    
pub fn test_threads (){
 
}

pub fn spawn_thread (){

        let thread_fn = ||{
            let mut x = 0u128;

            for i in 1..500_000_000{
                x += i; 
            }

            println!("{}",x);
        };

    let handle = spawn(
        thread_fn
    );

    println!("worder thread completed");

    handle.join();


}
