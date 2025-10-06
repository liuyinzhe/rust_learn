#[derive(Clone, Copy)] // 自动为 Owner 枚举实现 Clone 和 Copy trait
pub enum Owner {
    User,
    Group,
    Other,
}

impl Owner {
    pub fn masks(self) -> [u32; 3] {
        match self {
            Self::User => [0o400, 0o200, 0o100],
            Self::Group => [0o040, 0o020, 0o010],
            Self::Other => [0o004, 0o002, 0o001],
        }
    }
}
/*
0o004 (八进制 004) = 二进制 000000100 - 其他用户读权限
0o002 (八进制 002) = 二进制 000000010 - 其他用户写权限
0o001 (八进制 001) = 二进制 000000001 - 其他用户执行权限
*/