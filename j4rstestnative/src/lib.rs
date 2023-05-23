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
    let test_dto = TestDto10 {
        number0: 3333,
        str0: String::from("ok"),
        flag0: true,
        fraction0: 2.0,
        array0: vec![1, 2, 3],
        number1: 3333,
        str1: String::from("ok"),
        flag1: true,
        fraction1: 2.0,
        array1: vec![1, 2, 3],
        number2: 3333,
        str2: String::from("ok"),
        flag2: true,
        fraction2: 2.0,
        array2: vec![1, 2, 3],
        number3: 3333,
        str3: String::from("ok"),
        flag3: true,
        fraction3: 2.0,
        array3: vec![1, 2, 3],
        number4: 3333,
        str4: String::from("ok"),
        flag4: true,
        fraction4: 2.0,
        array4: vec![1, 2, 3],
        number5: 3333,
        str5: String::from("ok"),
        flag5: true,
        fraction5: 2.0,
        array5: vec![1, 2, 3],
        number6: 3333,
        str6: String::from("ok"),
        flag6: true,
        fraction6: 2.0,
        array6: vec![1, 2, 3],
        number7: 3333,
        str7: String::from("ok"),
        flag7: true,
        fraction7: 2.0,
        array7: vec![1, 2, 3],
        number8: 3333,
        str8: String::from("ok"),
        flag8: true,
        fraction8: 2.0,
        array8: vec![1, 2, 3],
        number9: 3333,
        str9: String::from("ok"),
        flag9: true,
        fraction9: 2.0,
        array9: vec![1, 2, 3],

        // anumber0: 3333,
        // astr0: String::from("ok"),
        // aflag0: true,
        // afraction0: 2.0,
        // aarray0: vec![1, 2, 3],
        // anumber1: 3333,
        // astr1: String::from("ok"),
        // aflag1: true,
        // afraction1: 2.0,
        // aarray1: vec![1, 2, 3],
        // anumber2: 3333,
        // astr2: String::from("ok"),
        // aflag2: true,
        // afraction2: 2.0,
        // aarray2: vec![1, 2, 3],
        // anumber3: 3333,
        // astr3: String::from("ok"),
        // aflag3: true,
        // afraction3: 2.0,
        // aarray3: vec![1, 2, 3],
        // anumber4: 3333,
        // astr4: String::from("ok"),
        // aflag4: true,
        // afraction4: 2.0,
        // aarray4: vec![1, 2, 3],
        // anumber5: 3333,
        // astr5: String::from("ok"),
        // aflag5: true,
        // afraction5: 2.0,
        // aarray5: vec![1, 2, 3],
        // anumber6: 3333,
        // astr6: String::from("ok"),
        // aflag6: true,
        // afraction6: 2.0,
        // aarray6: vec![1, 2, 3],
        // anumber7: 3333,
        // astr7: String::from("ok"),
        // aflag7: true,
        // afraction7: 2.0,
        // aarray7: vec![1, 2, 3],
        // anumber8: 3333,
        // astr8: String::from("ok"),
        // aflag8: true,
        // afraction8: 2.0,
        // aarray8: vec![1, 2, 3],
        // anumber9: 3333,
        // astr9: String::from("ok"),
        // aflag9: true,
        // afraction9: 2.0,
        // aarray9: vec![1, 2, 3],
        //
        // bnumber0: 3333,
        // bstr0: String::from("ok"),
        // bflag0: true,
        // bfraction0: 2.0,
        // barray0: vec![1, 2, 3],
        // bnumber1: 3333,
        // bstr1: String::from("ok"),
        // bflag1: true,
        // bfraction1: 2.0,
        // barray1: vec![1, 2, 3],
        // bnumber2: 3333,
        // bstr2: String::from("ok"),
        // bflag2: true,
        // bfraction2: 2.0,
        // barray2: vec![1, 2, 3],
        // bnumber3: 3333,
        // bstr3: String::from("ok"),
        // bflag3: true,
        // bfraction3: 2.0,
        // barray3: vec![1, 2, 3],
        // bnumber4: 3333,
        // bstr4: String::from("ok"),
        // bflag4: true,
        // bfraction4: 2.0,
        // barray4: vec![1, 2, 3],
        // bnumber5: 3333,
        // bstr5: String::from("ok"),
        // bflag5: true,
        // bfraction5: 2.0,
        // barray5: vec![1, 2, 3],
        // bnumber6: 3333,
        // bstr6: String::from("ok"),
        // bflag6: true,
        // bfraction6: 2.0,
        // barray6: vec![1, 2, 3],
        // bnumber7: 3333,
        // bstr7: String::from("ok"),
        // bflag7: true,
        // bfraction7: 2.0,
        // barray7: vec![1, 2, 3],
        // bnumber8: 3333,
        // bstr8: String::from("ok"),
        // bflag8: true,
        // bfraction8: 2.0,
        // barray8: vec![1, 2, 3],
        // bnumber9: 3333,
        // bstr9: String::from("ok"),
        // bflag9: true,
        // bfraction9: 2.0,
        // barray9: vec![1, 2, 3],
        child: None,
    };
    let ia = InvocationArg::new(&test_dto, "com.example.j4rstest.api.dto.TestDto0");
    Instance::try_from(ia).map_err(|error| format!("{}", error))
}
/*
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
*/

