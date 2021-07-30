#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;
use napi::{CallContext, Env, JsObject, Result, Task, JsBoolean, JsString};
use rodio::{OutputStream, Sink, Decoder};
use std::fs::File;
use std::io::BufReader;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
  static ref SINK: Mutex<Sink> = {
    println!("{}", "init player");
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    Mutex::new(Sink::try_new(&stream_handle).unwrap())
  };
}

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
  exports.create_named_method("play", play)?;
  Ok(())
}

struct PlayerTask {
  path: String,
  // sink: Sink
}

impl Task for PlayerTask {
    type Output = bool;
    type JsValue = JsBoolean;

    fn compute(&mut self) -> Result<Self::Output> {
      let file = BufReader::new(File::open(self.path.as_str()).unwrap());
      let source = Decoder::new(file).unwrap();
      let (_stream, stream_handle) = OutputStream::try_default().unwrap();
      let sink = Sink::try_new(&stream_handle).unwrap();
      if !sink.empty() {
        sink.stop();
      }
      sink.append(source);
      sink.sleep_until_end();

      // if !self.sink.empty() {
      //   self.sink.stop();
      // };
      // self.sink.append(source);
      // println!("well");
      // self.sink.sleep_until_end();
      // println!("well");
      Ok(true)
    }

    fn resolve(self, env: Env, _output: Self::Output) -> Result<Self::JsValue> {
      env.get_boolean(true)
    }
}

#[js_function(1)]
fn play(ctx: CallContext) -> Result<JsObject> {
  let path = ctx.get::<JsString>(0)?.into_utf8()?;
  // let (_stream, stream_handle) = OutputStream::try_default().unwrap();
  // let sink = Sink::try_new(&stream_handle).unwrap();
  let task = PlayerTask {
    path: path.as_str()?.to_string(),
    // sink
  };
  let async_task = ctx.env.spawn(task)?;
  Ok(async_task.promise_object())
}