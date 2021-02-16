use cmd_lib::run_cmd;
use cmd_lib::CmdResult;

fn main() {
    let msg = "I love rust!";
    run_cmd!(echo $msg)?;
    run_cmd!(|msg| echo "This is the message: $msg")?; 

    // piped commands 
    run_cmd!(ls -la | grep rust)?;

    // If a command fails, return error 

    let file = "/tmp/f";
    let keyword = "rust";

    if run_cmd! {
        cat ${file} | grep ${keyword};
        echo "bad cmd" >&2;

        ls /nofile || true ; 
        date; 
        ls oops; 
        cat oops; 
    }.is_err() {
        panic!("CRASH!")
    }
}