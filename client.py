import subprocess, threading, sys, select, os


def show(*args, **kwargs):
    args = list(args)
    if len(args) > 0:
        if args[0] != "":
            args[0] = f"\n{args[0]}"
            print(*args, **kwargs, end="")


def main():

    script_name = os.environ.get('SCRIPT_NAME')
    if script_name is None:
        print("Invalid script name. Exiting") 
        return

    INPUT_TIMEOUT = 0.1
    args = sys.argv[1:]
    
    src = "./target/debug/cli_template"
    if len(args) > 0 and args[0] == "1":
        src = "./target/release/cli_template"

    # Send a message to the microservice
    proc = subprocess.Popen(
        [src],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )
    
    start_msg = proc.stdout.readline().strip()
    service_is_active = start_msg == "service_started"
    
    stop_event = threading.Event()

    show(
        "\033[1;36mClient connected. Enter commands below "
        "(or 'stop' to quit)\033[0m",
    )

    def new_line(pre="\n"):
        print(f"\033[1;32m{pre}\n>>> \033[0m", end="")

    new_line("")
    
    # Threads
    def input_thread():
       
        def get_input():
            ready, _, _ = select.select([sys.stdin], [], [], INPUT_TIMEOUT)
            if ready:
                text = sys.stdin.readline()
                return text 
             
        try:
            while not stop_event.is_set():
                user = get_input()
                if user is not None:
                    proc.stdin.write(user)
                    proc.stdin.flush()
        
        except KeyboardInterrupt:
            return

    def process_reader():
        while not stop_event.is_set():
            line = proc.stdout.readline().rstrip()
           
            if line == "":
                break 

            if line == "service_stopping":
                show("\033[1;33mService shutting down\033[0m")
                stop_event.set()
                break 
            
            elif line and line != "ok":
                show(line.replace("\0", "\n"))
            
            new_line("")

    def error_reader():
        for line in proc.stderr:
            if stop_event.is_set():
                break 

            if line == "":
                stop_event.set()
                return 
            else:
                show(line.rstrip())
                new_line("")

    r_thread = threading.Thread(target=process_reader)
    e_thread = threading.Thread(target=error_reader)
    w_thread = threading.Thread(target=input_thread, daemon=True)

    if service_is_active:
        r_thread.start() 
        e_thread.start()
        w_thread.start()

        try:
            e_thread.join()
            r_thread.join()

        except KeyboardInterrupt:
            stop_event.set()
            proc.stdin.write('stop\n')
            proc.stdin.flush()
            proc.terminate()

        show("\nGoodbye!\033[1;31m <3\033[0m\n") 


if __name__ == "__main__":
    main()

