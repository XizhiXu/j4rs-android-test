use j4rs::InvocationArg;
use j4rs::jni_sys::{JavaVM, jint, JNI_VERSION_1_6};
use j4rs::prelude::*;
use j4rs_derive::*;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[no_mangle]
pub extern fn JNI_OnLoad(env: *mut JavaVM, _reserved: jobject) -> jint {
    j4rs::set_java_vm(env);
    JNI_VERSION_1_6
}

#[call_from_java("com.example.j4rstest.RustFunctionCalls.fnstring")]
fn my_function_with_1_string_arg(i1: Instance) -> Result<Instance, String> {
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let s: String = jvm.to_rust(i1).unwrap();
    println!("A String Instance was passed to Rust: {}", s);
    let ia = InvocationArg::try_from(format!("Rust received {}", s)).unwrap();
    Instance::try_from(ia).map_err(|error| format!("{}", error))
}

#[call_from_java("com.example.j4rstest.RustFunctionCalls.addintegers")]
fn add_integers(integer_instance1: Instance, integer_instance2: Instance) -> Result<Instance, String> {
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let i1: i32 = jvm.to_rust(integer_instance1).unwrap();
    let i2: i32 = jvm.to_rust(integer_instance2).unwrap();
    let sum = i1 + i2;
    let ia = InvocationArg::try_from(sum).map_err(|error| format!("{}", error)).unwrap();
    Instance::try_from(ia).map_err(|error| format!("{}", error))
}

#[call_from_java("com.example.j4rstest.RustFunctionCalls.addlongs")]
fn add_longs(instance1: Instance, instance2: Instance) -> Result<Instance, String> {
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let i1: i64 = jvm.to_rust(instance1).unwrap();
    let i2: i64 = jvm.to_rust(instance2).unwrap();
    let sum = i1 + i2;
    let ia = InvocationArg::try_from(sum).map_err(|error| format!("{}", error)).unwrap();
    Instance::try_from(ia).map_err(|error| format!("{}", error))
}

// region dto1
#[call_from_java("com.example.j4rstest.RustFunctionCalls.fncustomobject10")]
fn use_custom_object1_0(i: Instance) -> Result<Instance, String> {
    println!("---------------------IN RUST");
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let test_dto: TestDto10 = jvm.to_rust(i).unwrap();
    println!("{:?}", test_dto);
    let test_dto = TestDto10 {number: 3333, str: String::from("ok"), flag: true, fraction: 2.0, array: vec![1, 2, 3]};
    let ia = InvocationArg::new(&test_dto, "com.example.j4rstest.api.dto.TestDto0");
    Instance::try_from(ia).map_err(|error| format!("{}", error))
}

#[call_from_java("com.example.j4rstest.RustFunctionCalls.fncustomobject11")]
fn use_custom_object1_1(i: Instance) -> Result<Instance, String> {
    println!("---------------------IN RUST");
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let test_dto: TestDto11 = jvm.to_rust(i).unwrap();
    println!("{:?}", test_dto);
    let test_dto = TestDto11 {number: 3333, str: String::from("ok"), flag: true, fraction: 2.0, array: vec![1, 2, 3]};
    let ia = InvocationArg::new(&test_dto, "com.example.j4rstest.api.dto.TestDto1");
    Instance::try_from(ia).map_err(|error| format!("{}", error))
}

#[call_from_java("com.example.j4rstest.RustFunctionCalls.fncustomobject12")]
fn use_custom_object1_2(i: Instance) -> Result<Instance, String> {
    println!("---------------------IN RUST");
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let test_dto: TestDto12 = jvm.to_rust(i).unwrap();
    println!("{:?}", test_dto);
    let test_dto = TestDto12 {number: 3333, str: String::from("ok"), flag: true, fraction: 2.0, array: vec![1, 2, 3]};
    let ia = InvocationArg::new(&test_dto, "com.example.j4rstest.api.dto.TestDto2");
    Instance::try_from(ia).map_err(|error| format!("{}", error))
}

