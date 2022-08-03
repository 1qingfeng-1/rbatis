#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::Read;

    use rbatis::executor::RbatisExecutor;
    use rbatis::rbatis::Rbatis;

    use crate::{init_sqlite, BizActivity};

    pub trait IsTest {
        fn is_test(&self) -> bool;
    }

    impl IsTest for rbson::Bson {
        fn is_test(&self) -> bool {
            match self {
                Bson::String(v) => v.eq("test"),
                _ => false,
            }
        }
    }

    /// you can see custom fn on xml:
    /// ```xml
    /// <if test="name.is_test()">
    ///    ....
    ///  </if>
    /// ```
    #[html_sql("example/example.html")]
    async fn custom_func(rb: &mut RbatisExecutor<'_>, name: &str) -> Vec<BizActivity> {
        impled!()
    }

    #[tokio::test]
    pub async fn test_custom_func() {
        fast_log::init(fast_log::config::Config::new().console());
        //use static ref
        let rb = init_sqlite().await;
        let a = custom_func(&mut rb.as_executor(), "test").await.unwrap();
        println!("{:?}", a);
    }


    /// you can see custom fn on py_sql
    #[py_sql("select * from biz_activity where delete_flag = 0
                 if name.is_test():
                    and name != 'test'")]
    async fn custom_func_py(rb: &Rbatis) -> Vec<BizActivity> {
        impled!()
    }

    #[tokio::test]
    pub async fn test_custom_func_py() {
        fast_log::init(fast_log::config::Config::new().console());
        //use static ref
        let rb = init_sqlite().await;
        let a = custom_func_py(&rb).await.unwrap();
        println!("{:?}", a);
    }
}
