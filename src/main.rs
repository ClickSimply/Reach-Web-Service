#[macro_use]
extern crate lazy_static;
extern crate hyper;
use actix_rt;
use actix_web::HttpRequest;
use actix_web::{get, web, App, HttpServer, Responder};
use async_recursion::async_recursion;
use core::marker::PhantomData;
use futures::{Future, Stream, StreamExt};
use hyper::service::{make_service_fn, service_fn};
use hyper::Client;
use hyper::{Body, Request, Response, Server};
use num_cpus;
use quick_js::{Context, JsAsync, JsValue};
use std::convert::Infallible;
use std::pin::Pin;
use std::sync::Mutex;
use std::{
    cell::{Cell, RefCell},
    net::SocketAddr,
    rc::Rc,
    sync::{Arc, RwLock},
    task::Waker,
    thread::LocalKey,
    time::SystemTime,
};
use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio::runtime::*;
use tokio::time::*;
use deno_core::*;
use deno_core::ErrBox;
use deno_core::ModuleLoader;
use deno_core::EsIsolate;

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

    let host = req.uri().to_string();

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
    CTX.with(|ctx| {});
}

async fn run() {
    CONTEXT.with(|context| {
        // JsAsync::init(&CONTEXT).unwrap();

        context
            .add_callback("print", |val: String| {
                println!("{}", val);
                return "";
            })
            .unwrap();

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

        context
            .eval(
                "const fetch = (url) => {
                return new Promise((res, rej) => {
                    let ln = __fetch_cbs.length;
                    __fetch_async(ln, url);
                    __fetch_cbs.push([res, rej]);
                });
            };
            const __fetch_cbs = [];
        ",
            )
            .unwrap();

        context
            .add_callback("__fetch_async", |i: i32, url: String| {
                tokio::task::spawn_local(async move {
                    // Await the response...
                    let body = reqwest::get(url.as_str())
                        .await
                        .unwrap()
                        .text()
                        .await
                        .unwrap();

                    CONTEXT.with(|ctx| {
                        ctx.eval(format!("__fetch_cbs[{}][0]({:?});", i, body).as_str())
                            .unwrap();
                        ctx.step(); // resolve promise
                    });
                });
                0i32
            })
            .unwrap();

        context
            .eval(
                "setTimeout(() => {
            print('hello');
        }, 1000)",
            )
            .unwrap();
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
                    /*let str_result = JsAsync::eval_as::<String>(&CONTEXT, r####"
                        complete(await fetch("https://google.com"));
                    "####.to_string()).await.unwrap();*/
                    let str_result = "hello".to_owned();

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

/*
thread_local!(pub static V8_RUST: V8_Runtime<'static> = {
    let mut isolate = v8::Isolate::new(Default::default());

    let mut handle_scope = v8::HandleScope::new(&mut isolate);
    let scope = handle_scope.enter();
    let context = v8::Context::new(scope);
    let mut context_scope = v8::ContextScope::new(scope, context);
    let scope = context_scope.enter();

    V8_Runtime {
        scope: RefCell::new(**scope),
        context: RefCell::new(context)
    }
});*/

pub struct SuperUnsafeCell<T> {
    item: core::cell::UnsafeCell<T>
}

impl<T> SuperUnsafeCell<T> {
    pub fn new(item: T) -> Self {
        Self { item: core::cell::UnsafeCell::new(item) }
    }

    pub fn borrow(&self) -> &T {
        let self_bytes = unsafe { &*self.item.get() };
        self_bytes
    }

    pub fn borrow_mut(&self) -> &mut T {
        let self_bytes = unsafe { &mut *self.item.get() };
        self_bytes
    }
}


async fn main_handler(
    ctx: actix_web::web::Data<SuperUnsafeCell<EsIsolate>>,
    req: HttpRequest,
    mut body: actix_web::web::Payload,
) -> actix_web::HttpResponse {
    // actix_web::HttpResponse::from("<h1>Hello, world!</h1>")
    let mut result = actix_web::HttpResponse::Ok();

    let mut bytes = actix_web::web::BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item.unwrap());
    }

    // println!("thread: {:?}", std::thread::current().id());

    if bytes.len() > 0 {
        println!("Req bytes: {:?}", bytes);
    }


    let mut js_context: &mut EsIsolate = &mut *ctx.borrow_mut();

    match js_context.execute("file.js", "Deno.stdout.write(new Uint8Array(0))") {
        Ok(x) => {
            println!("Success {:?}", x);
        },
        Err(x) => {
            println!("Error {:?}", x);
        }
    }



    // let mut scope = V8_RUST.scope.borrow_mut();
    // let mut context = *V8_RUST.context.borrow_mut();

    // let code = v8::String::new(&mut *scope, "'Hello' + ' World!'").unwrap();
    // println!("javascript code: {}", code.to_rust_string_lossy(scope));

    // let mut script = v8::Script::compile(&mut *scope, context, code, None).unwrap();

    // let result = script.run(&mut *scope, context).unwrap();
    //  let result = result.to_string(&mut *scope).unwrap();
    // println!("result: {}", result.to_rust_string_lossy(scope));

    /*let js_result: String = ctx.eval_as(format!("(() => {{
        let url = '{:?}';
        return '<h1>Hello, ' + url.toLowerCase() + '</h1>';
    }})()", req.uri()).as_str()).unwrap();*/

    /*let js_context = ctx.get_ref() as *const Context;

    JsAsync::eval(js_context,"setTimeout(() => {
        complete();
    }, 1000)".to_owned()).await.unwrap();*/

    actix_web::HttpResponse::Ok()
        .content_type("text/html")
        .body("<h2>hello</h2>")
    /*let js_result = JsAsync::eval_as::<String>(js_context, "complete('<h1>hello</h1>')".to_owned()).await;

    match js_result {
        Ok(x) => {
            actix_web::HttpResponse::Ok().content_type("text/html").body(x)
        },
        Err(x) => {
            println!("{:?}", x);
            actix_web::HttpResponse::Ok().content_type("text/html").body("Error")
        }
    }*/

    // let js_result = format!("<h1>Hello, {}</h1>", req.uri());
}

struct Null_Loader {

}

impl Null_Loader {
    pub fn new() -> Self {
        Null_Loader {}
    }
}

impl deno_core::ModuleLoader for Null_Loader {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        is_main: bool,
      ) -> Result<ModuleSpecifier, ErrBox> {
          todo!()
    }

    fn load(
        &self,
        module_specifier: &ModuleSpecifier,
        maybe_referrer: Option<ModuleSpecifier>,
        _is_dyn_import: bool,
      ) -> Pin<Box<deno_core::ModuleSourceFuture>> {
          todo!()
      }
}

// rusty_v8::isolate::OwnedIsolate
fn new_js_context() -> SuperUnsafeCell<EsIsolate> {

    let worker = deno::worker::MainWorker::new();

    let startup = deno_core::StartupData::None;

    let null_loader = Rc::new(Null_Loader::new());

    let mut isolate = EsIsolate::new(null_loader, startup, false);

    // isolate.execute("cli.js", std::str::from_utf8(CLI_SNAPSHOT).unwrap()).unwrap();

    SuperUnsafeCell::new(isolate)
}

fn main() {
    let mut single_rt = Builder::new()
        .basic_scheduler()
        .enable_all()
        .build()
        .unwrap();

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
            actix_web::App::new()
                .data(new_js_context())
                .service(actix_web::web::resource("*").to(main_handler))
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
    pub fn new<F>(cb: &'static F, delayMS: u64)
    where
        F: Fn(),
    {
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