#[call_from_java("com.example.j4rstest.RustFunctionCalls.fncustomobject13")]
fn use_custom_object1_3(i: Instance) -> Result<Instance, String> {
    println!("---------------------IN RUST");
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let test_dto: TestDto13 = jvm.to_rust(i).unwrap();
    println!("{:?}", test_dto);
    let test_dto = TestDto13 {number: 3333, str: String::from("ok"), flag: true, fraction: 2.0, array: vec![1, 2, 3]};
    let ia = InvocationArg::new(&test_dto, "com.example.j4rstest.api.dto.TestDto3");
    Instance::try_from(ia).map_err(|error| format!("{}", error))
}

#[call_from_java("com.example.j4rstest.RustFunctionCalls.fncustomobject14")]
fn use_custom_object1_4(i: Instance) -> Result<Instance, String> {
    println!("---------------------IN RUST");
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let test_dto: TestDto14 = jvm.to_rust(i).unwrap();
    println!("{:?}", test_dto);
    let test_dto = TestDto14 {number: 3333, str: String::from("ok"), flag: true, fraction: 2.0, array: vec![1, 2, 3]};
    let ia = InvocationArg::new(&test_dto, "com.example.j4rstest.api.dto.TestDto4");
    Instance::try_from(ia).map_err(|error| format!("{}", error))
}

#[call_from_java("com.example.j4rstest.RustFunctionCalls.fncustomobject15")]
fn use_custom_object1_5(i: Instance) -> Result<Instance, String> {
    println!("---------------------IN RUST");
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let test_dto: TestDto15 = jvm.to_rust(i).unwrap();
    println!("{:?}", test_dto);
    let test_dto = TestDto15 {number: 3333, str: String::from("ok"), flag: true, fraction: 2.0, array: vec![1, 2, 3]};
    let ia = InvocationArg::new(&test_dto, "com.example.j4rstest.api.dto.TestDto5");
    Instance::try_from(ia).map_err(|error| format!("{}", error))
}

#[call_from_java("com.example.j4rstest.RustFunctionCalls.fncustomobject16")]
fn use_custom_object1_6(i: Instance) -> Result<Instance, String> {
    println!("---------------------IN RUST");
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let test_dto: TestDto16 = jvm.to_rust(i).unwrap();
    println!("{:?}", test_dto);
    let test_dto = TestDto16 {number: 3333, str: String::from("ok"), flag: true, fraction: 2.0, array: vec![1, 2, 3]};
    let ia = InvocationArg::new(&test_dto, "com.example.j4rstest.api.dto.TestDto6");
    Instance::try_from(ia).map_err(|error| format!("{}", error))
}

#[call_from_java("com.example.j4rstest.RustFunctionCalls.fncustomobject17")]
fn use_custom_object1_7(i: Instance) -> Result<Instance, String> {
    println!("---------------------IN RUST");
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let test_dto: TestDto17 = jvm.to_rust(i).unwrap();
    println!("{:?}", test_dto);
    let test_dto = TestDto17 {number: 3333, str: String::from("ok"), flag: true, fraction: 2.0, array: vec![1, 2, 3]};
    let ia = InvocationArg::new(&test_dto, "com.example.j4rstest.api.dto.TestDto7");
    Instance::try_from(ia).map_err(|error| format!("{}", error))
}

#[call_from_java("com.example.j4rstest.RustFunctionCalls.fncustomobject18")]
fn use_custom_object1_8(i: Instance) -> Result<Instance, String> {
    println!("---------------------IN RUST");
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let test_dto: TestDto18 = jvm.to_rust(i).unwrap();
    println!("{:?}", test_dto);
    let test_dto = TestDto18 {number: 3333, str: String::from("ok"), flag: true, fraction: 2.0, array: vec![1, 2, 3]};
    let ia = InvocationArg::new(&test_dto, "com.example.j4rstest.api.dto.TestDto8");
    Instance::try_from(ia).map_err(|error| format!("{}", error))
}

