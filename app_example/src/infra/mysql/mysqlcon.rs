use std::sync::{Once, ONCE_INIT};

// TODO
pub trait MySqlConnection {
    fn try_con(&self) -> &Option<Box<u8>>;
}

pub trait MySqlConnectionDependencies {}

impl<T> MySqlConnection for T
where
    T: MySqlConnectionDependencies,
{
    fn try_con(&self) -> &Option<Box<u8>> {
        static mut MYSQL_CON: Option<Box<u8>> = None;
        unsafe {
            match &MYSQL_CON {
                Some(n) => println!("number: {}", n),
                None => println!("Still none"),
            }
            static ONCE: Once = ONCE_INIT;
            ONCE.call_once(|| {
                println!("dummy mysql connection");
                MYSQL_CON = match &MYSQL_CON {
                    Some(n) => {
                        println!("Some side: {}", n);
                        Some(Box::new(**n + 1 as u8))
                    }
                    None => {
                        println!("None side");
                        Some(Box::new(3))
                    }
                };
            });
            &MYSQL_CON
        }
    }
}

pub trait UsesMySqlConnection {
    type MySqlConnection: MySqlConnection;
    fn mysql_connection(&self) -> &Self::MySqlConnection;
}
