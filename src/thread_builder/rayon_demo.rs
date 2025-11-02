use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn is_prime(n: u32) -> bool {
    (2..=n / 2).into_par_iter().all(|i| n % i != 0)
}

// 测试
#[cfg(test)]
mod tests {
    use std::time::Instant;

    use rayon::prelude::*;

    use crate::thread_builder::rayon_demo::is_prime;

    #[test]
    fn test_rayon() {
        let now = Instant::now();

        let nums: Vec<u64> = (2..100000).collect();
        let mut primes = nums
            .par_iter()
            .filter(|&n| is_prime(*n as u32))
            .collect::<Vec<&u64>>();

        let elapsed = now.elapsed();
        primes.par_sort_unstable();

        println!("primes: {:?}", primes);
        println!(
            "elapsed: {} ms, find {} primes",
            elapsed.as_millis(),
            primes.len()
        );
    }

    #[test]
    fn test_rayon_demo2() {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(4)
            .build()
            .unwrap();
        let matrix = [
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
            vec![10, 11, 12],
        ];

        pool.scope(|s| {
            for (i, row) in matrix.iter().enumerate() {
                s.spawn(move |_| {
                    let sum = row.iter().sum::<i32>();
                    println!("Row {} sum = {}", i, sum);
                });
            }
        });
        println!("Main thread finished")
    }

    #[test]
    fn test_rayon_demo3() {
        let outer_pool = rayon::ThreadPoolBuilder::new()
            .num_threads(2)
            .build()
            .unwrap();

        outer_pool.scope(|s| {
            for stage in 0..2 {
                s.spawn(move |_| {
                    println!("stage {stage} started");

                    let inner_pool = rayon::ThreadPoolBuilder::new()
                        .num_threads(2)
                        .build()
                        .unwrap();

                    inner_pool.scope(|s| {
                        for task in 0..2 {
                            s.spawn(move |_| {
                                println!("\t -> Inner task {task} of stage {stage}");
                            });
                        }
                    });

                    println!("stage {stage} finished");
                });
            }
        });
        println!("Main thread finished");
    }

    #[test]
    fn test_rayon_demo4() {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(4)
            .build()
            .unwrap();

        pool.scope(|s| {
            s.spawn_broadcast(|_scope, ctx| {
                let id = ctx.index();
                println!("Thread {id}.");
            });
        });
    }

    #[test]
    fn test_rayon_demo5() {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(4)
            .build()
            .unwrap();

        let func = || println!("Hello!");
        pool.join(func, func);
    }
}
