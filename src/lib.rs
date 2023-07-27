use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

pub type UserId = i32;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    User(UserRequest),
    Admin(AdminRequest),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum UserRequest {
    Login {
        login: String,
        password: String,
    },
    GetUserInfo,
    GetSchedule {
        year: u16,
        month: u8,
    },
    SetWorkday {
        year: u16,
        month: u8,
        day: u8,
        is_working: bool,
    },
    ChangePassword {
        old_password: String,
        new_password: String,
    },
    GetUserNames {
        ids: Vec<UserId>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AdminRequest {
    GetUsers,
    AddUser(User),
    ResetPassword {
        id: UserId,
    },
    UpdateUser(User),
    GetRevenue {
        year: u16,
        month: u8,
    },
    SetRevenue {
        year: u16,
        month: u8,
        revenue: Revenue,
    },
    GetSalaryCalculation {
        year: u16,
        month: u8,
    },
}

pub type Response = Result<ResponseData, ProtocolError>;

#[derive(Serialize, Deserialize, Debug)]
pub enum ResponseData {
    // User responses:
    Login {
        token: String,
        id: UserId,
    },
    UserInfo(User),
    Schedule {
        year: u16,
        month: u8,
        schedule: HashMap<UserId, Vec<bool>>,
    },
    PasswordChanged,
    UserNames {
        names: HashMap<UserId, String>,
    },
    // Admin responses:
    Users(Vec<User>),
    PasswordReset,
    Revenue {
        year: u16,
        month: u8,
        revenue: Vec<Revenue>,
    },
    SalaryCalculation {
        salaries: Vec<Salary>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Revenue {
    pub day: u8,
    pub with_percent: f64,
    pub without_percent: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: UserId,
    pub login: String,
    pub name: String,
    pub is_admin: bool,
    pub is_worker: bool,
    pub pay: f64,
    pub percent: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Salary {
    pub id: UserId,
    pub total: f64,
    pub paid: f64,
}

#[derive(Serialize, Deserialize, Error, Debug)]
pub enum ProtocolError {
    #[error("Неверный логин или пароль")]
    LoginFailed,
    #[error("Недостаточно прав")]
    Forbidden,
    #[error("Войдите снова")]
    UnknownToken,
    #[error("Пользователь уже существует")]
    UserExist,
    #[error("Неизвестная ошибка: {0}")]
    Unknown(String),
}