#[call_from_java("com.example.j4rstest.RustFunctionCalls.fncustomobject19")]
fn use_custom_object1_9(i: Instance) -> Result<Instance, String> {
    println!("---------------------IN RUST");
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let test_dto: TestDto19 = jvm.to_rust(i).unwrap();
    println!("{:?}", test_dto);
    let test_dto = TestDto19 {number: 3333, str: String::from("ok"), flag: true, fraction: 2.0, array: vec![1, 2, 3]};
    let ia = InvocationArg::new(&test_dto, "com.example.j4rstest.api.dto.TestDto9");
    Instance::try_from(ia).map_err(|error| format!("{}", error))
}

#[derive(Deserialize, Serialize, Debug)]
struct TestDto10 {
    number: i32,
    str: String,
    flag: bool,
    fraction: f64,
    array: Vec<i32>
}

#[derive(Deserialize, Serialize, Debug)]
struct TestDto11 {
    number: i32,
    str: String,
    flag: bool,
    fraction: f64,
    array: Vec<i32>
}

#[derive(Deserialize, Serialize, Debug)]
struct TestDto12 {
    number: i32,
    str: String,
    flag: bool,
    fraction: f64,
    array: Vec<i32>
}

#[derive(Deserialize, Serialize, Debug)]
struct TestDto13 {
    number: i32,
    str: String,
    flag: bool,
    fraction: f64,
    array: Vec<i32>
}

#[derive(Deserialize, Serialize, Debug)]
struct TestDto14 {
    number: i32,
    str: String,
    flag: bool,
    fraction: f64,
    array: Vec<i32>
}

#[derive(Deserialize, Serialize, Debug)]
struct TestDto15 {
    number: i32,
    str: String,
    flag: bool,
    fraction: f64,
    array: Vec<i32>
}

#[derive(Deserialize, Serialize, Debug)]
struct TestDto16 {
    number: i32,
    str: String,
    flag: bool,
    fraction: f64,
    array: Vec<i32>
}

#[derive(Deserialize, Serialize, Debug)]
struct TestDto17 {
    number: i32,
    str: String,
    flag: bool,
    fraction: f64,
    array: Vec<i32>
}

#[derive(Deserialize, Serialize, Debug)]
struct TestDto18 {
    number: i32,
    str: String,
    flag: bool,
    fraction: f64,
    array: Vec<i32>
}

#[derive(Deserialize, Serialize, Debug)]
struct TestDto19 {
    number: i32,
    str: String,
    flag: bool,
    fraction: f64,
    array: Vec<i32>
}
// endregion

// region dto2
#[call_from_java("com.example.j4rstest.RustFunctionCalls.fncustomobject20")]
fn use_custom_object2_0(i: Instance) -> Result<Instance, String> {
    println!("---------------------IN RUST");
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let test_dto: TestDto20 = jvm.to_rust(i).unwrap();
    println!("{:?}", test_dto);
    let test_dto = TestDto20 {number: 3333, str: String::from("ok"), flag: true, fraction: 2.0, array: vec![1, 2, 3]};
    let ia = InvocationArg::new(&test_dto, "com.example.j4rstest.api.dto2.TestDto0");
    Instance::try_from(ia).map_err(|error| format!("{}", error))
}

#[call_from_java("com.example.j4rstest.RustFunctionCalls.fncustomobject21")]
fn use_custom_object2_1(i: Instance) -> Result<Instance, String> {
    println!("---------------------IN RUST");
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let test_dto: TestDto21 = jvm.to_rust(i).unwrap();
    println!("{:?}", test_dto);
    let test_dto = TestDto21 {number: 3333, str: String::from("ok"), flag: true, fraction: 2.0, array: vec![1, 2, 3]};
    let ia = InvocationArg::new(&test_dto, "com.example.j4rstest.api.dto2.TestDto1");
    Instance::try_from(ia).map_err(|error| format!("{}", error))
}

