#[macro_use]
extern crate lazy_static;
extern crate hyper;
use actix_web::HttpRequest;
use core::marker::PhantomData;
use std::sync::Mutex;
use tokio::net::TcpListener;
use tokio::prelude::*;
use quick_js::{Context, JsValue, JsAsync};
use tokio::runtime::*;
use tokio::time::*;
use std::{pin::Pin, future::Future};
use async_recursion::async_recursion;
use std::convert::Infallible;
use std::{sync::{RwLock, Arc}, net::SocketAddr, cell::{Cell, RefCell}, rc::Rc, thread::LocalKey, task::Waker, time::SystemTime};
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use num_cpus;
use hyper::Client;
use actix_web::{get, web, App, HttpServer, Responder};
use actix_rt;

/*
fn js_std_loop(ctx: *mut libquickjs_sys::JSContext) {

    unsafe {
        let mut err: i32;
        let ctx1 = ptr::null();

        loop {
            err = libquickjs_sys::JS_ExecutePendingJob(libquickjs_sys::JS_GetRuntime(ctx), ctx1);
            if err <= 0 {
                if err < 0 {
                    let exception_val = libquickjs_sys::JS_GetException(ctx);
                    // js_std_dump_error1(ctx, exception_val);
                    // JS_FreeValue(ctx, exception_val);
                }
                break;
            }

            break;
        }
    }
}*/

#[async_recursion]
async fn looping<'a>(cnt: i32) {
    tokio::time::delay_for(Duration::from_millis(1000)).await;
    println!("COUNT: {:?}", cnt + 1);
    looping(cnt + 1).await
}

thread_local!(pub static CONTEXT: Context = Context::new().unwrap());

async fn hello(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // println!("{:?}", req);

    // let now = SystemTime::now();

    // let ctx = Context::new().unwrap();

    let host= req.uri().to_string();



    // let result = ctx.eval_as::<String>(format!("'<h1>' + '{}'.toLowerCase() + '</h1>'", host).as_str()).unwrap();
    /*
    let eval_result = CONTEXT.with(|ctx| {

        ctx.eval_async::<String>("setTimeout(() => {
            complete('hello');
        })")
        // ctx.step();
        
        // println!("{:?}", promise); 

        // let result = format!("lowerCase({})", String::from_utf8(host).unwrap());

        // return ctx.call_function("lowerCase", vec![host]).unwrap().into_string().unwrap();

        // return ctx.eval_as::<String>(result.as_str()).unwrap();
        // return ctx.eval_as::<String>(format!("'<h1>' + '{}'.toLowerCase() + '</h1>'", host).as_str()).unwrap();
    }).await;*/

    // Ok(Response::new(eval_result.unwrap().into()))

    Ok(Response::new("".into()))
}

fn test(CTX: &'static LocalKey<Context>) {
    CTX.with(|ctx| {

    });
}

