use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

pub const OK_CODE: &'static str = "ok";
pub const DEFAULT_ERROR_CODE: &str = "error";

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResult<T> {
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args:  Option<Map<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T: Serialize + for<'de> Deserialize<'de>> ApiResult<T> {
    pub fn ok(data: impl Into<Option<T>>) -> Self {
        ApiResult {
            code: OK_CODE.to_string(),
            msg:  None,
            args: None,
            data: data.into(),
        }
    }

    pub fn error(code: impl Into<String>) -> Self {
        ApiResult {
            code: code.into(),
            msg:  None,
            args: None,
            data: None,
        }
    }

    pub fn fail(code: impl Into<String>, msg: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            msg: Some(msg.into()),
            args: None,
            data: None,
        }
    }

    pub fn with_msg(mut self: Self, msg: impl Into<String>) -> Self {
        self.msg = Some(msg.into());
        self
    }

    pub fn with_args(mut self: Self, args: impl Into<Map<String, Value>>) -> Self {
        self.args = Some(args.into());
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageResult<T> {
    pub total: u64,
    pub items: Vec<T>,
}

pub type ApiPageResult<T> = ApiResult<PageResult<T>>;

impl<T: Serialize + for<'de> Deserialize<'de>> ApiPageResult<T> {
    pub fn page(total: impl Into<u64>, items: impl Into<Vec<T>>) -> Self {
        Self::ok(PageResult { total: total.into(), items: items.into() })
    }
}