#[derive(Deserialize, Serialize, Debug)]
struct TestDto10 {
    number0: i32,
    str0: String,
    flag0: bool,
    fraction0: f64,
    array0: Vec<i32>,

    number1: i32,
    str1: String,
    flag1: bool,
    fraction1: f64,
    array1: Vec<i32>,

    number2: i32,
    str2: String,
    flag2: bool,
    fraction2: f64,
    array2: Vec<i32>,

    number3: i32,
    str3: String,
    flag3: bool,
    fraction3: f64,
    array3: Vec<i32>,

    number4: i32,
    str4: String,
    flag4: bool,
    fraction4: f64,
    array4: Vec<i32>,

    number5: i32,
    str5: String,
    flag5: bool,
    fraction5: f64,
    array5: Vec<i32>,

    number6: i32,
    str6: String,
    flag6: bool,
    fraction6: f64,
    array6: Vec<i32>,

    number7: i32,
    str7: String,
    flag7: bool,
    fraction7: f64,
    array7: Vec<i32>,

    number8: i32,
    str8: String,
    flag8: bool,
    fraction8: f64,
    array8: Vec<i32>,

    number9: i32,
    str9: String,
    flag9: bool,
    fraction9: f64,
    array9: Vec<i32>,

    /*anumber0: i32,
    astr0: String,
    aflag0: bool,
    afraction0: f64,
    aarray0: Vec<i32>,

    anumber1: i32,
    astr1: String,
    aflag1: bool,
    afraction1: f64,
    aarray1: Vec<i32>,

    anumber2: i32,
    astr2: String,
    aflag2: bool,
    afraction2: f64,
    aarray2: Vec<i32>,

    anumber3: i32,
    astr3: String,
    aflag3: bool,
    afraction3: f64,
    aarray3: Vec<i32>,

    anumber4: i32,
    astr4: String,
    aflag4: bool,
    afraction4: f64,
    aarray4: Vec<i32>,

    anumber5: i32,
    astr5: String,
    aflag5: bool,
    afraction5: f64,
    aarray5: Vec<i32>,

    anumber6: i32,
    astr6: String,
    aflag6: bool,
    afraction6: f64,
    aarray6: Vec<i32>,

    anumber7: i32,
    astr7: String,
    aflag7: bool,
    afraction7: f64,
    aarray7: Vec<i32>,

    anumber8: i32,
    astr8: String,
    aflag8: bool,
    afraction8: f64,
    aarray8: Vec<i32>,

    anumber9: i32,
    astr9: String,
    aflag9: bool,
    afraction9: f64,
    aarray9: Vec<i32>,

    bnumber0: i32,
    bstr0: String,
    bflag0: bool,
    bfraction0: f64,
    barray0: Vec<i32>,

    bnumber1: i32,
    bstr1: String,
    bflag1: bool,
    bfraction1: f64,
    barray1: Vec<i32>,

    bnumber2: i32,
    bstr2: String,
    bflag2: bool,
    bfraction2: f64,
    barray2: Vec<i32>,

    bnumber3: i32,
    bstr3: String,
    bflag3: bool,
    bfraction3: f64,
    barray3: Vec<i32>,

    bnumber4: i32,
    bstr4: String,
    bflag4: bool,
    bfraction4: f64,
    barray4: Vec<i32>,

    bnumber5: i32,
    bstr5: String,
    bflag5: bool,
    bfraction5: f64,
    barray5: Vec<i32>,

    bnumber6: i32,
    bstr6: String,
    bflag6: bool,
    bfraction6: f64,
    barray6: Vec<i32>,

    bnumber7: i32,
    bstr7: String,
    bflag7: bool,
    bfraction7: f64,
    barray7: Vec<i32>,

    bnumber8: i32,
    bstr8: String,
    bflag8: bool,
    bfraction8: f64,
    barray8: Vec<i32>,

    bnumber9: i32,
    bstr9: String,
    bflag9: bool,
    bfraction9: f64,
    barray9: Vec<i32>,*/
    child: Option<TestDto11>,
}

