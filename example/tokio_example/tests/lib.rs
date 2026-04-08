pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub async fn double(n: i32) -> i32 {
    n * 2
}

// cargo add tokio -F full
// cargo test -- --test 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn test_double() {
        // let result = double(3).await;
        assert_eq!(double(3).await, 6);
    }

    #[tokio::test(flavor = "current_thread")]// 单线程
    async fn test_double_current_thread() {
        // let result = double(3).await;
        assert_eq!(double(3).await, 6);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]// 多线程
    async fn test_double_multi_thread() {
        // let result = double(3).await;
        assert_eq!(double(3).await, 6);
    }

    #[test]
    fn test_double_sync() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        assert_eq!(rt.block_on(double(3)), 6);
    }
}
