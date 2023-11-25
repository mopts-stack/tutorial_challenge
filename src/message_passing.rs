use crossbeam_channel::unbounded;
use std::thread;

enum WorkerMsg {
    PrintData(String),
    Sum(i64, i64),
    Quit,
}

fn example1() {
    let (s, r) = unbounded();
    let handle = thread::spawn(move || loop {
        match r.recv() {
            Ok(msg) => match msg {
                WorkerMsg::PrintData(d) => println!("{}", d),
                WorkerMsg::Sum(lhs, rhs) => println!("{}+{}={}", lhs, rhs, lhs + rhs),
                WorkerMsg::Quit => {
                    println!("thread terminated");
                    break;
                }
            },
            Err(e) => {
                println!("disconnected: {:?}", e);
                break;
            }
        }
    });

    s.send(WorkerMsg::PrintData("Hello from main".to_owned()))
        .unwrap();
    s.send(WorkerMsg::Sum(10, 20)).unwrap();
    s.send(WorkerMsg::Quit).unwrap(); // we need to quit otherwise it will run forever

    // drop(s); // will delete the sending end and the channel will be disconnected

    handle.join().unwrap();
}

enum MainMsg {
    SumResult(i64),
    WorkerQuit,
}

fn example2() {
    let (worker_tx, worker_rx) = unbounded();
    let (main_tx, main_rx) = unbounded();

    let worker = thread::spawn(move || loop {
        match worker_rx.recv() {
            Ok(msg) => match msg {
                WorkerMsg::PrintData(d) => println!("Worker: {}", d),
                WorkerMsg::Sum(lhs, rhs) => {
                    println!("Worker: summing...");
                    main_tx.send(MainMsg::SumResult(lhs + rhs)).unwrap();
                }
                WorkerMsg::Quit => {
                    println!("Worker: terminating...");
                    main_tx.send(MainMsg::WorkerQuit).unwrap();
                    break;
                }
            },
            Err(_) => {
                println!("Worker: disconnected");
                main_tx.try_send(MainMsg::WorkerQuit).unwrap();
                break;
            }
        }
    });

    worker_tx
        .send(WorkerMsg::PrintData("Hello from main".to_owned()))
        .unwrap();
    worker_tx.send(WorkerMsg::Sum(10, 20)).unwrap();
    worker_tx.send(WorkerMsg::Quit).unwrap();

    while let Ok(msg) = main_rx.recv() {
        match msg {
            MainMsg::SumResult(answer) => println!("Main: answer = {}", answer),
            MainMsg::WorkerQuit => println!("Main: worker terminated"),
        }
    }

    worker.join().unwrap();
}

pub fn start() {
    example1();
    example2();
}