#[call_from_java("com.example.j4rstest.RustFunctionCalls.fncustomobject22")]
fn use_custom_object2_2(i: Instance) -> Result<Instance, String> {
    println!("---------------------IN RUST");
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let test_dto: TestDto22 = jvm.to_rust(i).unwrap();
    println!("{:?}", test_dto);
    let test_dto = TestDto22 {number: 3333, str: String::from("ok"), flag: true, fraction: 2.0, array: vec![1, 2, 3]};
    let ia = InvocationArg::new(&test_dto, "com.example.j4rstest.api.dto2.TestDto2");
    Instance::try_from(ia).map_err(|error| format!("{}", error))
}

#[call_from_java("com.example.j4rstest.RustFunctionCalls.fncustomobject23")]
fn use_custom_object2_3(i: Instance) -> Result<Instance, String> {
    println!("---------------------IN RUST");
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let test_dto: TestDto23 = jvm.to_rust(i).unwrap();
    println!("{:?}", test_dto);
    let test_dto = TestDto23 {number: 3333, str: String::from("ok"), flag: true, fraction: 2.0, array: vec![1, 2, 3]};
    let ia = InvocationArg::new(&test_dto, "com.example.j4rstest.api.dto2.TestDto3");
    Instance::try_from(ia).map_err(|error| format!("{}", error))
}

#[call_from_java("com.example.j4rstest.RustFunctionCalls.fncustomobject24")]
fn use_custom_object2_4(i: Instance) -> Result<Instance, String> {
    println!("---------------------IN RUST");
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let test_dto: TestDto24 = jvm.to_rust(i).unwrap();
    println!("{:?}", test_dto);
    let test_dto = TestDto24 {number: 3333, str: String::from("ok"), flag: true, fraction: 2.0, array: vec![1, 2, 3]};
    let ia = InvocationArg::new(&test_dto, "com.example.j4rstest.api.dto2.TestDto4");
    Instance::try_from(ia).map_err(|error| format!("{}", error))
}

#[call_from_java("com.example.j4rstest.RustFunctionCalls.fncustomobject25")]
fn use_custom_object2_5(i: Instance) -> Result<Instance, String> {
    println!("---------------------IN RUST");
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let test_dto: TestDto25 = jvm.to_rust(i).unwrap();
    println!("{:?}", test_dto);
    let test_dto = TestDto25 {number: 3333, str: String::from("ok"), flag: true, fraction: 2.0, array: vec![1, 2, 3]};
    let ia = InvocationArg::new(&test_dto, "com.example.j4rstest.api.dto2.TestDto5");
    Instance::try_from(ia).map_err(|error| format!("{}", error))
}

#[call_from_java("com.example.j4rstest.RustFunctionCalls.fncustomobject26")]
fn use_custom_object2_6(i: Instance) -> Result<Instance, String> {
    println!("---------------------IN RUST");
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let test_dto: TestDto26 = jvm.to_rust(i).unwrap();
    println!("{:?}", test_dto);
    let test_dto = TestDto26 {number: 3333, str: String::from("ok"), flag: true, fraction: 2.0, array: vec![1, 2, 3]};
    let ia = InvocationArg::new(&test_dto, "com.example.j4rstest.api.dto2.TestDto6");
    Instance::try_from(ia).map_err(|error| format!("{}", error))
}

#[call_from_java("com.example.j4rstest.RustFunctionCalls.fncustomobject27")]
fn use_custom_object2_7(i: Instance) -> Result<Instance, String> {
    println!("---------------------IN RUST");
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let test_dto: TestDto27 = jvm.to_rust(i).unwrap();
    println!("{:?}", test_dto);
    let test_dto = TestDto27 {number: 3333, str: String::from("ok"), flag: true, fraction: 2.0, array: vec![1, 2, 3]};
    let ia = InvocationArg::new(&test_dto, "com.example.j4rstest.api.dto2.TestDto7");
    Instance::try_from(ia).map_err(|error| format!("{}", error))
}