async fn run() {

    CONTEXT.with(|context | {

        JsAsync::init(&CONTEXT).unwrap();

        context.add_callback("print", |val: String| {
            println!("{}", val);
            return "";
        }).unwrap();

/*
        context.eval("let timerCbs = []; const setTimeout = (cb, timeout) => {
            let len = timerCbs.length;
            timerCbs.push(cb);
            async_timers(len, timeout);
            return len;
        };").unwrap();
        
        context.add_callback("async_timers", |index: i32, timeout: i32| {
            let time = timeout as u64;
            let idx = index;
            tokio::task::spawn_local(async move {
                tokio::time::delay_for(Duration::from_millis(time)).await;
                CONTEXT.with(|ctx| {
                    // ctx.call_function("callTimer", vec![idx]).unwrap();
                    ctx.eval(format!("timerCbs[{}]();", idx).as_str()).unwrap();
                });
            });
            index
        }).unwrap();*/

        context.eval("const fetch = (url) => {
                return new Promise((res, rej) => {
                    let ln = __fetch_cbs.length;
                    __fetch_async(ln, url);
                    __fetch_cbs.push([res, rej]);
                });
            };
            const __fetch_cbs = [];
        ").unwrap();
        
        context.add_callback("__fetch_async", |i: i32, url: String| {
            tokio::task::spawn_local(async move {
                // Await the response...
                let body = reqwest::get(url.as_str()).await.unwrap().text().await.unwrap();

                CONTEXT.with(|ctx| {
                    ctx.eval(format!("__fetch_cbs[{}][0]({:?});", i, body).as_str()).unwrap();
                    ctx.step(); // resolve promise
                });
            });
            0i32
        }).unwrap();

        context.eval("setTimeout(() => {
            print('hello');
        }, 1000)").unwrap();
        // println!("{:?}", value);
        
    });

    let addr = ([127, 0, 0, 1], 3000).into();

    // Using a !Send request counter is fine on 1 thread...
    // let counter = Rc::new(Cell::new(0));

    let make_service = make_service_fn(move |_| {
        // While the state was moved into the make_service closure,
        // we need to clone it here because this closure is called
        // once for every connection.
        //
        // Each connection could send multiple requests, so
        // the `Service` needs a clone to handle later requests.

        async move {
            // This is the `Service` that will handle the connection.
            // `service_fn` is a helper to convert a function that
            // returns a Response into a `Service`.
            Ok::<_, Error>(service_fn(move |_req: Request<Body>| {

                async move {

                    let str_result = JsAsync::eval_as::<String>(&CONTEXT, r####"
                        complete(await fetch("https://google.com"));
                    "####.to_string()).await.unwrap();

                    Ok::<_, Error>(Response::new(Body::from(str_result)))
                    // Ok::<_, Error>(Response::new(Body::from("hello")))
                }
            }))
        }
    });

    let server = Server::bind(&addr).executor(LocalExec).serve(make_service);

    println!("Listening on http://{}", addr);

    // The server would block on current thread to await !Send futures.
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn hello_world(req: HttpRequest, _stream: actix_web::web::Payload) -> actix_web::HttpResponse {
    // actix_web::HttpResponse::from("<h1>Hello, world!</h1>")
    let mut result = actix_web::HttpResponse::Ok();
    
    actix_web::HttpResponse::Ok().content_type("text/html").body(format!("<h1>Hello, {}</h1>", req.uri()))
}

fn main() {
    let mut single_rt = Builder::new()
    .basic_scheduler()
    .enable_all()
    .build().unwrap();

    single_rt.spawn(async {
        //tokio::time::delay_for(Duration::from_millis(1000)).await;
        // println!("One second later!");
        let mut total = 0u128;
        println!("TOTAL {:?}", total);
    });

    let local = tokio::task::LocalSet::new();
    let system_fut = actix_rt::System::run_in_tokio("main", &local);
    local.block_on(&mut single_rt, async {
        tokio::task::spawn_local(system_fut);

        let _ = actix_web::HttpServer::new(|| {
            // actix_web::App::new().service(actix_web::web::resource("/").to(|| async { "<h1>Hello world!</h1>" }))
            actix_web::App::new().service(actix_web::web::resource("*").to(hello_world))
        })
        .bind("127.0.0.1:8082")
        .unwrap()
        .run()
        .await;
    });
    // local.block_on(&mut single_rt, run());
}
 
// Since the Server needs to spawn some background tasks, we needed
// to configure an Executor that can spawn !Send futures...
#[derive(Clone, Copy, Debug)]
struct LocalExec;

impl<F> hyper::rt::Executor<F> for LocalExec
where
    F: std::future::Future + 'static, // not requiring `Send`
{
    fn execute(&self, fut: F) {
        // This will spawn into the currently running `LocalSet`.
        tokio::task::spawn_local(fut);
    }
}

/*
let js_result = js_await_eval("new Promise((res, rej) => { setTimeout(() => { res(42); }, 500)  }").await;

new_js_promise("name_of_function", some_rs_future);
// in javascript land:
name_of_function().then...

*/


struct SetTimeout {}

impl SetTimeout {

    pub fn new<F>(cb: &'static F, delayMS: u64) where F: Fn() {

        let delay = delayMS;
        tokio::task::spawn_local(async move {
            tokio::time::delay_for(Duration::from_millis(delay)).await;
            cb();
        });

        /*let delay = delayMS;
        rt.spawn(async move {
            tokio::time::delay_for(Duration::from_millis(delay)).await;
            cb();
        });*/
    }
}

/*
impl<'a, F> Future for SetTimeout<'a, F> where F: Fn() {
    type Output = i32;
    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {

        let delay = self.delayMS;
        

        let mut is_done = *self.done.lock().unwrap();

        println!("HERE {}", is_done);

        if is_done {
            let cb = &*self.cb.lock().unwrap();
            cb();
            std::task::Poll::Ready(0i32)
        } else {
            let waker = cx.waker().clone();
            self.runtime.spawn(async move {
                tokio::time::delay_for(Duration::from_millis(delay)).await;
                is_done = true;
                waker.wake();
            });
            std::task::Poll::Pending
        }
   
    }
}*/