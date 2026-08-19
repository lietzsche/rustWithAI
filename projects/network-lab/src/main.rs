mod n1_1;
mod n1_2;
mod n1_3;
mod n2_1;
mod n2_2;

fn main() {
    println!("network lab");
    n1_1::n1_1();
    n1_2::n1_2();
    n1_3::n1_3();
    n2_1::n2_1();

    let run_stdin_demo = std::env::args().any(|argument| argument == "--stdin-demo");
    if run_stdin_demo {
        n2_1::blocking_stdin_demo();
    }
    let tcp_server_demo = std::env::args().any(|argument| argument == "--tcp-server");
    if tcp_server_demo {
        n2_2::tcp_server_demo();
    }
    if std::env::args().any(|arg| arg == "--tcp-client") {
        n2_2::tcp_client_demo();
    }
}