#[call_from_java("com.example.j4rstest.RustFunctionCalls.fncustomobject28")]
fn use_custom_object2_8(i: Instance) -> Result<Instance, String> {
    println!("---------------------IN RUST");
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let test_dto: TestDto28 = jvm.to_rust(i).unwrap();
    println!("{:?}", test_dto);
    let test_dto = TestDto28 {number: 3333, str: String::from("ok"), flag: true, fraction: 2.0, array: vec![1, 2, 3]};
    let ia = InvocationArg::new(&test_dto, "com.example.j4rstest.api.dto2.TestDto8");
    Instance::try_from(ia).map_err(|error| format!("{}", error))
}

#[call_from_java("com.example.j4rstest.RustFunctionCalls.fncustomobject29")]
fn use_custom_object2_9(i: Instance) -> Result<Instance, String> {
    println!("---------------------IN RUST");
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let test_dto: TestDto29 = jvm.to_rust(i).unwrap();
    println!("{:?}", test_dto);
    let test_dto = TestDto29 {number: 3333, str: String::from("ok"), flag: true, fraction: 2.0, array: vec![1, 2, 3]};
    let ia = InvocationArg::new(&test_dto, "com.example.j4rstest.api.dto2.TestDto9");
    Instance::try_from(ia).map_err(|error| format!("{}", error))
}

#[derive(Deserialize, Serialize, Debug)]
struct TestDto20 {
    number: i32,
    str: String,
    flag: bool,
    fraction: f64,
    array: Vec<i32>
}

#[derive(Deserialize, Serialize, Debug)]
struct TestDto21 {
    number: i32,
    str: String,
    flag: bool,
    fraction: f64,
    array: Vec<i32>
}

#[derive(Deserialize, Serialize, Debug)]
struct TestDto22 {
    number: i32,
    str: String,
    flag: bool,
    fraction: f64,
    array: Vec<i32>
}

#[derive(Deserialize, Serialize, Debug)]
struct TestDto23 {
    number: i32,
    str: String,
    flag: bool,
    fraction: f64,
    array: Vec<i32>
}

#[derive(Deserialize, Serialize, Debug)]
struct TestDto24 {
    number: i32,
    str: String,
    flag: bool,
    fraction: f64,
    array: Vec<i32>
}

#[derive(Deserialize, Serialize, Debug)]
struct TestDto25 {
    number: i32,
    str: String,
    flag: bool,
    fraction: f64,
    array: Vec<i32>
}

#[derive(Deserialize, Serialize, Debug)]
struct TestDto26 {
    number: i32,
    str: String,
    flag: bool,
    fraction: f64,
    array: Vec<i32>
}

#[derive(Deserialize, Serialize, Debug)]
struct TestDto27 {
    number: i32,
    str: String,
    flag: bool,
    fraction: f64,
    array: Vec<i32>
}

#[derive(Deserialize, Serialize, Debug)]
struct TestDto28 {
    number: i32,
    str: String,
    flag: bool,
    fraction: f64,
    array: Vec<i32>
}

#[derive(Deserialize, Serialize, Debug)]
struct TestDto29 {
    number: i32,
    str: String,
    flag: bool,
    fraction: f64,
    array: Vec<i32>
}
// endregion

// region dto3
#[call_from_java("com.example.j4rstest.RustFunctionCalls.fncustomobject30")]
fn use_custom_object3_0(i: Instance) -> Result<Instance, String> {
    println!("---------------------IN RUST");
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let test_dto: TestDto30 = jvm.to_rust(i).unwrap();
    println!("{:?}", test_dto);
    let test_dto = TestDto30 {number: 3333, str: String::from("ok"), flag: true, fraction: 2.0, array: vec![1, 2, 3]};
    let ia = InvocationArg::new(&test_dto, "com.example.j4rstest.api.dto3.TestDto0");
    Instance::try_from(ia).map_err(|error| format!("{}", error))
}

