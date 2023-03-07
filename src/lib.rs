

pub mod time {
    use std::fmt::Display;

    pub struct Time {
        hours: u8,
        minutes: u8
    }

    impl Time {
        pub fn new(hours: u8, minutes: u8) -> Self {
            Time{hours, minutes}
        }
    }

    #[derive(Clone)]
    pub struct Duration {
        minutes: u32
    }

    impl Default for Duration {
        fn default() -> Self {
            Duration{minutes: 0}
        }
    }

    impl From<(u8, u8)> for Duration {
        fn from(tuple: (u8, u8)) -> Self {
            Duration{ minutes: tuple.0 as u32 * 60 + tuple.1 as u32}
        }
    }

    impl Duration {
        fn into_tuple(&self) -> (u8, u8) {
            let hours = self.minutes % 60;
            let minutes = self.minutes - hours * 60;
            (hours as u8, minutes as u8)
        }
    }

    impl Display for Duration {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let (hours, minutes) = self.into_tuple();
            write!(f, "{}h{:02}", hours, minutes)
        }
    }

}
