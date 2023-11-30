Reproducing issuing `autometrics` along with `async-trait`.


### Instructions

Run in sequence:

* Terminal 1: Server
    ```
    $ cargo run
    Compiling autometrics-async-trait-repro v0.1.0
     Finished dev [unoptimized + debuginfo] target(s) in 1.75s
      Running `target/debug/autometrics-async-trait-repro`
    ```
* Terminal 2: Client
    ```
    $ cargo run -- client
    Compiling autometrics-async-trait-repro v0.1.0
     Finished dev [unoptimized + debuginfo] target(s) in 0.77s
      Running `target/debug/autometrics-async-trait-repro client`
    RESPONSE=Ok(Response { metadata: MetadataMap { headers: {"content-type": "application/grpc", "date": "Thu, 30 Nov 2023 16:57:35 GMT", "grpc-status": "0"} }, message: HelloReply { message: "Hello Tonic!" }, extensions: Extensions })
    RESPONSE=Err(Status { code: Internal, message: "error!", metadata: MetadataMap { headers: {"content-type": "application/grpc", "date": "Thu, 30 Nov 2023 16:57:35 GMT", "content-length": "0"} }, source: None })
    RESPONSE=Ok(Response { metadata: MetadataMap { headers: {"content-type": "application/grpc", "date": "Thu, 30 Nov 2023 16:57:35 GMT", "grpc-status": "0"} }, message: HelloReply { message: "Hello Tonic!" }, extensions: Extensions })
    RESPONSE=Err(Status { code: Internal, message: "error!", metadata: MetadataMap { headers: {"content-type": "application/grpc", "date": "Thu, 30 Nov 2023 16:57:35 GMT", "content-length": "0"} }, source: None })

    ```
* Terminal 3: Query
    ```
    $ curl localhost:3000/metrics | rg 'result='
    function_calls_total{function="MyGreeter::say_hello",module="autometrics_async_trait_repro",service_name="autometrics",caller_function="",caller_module="",result="",ok="",error="",objective_name="",objective_percentile=""} 4
    ```