#[call_from_java("com.example.j4rstest.RustFunctionCalls.fncustomobject31")]
fn use_custom_object3_1(i: Instance) -> Result<Instance, String> {
    println!("---------------------IN RUST");
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let test_dto: TestDto31 = jvm.to_rust(i).unwrap();
    println!("{:?}", test_dto);
    let test_dto = TestDto31 {number: 3333, str: String::from("ok"), flag: true, fraction: 2.0, array: vec![1, 2, 3]};
    let ia = InvocationArg::new(&test_dto, "com.example.j4rstest.api.dto3.TestDto1");
    Instance::try_from(ia).map_err(|error| format!("{}", error))
}

#[call_from_java("com.example.j4rstest.RustFunctionCalls.fncustomobject32")]
fn use_custom_object3_2(i: Instance) -> Result<Instance, String> {
    println!("---------------------IN RUST");
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let test_dto: TestDto32 = jvm.to_rust(i).unwrap();
    println!("{:?}", test_dto);
    let test_dto = TestDto32 {number: 3333, str: String::from("ok"), flag: true, fraction: 2.0, array: vec![1, 2, 3]};
    let ia = InvocationArg::new(&test_dto, "com.example.j4rstest.api.dto3.TestDto2");
    Instance::try_from(ia).map_err(|error| format!("{}", error))
}

#[call_from_java("com.example.j4rstest.RustFunctionCalls.fncustomobject33")]
fn use_custom_object3_3(i: Instance) -> Result<Instance, String> {
    println!("---------------------IN RUST");
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let test_dto: TestDto33 = jvm.to_rust(i).unwrap();
    println!("{:?}", test_dto);
    let test_dto = TestDto33 {number: 3333, str: String::from("ok"), flag: true, fraction: 2.0, array: vec![1, 2, 3]};
    let ia = InvocationArg::new(&test_dto, "com.example.j4rstest.api.dto3.TestDto3");
    Instance::try_from(ia).map_err(|error| format!("{}", error))
}

#[call_from_java("com.example.j4rstest.RustFunctionCalls.fncustomobject34")]
fn use_custom_object3_4(i: Instance) -> Result<Instance, String> {
    println!("---------------------IN RUST");
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let test_dto: TestDto34 = jvm.to_rust(i).unwrap();
    println!("{:?}", test_dto);
    let test_dto = TestDto34 {number: 3333, str: String::from("ok"), flag: true, fraction: 2.0, array: vec![1, 2, 3]};
    let ia = InvocationArg::new(&test_dto, "com.example.j4rstest.api.dto3.TestDto4");
    Instance::try_from(ia).map_err(|error| format!("{}", error))
}

#[call_from_java("com.example.j4rstest.RustFunctionCalls.fncustomobject35")]
fn use_custom_object3_5(i: Instance) -> Result<Instance, String> {
    println!("---------------------IN RUST");
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let test_dto: TestDto35 = jvm.to_rust(i).unwrap();
    println!("{:?}", test_dto);
    let test_dto = TestDto35 {number: 3333, str: String::from("ok"), flag: true, fraction: 2.0, array: vec![1, 2, 3]};
    let ia = InvocationArg::new(&test_dto, "com.example.j4rstest.api.dto3.TestDto5");
    Instance::try_from(ia).map_err(|error| format!("{}", error))
}

#[call_from_java("com.example.j4rstest.RustFunctionCalls.fncustomobject36")]
fn use_custom_object3_6(i: Instance) -> Result<Instance, String> {
    println!("---------------------IN RUST");
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let test_dto: TestDto36 = jvm.to_rust(i).unwrap();
    println!("{:?}", test_dto);
    let test_dto = TestDto36 {number: 3333, str: String::from("ok"), flag: true, fraction: 2.0, array: vec![1, 2, 3]};
    let ia = InvocationArg::new(&test_dto, "com.example.j4rstest.api.dto3.TestDto6");
    Instance::try_from(ia).map_err(|error| format!("{}", error))
}