#[derive(Deserialize, Serialize, Debug)]
struct TestDto11 {
    number0: i32,
    str0: String,
    flag0: bool,
    fraction0: f64,
    array0: Vec<i32>,

    number1: i32,
    str1: String,
    flag1: bool,
    fraction1: f64,
    array1: Vec<i32>,

    number2: i32,
    str2: String,
    flag2: bool,
    fraction2: f64,
    array2: Vec<i32>,

    number3: i32,
    str3: String,
    flag3: bool,
    fraction3: f64,
    array3: Vec<i32>,

    number4: i32,
    str4: String,
    flag4: bool,
    fraction4: f64,
    array4: Vec<i32>,

    number5: i32,
    str5: String,
    flag5: bool,
    fraction5: f64,
    array5: Vec<i32>,

    number6: i32,
    str6: String,
    flag6: bool,
    fraction6: f64,
    array6: Vec<i32>,

    number7: i32,
    str7: String,
    flag7: bool,
    fraction7: f64,
    array7: Vec<i32>,

    number8: i32,
    str8: String,
    flag8: bool,
    fraction8: f64,
    array8: Vec<i32>,

    number9: i32,
    str9: String,
    flag9: bool,
    fraction9: f64,
    array9: Vec<i32>,

    child: Option<TestDto12>,
}

#[derive(Deserialize, Serialize, Debug)]
struct TestDto12 {
    number0: i32,
    str0: String,
    flag0: bool,
    fraction0: f64,
    array0: Vec<i32>,

    number1: i32,
    str1: String,
    flag1: bool,
    fraction1: f64,
    array1: Vec<i32>,

    number2: i32,
    str2: String,
    flag2: bool,
    fraction2: f64,
    array2: Vec<i32>,

    number3: i32,
    str3: String,
    flag3: bool,
    fraction3: f64,
    array3: Vec<i32>,

    number4: i32,
    str4: String,
    flag4: bool,
    fraction4: f64,
    array4: Vec<i32>,

    number5: i32,
    str5: String,
    flag5: bool,
    fraction5: f64,
    array5: Vec<i32>,

    number6: i32,
    str6: String,
    flag6: bool,
    fraction6: f64,
    array6: Vec<i32>,

    number7: i32,
    str7: String,
    flag7: bool,
    fraction7: f64,
    array7: Vec<i32>,

    number8: i32,
    str8: String,
    flag8: bool,
    fraction8: f64,
    array8: Vec<i32>,

    number9: i32,
    str9: String,
    flag9: bool,
    fraction9: f64,
    array9: Vec<i32>,

    child: Option<TestDto13>,
}


#[derive(Deserialize, Serialize, Debug)]
struct TestDto13 {
    number0: i32,
    str0: String,
    flag0: bool,
    fraction0: f64,
    array0: Vec<i32>,

    number1: i32,
    str1: String,
    flag1: bool,
    fraction1: f64,
    array1: Vec<i32>,

    number2: i32,
    str2: String,
    flag2: bool,
    fraction2: f64,
    array2: Vec<i32>,

    number3: i32,
    str3: String,
    flag3: bool,
    fraction3: f64,
    array3: Vec<i32>,

    number4: i32,
    str4: String,
    flag4: bool,
    fraction4: f64,
    array4: Vec<i32>,

