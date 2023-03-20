use std::task::{Context, Poll};

use ntex::{
    http::{
        body::{Body, ResponseBody},
        HttpMessage,
    },
    util::BoxFuture,
    web::{WebRequest, WebResponse},
    Middleware, Service,
};
use reqwest::{Client, StatusCode};
use sqlx::{Pool, Postgres};

pub struct CheckLogin {
    /// 数据库连接池
    pub db_pool: Pool<Postgres>,
    /// 只允许管理员操作
    pub admin: bool,
}

pub struct CheckLoginMiddleware<S> {
    db_pool: Pool<Postgres>,
    admin: bool,
    service: S,
}

impl<S> Middleware<S> for CheckLogin {
    type Service = CheckLoginMiddleware<S>;

    fn create(&self, service: S) -> Self::Service {
        CheckLoginMiddleware {
            db_pool: self.db_pool.clone(),
            admin: self.admin,
            service,
        }
    }
}

impl<S, E> Service<WebRequest<E>> for CheckLoginMiddleware<S>
where
    S: Service<WebRequest<E>, Response = WebResponse>,
    E: 'static,
{
    type Response = WebResponse;
    type Error = S::Error;
    type Future<'f> = BoxFuture<'f, Result<Self::Response, Self::Error>> where S: 'f, E: 'f;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: WebRequest<E>) -> Self::Future<'_> {
        todo!()
    }

    // fn call(&self, req: WebRequest<E>) -> Self::Future<'_> {
    //     Box::pin(async move {
    //         let request_method = req.method().to_owned();

    //         // get 请求不认证
    //         if request_method != Method::GET {
    //             let db_pool = &self.db_pool;

    //             let cookie = req.cookie("ACCESS_TOKEN");

    //             let mut res = self.service.call(req).await?;

    //             let access_token = match cookie {
    //                 Some(c) => c,
    //                 None => {
    //                     res.response_mut().head_mut().status = StatusCode::UNAUTHORIZED;
    //                     res = res.map_body(|_, _| {
    //                         ResponseBody::from(Body::from_slice("你还没有登录".as_bytes()))
    //                     });
    //                     return Ok(res);
    //                 }
    //             };
    //             let client = Client::new();

    //             let user_info = client
    //                 .get("https://api.github.com/user")
    //                 .bearer_auth(access_token.clone())
    //                 .header("User-Agent", "miracleyin")
    //                 .send()
    //                 .await;

    //             let 
    //         }
    //     })
    // }
}