#[call_from_java("com.example.j4rstest.RustFunctionCalls.fncustomobject37")]
fn use_custom_object3_7(i: Instance) -> Result<Instance, String> {
    println!("---------------------IN RUST");
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let test_dto: TestDto37 = jvm.to_rust(i).unwrap();
    println!("{:?}", test_dto);
    let test_dto = TestDto37 {number: 3333, str: String::from("ok"), flag: true, fraction: 2.0, array: vec![1, 2, 3]};
    let ia = InvocationArg::new(&test_dto, "com.example.j4rstest.api.dto3.TestDto7");
    Instance::try_from(ia).map_err(|error| format!("{}", error))
}

#[call_from_java("com.example.j4rstest.RustFunctionCalls.fncustomobject38")]
fn use_custom_object3_8(i: Instance) -> Result<Instance, String> {
    println!("---------------------IN RUST");
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let test_dto: TestDto38 = jvm.to_rust(i).unwrap();
    println!("{:?}", test_dto);
    let test_dto = TestDto38 {number: 3333, str: String::from("ok"), flag: true, fraction: 2.0, array: vec![1, 2, 3]};
    let ia = InvocationArg::new(&test_dto, "com.example.j4rstest.api.dto3.TestDto8");
    Instance::try_from(ia).map_err(|error| format!("{}", error))
}

#[call_from_java("com.example.j4rstest.RustFunctionCalls.fncustomobject39")]
fn use_custom_object3_9(i: Instance) -> Result<Instance, String> {
    println!("---------------------IN RUST");
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let test_dto: TestDto39 = jvm.to_rust(i).unwrap();
    println!("{:?}", test_dto);
    let test_dto = TestDto39 {number: 3333, str: String::from("ok"), flag: true, fraction: 2.0, array: vec![1, 2, 3]};
    let ia = InvocationArg::new(&test_dto, "com.example.j4rstest.api.dto3.TestDto9");
    Instance::try_from(ia).map_err(|error| format!("{}", error))
}

#[derive(Deserialize, Serialize, Debug)]
struct TestDto30 {
    number: i32,
    str: String,
    flag: bool,
    fraction: f64,
    array: Vec<i32>
}

#[derive(Deserialize, Serialize, Debug)]
struct TestDto31 {
    number: i32,
    str: String,
    flag: bool,
    fraction: f64,
    array: Vec<i32>
}

#[derive(Deserialize, Serialize, Debug)]
struct TestDto32 {
    number: i32,
    str: String,
    flag: bool,
    fraction: f64,
    array: Vec<i32>
}

#[derive(Deserialize, Serialize, Debug)]
struct TestDto33 {
    number: i32,
    str: String,
    flag: bool,
    fraction: f64,
    array: Vec<i32>
}

#[derive(Deserialize, Serialize, Debug)]
struct TestDto34 {
    number: i32,
    str: String,
    flag: bool,
    fraction: f64,
    array: Vec<i32>
}

#[derive(Deserialize, Serialize, Debug)]
struct TestDto35 {
    number: i32,
    str: String,
    flag: bool,
    fraction: f64,
    array: Vec<i32>
}

#[derive(Deserialize, Serialize, Debug)]
struct TestDto36 {
    number: i32,
    str: String,
    flag: bool,
    fraction: f64,
    array: Vec<i32>
}

#[derive(Deserialize, Serialize, Debug)]
struct TestDto37 {
    number: i32,
    str: String,
    flag: bool,
    fraction: f64,
    array: Vec<i32>
}

#[derive(Deserialize, Serialize, Debug)]
struct TestDto38 {
    number: i32,
    str: String,
    flag: bool,
    fraction: f64,
    array: Vec<i32>
}

#[derive(Deserialize, Serialize, Debug)]
struct TestDto39 {
    number: i32,
    str: String,
    flag: bool,
    fraction: f64,
    array: Vec<i32>
}
// endregion