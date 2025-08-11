use std::io;

async fn might_fail(ok: bool) -> io::Result<&'static str> {
    if ok {
        Ok("Success")
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Fail"))
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let result: io::Result<()> = async {
        let val = might_fail(false).await?; // exits only the async block, not main()
        println!("Got: {val}");
        Ok(())
    }
    .await; // <- main() still continues even if error happened

    println!("After async block, result = {:?}", result);

    // To propagate error from async block to outer function:
    async {
        let val = might_fail(false).await?;
        println!("Got: {val}");
        Ok::<(), io::Error>(())
    }
    .await?; // <- now `?` applies to main()

    Ok(())
}

