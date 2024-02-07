#[macro_export]
macro_rules! concurrent_step {
    ($error_batch:expr, $inputs:expr, $step:expr) => {{
        let handles: Vec<_> = $inputs
            .into_iter()
            .map(|input| {
                thread::spawn(|| $step(input))
            })
            .collect();

        let mut results: Vec<_> = vec![];

        for handle in handles {
            let result = handle.join();
            match result {
                Ok(Ok(result)) => results.push(result),
                Ok(Err(e)) => $error_batch.add(e),
                Err(e) => $error_batch.add(anyhow!("Thread panicked: {:?}", e)),
            }
        }

        results
    }};
    ($error_batch:expr, $inputs:expr, $step:expr, { $($prefunc:tt)* }) => {{
        let handles: Vec<_> = $inputs
            .into_iter()
            .map(|input| {
                // Run the prefunc
                $($prefunc)*

                thread::spawn(move || $step(input))
            })
            .collect();

        let mut results: Vec<_> = vec![];

        for handle in handles {
            let result = handle.join();
            match result {
                Ok(Ok(result)) => results.push(result),
                Ok(Err(e)) => $error_batch.add(e),
                Err(e) => $error_batch.add(anyhow!("Thread panicked: {:?}", e)),
            }
        }

        results
    }};
}