    number5: i32,
    str5: String,
    flag5: bool,
    fraction5: f64,
    array5: Vec<i32>,

    number6: i32,
    str6: String,
    flag6: bool,
    fraction6: f64,
    array6: Vec<i32>,

    number7: i32,
    str7: String,
    flag7: bool,
    fraction7: f64,
    array7: Vec<i32>,

    number8: i32,
    str8: String,
    flag8: bool,
    fraction8: f64,
    array8: Vec<i32>,

    number9: i32,
    str9: String,
    flag9: bool,
    fraction9: f64,
    array9: Vec<i32>,

    child: Option<TestDto14>,
}

#[derive(Deserialize, Serialize, Debug)]
struct TestDto14 {
    number0: i32,
    str0: String,
    flag0: bool,
    fraction0: f64,
    array0: Vec<i32>,

    number1: i32,
    str1: String,
    flag1: bool,
    fraction1: f64,
    array1: Vec<i32>,

    number2: i32,
    str2: String,
    flag2: bool,
    fraction2: f64,
    array2: Vec<i32>,

    number3: i32,
    str3: String,
    flag3: bool,
    fraction3: f64,
    array3: Vec<i32>,

    number4: i32,
    str4: String,
    flag4: bool,
    fraction4: f64,
    array4: Vec<i32>,

    number5: i32,
    str5: String,
    flag5: bool,
    fraction5: f64,
    array5: Vec<i32>,

    number6: i32,
    str6: String,
    flag6: bool,
    fraction6: f64,
    array6: Vec<i32>,

    number7: i32,
    str7: String,
    flag7: bool,
    fraction7: f64,
    array7: Vec<i32>,

    number8: i32,
    str8: String,
    flag8: bool,
    fraction8: f64,
    array8: Vec<i32>,

    number9: i32,
    str9: String,
    flag9: bool,
    fraction9: f64,
    array9: Vec<i32>,

    child: Option<TestDto15>,
}

#[derive(Deserialize, Serialize, Debug)]
struct TestDto15 {
    number0: i32,
    str0: String,
    flag0: bool,
    fraction0: f64,
    array0: Vec<i32>,

    number1: i32,
    str1: String,
    flag1: bool,
    fraction1: f64,
    array1: Vec<i32>,

    number2: i32,
    str2: String,
    flag2: bool,
    fraction2: f64,
    array2: Vec<i32>,

    number3: i32,
    str3: String,
    flag3: bool,
    fraction3: f64,
    array3: Vec<i32>,

    number4: i32,
    str4: String,
    flag4: bool,
    fraction4: f64,
    array4: Vec<i32>,

    number5: i32,
    str5: String,
    flag5: bool,
    fraction5: f64,
    array5: Vec<i32>,

    number6: i32,
    str6: String,
    flag6: bool,
    fraction6: f64,
    array6: Vec<i32>,

    number7: i32,
    str7: String,
    flag7: bool,
    fraction7: f64,
    array7: Vec<i32>,

    number8: i32,
    str8: String,
    flag8: bool,
    fraction8: f64,
    array8: Vec<i32>,

    number9: i32,
    str9: String,
    flag9: bool,
    fraction9: f64,
    array9: Vec<i32>,

    child: Option<TestDto16>,
}

#[derive(Deserialize, Serialize, Debug)]
struct TestDto16 {
    number0: i32,
    str0: String,
    flag0: bool,
    fraction0: f64,
    array0: Vec<i32>,

    number1: i32,
    str1: String,
    flag1: bool,
    fraction1: f64,
    array1: Vec<i32>,

    number2: i32,
    str2: String,
    flag2: bool,
    fraction2: f64,
    array2: Vec<i32>,

    number3: i32,
    str3: String,
    flag3: bool,
    fraction3: f64,
    array3: Vec<i32>,

    number4: i32,
    str4: String,
    flag4: bool,
    fraction4: f64,
    array4: Vec<i32>,

    number5: i32,
    str5: String,
    flag5: bool,
    fraction5: f64,
    array5: Vec<i32>,

    number6: i32,
    str6: String,
    flag6: bool,
    fraction6: f64,
    array6: Vec<i32>,

    number7: i32,
    str7: String,
    flag7: bool,
    fraction7: f64,
    array7: Vec<i32>,

