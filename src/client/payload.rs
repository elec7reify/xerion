use serde::Serialize;
use std::ops;

#[derive(Debug)]
pub struct ApiResponse<T, M> {
    payload: ApiPayload<T, M>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiPayload<T, M> {
    pub data: T,
    pub meta: Option<M>,
}

impl<T, M> ApiResponse<T, M> {
    pub(crate) fn new(payload: ApiPayload<T, M>) -> Self {
        Self { payload }
    }

    pub fn payload(&self) -> &ApiPayload<T, M> {
        &self.payload
    }

    pub fn into_payload(self) -> ApiPayload<T, M> {
        self.payload
    }

    pub fn into_data(self) -> T {
        self.payload.data
    }

    pub fn into_meta(self) -> Option<M> {
        self.payload.meta
    }
}

impl<T, M> ops::Deref for ApiResponse<T, M> {
    type Target = ApiPayload<T, M>;
    fn deref(&self) -> &Self::Target {
        &self.payload
    }
}

impl<T, M> Serialize for ApiResponse<T, M>
where
    T: Serialize,
    M: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> crate::error::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.payload.serialize(serializer)
    }
}