    number8: i32,
    str8: String,
    flag8: bool,
    fraction8: f64,
    array8: Vec<i32>,

    number9: i32,
    str9: String,
    flag9: bool,
    fraction9: f64,
    array9: Vec<i32>,

    child: Option<TestDto17>,
}

#[derive(Deserialize, Serialize, Debug)]
struct TestDto17 {
    number0: i32,
    str0: String,
    flag0: bool,
    fraction0: f64,
    array0: Vec<i32>,

    number1: i32,
    str1: String,
    flag1: bool,
    fraction1: f64,
    array1: Vec<i32>,

    number2: i32,
    str2: String,
    flag2: bool,
    fraction2: f64,
    array2: Vec<i32>,

    number3: i32,
    str3: String,
    flag3: bool,
    fraction3: f64,
    array3: Vec<i32>,

    number4: i32,
    str4: String,
    flag4: bool,
    fraction4: f64,
    array4: Vec<i32>,

    number5: i32,
    str5: String,
    flag5: bool,
    fraction5: f64,
    array5: Vec<i32>,

    number6: i32,
    str6: String,
    flag6: bool,
    fraction6: f64,
    array6: Vec<i32>,

    number7: i32,
    str7: String,
    flag7: bool,
    fraction7: f64,
    array7: Vec<i32>,

    number8: i32,
    str8: String,
    flag8: bool,
    fraction8: f64,
    array8: Vec<i32>,

    number9: i32,
    str9: String,
    flag9: bool,
    fraction9: f64,
    array9: Vec<i32>,

    child: Option<TestDto18>,
}

#[derive(Deserialize, Serialize, Debug)]
struct TestDto18 {
    number0: i32,
    str0: String,
    flag0: bool,
    fraction0: f64,
    array0: Vec<i32>,

    number1: i32,
    str1: String,
    flag1: bool,
    fraction1: f64,
    array1: Vec<i32>,

    number2: i32,
    str2: String,
    flag2: bool,
    fraction2: f64,
    array2: Vec<i32>,

    number3: i32,
    str3: String,
    flag3: bool,
    fraction3: f64,
    array3: Vec<i32>,

    number4: i32,
    str4: String,
    flag4: bool,
    fraction4: f64,
    array4: Vec<i32>,

    number5: i32,
    str5: String,
    flag5: bool,
    fraction5: f64,
    array5: Vec<i32>,

    number6: i32,
    str6: String,
    flag6: bool,
    fraction6: f64,
    array6: Vec<i32>,

    number7: i32,
    str7: String,
    flag7: bool,
    fraction7: f64,
    array7: Vec<i32>,

    number8: i32,
    str8: String,
    flag8: bool,
    fraction8: f64,
    array8: Vec<i32>,

    number9: i32,
    str9: String,
    flag9: bool,
    fraction9: f64,
    array9: Vec<i32>,

    child: Option<TestDto19>,
}

#[derive(Deserialize, Serialize, Debug)]
struct TestDto19 {
    number0: i32,
    str0: String,
    flag0: bool,
    fraction0: f64,
    array0: Vec<i32>,

    number1: i32,
    str1: String,
    flag1: bool,
    fraction1: f64,
    array1: Vec<i32>,

    number2: i32,
    str2: String,
    flag2: bool,
    fraction2: f64,
    array2: Vec<i32>,

    number3: i32,
    str3: String,
    flag3: bool,
    fraction3: f64,
    array3: Vec<i32>,

    number4: i32,
    str4: String,
    flag4: bool,
    fraction4: f64,
    array4: Vec<i32>,

    number5: i32,
    str5: String,
    flag5: bool,
    fraction5: f64,
    array5: Vec<i32>,

    number6: i32,
    str6: String,
    flag6: bool,
    fraction6: f64,
    array6: Vec<i32>,

    number7: i32,
    str7: String,
    flag7: bool,
    fraction7: f64,
    array7: Vec<i32>,

    number8: i32,
    str8: String,
    flag8: bool,
    fraction8: f64,
    array8: Vec<i32>,

    number9: i32,
    str9: String,
    flag9: bool,
    fraction9: f64,
    array9: Vec<i32>,
}
// endregion*/
/*
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
// endregion*/